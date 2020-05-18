# Use

Counter example part:

```rust
// (Lines like the one below ignore selected Clippy rules
//  - it's useful when you want to check your code with `cargo make verify`
// but some rules are too "annoying" or are not applicable for your case.)
#![allow(clippy::wildcard_imports)]

use seed::{prelude::*, *};
```

## Rust keyword `use`

> `use seed::{prelude::*, *};`

- [Official docs](https://doc.rust-lang.org/book/ch07-04-bringing-paths-into-scope-with-the-use-keyword.html)

`use` makes app code more readable - imagine you would need to write something like 
```rust
div![
    "Save",
    seed::browser::dom::event_handler::ev(
        seed::dom_entity_names::events::event_names::Ev::Click,
        |_| Msg::Save
    )
]
``` 
instead of
```rust
div![
    "Save",
    ev(Ev::Click, |_| Msg::Save)
]
``` 
Fortunately, `ev` and `Ev` are "hidden" behind the star (aka glob operator / asterisk wildcard) in `prelude::*`. Module `prelude` is a standard way in Rust world to group the most used items in your library so users can import them all at once.

However [macros](https://doc.rust-lang.org/book/ch19-06-macros.html) live only in the root, so we can't import them through `prelude` - we have to include them from root => 2nd star in `use seed::{prelude::*, *};`. There are many macros in Seed library because all elements are macros - `div!`, `span!`, etc. So it's practical to include them at once, too.

(_Note:_ Don't bother to learn about `ev`, `Ev`, elements and macros now, we will look at them in other chapters.)

Seed also automatically includes some [traits](https://doc.rust-lang.org/book/ch10-02-traits.html) and macro helpers through those stars - they are needed for user comfort and performance - it would be pain to include them manually.

## Rust Attributes

> `#![allow(clippy::wildcard_imports)]`

- [Official docs](https://doc.rust-lang.org/reference/attributes.html#attributes)

[Clippy](https://github.com/rust-lang/rust-clippy) is the official Rust linter. It has many useful rules that help you to write faster and more readable code. However programming is often more art than science so Clippy is sometimes annoying and doesn't let you to write cleaner code that breaks a rule.

One of these rules is [wildcard_imports](https://rust-lang.github.io/rust-clippy/stable/index.html#wildcard_imports). It's generally a good best practice, but it fights with our `use seed::{prelude::*, *};` - and we have good reasons for these wildcards (as we explained in the previous section). So we have to disable this rule. There are multiple ways but we need only two in practice:
1. `#![allow(clippy::wildcard_imports)]` - with bang (`!`) - disable selected rule(s) for all items in the same scope / module.
1. `#[allow(clippy::wildcard_imports)]` - without bang - disable selected rule(s) only for the item below.

There are many more Rust Attributes, we will explain other ones when needed.

---

_Try:_ Remove attribute `#![allow(clippy::wildcard_imports)]` from your code in Counter example project and then run `cargo make verify` in a terminal window from the project root. 
<details>
<summary>Expected Clippy error</summary>

```bash
$ cargo make verify
[cargo-make] INFO - cargo make 0.30.7
[cargo-make] INFO - Project: app_name
[cargo-make] INFO - Build File: Makefile.toml
[cargo-make] INFO - Task: verify
[cargo-make] INFO - Profile: development
[cargo-make] INFO - Running Task: fmt
[cargo-make] INFO - Execute Command: "cargo" "fmt"
[cargo-make] INFO - Running Task: clippy
[cargo-make] INFO - Execute Command: "cargo" "clippy" "--all-features" "--" "--deny" "warnings" "--deny" "clippy::pedantic" "--deny" "clippy::nursery"
    Checking app_name v0.1.0 (C:\work\repos\seed-app-counter)
error: usage of wildcard import
 --> src\lib.rs:5:24
  |
5 | use seed::{prelude::*, *};
  |                        ^ help: try: `C, button, div`
  |
  = note: `-D clippy::wildcard-imports` implied by `-D clippy::pedantic`
  = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#wildcard_imports

error: aborting due to previous error

error: could not compile `app_name`.

To learn more, run the command again with --verbose.
[cargo-make] ERROR - Error while executing command, exit code: 101
[cargo-make] WARN - Build Failed.

```
</details>

---
