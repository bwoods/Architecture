# A Composable Architecture

The Swift Composable Architecture library improved upon previous Redux-inspired patterns by leveraging the capabilies of the Swift language to acheive better **Type Safety**, **Ergonomics**, and **Performance**.

This crate attempts to do the same to The Swift Composable Architecture by leveraing the capabilities of the Rust language and ecosystem.

<details>
<summary><strong>What is the Swift Composable Architecture?</strong></summary>
<blockquote>
<p>The <a href='https://github.com/pointfreeco/swift-composable-architecture'>Composable Architecture</a> (TCA, for short) is a library for building applications in a consistent and understandable way, with composition, testing, and ergonomics in mind. It can be used in SwiftUI, UIKit, and more, and on any Apple platform (iOS, macOS, tvOS, and watchOS).</p>
<h2>Learn More</h2>
<p>The Composable Architecture was designed over the course of many episodes on <a href='https://www.pointfree.co/'>Point•Free</a>, a video series exploring functional programming and the Swift language, hosted by Brandon Williams and Stephen Celis.</p>
<p>You can watch all of the episodes <a href='https://www.pointfree.co/collections/composable-architecture'>here</a>, as well as a dedicated, multipart tour of the architecture from scratch: <a href='https://www.pointfree.co/collections/composable-architecture/a-tour-of-the-composable-architecture/ep100-a-tour-of-the-composable-architecture-part-1'>part 1</a>, <a href='https://www.pointfree.co/collections/composable-architecture/a-tour-of-the-composable-architecture/ep101-a-tour-of-the-composable-architecture-part-2'>part 2</a>, <a href='https://www.pointfree.co/collections/composable-architecture/a-tour-of-the-composable-architecture/ep102-a-tour-of-the-composable-architecture-part-3'>part 3</a> and <a href='https://www.pointfree.co/collections/composable-architecture/a-tour-of-the-composable-architecture/ep103-a-tour-of-the-composable-architecture-part-4'>part 4</a>.</p>
<p><img src="https://d3rccdn33rt8ze.cloudfront.net/episodes/0069.jpeg" referrerpolicy="no-referrer"></p>
</blockquote>
</details>

The API has diverged to better reflect the different strengths (and weaknesses) of Rust and Swift, but the [core ideals](https://pointfreeco.github.io/swift-composable-architecture/main/documentation/composablearchitecture/) are the same.

- **State management**

  Managing … Rust’s restrictions on [Variables and Mutability](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html#variables-and-mutability):

- **Composition**
- **Side effects**
- **Testing**
- **Ergonomics**



## License

Distributed under the terms of both the MIT license and the Apache License (Version 2.0)

See [LICENSE-APACHE](LICENSE-APACHE.md) and [LICENSE-MIT](LICENSE-MIT.md) for details.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.



# Why use Composable?

…



> ### Note
>
> If you have already used another unidirectional data flow architecture for application state management, the main take-away is that the State-Reducer pattern is a great fit to Rust’s restrictions on [Variables and Mutability](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html#variables-and-mutability):
>
> - Rust requires mutable references to be unique
> - State mutations may only happen within a Reducer





## Usage

To use Composable, place the following line under the `[dependencies]` section in your `Cargo.toml`:

```toml
compoable = "x.y"
```

### Optional Features

- `spin`: use spinlocks when passing data between threads; which may be more performant for certain platforms or extreme workloads.

  In fact the feature mainly exists to quickly test that doing so does not improve an application’s throiughput as the overhead of `send` is so low.
