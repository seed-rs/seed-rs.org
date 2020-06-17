# TodoMVC - Model

Let's look at official [specs](https://github.com/tastejs/todomvc/blob/master/app-spec.md#functionality) and try to guess what data we'll need to store in our `Model`:

---

> ### No todos
>
> When there are no todos, `#main` and `#footer` should be hidden.

We don't need to create a special value to store `#main` and `#footer` visibility - we can derive it from the number of saved todos. _#single_source_of_truth_ 

So we can just take a note that we'll need a todo container that is aware of number of its items - it should be able to told us if it's empty or not quickly because we would need the value for every render. Let's introduce the `Model` field `todos: Vec<Todo>` - it's the first and the simplest idea. We'll define `Todo` later.

---

> ### New todo
>
> New todos are entered in the input at the top of the app. The input element should be focused when the page is loaded, preferably by using the `autofocus` input attribute. Pressing Enter creates the todo, appends it to the todo list, and clears the input. Make sure to `.trim()` the input and then check that it's not empty before creating a new todo.

We will need a field to store the input value. Let's define it as `new_todo_title: String`.

---

> ### Mark all as complete
>
> This checkbox toggles all the todos to the same state as itself. Make sure to clear the checked state after the "Clear completed" button is clicked. The "Mark all as complete" checkbox should also be updated when single todo items are checked/unchecked. Eg. When all the todos are checked it should also get checked.

We don't need to introduce some complex logic if we just derive the "Mark all as complete" checkox state from todos. It means we should be able to iterate todos quickly - our `Vec<Todos>` looks like a good choice so far. And we need a field `completed: bool` in our `Todo` for sure.

---

> ### Item
> 
> A todo item has three possible interactions:
>
>1. Clicking the checkbox marks the todo as complete by updating its `completed` value and toggling the class `completed` on its parent `<li>`
>
>2. Double-clicking the `<label>` activates editing mode, by toggling the `.editing` class on its `<li>`
>
>3. Hovering over the todo shows the remove button (`.destroy`)

Now we know that we have to be able:
1. Toggle chosen todo.
1. Edit todo.
1. Remove chosen todo.

We need to distinguish individual todos somehow to know which one should be toggled/removed on click. Let's introduce field `id: ID` in the struct `Todo`. `ID` is upper-case because we don't want to decide the concrete type yet and it may stay `ID` as a type alias.

Let's discuss editing in the next section.

---

> ### Editing
>
> When editing mode is activated it will hide the other controls and bring forward an input that contains the todo title, which should be focused (`.focus()`). The edit should be saved on both blur and enter, and the `editing` class should be removed. Make sure to `.trim()` the input and then check that it's not empty. If it's empty the todo should instead be destroyed. If escape is pressed during the edit, the edit state should be left and any changes be discarded.

Important facts:

1. Either one todo or none can be selected for editing.
   - `Some/one` or `None` is basically `Option` definition. 
1. We need to focus the selected todo.
   - It's a side-effect that modifies DOM.
1. Changes may be discarded.
   - We can't modify the original todo title.

Let's introduce a `Model` field: `selected_todo: Option<SelectedTodo>` 
   - where `SelectedTodo` contains `id: ID` and `title: String`.
   - We need to add the same field `title: String` to basic `Todo` struct.

