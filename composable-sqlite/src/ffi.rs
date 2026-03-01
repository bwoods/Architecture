//! All of the unsafe code in this crate is confined to this module.
#![allow(unsafe_code)]

use super::{Event, Failed};
use flume::{Sender, unbounded};
use futures::Stream;
use sqlite3_sys::{
    SQLITE_OK, SQLITE_READ, SQLITE_TOOBIG, sqlite3, sqlite3_commit_hook, sqlite3_finalize,
    sqlite3_prepare_v3, sqlite3_rollback_hook, sqlite3_set_authorizer, sqlite3_stmt,
    sqlite3_update_hook,
};
use std::cmp::Ordering;
use std::collections::hash_set::IntoIter;
use std::collections::{BTreeMap, HashSet};
use std::ffi::{CStr, CString, c_char, c_int, c_void};
use std::hash::{BuildHasherDefault, Hash, Hasher};
use std::iter::Map;
use std::mem::take;
use std::ops::{Bound, RangeBounds};
use std::ptr::null_mut;
use std::sync::Mutex;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering::Relaxed;

pub(super) struct Commits<H> {
    changes: *mut Changes<H>,
    next_id: AtomicU64,
}

impl<H> Commits<H>
where
    H: Default + Hasher,
{
    pub fn new(db: *mut sqlite3) -> Self {
        Self {
            changes: Changes::new(db),
            next_id: AtomicU64::new(0),
        }
    }

    pub fn shrink_to_fit(&self) {
        let changes = unsafe { &*self.changes };
        changes.shrink_to_fit()
    }

    pub fn track<F, R, E>(&self, compilation: F) -> Result<(impl Stream<Item = Event>, R), E>
    where
        F: FnOnce() -> Result<R, E>,
        E: From<Failed>,
    {
        let mut dependencies = Identifiers::<H>::default();

        let changes = unsafe { &*self.changes };
        let db = changes.db;

        let status = unsafe {
            sqlite3_set_authorizer(
                db,
                Some(Self::authorization),
                (&raw mut dependencies).cast(),
            )
        };

        if status != SQLITE_OK {
            return Err(Failed::Authorizer(status).into());
        }

        let ret = compilation()?;

        unsafe {
            sqlite3_set_authorizer(db, None, null_mut());
        }

        let (send, recv) = unbounded();
        let id = self.next_id.fetch_add(1, Relaxed);

        let mut senders = changes.senders.lock().unwrap();

        for (database, table) in dependencies {
            let key = (Id::owned(database, table), id);
            senders.insert(key, send.clone());
        }

        Ok((recv.into_stream(), ret))
    }

    pub fn parse_sql(&self, sql: &str) -> Result<impl Stream<Item = Event>, Failed> {
        self.track(|| {
            let changes = unsafe { &*self.changes };
            let db = changes.db;

            if sql.len() > i32::MAX as usize {
                return Err(Failed::Prepare(SQLITE_TOOBIG));
            }

            let mut stmt: *mut sqlite3_stmt = null_mut();
            // SAFETY: sql.len() was checked above
            let status = unsafe {
                sqlite3_prepare_v3(
                    db,
                    sql.as_ptr().cast(),
                    sql.len() as i32,
                    0,
                    &mut stmt,
                    null_mut(),
                )
            };

            // SAFETY: all pointers were allocated/initialized by SQLite
            unsafe {
                sqlite3_finalize(stmt);
            }

            if status != SQLITE_OK {
                return Err(Failed::Prepare(status));
            }

            Ok(())
        })
        .map(|(stream, _)| stream)
    }

    extern "C" fn authorization(
        ctx: *mut c_void,
        op: c_int,
        table: *const c_char,
        _column: *const c_char,
        database: *const c_char,
        _view: *const c_char,
    ) -> c_int {
        if op == SQLITE_READ {
            let dependencies: &mut Identifiers<H> = unsafe { &mut *ctx.cast() };
            dependencies.insert(database, table);
        }

        SQLITE_OK
    }
}

impl<S> Drop for Commits<S> {
    fn drop(&mut self) {
        let _ = unsafe { Box::from_raw(self.changes) };
    }
}

struct Changes<H> {
    pending: Mutex<Identifiers<H>>, // tables with uncommited changes
    senders: Mutex<BTreeMap<(Id, u64), Sender<Event>>>,
    db: *mut sqlite3,
}

