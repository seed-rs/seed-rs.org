# TodoMVC - Project Setup

There are two reasons why we are creating the project now, when the `Model` and `Msg` are prepared:

1. It forces us to think and design before we start to write code. It leads in the most cases to better architecture.

1. `update` and especially `view` is better to write and test "live" because it's fun and fast feedback loop reveals design and implementation issues quickly.

## How to setup TodoMVC project

I assume you've already tried to setup a new project for the `counter` example. If not, open [seed-quickstart repo](https://github.com/seed-rs/seed-quickstart) and read its `README.md`.

Please, create and start a new Seed app if you want to follow the steps below.

1. Remove unnecessary comments from your `lib.rs`.
    <details>
    <summary><code>lib.rs</code> without comments</summary>

    ```rust
    #![allow(clippy::wildcard_imports)]

    use seed::{prelude::*, *};

    // ------ ------
    //     Init
    // ------ ------

    fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
        Model::default()
    }

    // ------ ------
    //     Model
    // ------ ------

    type Model = i32;

    // ------ ------
    //    Update
    // ------ ------

    #[derive(Copy, Clone)]
    enum Msg {
        Increment,
    }

    fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
        match msg {
            Msg::Increment => *model += 1,
        }
    }

    // ------ ------
    //     View
    // ------ ------

    #[allow(clippy::trivially_copy_pass_by_ref)]
    fn view(model: &Model) -> Node<Msg> {
        div![
            "This is a counter: ",
            C!["counter"],
            button![model, ev(Ev::Click, |_| Msg::Increment),],
        ]
    }

    // ------ ------
    //     Start
    // ------ ------

    #[wasm_bindgen(start)]
    pub fn start() {
        App::start("app", init, update, view);
    }

    ```
    </details>

1. Add our designed `Model` and `Msg`. (We'll resolve compilation errors in next steps.)
    <details>
    <summary>Updated <code>Model</code> and <code>Msg</code></summary>

    ```rust
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

    // ------ ------
    //    Update
    // ------ ------

    #[derive(Copy, Clone)]
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
    </details>

1. Add dependency [ulid](https://crates.io/crates/ulid) into `Cargo.toml`.
    ```toml
    [dependencies]
    ulid = "0.3.3"
    ...
    ```

1. Import `BTreeMap` and `Ulid` into `lib.rs`.
    ```rs
    use seed::{prelude::*, *};
    use std::collections::BTreeMap;
    use ulid::Ulid;
    ```

1. Remove the line 
    ```rust
    #[derive(Copy, Clone)]
    ```
    from `lib.rs`, because `subs::UrlChanged` and `String` don't implement `Copy` and standalone `Clone` for `Msg` is an anti-pattern.

1. Remove `allow` attribute and `div!` content from `view`. You can write simple alternative content (e.g. `"I'm a placeholder"`) to check in your browser that everything works once we fix all compilation errors.
    ```rust
    // ------ ------
    //     View
    // ------ ------

    fn view(model: &Model) -> Node<Msg> {
        div![
            "I'm a placeholder"
        ]
    }
    ```

1. Write `update` skeleton with all `match` arms. Each arm contains `log!` with `Msg` data - it'll help us to write and test `view`.
    <details>
    <summary><code>update</code> skeleton</summary>

    ```rust
    fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
        match msg {
            Msg::UrlChanged(subs::UrlChanged(url)) => {
                log!("UrlChanged", url);
            }
            Msg::NewTodoTitleChanged(title) => {
                log!("NewTodoTitleChanged", title);
            }
        
            // ------ Basic Todo operations ------

            Msg::CreateTodo => {
                log!("CreateTodo");
            }
            Msg::ToggleTodo(id) => {
                log!("ToggleTodo");
            }
            Msg::RemoveTodo(id) => {
                log!("RemoveTodo");
            }
            
            // ------ Bulk operations ------

            Msg::CheckOrUncheckAll => {
                log!("CheckOrUncheckAll");
            }
            Msg::ClearCompleted => {
                log!("ClearCompleted");
            }
            
            // ------ Selection ------

            Msg::SelectTodo(opt_id) => {
                log!("SelectTodo", opt_id);
            },
            Msg::SelectedTodoTitleChanged(title) => {
                log!("SelectedTodoTitleChanged", title);
            },
            Msg::SaveSelectedTodo => {
                log!("SaveSelectedTodo");
            }
        }
    }
    ```
    </details>

1. Let's create the most simple `Model` instance in our `init`.
    ```rust
    fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
        Model {
            todos: BTreeMap::new(),
            new_todo_title: String::new(),
            selected_todo: None,
            filter: Filter::All,
            base_url: Url::new(),
        }
    }
    ```

1. Temporarily disable compiler linters `dead_code` and `unused_variables`. They are very useful in later stages of the project however they cause a sea of warnings in the terminal now and it's annoying when you want to read compilation errors. 
    ```rust
    #![allow(clippy::wildcard_imports)]
    // TODO: Remove
    #![allow(dead_code, unused_variables)]
    ```

1. Implement `Model` method `add_mock_data`. It'll allow us to test `view` function until our `update` function is complete.
    <details>
    <summary>Updated <code>init</code> and <code>add_mock_data</code></summary>

    ```rust
    fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
        Model {
            ...
        }.add_mock_data()
    }

    ...

    struct Model {
        ...
    }

    // TODO: Remove
    impl Model {
        fn add_mock_data(mut self) -> Self {
            let (id_a, id_b) = (Ulid::new(), Ulid::new());
            
            self.todos.insert(id_a, Todo {
                id: id_a,
                title: "I'm todo A".to_owned(),
                completed: false,
            });

            self.todos.insert(id_b, Todo {
                id: id_b,
                title: "I'm todo B".to_owned(),
                completed: true,
            });

            self.new_todo_title = "I'm a new todo title".to_owned();

            self.selected_todo = Some(SelectedTodo {
                id: id_b,
                title: "I'm better todo B".to_owned(),
                input_element: ElRef::new(),
            });
            self
        }
    }
    ```
    </details>

1. Make sure there aren't any errors or warnings in your terminal and you see something like `I'm a placeholder` in your browser.
   - If you don't see anything, make sure your dev server is running (consult [quickstart's README.md](https://github.com/seed-rs/seed-quickstart)) and that you didn't forget to refresh you browser tab. 

---

_Note:_ There are `.to_owned()` calls instead of `to_string()` or `into()` in the code above and in other chapters.
    
- `"foo".to_string()` wouldn't make any sense if you don't know Rust: "Why we are casting string to string??". If you understand `&str` as a kind of text/string, then it makes more sense to write `to_owned()`. `to_owned()` better expresses the operation: "promoting" a string reference to the owned string. And you can accidentally introduce more expensive `to_string` operation when you replace `a_str.to_string()` with `a_complex_item.to_string()`.

- `into()` has similar problems like `to_string` but they are worse because `into` is even more general than `to_string`. For example, you have no idea what type `bar` has in the expression `let bar = "foo".into()`.

---

Done! We are ready to write `view` in the next chapter!
