# A Composable Architecture



## Origins

This is a rust native implementation of architectural patterns that draw heavily from those found in the [Swift Composable Architecture](https://github.com/pointfreeco/swift-composable-architecture). [^docs]

[^docs]: In fact, much of the initial `rustdoc` herein was inspired by the `swiftdoc` of that project (which is released under the [MIT license](https://github.com/pointfreeco/swift-composable-architecture/blob/main/LICENSE)).

> The Composable Architecture was designed over the course of many episodes on [Point•Free](https://www.pointfree.co/), a video series exploring functional programming and the Swift language, hosted by Brandon Williams and Stephen Celis.
>
> You can watch all of the episodes [here](https://www.pointfree.co/collections/composable-architecture), as well as a dedicated, multipart tour of the architecture from scratch: [part 1](https://www.pointfree.co/collections/composable-architecture/a-tour-of-the-composable-architecture/ep100-a-tour-of-the-composable-architecture-part-1), [part 2](https://www.pointfree.co/collections/composable-architecture/a-tour-of-the-composable-architecture/ep101-a-tour-of-the-composable-architecture-part-2), [part 3](https://www.pointfree.co/collections/composable-architecture/a-tour-of-the-composable-architecture/ep102-a-tour-of-the-composable-architecture-part-3) and [part 4](https://www.pointfree.co/collections/composable-architecture/a-tour-of-the-composable-architecture/ep103-a-tour-of-the-composable-architecture-part-4).
>
> ![](https://d3rccdn33rt8ze.cloudfront.net/episodes/0069.jpeg)


The API itself has diverged to better reflect the different strengths (and weaknesses) of Rust and Swift, but the [core ideals](https://pointfreeco.github.io/swift-composable-architecture/main/documentation/composablearchitecture/) are the same:

- **State management**
- **Composition**
- **Side effects**
- **Testing**
- **Ergonomics**



## License

Distributed under the terms of both the MIT license and the Apache License (Version 2.0)

See [LICENSE-APACHE](LICENSE-APACHE.md) and [LICENSE-MIT](LICENSE-MIT.md) for details.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
