# TodoMVC - Routing

>### Routing
>
>Routing is required for all implementations. If supported by the framework, use its built-in capabilities. Otherwise, use the  [Flatiron Director](https://github.com/flatiron/director) routing library located in the `/assets` folder. The following routes should be implemented: `#/` (all - default), `#/active` and `#/completed` (`#!/` is also allowed). When the route changes, the todo list should be filtered on a model level and the `selected` class on the filter links should be toggled. When an item is updated while in a filtered state, it should be updated accordingly. E.g. if the filter is `Active` and the item is checked, it should be hidden. Make sure the active filter is persisted on reload.

## 1. `view` updates

Let's implement proper filter handling in our `view` functions. (We could do it in the chapter `View`, however it's better to do it now to refresh you memories - it'll help you to understand routing integration faster.)

```rust
fn view(model: &Model) -> Vec<Node<Msg>> {
    ...
            view_footer(&model.todos, model.filter),
...

fn view_main(todos: &BTreeMap<Ulid, Todo>, selected_todo: Option<&SelectedTodo>, filter: Filter) -> Node<Msg> {
    ...
        view_todo_list(todos, selected_todo, filter),
...

fn view_todo_list(todos: &BTreeMap<Ulid, Todo>, selected_todo: Option<&SelectedTodo>, filter: Filter) -> Node<Msg> {
    let todos = todos.values().filter(|todo| {
        match filter {
            Filter::All => true,
            Filter::Active => not(todo.completed),
            Filter::Completed => todo.completed,
        }
    });
    ul![C!["todo-list"],
        todos.map(|todo| {
...
```

## 2. `init` `Url`

There is a hard-coded default filter `Filter::All` in our app. Let's choose the right filter in `init` function according to the url that is present when the app is starting.

```rust
fn init(mut url: Url, _: &mut impl Orders<Msg>) -> Model {
    // TODO: Remove
    log!(url);

    let filter = match url.next_hash_path_part(){
        Some("active") => Filter::Active,
        Some("completed") => Filter::Completed,
        _ => Filter::All,
    };

    // TODO: Remove
    log!(url);

    Model {
        ...
        filter,
        ...
    }
}
```
Update the code in your app and then go to [localhost:8000/#/active](http://localhost:8000/#/active) and look at the [console log](https://developer.mozilla.org/en-US/docs/Tools/Web_Console/Opening_the_Web_Console). You should see:
```
Url {
    next_path_part_index: 0,
    next_hash_path_part_index: 0,
    path: [],
    hash_path: [
        "active",
    ],
    hash: Some(
        "/active",
    ),
    search: UrlSearch {
        search: {},
        invalid_components: [],
    },
    invalid_components: [],
}
Url {
    next_path_part_index: 0,
    next_hash_path_part_index: 1,
    path: [],
    hash_path: [
        "active",
    ],
    hash: Some(
        "/active",
    ),
    search: UrlSearch {
        search: {},
        invalid_components: [],
    },
    invalid_components: [],
}
```

Interesting parts are:
```
    next_hash_path_part_index: 0, // changed to 1
    hash_path: [
        "active",
    ],
    hash: Some(
        "/active",
    ),
```

There are two similar fields - `hash_path` and `hash`. One field - `hash` - would be enough however Seed would need to parse hash too often into `Vec` and you wouldn't be able to see parsed hash in `log!` to debug your routing. And we can't use only `hash_path` because hash is often used for non-routing stuff.

Combination `hash_path` and `next_hash_path_part_index` simulates a trivial [Iterator](https://doc.rust-lang.org/std/iter/index.html) - when you call `url.next_hash_path_part()`:
1. The _path part_ (item in `hash_path`) at the position `next_hash_path_part_index` is returned (or `None`)
2. `next_hash_path_part_index` is incremented.

Because of that incrementation, `url` has to be mutable - `fn init(mut url: Url, ...`

_Seed API design decision_: Once your app is big enough and there are many pages and nested paths, your router becomes unreadable and hard to maintain. That's the time when you want to handle each path part in the associated page and pass `url` into the nested handlers. Then you'll appreciate that `url` keeps its internal "iterator" states and you don't have to pass multiple iterators and variables through handlers by yourself.

## 3. `remaining_hash_path_parts`

Go to [localhost:8000/#/active/foo/bar](http://localhost:8000/#/active/foo/bar). `Active` filter is activated. It isn't expected behaviour in the most cases. App should show something like "404 Page Not Found" or at least ignore it to prevent future conflicts when e.g. `/active/foo` becomes a valid standalone page.

There are two solutions:
    
1. Make sure there aren't any other path parts after the first one:

    ```rust
    let filter = match url.next_hash_path_part(){
        Some("active") => {
            if url.next_hash_path_part().is_none() {
                Filter::Active
            } else {
                Filter::All
            }
        }
        ...
    ```

2. Match all path parts at once:

    ```rust
    let filter = match url.remaining_hash_path_parts().as_slice() {
        ["active"] => Filter::Active,
        ["completed"] => Filter::Completed,
        _ => Filter::All,
    };
    ```
    - `remaining_hash_path_parts()` increments `next_hash_path_part_index` in the loop until it points to a non-existent item and returns references to all iterated items. It's similar to [collect](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.collect).
        - Example: `remaining_hash_path_parts()` returns `vec!["active", "foo", "bar"]` for url `/#/active/m 0).foo/bar` if you haven't called `url.next_hash_path_part()` before (i.e. if "iterator" starts from 0).

    - `.as_slice()` call is basically a leaked implementation detail because Rust can't pattern match on `Vec` and we can't return a reference to that `Vec`, too.

    - I recommend to study Rust [pattern matching](https://doc.rust-lang.org/book/ch18-03-pattern-syntax.html#pattern-syntax). There are many useful features, see e.g.:
        ```rust
        match url.remaining_hash_path_parts().as_slice() {
            [] => Page::Home,
            ["report", rest @ ..] => {
                match rest {
                    ["day"] => Page::ReportDay,
                    _ => Page::ReportWeek,
                }
            },
            _ => Page::NotFound,
        }
        ```

P.S. You can remove both `log!(url);` now. If you can play more with `Url`, run the example [url](https://github.com/seed-rs/seed/blob/0a538f03d6aeb56b00d997c80a666e388279a727/examples/url/src/lib.rs). 

## 4. Subscriptions

Let's try to add this one line:

```rust
fn init(mut url: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.subscribe(|_: subs::UrlChanged| log!("url changed!"));
```
Update your code & refresh browser tab. Then click the filter buttons and you should see _"url changed!"_ in your console log.

Let's add another line:
```rust
orders
    .subscribe(|_: subs::UrlChanged| log!("url changed!"))
    .notify(subs::UrlChanged(url.clone()));
```
Now you don't have to even click the buttons - there is _"url changed!"_ in the console log just after the app start!

There is no magic - Seed just matches _notifications_ and _subscriptions_ by type. You can write something like:
```rust
orders
    .subscribe(|_: i32| log!("a number!"))
    .subscribe(|_: i32| log!("a number!"))
    .notify(123);
```
and it would work the same. As you can see, you can write multiple subscriptions - that's why the type of sent variables has to implement `Clone`.

Let's return to `UrlChanged`. Please change your `init` code again:
```rust
orders.subscribe(|subs::UrlChanged(url)| log!(url));
```
Rust supports pattern matching also in closure parameters. Another important fact is that this closure behaves like a regular event handler - the output value can be `()` or `Option<Msg>` or `Msg`. We can leverage that fact and write just:
```rust
orders.subscribe(Msg::UrlChanged);
```
The message will be handled by our "old" code in the `update` function:
```rust
fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::UrlChanged(subs::UrlChanged(url)) => {
            log!("UrlChanged", url);
        }
...
```

P.S. 

- If you want to play more with subscriptions, run the example [subscribe](https://github.com/seed-rs/seed/tree/d514b2131a9e94f5ffe965f3d0ac74763a11aeb6/examples/subscribe). 

- Keep in mind there is no magic - `subscribe`, `stream`, `notify`, `UrlChanged`, etc. work the same - Seed or user create a _notification_ (bascially any item) and Seed's or user's _subscriptions_ (closures) handle it.

- Notify/Subscriptions mechanism in Seed is implemented with the help of [any](https://doc.rust-lang.org/std/any/index.html).

## 5. Handle `UrlChanged`

Let's write proper `Msg::UrlChanged` handling. We'll start by copy-pasting the code from `init` function:

```rust
fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::UrlChanged(subs::UrlChanged(mut url)) => {
            model.filter = match url.remaining_hash_path_parts().as_slice() {
                ["active"] => Filter::Active,
                ["completed"] => Filter::Completed,
                _ => Filter::All,
...
```

Try to click filter buttons in your browser. Yahoo! Our app should be feature-complete now, everything else is basically refactor...

We want to respect [DRY principle](https://en.wikipedia.org/wiki/Don%27t_repeat_yourself) and idiomatic Rust so let's implement trait [From](https://doc.rust-lang.org/std/convert/trait.From.html) for `Filter` and update the code:
```rust
fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.subscribe(Msg::UrlChanged);

    Model {
        ...
        filter: Filter::from(url),
        ...
    }
}
...

// ------ Filter ------

#[derive(Copy, Clone, Eq, PartialEq, EnumIter)]
enum Filter {
   All,
   Active,
   Completed,
}

impl From<Url> for Filter {
    fn from(mut url: Url) -> Self {
        match url.remaining_hash_path_parts().as_slice() {
            ["active"] => Self::Active,
            ["completed"] => Self::Completed,
            _ => Self::All,
        }
    }
}

...

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::UrlChanged(subs::UrlChanged(url)) => {
            model.filter = Filter::from(url);
        }
...
```

_Tips_:

- Do NOT respect DRY at all costs - it often leads to cumbersome abstractions and tight [coupling](https://en.wikipedia.org/wiki/Coupling_(computer_programming)). And writing abstraction is always much easier than removing it later. See [Rule of three](https://en.wikipedia.org/wiki/Rule_of_three_(computer_programming)).

- Do NOT try to implement as many standard Rust traits as possible. We decided to implement `From` for `Filter` because we are sure it makes perfect sense in our case. If you are not sure, write only `Filter::new` and implement `From` only when necessary.

- Write `Filter::from(url)` instead of `url.into()`. The former one is much more expressive. You'll appreciate it once you use a type with many `From` implementation - especially in places with limited readable context like function calls: `foo(url.into())` vs `foo(Filter::from(url))`.

---

We'll refactor our filter links in the next chapter.

<details>
<summary>Our complete <code>lib.rs</code></summary>

```rust
#![allow(clippy::wildcard_imports)]
// TODO: Remove
#![allow(dead_code, unused_variables)]

use seed::{prelude::*, *};

use std::collections::BTreeMap;
use std::mem;
use std::convert::TryFrom;

use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;
use strum::IntoEnumIterator;
use ulid::Ulid;

const ENTER_KEY: &str = "Enter";
const ESCAPE_KEY: &str = "Escape";

const STORAGE_KEY: &str = "todos-seed";

// ------ ------
//     Init
// ------ ------

fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.subscribe(Msg::UrlChanged);

    Model {
        todos: LocalStorage::get(STORAGE_KEY).unwrap_or_default(),
        new_todo_title: String::new(),
        selected_todo: None,
        filter: Filter::from(url),
        base_url: Url::new(),
    }
}

// ------ ------
//     Model
// ------ ------

struct Model {
    todos: BTreeMap<Ulid, Todo>,
    new_todo_title: String,
    selected_todo: Option<SelectedTodo>,
    filter: Filter,
    base_url: Url,
}

#[derive(Deserialize, Serialize)]
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

// ------ Filter ------

#[derive(Copy, Clone, Eq, PartialEq, EnumIter)]
enum Filter {
   All,
   Active,
   Completed,
}

impl From<Url> for Filter {
    fn from(mut url: Url) -> Self {
        match url.remaining_hash_path_parts().as_slice() {
            ["active"] => Self::Active,
            ["completed"] => Self::Completed,
            _ => Self::All,
        }
    }
}

// ------ ------
//    Update
// ------ ------

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

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::UrlChanged(subs::UrlChanged(url)) => {
            model.filter = Filter::from(url);
        }
        Msg::NewTodoTitleChanged(title) => {
            model.new_todo_title = title;
        }
     
        // ------ Basic Todo operations ------

        Msg::CreateTodo => {
            let title = model.new_todo_title.trim();
            if not(title.is_empty()) {
                let id = Ulid::new();
                model.todos.insert(id, Todo {
                    id,
                    title: title.to_owned(),
                    completed: false,
                });
                model.new_todo_title.clear();
            }
        }
        Msg::ToggleTodo(id) => {
            if let Some(todo) = model.todos.get_mut(&id) {
                todo.completed = not(todo.completed);
            }
        }
        Msg::RemoveTodo(id) => {
            model.todos.remove(&id);
        }
        
        // ------ Bulk operations ------

        Msg::CheckOrUncheckAll => {
            let all_checked = model.todos.values().all(|todo| todo.completed);
            for todo in model.todos.values_mut() {
                todo.completed = not(all_checked);
            }
        }
        Msg::ClearCompleted => {
            // TODO: Refactor with `BTreeMap::drain_filter` once stable.
            model.todos = mem::take(&mut model.todos)
                .into_iter()
                .filter(|(_, todo)| not(todo.completed))
                .collect();
        }
        
        // ------ Selection ------

        Msg::SelectTodo(Some(id)) => {
            if let Some(todo) = model.todos.get(&id) {
                let input_element = ElRef::new();
                
                model.selected_todo = Some(SelectedTodo {
                    id,
                    title: todo.title.clone(),
                    input_element: input_element.clone(),
                });

                let title_length = u32::try_from(todo.title.len()).expect("title length as u32");
                orders.after_next_render(move |_| {
                    let input_element = input_element.get().expect("input_element");

                    input_element
                        .focus()
                        .expect("focus input_element");

                    input_element
                        .set_selection_range(title_length, title_length)
                        .expect("move cursor to the end of input_element");
                });
            }
        },
        Msg::SelectTodo(None) => {
            model.selected_todo = None;
        },
        Msg::SelectedTodoTitleChanged(title) => {
            if let Some(selected_todo) = &mut model.selected_todo {
                selected_todo.title = title;
            }
        },
        Msg::SaveSelectedTodo => {
            if let Some(selected_todo) = model.selected_todo.take() {
                if let Some(todo) = model.todos.get_mut(&selected_todo.id) {
                    todo.title = selected_todo.title;
                }
            }
        }
    }
    LocalStorage::insert(STORAGE_KEY, &model.todos).expect("save todos to LocalStorage");
}

// ------ ------
//     View
// ------ ------

fn view(model: &Model) -> Vec<Node<Msg>> {
    nodes![
        view_header(&model.new_todo_title),
        IF!(not(model.todos.is_empty()) => vec![
            view_main(&model.todos, model.selected_todo.as_ref(), model.filter), 
            view_footer(&model.todos, model.filter),
        ]),
    ]
}

// ------ header ------

fn view_header(new_todo_title: &str) -> Node<Msg> {
    header![C!["header"],
        h1!["todos"],
        input![C!["new-todo"],
            attrs!{
                At::Placeholder => "What needs to be done?", 
                At::AutoFocus => AtValue::None,
                At::Value => new_todo_title,
            },
            input_ev(Ev::Input, Msg::NewTodoTitleChanged),
            keyboard_ev(Ev::KeyDown, |keyboard_event| {
                IF!(keyboard_event.key() == ENTER_KEY => Msg::CreateTodo)
            }),
        ]
    ]
}

// ------ main ------

fn view_main(todos: &BTreeMap<Ulid, Todo>, selected_todo: Option<&SelectedTodo>, filter: Filter) -> Node<Msg> {
    section![C!["main"],
        view_toggle_all(todos),
        view_todo_list(todos, selected_todo, filter),
    ]
}

fn view_toggle_all(todos: &BTreeMap<Ulid, Todo>) -> Vec<Node<Msg>> {
    let all_completed = todos.values().all(|todo| todo.completed);
    vec![
        input![C!["toggle-all"], 
            attrs!{
                At::Id => "toggle-all", At::Type => "checkbox", At::Checked => all_completed.as_at_value()
            },
            ev(Ev::Change, |_| Msg::CheckOrUncheckAll),
        ],
        label![attrs!{At::For => "toggle-all"}, "Mark all as complete"],
    ]
}

fn view_todo_list(todos: &BTreeMap<Ulid, Todo>, selected_todo: Option<&SelectedTodo>, filter: Filter) -> Node<Msg> {
    let todos = todos.values().filter(|todo| {
        match filter {
            Filter::All => true,
            Filter::Active => not(todo.completed),
            Filter::Completed => todo.completed,
        }
    });
    ul![C!["todo-list"],
        todos.map(|todo| {
            let id = todo.id;
            let is_selected = Some(id) == selected_todo.map(|selected_todo| selected_todo.id);

            li![C![IF!(todo.completed => "completed"), IF!(is_selected => "editing")],
                el_key(&todo.id),
                div![C!["view"],
                    input![C!["toggle"], 
                        attrs!{At::Type => "checkbox", At::Checked => todo.completed.as_at_value()},
                        ev(Ev::Change, move |_| Msg::ToggleTodo(id)),
                    ],
                    label![
                        &todo.title,
                        ev(Ev::DblClick, move |_| Msg::SelectTodo(Some(id))),
                    ],
                    button![C!["destroy"],
                        ev(Ev::Click, move |_| Msg::RemoveTodo(id))
                    ],
                ],
                IF!(is_selected => {
                    let selected_todo = selected_todo.unwrap();
                    input![C!["edit"], 
                        el_ref(&selected_todo.input_element), 
                        attrs!{At::Value => selected_todo.title},
                        input_ev(Ev::Input, Msg::SelectedTodoTitleChanged),
                        keyboard_ev(Ev::KeyDown, |keyboard_event| {
                            Some(match keyboard_event.key().as_str() {
                                ESCAPE_KEY => Msg::SelectTodo(None),
                                ENTER_KEY => Msg::SaveSelectedTodo,
                                _ => return None
                            })
                        }),
                        ev(Ev::Blur, |_| Msg::SaveSelectedTodo),
                    ]
                }),
            ]
        })
    ]
}

// ------ footer ------

fn view_footer(todos: &BTreeMap<Ulid, Todo>, selected_filter: Filter) -> Node<Msg> {
    let completed_count = todos.values().filter(|todo| todo.completed).count();
    let active_count = todos.len() - completed_count;

    footer![C!["footer"],
        span![C!["todo-count"],
            strong![active_count],
            format!(" item{} left", if active_count == 1 { "" } else { "s" }),
        ],
        view_filters(selected_filter),
        IF!(completed_count > 0 =>
            button![C!["clear-completed"],
                "Clear completed",
                ev(Ev::Click, |_| Msg::ClearCompleted),
            ]
        )
    ]
}

fn view_filters(selected_filter: Filter) -> Node<Msg> {
    ul![C!["filters"],
        Filter::iter().map(|filter| {
            let (link, title) = match filter {
                Filter::All => ("#/", "All"),
                Filter::Active => ("#/active", "Active"),
                Filter::Completed => ("#/completed", "Completed"),
            };
            li![
                a![C![IF!(filter == selected_filter => "selected")],
                    attrs!{At::Href => link},
                    title,
                ],
            ]
        })
    ]
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
    
    let root_element = document()
        .get_elements_by_class_name("todoapp")
        .item(0)
        .expect("element with the class `todoapp`");

    App::start(root_element, init, update, view);
}

```
</details>
