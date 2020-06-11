# TodoMVC - Msg

Our `Model` is ready so we can move to `Msg` now. We'll take the similar steps - go through [specifications](https://github.com/tastejs/todomvc/blob/master/app-spec.md#functionality), write draft `Msg` and then refactor it. 

However we'll also try to imagine how each `Msg` affects our `Model`. It basically verifies our `Model` design and it'll help us to write a better `update` function later.

<details>
<summary>Current <code>Model</code></model></summary>

```rust
struct Model {
    todos: BtreeMap<Ulid, Todo>,
    new_todo_title: String,
    selected_todo: Option<SelectedTodo>,
    filter: Filter;
    base_url: Url
}

struct Todo {
    id: Ulid,
    title: String,
    completed: bool,
    element: ElRef<web_sys::HtmlElement>,
}

struct SelectedTodo {
    id: Ulid,
    title: String,
}

enum Filter {
   All,
   Active,
   Completed,
}
```

</details>

---

> ### No todos
>
> When there are no todos, `#main` and `#footer` should be hidden.

Nothing `Msg`-related here.

---

> ### New todo
>
> New todos are entered in the input at the top of the app. The input element should be focused when the page is loaded, preferably by using the `autofocus` input attribute. Pressing Enter creates the todo, appends it to the todo list, and clears the input. Make sure to `.trim()` the input and then check that it's not empty before creating a new todo.

We will need:
- `Msg::NewTodoTitleChanged(String)` to store input element content.
  - It just saves the value into `Model` field `new_todo_title`.

- `Msg::CreateTodo` to signal that user wants to push a new todo into the list.
  - We'll create a new `Todo` instance with the value in `new_todo_title` and push it into `todos`.

---

> ### Mark all as complete
>
> This checkbox toggles all the todos to the same state as itself. Make sure to clear the checked state after the "Clear completed" button is clicked. The "Mark all as complete" checkbox should also be updated when single todo items are checked/unchecked. Eg. When all the todos are checked it should also get checked.

- `Msg::CheckOrUncheckAll`
  - It affects the field `completed` in some or all `Todo`s in `todos`.

- `Msg::ClearCompleted`
  - It removes some or all todos in `todos`.

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

- `Msg::ToggleTodo(Ulid)`
  - It affects one todo in `todos` - toggles the field `completed`.

- `Msg::SelectTodo(Ulid)`
  - It affects the field `selected_todo`.

- `Msg::RemoveTodo(Ulid)`
  - It affects one todo in `todos` and maybe also `selected_todo`.

---

> ### Editing
>
> When editing mode is activated it will hide the other controls and bring forward an input that contains the todo title, which should be focused (`.focus()`). The edit should be saved on both blur and enter, and the `editing` class should be removed. Make sure to `.trim()` the input and then check that it's not empty. If it's empty the todo should instead be destroyed. If escape is pressed during the edit, the edit state should be left and any changes be discarded.

- `Msg::SelectedTodoTitleChanged(String)`
   - It stores a new `title` to `SelectedTodo`.

- `Msg::SaveSelectedTodo`
  - It "moves" `title` from `SelectedTodo` into the corresponding `Todo` in `todos`.

And we want to change `Msg::SelectTodo(Ulid)` to `Msg::SelectTodo(Option<Ulid>)`, because it's cleaner than adding something like `Msg::Deselect` and it also plays nicely with the `Model` field type `Option<SelectedTodo>`.

---

> ### Counter
>
> Displays the number of active todos in a pluralized form. Make sure the number is wrapped by a `<strong>` tag. Also make sure to pluralize the `item` word correctly: `0 items`, `1 item`, `2 items`. Example: **2** items left

Nothing interesting here.

---

> ### Clear completed button
>
> Removes completed todos when clicked. Should be hidden when there are no completed todos.

We've already added `Msg::ClearCompleted`.

---

