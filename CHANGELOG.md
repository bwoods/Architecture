# Changelog

Please keep one empty line before and after all headers. (This is required for `git` to produce a conflict when a release is made while a PR is open and the PR's changelog entry would go into the wrong section).

And please only add new entries to the top of this list, right below the `# Unreleased` header.

> The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/)/[Common Changelog](https://common-changelog.org),
> and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).



## Unreleased

### Added

### Removed

### Changed

### Fixed



## 0.6.0 - 2024-07-22

### Added

- Asynchronous `Effects` that where removed on version 0.5 have been restored. They now run in a [Local Async Executor](https://maciej.codes/2022-06-09-local-async.html), rather than a mulit-threaded one, 
- The `views` feature is gated behind a new `unstable` feature flag. Development on `View`s may now continue without causing SemVar issues.  
  Unstable features **are not** considered when determining versioning.

### Removed

- `View`s are being rebuilt to better reflect the (new) separation between features:
  - `view`: the building blocks for creating UI elements around `Action`s, `State` and `Reducer`s. and
  - `default_ui`: a useful set of pre-built UI elements usable as-is, or as examples of how to build more complex elements.

### Changed

- **Breaking:** All traits and structs have been redesigned around the `return_position_impl_trait_in_trait` feature.
- `View` drawing is governed by an `Output` trait; sending geometry to the GPU is now just _one_ of the options available.
- Gesture states are now expressed as `.active`, `.hover`, `.focus` to match the W3C’s [user action pseudo classes](https://www.w3.org/TR/selectors-3/#the-user-action-pseudo-classes-hover-act) as they are more widely known than the terms used in original **Immediate-Mode Graphical User Interfaces** [presentation](https://www.youtube.com/watch?v=Z1qyvQsjK5Y&t=731s) by Casey Muratori.
- The [Inter font](https://rsms.me/inter/) has been updated to version 4.0.

### Fixed

- Better documentation and automated testing throughout.
- Shape plans are cached for text layout.

