# Rust

> A language empowering everyone
to build reliable and efficient software. [[rust-lang.org]](https://www.rust-lang.org/)

I think the most important Rust features are:
  - [Statically typed](https://stackoverflow.com/questions/1517582/what-is-the-difference-between-statically-typed-and-dynamically-typed-languages/1517670#1517670).
  - Very fast, efficient and safe.
  - You can write both low-level code and high-level abstraction. 
  - Cross-platform.
  - Pragmatic - it's designed to solve real-world problems.

## Official resources

- Website: [rust-lang.org](https://www.rust-lang.org/)
- Repository: [github.com/rust-lang/rust](https://github.com/rust-lang/rust)
- Playground: [play.rust-lang.org](https://play.rust-lang.org/)
- Learning resources: [rust-lang.org/learn](https://www.rust-lang.org/learn)

## Seed-related notes
 - Don't try to learn and understand all Rust features and concepts at once. I recommend to follow guides in next chapters - they contain Rust notes, recommendations and links to learning materials.
 - Seed API is designed to be as simple and readable as possible => You don't have to be Rust expert to read and write Seed apps.
 - Even if you are a complete beginner, don't hesitate to write feedback because we can be blind to some API issues, strange behavior or missing documentation.

## Detailed explanations

<details>
<summary>Why is Seed written in Rust</summary>

- Performance, low memory consuption and safety are reasons why also an [operating system](https://www.redox-os.org/), embedded devices and proxy servers are written in Rust.

- One of the Rust compilation targets is [WebAssembly](https://developer.mozilla.org/en-US/docs/WebAssembly). It means very fast front-end apps. (Unfortunatelly there are current limitations because of missing Rust/native browser API, however it's still fast enough even for production apps.)

- There are many high-quality Rust libraries and tools - we don't have to reinvent wheels while we are developing Seed.

- Rust is [the most loved language](https://insights.stackoverflow.com/survey/2019#most-loved-dreaded-and-wanted). It attracts many skillful developers, contributors and companies.

- It's pretty easy to contribute, even into the Seed core, once you learn Rust thanks to Rust type system - we (core members) don't have to think about all bad things known from other languages - `null`s, typos, unformatted code, memory allocation problems, general anti-patterns, etc. - because we have a good compiler and linters. We can focus on business logic and style during code reviews. There are pull requests with thousands changes from the first-time contributors in the Seed repository.

- Rust and all Rust tools are cross-platform - it allows you to develop Seed apps on Mac, Windows or Linux without problems.

- Documentation is priority - official learning resources are very helpful, libraries are well documented and you can write even runnable examples in your code comments.

- There are many advanced Rust features / APIs that allow us (core developers) to design the best Seed public API for users and THEN we can implement it without problems - in other words: It allows us to hide unnecessary complexity. For instance - we are able to write React-like Hooks without [limitations](https://reactjs.org/docs/hooks-rules.html).

- Rust has been created and is sponsored by Mozilla. Some Firefox parts are already written in Rust. Also other big companies like Microsoft uses Rust. So we expect that Rust will become even more popular and integration with browsers will become easier.

</details>

<details>
<summary>Why I want only Rust</summary>

I've written commercial or hobby projects in multiple languages (Js, CoffeeScript, TS, Elm, Elixir, PHP, C, C++, C#, Go, ..). However I want to write only in Rust. 

Rust is hard to learn even for experienced developers, because they have to unlearn many things and adapt thought process to Rust concepts and best practices. However once you stop fighting with the compiler, Rust takes your hand and push you to correct and efficient solutions. 

I had similar feeling when I was learning to drive a car - it seems pretty hard/strange from the start but once you get used to it, you know that each control / button / pedal has it's specific place and purpose for a good reason. And it makes even more sense once you learn low-level stuff - e.g. how the transmission and a clutch work.

However steep learning curve isn't bad - it's almost impossible for complete beginners to publish uncomplete/buggy libraries. And it means that Rust doesn't hide real complexity behind too simple models.

Rust is designed so well that I feel nervous while I'm writing in other languages - I have to do compiler's work again in my head and think about weird things like typos in code, `null`s, `undefined`s, memory leaks, accidental mutations, how to write fast code without mutability, etc. It generates significant cognitive load so I can't focus so much on business logic and other important stuff.

I don't believe that you should use the most suitable language for specific domain or problem at all costs. I think consistency among your / company projects and simplicity should have the highest priority. And Rust is a very universal language so I think it's a good choice for almost all cases.

There are also things that should be improved (and are improving):
  - Compilation is still slow, but it's not so frustrating now.
  - It's not possible to compile Rust in a browser so we can't provide live examples but it should be doable once Rust compiler works in WASM.
  - IDE support still isn't very good because of Rust complex types and macros but thanks to [Rust Analyzer](https://rust-analyzer.github.io/) it's getting better every day.
  - Many libraries still aren't WASM/browser-friendly but it's also getting better quickly.

P.S. Write us your opinion about Rust if you are a beginner and learning it.

</details>