> ### Persistence
>
> Your app should dynamically persist the todos to localStorage. If the framework has capabilities for persisting data (e.g. Backbone.sync), use that. Otherwise, use vanilla localStorage. If possible, use the keys `id`, `title`, `completed` for each item. Make sure to use this format for the localStorage name: `todos-[framework]`. Editing mode should not be persisted.

Nothing interesting here.

---

> ### Routing
>
> Routing is required for all implementations. If supported by the framework, use its built-in capabilities. Otherwise, use the  [Flatiron Director](https://github.com/flatiron/director) routing library located in the `/assets` folder. The following routes should be implemented: `#/` (all - default), `#/active` and `#/completed` (`#!/` is also allowed). When the route changes, the todo list should be filtered on a model level and the `selected` class on the filter links should be toggled. When an item is updated while in a filtered state, it should be updated accordingly. E.g. if the filter is `Active` and the item is checked, it should be hidden. Make sure the active filter is persisted on reload.

- `Msg::UrlChanged(subs::UrlChanged)`
  - When the url is changed (user clicked on a filter button or changed url directly in the browser window), we need to update the field `filter`.
  - (We'll talk about `subs` and routing in next chapters.)

---

## Msg v.1

```rust
enum Msg {
   Msg::NewTodoTitleChanged(String),
   Msg::CreateTodo,
   Msg::CheckOrUncheckAll,
   Msg::ClearCompleted,
   Msg::ToggleTodo(Ulid),
   Msg::SelectTodo(Option<Ulid>),
   Msg::RemoveTodo(Ulid),
   Msg::SelectedTodoTitleChanged(String),
   Msg::SaveSelectedTodo,
   Msg::UrlChanged(subs::UrlChanged),
}
```

## Naming

`NewTodoTitleChanged` and `SelectedTodoTitleChanged` are pretty long names but they will be used only on a few places and often on a standalone lines so the length is a good trade-off for expressiveness.

`CheckOrUncheckAll` name is a bit strange but it says exactly what it does. I was thinking also about `ToggleAll` and `CheckAll` but they would misinterpreted sooner or later.

## Types

They are 2 `String`s and 3 `Ulid`s but `String` always represent title here and `Ulid` always represents todo id so there aren't any context/domain conflicts and we can leave it as is. 

## Grouping

Our `Msg` enum is still unreadable. Let's create visual groups to improve it.
I recommend to try to write some combinations before you choose the best one. 

You can group by message type (commands vs events), by the similar name/suffix/prefix (e.g. `CreateTodo`, `ToggleTodo`), by the predicted complexity in `update` function, etc. 

Keep in mind that you'll use the same groups also in your `update` function.

I have two favorite combinations:

<details>
<summary>The first combination</summary>

```rust
enum Msg {
   UrlChanged(subs::UrlChanged),

   // ------ Title changes ------
   NewTodoTitleChanged(String),
   SelectedTodoTitleChanged(String),

   // ------ Basic Todo operations ------
   CreateTodo,
   ToggleTodo(Ulid),
   RemoveTodo(Ulid),
   SelectTodo(Option<Ulid>),
   SaveSelectedTodo,
   
   // ------ Bulk operations ------
   CheckOrUncheckAll,
   ClearCompleted,
}
```

</details>

And the second and winning combination below. 

--- 

## Msg v.2

```rust
enum Msg {
   UrlChanged(subs::UrlChanged),
   NewTodoTitleChanged(String),

   // ------ Basic Todo operations ------
   CreateTodo,
   ToggleTodo(Ulid),
   RemoveTodo(Ulid),
   
   // ------ Bulk operations ------
   CheckOrUncheckAll,
   ClearCompleted,
   
   // ------ Selection ------
   SelectTodo(Option<Ulid>),
   SelectedTodoTitleChanged(String),
   SaveSelectedTodo,
}
```

I think we've successfully covered all interactions in our app and nothing should surprise us during implementation.

---

We'll setup a new Seed project in the next chapter and integrate our `Model` and `Msg` into it.
