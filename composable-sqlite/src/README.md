# Composable-SQLite

The main feature of this crate is to report COMMIT and ROLLBACK events on an SQLite database. This reporting is down per SQL query; only COMMITs or ROLLBACKs or data relevant to that query will be reported.

These notifications are exposed as an enum of events

```rust
#[derive(Copy, Clone)]
pub enum Event {
    Commit,
    Rollback,
}
```

that are sent via an `futures::Stream`.

```rust
impl Stream<Item = Event>
```




This feature is enabled by default, but may be explicitly enable by

```toml
composable-sqlite = { version = "*", features = ["commit-notifiations"] }
```



## Requirements

The determination of the dependencies of a given SQL query of down via SQLite’s [Compile-Time Authorization Callback](https://sqlite.org/c3ref/set_authorizer.html)  API. As such, an application that needs to use that functionality itself, cannot take advantage of this feature.

Similarly, [Data Change Notification Callbacks](https://sqlite.org/c3ref/update_hook.html) are needed to track what tables are being modified to then send the appropriate notifications. Applications that need to use this functionality also cannot use this feature.

### SQLite Limitations

> ### [Data Change Notification Callbacks](https://sqlite.org/c3ref/update_hook.html)
>
> The update hook is not invoked when internal system tables are modified (i.e. sqlite_sequence). The update hook is not invoked when [WITHOUT ROWID](https://sqlite.org/withoutrowid.html) tables are modified.
>
> In the current implementation, the update hook is not invoked when conflicting rows are deleted because of an [ON CONFLICT REPLACE](https://sqlite.org/lang_conflict.html) clause.  Nor is the update hook invoked when rows are deleted using the [truncate optimization](https://sqlite.org/lang_delete.html#truncateopt). 
>
> > #### [The Truncate Optimization](https://sqlite.org/lang_delete.html#the_truncate_optimization)
> >
> > When the WHERE clause and RETURNING clause are both  omitted from a DELETE statement and the table being deleted has no triggers, SQLite uses an optimization to erase the entire table content without having to visit each row of the table individually. This "truncate" optimization makes the delete run much faster.



## Other features

The other, optional features, are

-  `checked-sql`
- `compressed-db`
  - [zeekstd](https://lib.rs/crates/zeekstd), 
  - …an SQLite [VFS](https://sqlite.org/vfs.html) to load…
    - …can be [ATTACH](https://sqlite.org/lang_attach.html)ed to the application’s main database…
    - …or `bootstrap_from` can be used to create an initial application database from this, compressed, 
- `serde` allows query results to be expanded into any `serde` compatible `struct`.