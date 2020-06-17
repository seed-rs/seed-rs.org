# TodoMVC - LocalStorage

Let's store & load todos from [LocalStorage](https://developer.mozilla.org/en-US/docs/Web/API/Window/localStorage)!

>### Persistence
>
>Your app should dynamically persist the todos to localStorage. If the framework has capabilities for persisting data (e.g. Backbone.sync), use that. Otherwise, use vanilla localStorage. If possible, use the keys `id`, `title`, `completed` for each item. Make sure to use this format for the localStorage name: `todos-[framework]`. Editing mode should not be persisted.

- We need a new dependency [serde](https://crates.io/crates/serde) to **ser**ialize and **de**serialize todos to/from JSON because we can store only JSON strings in `LocalStorage`.

- `serde` has [built-in support](https://github.com/serde-rs/serde/blob/3c97e1b9a989a7e9fb75b01bb026d9abfeb6311e/serde/src/ser/impls.rs#L371) for `BTreeMap` and many other common Rust items. However containers like `BTreeMap` are de/serializable only when all their items are also de/serializable. In our case it means we need to enable `serde` support for `Ulid` and `Todo`. Fortunately [ulid](https://crates.io/crates/ulid) crate has built-in `serde` support - we just need to enable the required _feature_ `"serde"`. You'll find available features in the crate docs or you can look at `Cargo.toml` or search through issues. Enabling `serde` support for the most custom items (like our `Todo` struct) is easy - just derive `Deserialize` and `Serialize`.

`Cargo.toml`:

```toml
[dependencies]
serde = "1.0.112"
strum = "0.18.0"
strum_macros = "0.18.0"
ulid = { version = "0.3.3", features = ["serde"]
...
```

`lib.rs`:

```rust
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;
...
const ESCAPE_KEY: &str = "Escape";

const STORAGE_KEY: &str = "todos-seed";
...

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model {
        todos: LocalStorage::get(STORAGE_KEY).unwrap_or_default(),
...

#[derive(Deserialize, Serialize)]
    struct Todo {
...

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        ...
    }
    LocalStorage::insert(STORAGE_KEY, &model.todos).expect("save todos to LocalStorage");
...
```

_Note:_ Yes, we insert todos into `LocalStorage` on each message. I don't see any performance problems like freezing UI or annoying delays during typing. So I don't want to resolve non-existent issues. And less code means less bugs. However when it becomes a problem, there are some potential solutions:

1. Update `LocalStorage` todos only in some `match` arms. 
   - It would help, but it would be error-prone - you'll forget to add the updating code in a new/updated arm sooner or later. Also it would introduce boilerplate and therefore reduce readability.

1. You can save `todos` hash into `Model` (`BTreeMap` implements [Hash](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html#impl-Hash)) and generate a new one on each message. You'll update `LocalStorage` todos only if those hashes are different. (See how to [calculate hash](https://github.com/seed-rs/seed/blob/0a538f03d6aeb56b00d997c80a666e388279a727/examples/unsaved_changes/src/lib.rs#L92-L96) in the example `unsaved_changes`.)
   - I assume that serialization, data transfer from Rust to JS world and saving into `LocalStorage` are the bottleneck. If hashing is slow, we would make it worse. It would need benchmarks.

1. Write / pick a smarter container instead of `BTreeMap` (or write a wrapper). It would allow you to implement synchronization with `LocalStorage` and mitigate problems from the solution 1).

1. Apply [debouncing or throttling](https://css-tricks.com/debouncing-throttling-explained-examples/) to `LocalStorage` updates.

1. Integrate manual saving and show something like _"Do you want to leave? Data won't be saved."_ when the user wants to leave/close browser tab (see example [unsaved_changes](https://github.com/seed-rs/seed/tree/0a538f03d6aeb56b00d997c80a666e388279a727/examples/unsaved_changes) to learn how to implement it). 
   - It would reduce UX in our TodoMVC, however there are use-cases where it would improve UX.
   - It would make the app less robust - there is a higher probability of losing changes.

1. Compress stored data. 
   1. Currently `id` is saved twice per each todo - `BTreeMap` key and `id` in `Todo` struct. We can save it as `[[id, title, completed], ..]` instead. 
   
   1. Then, we can apply a compressing algorithm and save it as a big string.

   1. It would need bechmarks to prove that additional computations mitigate slow transfer and saving.

---

I hope you are as happy as me - the app is working and `LocalStorage` integration wasn't too hard. Let's learn about routing and finish the app by proper filter implementation.


