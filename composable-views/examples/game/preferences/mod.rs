use crate::versioning::{CRATE, NAME, PACKAGE};
use atomic_write_file::AtomicWriteFile;
use directories::ProjectDirs;
use fallible_iterator::{self, FallibleIterator};
use log::error;
use serde::{Deserialize, Serialize};
use std::fs::{OpenOptions, create_dir_all};
use std::io::ErrorKind::{InvalidData, InvalidInput, NotADirectory, NotFound, Unsupported};
use std::io::{self, Read, Write};
use std::marker::PhantomData;
use std::path::PathBuf;
use toml_edit::{ArrayOfTables, DocumentMut, Table, de, ser};

pub struct Preferences<'a> {
    toml: DocumentMut,
    path: PathBuf,

    marker: PhantomData<&'a str>,
}

impl<'a> Preferences<'a> {
    pub fn default() -> io::Result<Self> {
        let directories = ProjectDirs::from("", PACKAGE, CRATE).ok_or(NotADirectory)?;
        let directory = directories.config_local_dir();

        let mut path = PathBuf::new();
        path.push(directory);

        create_dir_all(&path)?;

        path.push(NAME);
        path.set_extension("toml");

        Self::new(path)
    }

    pub fn new(path: PathBuf) -> io::Result<Self> {
        let mut file = OpenOptions::new()
            .read(true)
            .create(true)
            .truncate(false)
            .write(true)
            .open(&path)?;

        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;

        let mut toml = buffer
            .parse::<DocumentMut>()
            .inspect_err(|err| error!(target: module_path!(), "{err}"))
            .map_err(|_| InvalidData)?;

        // versioning in case we need schema migration
        toml.entry("version").or_insert(1.into());

        let prefs = Self {
            toml,
            path,
            marker: Default::default(),
        };

        Ok(prefs)
    }

    fn from_table<T>(table: &Table) -> io::Result<T>
    where
        T: Deserialize<'a>,
    {
        let deserializer = table
            .to_string()
            .parse::<de::Deserializer>()
            .inspect_err(|err| error!(target: module_path!(), "{err}"))
            .map_err(|_| InvalidData)?;

        let value = T::deserialize(deserializer)
            .inspect_err(|err| error!(target: module_path!(), "{err}"))
            .map_err(|_| Unsupported)?;

        Ok(value)
    }

    pub fn get<T: Deserialize<'a>>(&self, key: &str) -> io::Result<T> {
        let table = self
            .toml
            .get(key)
            .ok_or(NotFound)?
            .as_table()
            .ok_or(InvalidInput)?;

        Self::from_table(table)
    }

    pub fn set<T: Serialize>(&mut self, key: &str, val: T) -> io::Result<()> {
        let mut val = Serialize::serialize(&val, ser::ValueSerializer::new())
            .inspect_err(|err| error!(target: module_path!(), "{err}"))
            .map_err(|_| Unsupported)?;

        // preserve existing comments and whitespace
        if let Some(decor) = self
            .toml
            .get(key)
            .and_then(|item| item.as_table())
            .map(|table| table.decor())
            .filter(|decor| decor.prefix().or(decor.suffix()).is_some())
            .cloned()
        {
            *val.decor_mut() = decor;
        }

        self.toml.insert(key, val.into());

        Ok(())
    }

    pub fn update<T: Serialize + Deserialize<'a>>(
        &mut self,
        key: &str,
        f: impl FnOnce(&mut T) -> T,
    ) -> io::Result<()> {
        let mut val = self.get(key)?;
        f(&mut val);
        self.set(key, val)
    }

    pub fn iter<T: Deserialize<'a>>(
        &self,
        key: &str,
    ) -> io::Result<impl FallibleIterator<Item = T, Error = io::Error>> {
        let tables = self
            .toml
            .get(key)
            .ok_or(NotFound)?
            .as_array_of_tables()
            .ok_or(InvalidInput)?;

        let iter = // …
            fallible_iterator::convert(tables.into_iter()
                .map(Ok::<_, io::Error>)) // FallibleIterators return Results
                .map(Self::from_table);

        Ok(iter)
    }

    pub fn replace<T: Serialize>(
        &mut self,
        key: &str,
        iter: impl Iterator<Item = T>,
    ) -> io::Result<()> {
        let mut tables = ArrayOfTables::new();

        for val in iter {
            let table = ser::to_document(&val)
                .inspect_err(|err| error!(target: module_path!(), "{err}"))
                .map_err(|_| InvalidData)?
                .into_table();

            tables.push(table)
        }

        self.toml.insert(key, tables.into());

        Ok(())
    }

    pub fn commit(&mut self) -> io::Result<()> {
        let mut file = AtomicWriteFile::open(&self.path)?;
        file.write_all(self.toml.to_string().as_bytes())?;

        file.commit()
    }
}
