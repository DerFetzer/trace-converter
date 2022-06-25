# **trace-converter**

# Setup
- [Install Rust](https://www.rust-lang.org/tools/install).

# Rust
> Low-level code is prone to a variety of subtle bugs, which in most other languages can be caught only through extensive testing and careful code review by experienced developers.
> In Rust, the compiler plays a gatekeeper role by refusing to compile code with these elusive bugs, including concurrency bugs.
> ...
> By striving for zero-cost abstractions, higher-level features that compile to lower-level code as fast as code written manually, Rust endeavors to make safe code be fast code as well.
> 
> -- <cite>Rust Book</cite>

## Learning Resources
- [Rust by Example](https://doc.rust-lang.org/stable/rust-by-example/)
  1. [Hello World](https://doc.rust-lang.org/stable/rust-by-example/hello.html)
  2. [Primitives](https://doc.rust-lang.org/stable/rust-by-example/primitives.html)
  3. [Custom Types](https://doc.rust-lang.org/stable/rust-by-example/custom_types.html)
  4. [Flow of Control](https://doc.rust-lang.org/stable/rust-by-example/flow_control.html)
  5. [Methods](https://doc.rust-lang.org/stable/rust-by-example/fn/methods.html)
  6. [Scoping rules](https://doc.rust-lang.org/stable/rust-by-example/scope.html)
  7. [Traits](https://doc.rust-lang.org/stable/rust-by-example/trait.html)
  8. [Error handling](https://doc.rust-lang.org/stable/rust-by-example/error.html)
- [Rust Book](https://doc.rust-lang.org/stable/book/)
- [Playground](https://play.rust-lang.org/)
- [Crate registry](https://crates.io/)

# ToDo
1. Implement `converter::TraceReader::read` + tests
2. Implement `converter::Sample::to_json` + tests
3. Implement `main`