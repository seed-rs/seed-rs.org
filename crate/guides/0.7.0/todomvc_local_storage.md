# TodoMVC - LocalStorage

Let's store & load todos from [LocalStorage](https://developer.mozilla.org/en-US/docs/Web/API/Window/localStorage)!

>### Persistence
>
>Your app should dynamically persist the todos to localStorage. If the framework has capabilities for persisting data (e.g. Backbone.sync), use that. Otherwise, use vanilla localStorage. If possible, use the keys `id`, `title`, `completed` for each item. Make sure to use this format for the localStorage name: `todos-[framework]`. Editing mode should not be persisted.

- We need a new dependency [serde](https://crates.io/crates/serde) to **ser**ialize and **de**serialize todos to/from JSON because we can store only JSON strings in `LocalStorage`.

- `serde` has [built-in support](https://github.com/serde-rs/serde/blob/3c97e1b9a989a7e9fb75b01bb026d9abfeb6311e/serde/src/ser/impls.rs#L371) for `BTreeMap` and many othe common Rust items. However containers like `BTreeMap` are de/serializable only when all its items are also de/serializable. In our case it means we need to enable `serde` support for `Ulid` and `Todo`. Fortunately [ulid](https://crates.io/crates/ulid) crate has built-in `serde` support - we just need to enable the required feature `"serde"`. You'll find available features in the crate docs or you can look at `Cargo.toml` or search through issues. Enabling `serde` support for the most custom items is easy - just derive `Deserialize` and `Serialize`.

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
