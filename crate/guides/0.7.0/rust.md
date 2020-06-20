# Rust

> A language empowering everyone
to build reliable and efficient software. [[rust-lang.org]](https://www.rust-lang.org/)

I think the most important Rust features are:
  - [Statically typed](https://stackoverflow.com/questions/1517582/what-is-the-difference-between-statically-typed-and-dynamically-typed-languages/1517670#1517670).
  - Very fast, efficient and safe.
  - You can write both low-level code and high-level abstraction. 
  - Cross-platform.
  - Pragmatic - it's designed to solve real-world problems.
  - Good official [formatter](https://github.com/rust-lang/rustfmt) and [linter](https://github.com/rust-lang/rust-clippy).
  - See also other opinions - [Why the developers who use Rust love it so much](https://stackoverflow.blog/2020/06/05/why-the-developers-who-use-rust-love-it-so-much/)


## Official resources

- Website: [rust-lang.org](https://www.rust-lang.org/)
- Repository: [github.com/rust-lang/rust](https://github.com/rust-lang/rust)
- Playground: [play.rust-lang.org](https://play.rust-lang.org/)
- Learning resources: [rust-lang.org/learn](https://www.rust-lang.org/learn)

## Seed-related notes

 - [IMPORTANT] Debug builds are much bigger and slower. However they contain debug info and their compilation is much faster.
   - _Note:_ Chrome is better for debugging; Firefox is faster.

 - [IMPORTANT] Some crates (e.g. [url](https://crates.io/crates/url)) and all crates that use them may even double the Seed app size.

 - [IMPORTANT] Some crates are not WASM-compatible or require to enable their additional [features](https://doc.rust-lang.org/cargo/reference/features.html). Consult their docs when you encounter compilation problems.
 
 - Don't try to learn and understand all Rust features and concepts at once. I recommend to follow guides in next chapters - they contain Rust notes, recommendations and links to learning materials.
 
 - Seed API is designed to be as simple and readable as possible => You don't have to be Rust expert to read and write Seed apps.
 
 - Even if you are a complete beginner, don't hesitate to write feedback because we can be blind to some API issues, strange behavior or missing documentation.

## How to write in Rust

<details>
<summary>23 General Rules & Recommendations</summary>

1. Rust compiler is your friend.

1. [Document](https://doc.rust-lang.org/book/ch14-02-publishing-to-crates-io.html#making-useful-documentation-comments) your code, write expressive names, read docs; Testable code snippets in comments are nice.

1. Learn where to use [Result](https://doc.rust-lang.org/std/result/) and where [panic](https://doc.rust-lang.org/book/ch09-03-to-panic-or-not-to-panic.html#to-panic-or-not-to-panic).

1. Respect [naming conventions](https://rust-lang.github.io/api-guidelines/naming.html).

1. Learn about the famous couples:
    - [From](https://doc.rust-lang.org/std/convert/trait.From.html) and [Into](https://doc.rust-lang.org/std/convert/trait.Into.html) + [TryFrom](https://doc.rust-lang.org/std/convert/trait.TryFrom.html) and [TryInto](https://doc.rust-lang.org/std/convert/trait.TryInto.html)
    - [String](https://doc.rust-lang.org/std/string/struct.String.html) and [str](https://doc.rust-lang.org/std/primitive.str.html)
    - [Display](https://doc.rust-lang.org/std/fmt/trait.Display.html) and [ToString](https://doc.rust-lang.org/std/string/trait.ToString.html)
    - [fn](https://doc.rust-lang.org/std/primitive.fn.html) and [Fn](https://doc.rust-lang.org/std/ops/trait.Fn.html)
    - [FromStr](https://doc.rust-lang.org/std/str/trait.FromStr.html) and [parse](https://doc.rust-lang.org/std/primitive.str.html#method.parse)
    - [Rc](https://doc.rust-lang.org/std/rc/struct.Rc.html) and [RefCell](https://doc.rust-lang.org/stable/std/cell/struct.RefCell.html)
    - [Vec](https://doc.rust-lang.org/std/vec/struct.Vec.html) and [vec!](https://doc.rust-lang.org/std/macro.vec.html)

1. Use [early returns](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#recoverable-errors-with-result) where possible.

1. Don't use one programming paradigm for everything. Learn where to use [loops](https://doc.rust-lang.org/book/ch03-05-control-flow.html#repetition-with-loops) and where to use [Iterator](https://doc.rust-lang.org/std/iter/index.html)s.

1. Use [references](https://doc.rust-lang.org/std/primitive.reference.html) where possible - e.g. rather use [&str](https://doc.rust-lang.org/std/primitive.str.html) instead of [String](https://doc.rust-lang.org/std/string/struct.String.html); or [&[T]](https://doc.rust-lang.org/std/primitive.slice.html) instead of [Vec](https://doc.rust-lang.org/std/vec/struct.Vec.html).

1. Try to write minimum `clone` calls. When you need to use cheap `clone`, try to make it explicit - e.g. `Rc::clone(&value)` [[rc docs]](https://doc.rust-lang.org/std/rc/index.html)

1. One of the Rust features is safe mutability but use it only when best practices for writing [immutable variables](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html#variables-and-mutability) and [pure functions](https://en.wikipedia.org/wiki/Pure_function) make your code unreadable, too slow or error-prone.

1. *"Premature optimization is the root of all evil"* - especially in Rust because it's one of the fastest language. Don't do anything extra until your benchmarks are ready to test it. Personal examples:
   
   - When I was writing proxy server in Rust, there were two things that slowed down that proxy multiple times - forgotten `println` calls in the hot path and slow DNS server... I recommend to zoom-out and fix higher-level issues first.
   
   - Seed VDOM patching algorithm was fast enough on the first attempt. However it's heavily slowed down by DOM calls. I recommend to look at IO and external dependencies before you try to optimize your Rust code.

1. Write only cross-platform code and use only Rust tools.

1. Don't be afraid to write [async](https://rust-lang.github.io/async-book/01_getting_started/01_chapter.html) code.

1. Learn about the popular libraries like:
    - [serde](https://crates.io/crates/serde)
    - [rand](https://crates.io/crates/rand) 
    - [futures](https://crates.io/crates/futures)
    - [strum](https://crates.io/crates/strum)
    - [once_cell](https://crates.io/crates/once_cell)
    - [itertools](https://crates.io/crates/itertools)
    - [indexmap](https://crates.io/crates/indexmap)
    - [chrono](https://crates.io/crates/chrono)
    - [uuid](https://crates.io/crates/uuid) + [ulid](https://crates.io/crates/ulid) + [cuid](https://crates.io/crates/cuid)
    - [rayon](https://crates.io/crates/rayon)

1. [Clippy](https://github.com/rust-lang/rust-clippy) and [rustfmt](https://github.com/rust-lang/rustfmt) are also your friends. [cargo-make](https://sagiegurari.github.io/cargo-make/) is your unofficial friend. 
   - You can run command `cargo make verify` in almost all Seed/my projects. It formats code, lints it by pedantic `Clippy` and tests it. See the [task definition](https://github.com/seed-rs/seed-quickstart/blob/8c5807721e2e67d12e3f93533ebb75b871203800/Makefile.toml#L22-L24) in Rust quickstart.

1. Write [tests](https://doc.rust-lang.org/book/ch11-01-writing-tests.html) and benchmarks (see e.g. [Criterion.rs](https://bheisler.github.io/criterion.rs/book/criterion_rs.html)).

1. Experiment with function parameter types below to find out where there are useful (_Note:_ All of them are used on multiple places in the [Seed repo](https://github.com/seed-rs/seed)):
    - `fn(text: impl AsRef<str>)` - [AsRef](https://doc.rust-lang.org/std/convert/trait.AsRef.html)
    - `fn(text: impl ToString)` - [ToString](https://doc.rust-lang.org/std/string/trait.ToString.html)
    - `fn(text: impl Into<Cow<'static, str>>)` - [Cow](https://doc.rust-lang.org/std/borrow/enum.Cow.html)
    - `fn<'a>(text: impl Into<Cow<'a, str>>)`

1. Once in a while:
    - Run `rustup update` to update your compiler and tools like `Clippy`. 
    - Delete `target` folder (it's something like `node_modules`) in your projects to save some space on your disk.

1. Look at this [mem](https://doc.rust-lang.org/std/mem/index.html) functions:
    - [discriminant](https://doc.rust-lang.org/std/mem/fn.discriminant.html)
    - [drop](https://doc.rust-lang.org/std/mem/fn.drop.html)
    - [replace](https://doc.rust-lang.org/std/mem/fn.replace.html)
    - [swap](https://doc.rust-lang.org/std/mem/fn.swap.html)
    - [take](https://doc.rust-lang.org/std/mem/fn.take.html)

1. Use [unsafe](https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html?unsafe-rust) Rust only for special cases or when it's necessary for your domain (you write operating systems, super fast low-level libraries, etc.) There is only safe code in Seed projects (including Seed's core).

1. Use "magic" like [Any](https://doc.rust-lang.org/std/any/trait.Any.html) only to improve public API for your users. It always makes the code worse.

1. Learn to use channels. Docs for standard ones and the ones in some crates:
    - [std::sync::mpsc::channel](https://doc.rust-lang.org/std/sync/mpsc/fn.channel.html)
    - [futures::channel](https://docs.rs/futures/0.3.5/futures/channel/index.html)
    - [tokio::sync::mpsc::channel](https://docs.rs/tokio/0.2.21/tokio/sync/mpsc/fn.channel.html)
    - [crossbeam::channel](https://docs.rs/crossbeam/0.7.3/crossbeam/channel/index.html)
    - [flume](https://docs.rs/flume/0.7.1/flume/)

1. Write and use [macros](https://doc.rust-lang.org/book/ch19-06-macros.html#macros) only if it's really necessary and document them properly. There are many footguns. And IDEs often fight with them - e.g. autocomplete often doesn't work.

    - The exceptions are macros like `println`, `vec`, `include_str`, etc. - see [all standard macros](https://doc.rust-lang.org/std/index.html#macros).

    - Yes, there are many macros in Seed, but the most of them are used only as an alternative to HTML and are pretty short. And we fixed many bugs inside them already so the rule still applies. We decided to used them after considering many trade-offs.

    - However macros are useful where:
        - There is missing abstraction - e.g. macro [stop_and_prevent ](https://github.com/seed-rs/seed/blob/3134d21c6fcb2383685885687fe2a7610fb2ff74/examples/drop_zone/src/lib.rs#L89-L97) in `drop_zone` example.

        - It helps with readability a lot - e.g. macros [create_t](https://github.com/seed-rs/seed/blob/29666287eaf5e914c80e9fae7cc6736cd31ce087/examples/i18n/src/i18n.rs#L90-L131) and [t](https://github.com/seed-rs/seed/blob/29666287eaf5e914c80e9fae7cc6736cd31ce087/examples/i18n/src/i18n.rs#L116-L127) in `i18n` example.

        - It's hard/impossible to encode everything by proper Rust types - e.g. Seed element macros like `div!`.

        - It can hide boilerplate and where variable number of parameters is required - e.g. Seed's `log!` - it formats input parameters and calls Javascript `console.log` under the hood.

</details>

## Detailed explanations

<details>
<summary>Why is Seed written in Rust</summary>

- Performance, low memory consumption and safety are reasons why also an [operating system](https://www.redox-os.org/), embedded devices and proxy servers are written in Rust.

- One of the Rust compilation targets is [WebAssembly](https://developer.mozilla.org/en-US/docs/WebAssembly). It means very fast front-end apps. (Unfortunately there are current limitations because of missing Rust/native browser API, however it's still fast enough even for production apps.)

- There are many high-quality Rust libraries and tools - we don't have to reinvent wheels while we are developing Seed.

- Rust is [the most loved language](https://insights.stackoverflow.com/survey/2019#most-loved-dreaded-and-wanted). It attracts many skillful developers, contributors and companies.

- It's pretty easy to contribute, even into the Seed core, once you learn Rust thanks to Rust type system - we (core members) don't have to think about all bad things known from other languages - `null`s, typos, unformatted code, memory allocation problems, general anti-patterns, etc. - because we have a good compiler and linters. We can focus on business logic and style during code reviews. There are pull requests with thousands changes from the first-time contributors in the Seed repository.

- Rust and all Rust tools are cross-platform - it allows you to develop Seed apps on Mac, Windows or Linux without problems.

- Documentation is priority - official learning resources are very helpful, libraries are well documented and you can write even runnable examples in your code comments.

- There are many advanced Rust features / APIs that allow us (core developers) to design the best Seed public API for users and THEN we can implement it without problems - in other words: It allows us to hide unnecessary complexity. For instance - we are able to write [React-like Hooks](https://seed-style-hooks.netlify.app/hooks_home) without [limitations](https://reactjs.org/docs/hooks-rules.html).

- Rust has been created and is sponsored by Mozilla. Some Firefox parts are already written in Rust. Also other [big companies](https://blog.knoldus.com/some-extensive-projects-working-with-rust/) like Amazon, NPM, and Microsoft use Rust. So we expect that Rust will become even more popular and integration with browsers will become easier. And perhaps we'll see some Rust components also in Chrome - [Chromium Security article](https://www.chromium.org/Home/chromium-security/memory-safety).

</details>

<details>
<summary>Why I want only Rust</summary>

I've written commercial or hobby projects in multiple languages (Js, CoffeeScript, TS, Elm, Elixir, PHP, C, C++, C#, Go, ..). However I want to write only in Rust. 

Rust is hard to learn even (?) for experienced developers, because they have to unlearn many things and adapt thought process to Rust concepts and best practices. However once you stop fighting the compiler, Rust takes your hand and push you to correct and efficient solutions. 

I had similar feeling when I was learning to drive a car - it seems pretty hard/strange from the start but once you get used to it, you know that each control / button / pedal has it's specific place and purpose for a good reason. And it makes even more sense once you learn low-level stuff - e.g. how the transmission and a clutch work.

However steep learning curve isn't bad: 
  - It means that Rust doesn't hide real complexity behind too simple models.
  - It's almost impossible for complete beginners to publish incomplete/buggy libraries. 

Rust is designed so well that I feel nervous while I'm writing in other languages - I have to do compiler's work again in my head and think about weird things like typos in code, `null`s, `undefined`s, memory leaks, accidental mutations, how to write fast code without mutability, etc. It generates significant cognitive load so I can't focus so much on business logic and other important stuff.

I don't believe that you should use the most suitable language for specific domain or problem at all costs. I think consistency among your / company projects and simplicity should have the highest priority. And Rust is a very universal language so I think it's a good choice for almost all cases.

There are also things that should be improved (and are improving):
  1. Compilation is still slow, but it's not so frustrating now.
  1. It's not possible to compile Rust in a browser so we can't provide live examples but it should be doable once Rust compiler works in WASM.
  1. IDE support still isn't very good because of Rust complex types and macros but thanks to [Rust Analyzer](https://rust-analyzer.github.io/) it's getting better every day.
  1. Many libraries still aren't WASM/browser-friendly but it's also getting better quickly.
  1. `target` folder (it's something like `node_modules`) can be pretty big.

P.S. Write us your opinion about Rust if you are a beginner and learning it.

</details>