impl<H> Changes<H>
where
    H: Default + Hasher,
{
    pub fn new(db: *mut sqlite3) -> *mut Self {
        let ptr = Box::into_raw(Box::new(Self {
            senders: Default::default(),
            pending: Default::default(),
            db,
        }));

        let ctx = ptr.cast();
        unsafe {
            sqlite3_update_hook(db, Some(Self::update_hook), ctx);
            sqlite3_commit_hook(db, Some(Self::commit_hook), ctx);
            sqlite3_rollback_hook(db, Some(Self::rollback_hook), ctx)
        };

        ptr
    }

    fn notify_all(changes: &Changes<H>, event: Event) {
        let pending = take(&mut *changes.pending.lock().unwrap()); // unlocked immediately
        let mut senders = changes.senders.lock().unwrap();

        for (database, table) in pending {
            let range = Id::range(database, table);

            senders
                .extract_if(range, |_, sender| {
                    sender.send(event).is_err() // removes ("extracts") disconnected channels
                })
                .for_each(|_| { /* “iterators are lazy and do nothing unless consumed” */ });
        }
    }

    pub fn shrink_to_fit(&self) {
        let mut senders = self.senders.lock().unwrap();
        #[allow(clippy::bool_comparison)]
        senders.retain(|_, sender| sender.is_disconnected() == false)
    }

    extern "C" fn update_hook(
        ctx: *mut c_void,
        _op: c_int,
        database: *const c_char,
        table: *const c_char,
        _row: i64,
    ) {
        let changes: &Changes<H> = unsafe { *ctx.cast() };
        changes
            .pending
            .lock()
            .unwrap() //
            .insert(database, table);
    }

    extern "C" fn commit_hook(ctx: *mut c_void) -> c_int {
        let changes: &Changes<H> = unsafe { *ctx.cast() };
        Changes::notify_all(changes, Event::Commit);

        SQLITE_OK
    }

    extern "C" fn rollback_hook(ctx: *mut c_void) {
        let changes: &Changes<H> = unsafe { *ctx.cast() };
        Changes::notify_all(changes, Event::Rollback);
    }
}

impl<H> Drop for Changes<H> {
    fn drop(&mut self) {
        unsafe {
            sqlite3_update_hook(self.db, None, null_mut());
            sqlite3_commit_hook(self.db, None, null_mut());
            sqlite3_rollback_hook(self.db, None, null_mut());
        }
    }
}

struct Identifiers<H> {
    set: HashSet<Id, BuildHasherDefault<H>>,
}

impl<H> Identifiers<H>
where
    H: Default + Hasher,
{
    pub fn insert(&mut self, database: *const c_char, table: *const c_char) {
        // Avoid allocating a pair of CStrings for _every_ row or column…
        let key = unsafe { Id::borrowed(database, table) };

        if self.set.contains(&key) {
            return;
        }

        let database = unsafe { CStr::from_ptr(database) };
        let table = unsafe { CStr::from_ptr(table) };

        // …at the cost of a second hash probe for the _first_
        let key = Id::owned(database, table);
        self.set.insert(key);
    }
}

impl<H> Default for Identifiers<H> {
    fn default() -> Self {
        Self {
            set: HashSet::default(),
        }
    }
}

impl<H> IntoIterator for Identifiers<H> {
    type Item = (CString, CString);
    type IntoIter = Map<IntoIter<Id>, fn(Id) -> (CString, CString)>;

    fn into_iter(self) -> Self::IntoIter {
        self.set.into_iter().map(|table| table.into_tuple())
    }
}

pub(crate) enum Id {
    Owned {
        database: CString,
        table: CString,
    },
    Borrowed {
        database: *const c_char,
        table: *const c_char,
    },
}

impl Id {
    /// # Safety
    /// The `Table` constructed must not outlive the data it is being constructed from.
    pub unsafe fn borrowed(database: *const c_char, table: *const c_char) -> Self {
        Id::Borrowed { database, table }
    }

    pub fn owned(database: impl Into<CString>, table: impl Into<CString>) -> Self {
        Id::Owned {
            database: database.into(),
            table: table.into(),
        }
    }

    fn range(database: CString, table: CString) -> impl RangeBounds<(Id, u64)> {
        unsafe {
            (
                Bound::Included((Id::borrowed(database.as_ptr(), table.as_ptr()), u64::MIN)),
                Bound::Included((Id::owned(database, table), u64::MAX)),
            )
        }
    }

    fn as_tuple(&self) -> (&CStr, &CStr) {
        match self {
            Id::Owned { database, table } => (database.as_c_str(), table.as_c_str()),
            Id::Borrowed { database, table } => unsafe {
                (CStr::from_ptr(*database), CStr::from_ptr(*table))
            },
        }
    }

    fn into_tuple(self) -> (CString, CString) {
        match self {
            Id::Owned { database, table } => (database, table),
            Id::Borrowed { database, table } => unsafe {
                (
                    CStr::from_ptr(database).to_owned(),
                    CStr::from_ptr(table).to_owned(),
                )
            },
        }
    }
}

impl Hash for Id {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_tuple().hash(state)
    }
}

impl Eq for Id {}

impl PartialEq<Self> for Id {
    fn eq(&self, other: &Self) -> bool {
        self.as_tuple().eq(&other.as_tuple())
    }
}

impl Ord for Id {
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_tuple().cmp(&other.as_tuple())
    }
}

impl PartialOrd<Self> for Id {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
