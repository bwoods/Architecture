# Changelog

Please keep one empty line before and after all headers. (This is required for `git` to produce a conflict when a release is made while a PR is open and the PR's changelog entry would go into the wrong section).

And please only add new entries to the top of this list, right below the `# Unreleased` header.

> The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/)/[Common Changelog](https://common-changelog.org),
> and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).



## Unreleased

### Added

### Removed

### Changed

- `View` drawing is governed by an `Output` trait; sending geometry to the GPU is now just _one_ of the options available.

### Fixed



## 0.6.0 - 2024-01-xx

### Added

- Asynchronous Effects that where removed on version 0.5 have been restored. They now run in a [Local Async Executor](https://maciej.codes/2022-06-09-local-async.html), rather than a mulit-threaded one, 

### Changed

- **Breaking:** All traits an structs have been redesigned around the `return_position_impl_trait_in_trait` feature; as it is (finally) nearing stablization.
- Document the archtecture through `doctest`s

