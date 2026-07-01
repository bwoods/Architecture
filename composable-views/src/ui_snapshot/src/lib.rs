use proc_macro2::Span;
use quote::ToTokens;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::path::PathBuf;
use syn::{Error, LitStr, parse_macro_input};

/// Includes a UTF-8 encoded file as a string
///
/// This macro will yield an expression of type &'static str which is the
/// contents of the file. Where this macro differs from `include_str!` is that
/// the file is assumed to begin with a YAML header that should be skipped.
///
/// The file is located relative to the current file.
#[proc_macro]
pub fn include_snapshot(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let span = Span::call_site();
    let relative = parse_macro_input!(tokens as LitStr).value();
    let absolute: PathBuf = match span
        .local_file()
        .and_then(|path| path.parent().map(|path| path.to_path_buf()))
    {
        Some(mut path) => {
            path.push(&relative);
            path
        }
        None => {
            // Do not error here: IDE expansions may not have a proper parent path set,
            // and we do not want them to predict a compiler error for the macro usage
            return LitStr::new("\u{2381}", span).into_token_stream().into();
        }
    };

    let file = match File::open(&absolute) {
        Ok(file) => file,
        Err(err) => {
            return Error::new(
                span,
                format!("error: couldn't read `{:?}`: {}", relative, err),
            )
            .into_compile_error()
            .into();
        }
    };

    let mut string = String::new();
    let mut reader = BufReader::new(file);

    let mut separators = 0; // read, and discard, the YAML header
    while reader.read_line(&mut string).unwrap() != 0 && separators != 2 {
        if string.starts_with("---") {
            separators += 1;
        }

        string.clear();
    }

    // read the rest of the file
    reader.read_to_string(&mut string).unwrap();
    LitStr::new(&string, span).into_token_stream().into()
}