And we need to add another field `input_element: ElRef<web_sys::HtmlInputElement>` into `SelectedTodo` so we can perform side-effects safely, i.e. focus selected todo input without using error-prone JS/browser native selectors. It has to be `HtmlInputElement` and not just `Element` because only items that can be [dereferenced](https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.HtmlInputElement.html#impl-Deref) to `HtmlElement` have method [focus](https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.HtmlElement.html#method.focus). (We will talk about `ElRef` in next chapters.)

---

Our `Model` so far:

```rust
struct Model {
    todos: Vec<Todo>,
    new_todo_title: String,
    selected_todo: Option<SelectedTodo>,
}

struct Todo {
    id: ID,
    title: String,
    completed: bool,
}

struct SelectedTodo {
    id: ID,
    title: String,
    input_element: ElRef<web_sys::HtmlInputElement>,
}
```

---

> ### Counter
>
> Displays the number of active todos in a pluralized form. Make sure the number is wrapped by a `<strong>` tag. Also make sure to pluralize the `item` word correctly: `0 items`, `1 item`, `2 items`. Example: **2** items left

We can derive the number of active todos from our `todos: Vec<Todo>`. We will need to iterate our todos, but it should be fast enough.

---

> ### Clear completed button
>
> Removes completed todos when clicked. Should be hidden when there are no completed todos.

We would need to iterate our todos and remove some of them - it's not a super trivial task, it will affect our choice of the todos container and `Todo` `ID` type. 

Button visibility will be derived from todos.

---

> ### Persistence
>
> Your app should dynamically persist the todos to localStorage. If the framework has capabilities for persisting data (e.g. Backbone.sync), use that. Otherwise, use vanilla localStorage. If possible, use the keys `id`, `title`, `completed` for each item. Make sure to use this format for the localStorage name: `todos-[framework]`. Editing mode should not be persisted.

It means the part of our `Model` has to be JSON (de)serializable. The simplest way how to allow it is to just [derive `serde`'s traits](https://serde.rs/derive.html). We use only basic Rust types and some simple custom ones so serialization shouldn't force us to pick other types.

---

> ### Routing
>
> Routing is required for all implementations. If supported by the framework, use its built-in capabilities. Otherwise, use the  [Flatiron Director](https://github.com/flatiron/director) routing library located in the `/assets` folder. The following routes should be implemented: `#/` (all - default), `#/active` and `#/completed` (`#!/` is also allowed). When the route changes, the todo list should be filtered on a model level and the `selected` class on the filter links should be toggled. When an item is updated while in a filtered state, it should be updated accordingly. E.g. if the filter is `Active` and the item is checked, it should be hidden. Make sure the active filter is persisted on reload.

We have to remember which filter is selected - let's add another field into `Model`:
```rust
filter: Filter,
...
enum Filter {
   All,
   Active,
   Completed,
}
```
We will be building links - e.g. `https://example.com/seed/todomvc/#/active`. You'll learn about routing in next chapters, but we can already add `Model` field `base_url: Url`. This field will represent the url part `seed/todomvc` in case of the example url above.

---

## Model v.1

We've gone through all specifications and our `Model` looks like this:

```rust
struct Model {
    todos: Vec<Todo>,
    new_todo_title: String,
    selected_todo: Option<SelectedTodo>,
    filter: Filter,
    base_url: Url,
}

struct Todo {
    id: ID,
    title: String,
    completed: bool,
}

struct SelectedTodo {
    id: ID,
    title: String,
    input_element: ElRef<web_sys::HtmlInputElement>,
}

enum Filter {
   All,
   Active,
   Completed,
}
```

## Naming

I think all field and variant names are expressive enough - it's clear what they represent at the first glance.

I can imagine somebody would rename `new_todo_title` to `new_title` or `new_todo` but the former is too general and the latter implies that the field type is `Todo` (not `String`).

## Types

There are 3 fields representing todo title with type `String`. We can apply [newtype pattern](https://github.com/rust-unofficial/patterns/blob/master/patterns/newtype.md) and create a new struct `TodoTitle(String)`, however this `String` is the only one in the entire `Model` so the trade-off for more complex code isn't worth it. An alternative would be a type alias `type TodoTitle = String`, however type aliases often don't play well with Rust `String`s because you often [coerce](https://doc.rust-lang.org/book/ch15-02-deref.html#implicit-deref-coercions-with-functions-and-methods) them into `&str` and then aliases become cumbersome to work with - let's leave it as is until it's a problem.

There is only one `bool` (field `completed`) and it doesn't make sense to rewrite it to `enum` here. Our only `Option` (field `selected_todo`) also has a suitable type in the context of our `Model`.

### `ID` and `todos`

We need to discuss `ID` together with `todos: Vec<Todo>` because we will be doing many `todos` operations associated with chosen `ID`s. There are some known facts:
- We have to be able to remove any todo from the list;
- Push a new todo at the end;
- Filter according the `Todo` field `completed`.
- Todos have to keep it's ordering (from the oldest).

Some options:
1. todos: [BTreeMap](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html), ID: `u32` - where [u32](https://doc.rust-lang.org/std/primitive.u32.html) would be only incremented when adding a new todo.
   - There would be [tombstones](https://en.wikipedia.org/wiki/Tombstone_(data_store)) / "holes", because we have to only increment `ID` to keep ordering. We would need to remove them somehow to prevent `u32` (`usize`, ..) overflow in the distant future.

1. todos: [Vec](https://doc.rust-lang.org/std/vec/struct.Vec.html), ID: `usize` - where `ID` is todo's position in `Vec`.
   - Removing todos would change `ID`s of the remaining ones that may render "referenced" `ID` in `SelectedTodo` invalid. We would need to change `Model` and add field `selected: bool` into `Todo` but it would break the business rule that only one todo can be selected. Another way would be to implement a mechanism to update/sync the "reference" - probably with the help of [interior mutability](https://doc.rust-lang.org/book/ch15-05-interior-mutability.html) and maybe [Drop](https://doc.rust-lang.org/std/ops/trait.Drop.html).

1. todos: [Vec](https://doc.rust-lang.org/std/vec/struct.Vec.html), no ids.
   - All ids would be replaced by references with [interior mutability](https://doc.rust-lang.org/book/ch15-05-interior-mutability.html) help. It would be cool, but probably slower and complex and once you want to save todos to the server, it would be pain.

1. todos: [IndexMap](https://docs.rs/indexmap/1.4.0/indexmap/map/struct.IndexMap.html), ID: [Uuid](https://docs.rs/uuid/0.8.1/uuid/struct.Uuid.html#method.new_v4).
   - `IndexMap` is mix of `HashMap` and `Vec` - you can save key-value pairs into it, but it preserves order. It's a suitable option, but we would need to include two direct dependencies.

1. todos: [BTreeMap](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html), ID: [Ulid](https://docs.rs/ulid/0.3.2/ulid/struct.Ulid.html).
   - `Ulid` ids are unique and sortable. Their ordering depends on timestamps so it's suitable for our use-case (ordering from the oldest).

To respect [KISS principle](https://en.wikipedia.org/wiki/KISS_principle) I think we should choose either **4.** or **5.** option: 

- Both have some special features - e.g. manual reordering would be easier to implement with `IndexMap`; However to show datetime for each todo we would need just one simple call with `BTreeMap` + `Ulid`. But we shouldn't think about it too much to respect [YAGNI principle](https://en.wikipedia.org/wiki/You_aren%27t_gonna_need_it).

- `IndexMap` and `BTreemap` have different performance and memory characteristics (see also [performance](https://doc.rust-lang.org/std/collections/index.html#performance) table for `std` collections), but both are pretty fast and Rust is one of the fastest language so it shouldn't be a problem in the most cases. Also it doesn't make sense to choose the right collection from the performance / memory point of view without benchmarks, user behavior pattterns, etc. And keep in mind [_"premature optimization is the root of all evil"_](https://stackify.com/premature-optimization-evil/).

- So I would choose the option **5.**: `BTreeMap` + `Ulid`. You'll learn something about a standard Rust collection and we already have an older [TodoMVC example](https://github.com/seed-rs/seed/blob/0a538f03d6aeb56b00d997c80a666e388279a727/examples/todomvc/src/lib.rs) with `IndexMap` in the Seed repo.

---

## Model v.2

Model has been updated with `BTreeMap` and `Ulid`. We can add type alias `TodoId` for `Ulid` but it won't help too much with readability.

`v.2` is not named `final` because we never know what we'll find out during `Msg` designing and implementations.

```rust
struct Model {
    todos: BTreeMap<Ulid, Todo>,
    new_todo_title: String,
    selected_todo: Option<SelectedTodo>,
    filter: Filter,
    base_url: Url,
}

struct Todo {
    id: Ulid,
    title: String,
    completed: bool,
}

struct SelectedTodo {
    id: Ulid,
    title: String,
    input_element: ElRef<web_sys::HtmlInputElement>,
}

enum Filter {
   All,
   Active,
   Completed,
}
```

I think we've encoded business rules pretty successfully by the Rust type system. There is one exception - `SelectedTodo.id` may point to a non-existent todo, but we can't get non-existent items from `BTreeMap` like Javascript `null` or `undefined` - Rust will force us to respect the rule by returning `Option<Todo>`. 

_Tips_ 
- Try to be as expressive as possible, but also respect KISS.
- Encode ideally all business rules by the language type system.
- Developer experience is often more important than other features (e.g. performance) because it automatically means less bugs and faster development in the long run.

---

We'll design `Msg` in the next chapter and then we'll finally start writing the code to have more fun.
