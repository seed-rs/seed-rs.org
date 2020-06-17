# TodoMVC - Refactor

Finish him!

1. Enable disabled linters by removing `#![allow(dead_code, unused_variables)]`
   - You shouldn't see any warnings or compilation errors.

1. Run `cargo make clippy` (if you use [Rust basic quickstart](https://github.com/seed-rs/seed-quickstart)).
   - You shouldn't see any errors.

1. Run `cargo make fmt`

   - Rust formatter sometimes fight with macros, however we SHOULD use it to make our code consistent, especially when we are working in a team or if it's an open-source project.

   - I don't think that basic language tools like linters and formatters should be configurable (and configured). However I plan to write a specific configuration file for Seed apps to mitigate formatter issues.

    <details>
    <summary>Formatter issue</summary>

    When I run `cargo make fmt` in our project now, I see the `rustfmt` error:

    ```
    error[internal]: left behind trailing whitespace
      --> \\?\C:\work\repos\seed-app-todomvc\src\lib.rs:310:310:61
        |
    310 |                         el_ref(&selected_todo.input_element),
        |                                                              ^
    ```
    There are some related `rustfmt` issues - [#2916](https://github.com/rust-lang/rustfmt/issues/2916), [#3717](https://github.com/rust-lang/rustfmt/issues/3717), [#3904](https://github.com/rust-lang/rustfmt/issues/3904), [#4192](https://github.com/rust-lang/rustfmt/issues/4192). 

    We can resolve it by adding a temporary `skip` attribute:
    ```rust
    // TODO: Remove once rustfmt is updated.
    #[rustfmt::skip]
    fn view_todo_list(
    ```
    or with `#![rustfmt::skip::macros]` once stable.


    I hope it will be fixed in `rustftm v2.0` ([releases](https://github.com/rust-lang/rustfmt/releases)). If not, we should create a new issue in the `rustfmt` repo.
    </details>

1. Run `cargo make test_h firefox`
   - Well, there are no tests in our project, so you shouldn't see any errors.
   - It's possible to write [tests](https://github.com/seed-rs/seed/blob/d514b2131a9e94f5ffe965f3d0ac74763a11aeb6/src/browser/dom/css_units.rs#L92-L144), however some important [helpers](https://github.com/seed-rs/seed/issues/294) for good integration tests are missing. See also [wasm-bindgen-test](https://rustwasm.github.io/wasm-bindgen/wasm-bindgen-test/index.html) docs.

1. Run `cargo make verify`
   - It runs `clippy`, `fmt` and `test_h_firefox` tasks.
   - You SHOULD execute it before each Git `push`.

1. Let's do a final visual check to make sure we are satisfied with the code.
   - There should be comments to tell readers why we chose the particular types (e.g. `BTreeMap` vs `IndexMap`) or to explain some business logic. I didn't want to add unnecessary "noise" to examples but you should write them in real-world apps.

   - There shouldn't be almost any comments explaining HOW something works, only WHY is the code important, ideally from the business logic view. The only exception is complex algorithms that we can't simplify.

   - We could split our `view_todo_list` function, but I'm not sure it would improve readability too much. 

   - `lib.rs` isn't so long to split.

   - Seed app blocks should be in this order:
      1. `Init`
      1. `Model`
      1. `Urls` (optional)
      1. `Update`
      1. `View`
      1. `Start`
      1. `Exported` (optional, Rust functions available in JS/TS)
      1. `Extern` (optional, JS items used in Rust)

---

I hope you enjoyed the ride!

- TodoMVC repository - [github.com/MartinKavik/seed-app-todomvc](https://github.com/MartinKavik/seed-app-todomvc)
- Live Demo - [seed-app-todomvc.netlify.app/](https://seed-app-todomvc.netlify.app/)
- gzipped Wasm file size - 135 KB
  - We'll optimize Seed for size and speed (see related [GitHub issue](https://github.com/seed-rs/seed/issues/385)). 

---

Final code

<details>
<summary><code>Cargo.toml</code></summary>

```toml
[package]
version = "0.1.0"
name = "app_name"
repository = "https://github.com/seed-rs/seed-quickstart"
authors = ["Your Name <email@address.com>"]
description = "App Description"
categories = ["category"]
license = "MIT"
readme = "./README.md"
edition = "2018"

[lib]
crate-type = ["cdylib"]

[dev-dependencies]
wasm-bindgen-test = "0.3.13"

[dependencies]
serde = "1.0.112"
strum = "0.18.0"
strum_macros = "0.18.0"
ulid = { version = "0.3.3", features = ["serde"] }
# This commit points to Seed 0.7.0 with important fixes.
# Replace with `seed = "0.8.0"` (or newer) once released.
seed = { git = "https://github.com/seed-rs/seed", rev = "0a538f0" }

[profile.release]
lto = true
opt-level = 'z'
codegen-units = 1

[package.metadata.wasm-pack.profile.release]
wasm-opt = ['-Os']
```

</details>

<details>
<summary><code>index.html</code></summary>

```html
<!DOCTYPE html>
<html lang="en">

<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <title>Template • TodoMVC</title>
    <link rel="stylesheet" href="css/base.css">
    <link rel="stylesheet" href="css/index.css">
</head>

<body>
    <section class="todoapp"></section>

    <footer class="info">
        <p>Double-click to edit a todo</p>
        <p>Created by <a href="https://kavik.cz">Martin Kavík</a></p>
        <p>Part of <a href="http://todomvc.com">TodoMVC</a></p>
    </footer>

    <script type="module">
        import init from '/pkg/package.js';
        init('/pkg/package_bg.wasm');
    </script>
</body>

</html>

```

</details>

<details>
<summary><code>lib.rs</code></summary>

```rust
#![allow(clippy::wildcard_imports)]

use seed::{prelude::*, *};

use std::collections::BTreeMap;
use std::convert::TryFrom;
use std::mem;

use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use ulid::Ulid;

const ENTER_KEY: &str = "Enter";
const ESCAPE_KEY: &str = "Escape";

const STORAGE_KEY: &str = "todos-seed";

// ------ Url path parts ------
const ACTIVE: &str = "active";
const COMPLETED: &str = "completed";

// ------ ------
//     Init
// ------ ------

fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.subscribe(Msg::UrlChanged);

    Model {
        base_url: url.to_hash_base_url(),
        todos: LocalStorage::get(STORAGE_KEY).unwrap_or_default(),
        new_todo_title: String::new(),
        selected_todo: None,
        filter: Filter::from(url),
    }
}

// ------ ------
//     Model
// ------ ------

struct Model {
    base_url: Url,
    todos: BTreeMap<Ulid, Todo>,
    new_todo_title: String,
    selected_todo: Option<SelectedTodo>,
    filter: Filter,
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
            [ACTIVE] => Self::Active,
            [COMPLETED] => Self::Completed,
            _ => Self::All,
        }
    }
}

// ------ ------
//     Urls
// ------ ------

struct_urls!();
impl<'a> Urls<'a> {
    pub fn home(self) -> Url {
        self.base_url()
    }
    pub fn active(self) -> Url {
        self.base_url().add_hash_path_part(ACTIVE)
    }
    pub fn completed(self) -> Url {
        self.base_url().add_hash_path_part(COMPLETED)
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
                model.todos.insert(
                    id,
                    Todo {
                        id,
                        title: title.to_owned(),
                        completed: false,
                    },
                );
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

                    input_element.focus().expect("focus input_element");

                    input_element
                        .set_selection_range(title_length, title_length)
                        .expect("move cursor to the end of input_element");
                });
            }
        }
        Msg::SelectTodo(None) => {
            model.selected_todo = None;
        }
        Msg::SelectedTodoTitleChanged(title) => {
            if let Some(selected_todo) = &mut model.selected_todo {
                selected_todo.title = title;
            }
        }
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
            view_footer(&model.todos, model.filter, &model.base_url),
        ]),
    ]
}

// ------ header ------

fn view_header(new_todo_title: &str) -> Node<Msg> {
    header![
        C!["header"],
        h1!["todos"],
        input![
            C!["new-todo"],
            attrs! {
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

fn view_main(
    todos: &BTreeMap<Ulid, Todo>,
    selected_todo: Option<&SelectedTodo>,
    filter: Filter,
) -> Node<Msg> {
    section![
        C!["main"],
        view_toggle_all(todos),
        view_todo_list(todos, selected_todo, filter),
    ]
}

fn view_toggle_all(todos: &BTreeMap<Ulid, Todo>) -> Vec<Node<Msg>> {
    let all_completed = todos.values().all(|todo| todo.completed);
    vec![
        input![
            C!["toggle-all"],
            attrs! {
                At::Id => "toggle-all", At::Type => "checkbox", At::Checked => all_completed.as_at_value()
            },
            ev(Ev::Change, |_| Msg::CheckOrUncheckAll),
        ],
        label![attrs! {At::For => "toggle-all"}, "Mark all as complete"],
    ]
}

// TODO: Remove once rustfmt is updated.
#[rustfmt::skip]
fn view_todo_list(
    todos: &BTreeMap<Ulid, Todo>,
    selected_todo: Option<&SelectedTodo>,
    filter: Filter,
) -> Node<Msg> {
    let todos = todos.values().filter(|todo| match filter {
        Filter::All => true,
        Filter::Active => not(todo.completed),
        Filter::Completed => todo.completed,
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

fn view_footer(todos: &BTreeMap<Ulid, Todo>, selected_filter: Filter, base_url: &Url) -> Node<Msg> {
    let completed_count = todos.values().filter(|todo| todo.completed).count();
    let active_count = todos.len() - completed_count;

    footer![
        C!["footer"],
        span![
            C!["todo-count"],
            strong![active_count],
            format!(" item{} left", if active_count == 1 { "" } else { "s" }),
        ],
        view_filters(selected_filter, base_url),
        IF!(completed_count > 0 =>
            button![C!["clear-completed"],
                "Clear completed",
                ev(Ev::Click, |_| Msg::ClearCompleted),
            ]
        )
    ]
}

fn view_filters(selected_filter: Filter, base_url: &Url) -> Node<Msg> {
    ul![
        C!["filters"],
        Filter::iter().map(|filter| {
            let urls = Urls::new(base_url);

            let (url, title) = match filter {
                Filter::All => (urls.home(), "All"),
                Filter::Active => (urls.active(), "Active"),
                Filter::Completed => (urls.completed(), "Completed"),
            };

            li![a![
                C![IF!(filter == selected_filter => "selected")],
                attrs! {At::Href => url},
                title,
            ],]
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
