# TodoMVC - Update

Let's fire and handle messages!

And don't forget to check that everything works after each step as usual.

1. `Msg::ToggleTodo(Ulid)`
    
    ```rust
    fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
        match msg {
           ...
            Msg::ToggleTodo(id) => {
                if let Some(todo) = model.todos.get_mut(&id) {
                    todo.completed = not(todo.completed);
                }
            } 
    ...

    fn view_todo_list(todos: &BTreeMap<Ulid, Todo>, selected_todo: Option<&SelectedTodo>) -> Node<Msg> {
    ul![C!["todo-list"],
        todos.values().map(|todo| {
            let id = todo.id;
            let is_selected = Some(id) == selected_todo.map(|selected_todo| selected_todo.id);
            ...
                    input![C!["toggle"], 
                        attrs!{At::Type => "checkbox", At::Checked => todo.completed.as_at_value()},
                        ev(Ev::Change, move |_| Msg::ToggleTodo(id))
                    ],
    ...
    ```
    We can't write `move |_| Msg::ToggleTodo(todo.id)` because we can't close (and move) the referenced `todo`. And we can't write `|_| Msg::ToggleTodo(id)` because without `move` we only close the reference to `id`. We need to `move` the value into the closure so the closure is `'static` and can be used inside a listener. Fortunately our `id` implements `Copy` so `move` isn't a real move but copy - otherwise we would need to clone the `id`.

1. `Msg::RemoveTodo(Ulid)`
    
    ```rust
    fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
        match msg {
           ...
            Msg::RemoveTodo(id) => {
                model.todos.remove(&id);
            }
    ...

    fn view_todo_list(todos: &BTreeMap<Ulid, Todo>, selected_todo: Option<&SelectedTodo>) -> Node<Msg> {
        ...
                        button![C!["destroy"],
                            ev(Ev::Click, move |_| Msg::RemoveTodo(id))
                        ],
    ...
    ```

1. `Msg::NewTodoTitleChanged(String)`
    
    ```rust
    fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
        match msg {
           ...
            Msg::NewTodoTitleChanged(title) => {
                model.new_todo_title = title;
            }
    ...

    fn view_header(new_todo_title: &str) -> Node<Msg> {
            input![C!["new-todo"],
                ...
                input_ev(Ev::Input, Msg::NewTodoTitleChanged),
    ...
    ```
    There aren't any changes from the user point of view, but the main goal was to store changed input value to `Model`.

    _Note:_ 
    ```rust
    input_ev(Ev::Input, Msg::NewTodoTitleChanged)
    ```
    is almost the same as 
    ```rust
    input_ev(Ev::Input, |title| Msg::NewTodoTitleChanged(title))
    ```
    However there are cases where you have to use the latter one, because Rust can't apply all coercion rules without explicitly written variables.

1. `Msg::CreateTodo`

    >### New todo
    >
    >New todos are entered in the input at the top of the app. The input element should be focused when the page is loaded, preferably by using the `autofocus` input attribute. Pressing Enter creates the todo, appends it to the todo list, and clears the input. Make sure to `.trim()` the input and then check that it's not empty before creating a new todo.

    ```rust
    use ulid::Ulid;

    const ENTER_KEY: &str = "Enter";

    // ------ ------
    //     Init
    // ------ ------
    ...

    fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
        match msg {
            ...
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
    ...

    fn view_header(new_todo_title: &str) -> Node<Msg> {
        ...
                input_ev(...),
                keyboard_ev(Ev::KeyDown, |keyboard_event| {
                    IF!(keyboard_event.key() == ENTER_KEY => Msg::CreateTodo)
                }),
    ...
    ```

    _Notes:_

    - We used `const ENTER_KEY` instead of `static ENTER_KEY`. `const` is generally preferable because it's more expressive (it's clear that we don't want to mutate `const`) and because it's inlined and therefore faster in the most cases. However it's relatively easy to make the application (`*.wasm` file size) too big (it may even crash in runtime) if your `const` is complex. `static` is more suitable for such cases.

    - There is cloning hidden behind `title: title.to_owned()`. There isn't a safe and simple way how to "pour" the trimmed `string slice` from the original `String` (`new_todo_title`) into `Todo`'s `title`. So we have to clone the trimmed `string slice` and then `clear` the original `String`.

    - We used `Ev::KeyDown` because [keypress event](https://developer.mozilla.org/en-US/docs/Web/API/Document/keypress_event) is deprecated. And `.key()` because:
       - [keyCode](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/keyCode), [which](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/which) and [keyIdentifier](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/keyIdentifier) are deprecated.
       - `.code()` returns different values for "classic" Enter (`Enter`) and Enter on the numeric keyboard (`NumpadEnter`). 
       
        You can test inputs in [Keyboard Event Viewer](https://w3c.github.io/uievents/tools/key-event-viewer.html).


1. `Msg::ClearCompleted`

    ```rust
    use std::collections::BTreeMap;
    use std::mem;
    ...

    fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
        match msg {
            ...
            Msg::ClearCompleted => {
                // TODO: Refactor with `BTreeMap::drain_filter` once stable.
                model.todos = mem::take(&mut model.todos)
                    .into_iter()
                    .filter(|(_, todo)| not(todo.completed))
                    .collect();
            }
    ...

    fn view_footer(todos: &BTreeMap<Ulid, Todo>, selected_filter: Filter) -> Node<Msg> {
        ...
                button![C!["clear-completed"],
                    "Clear completed",
                    ev(Ev::Click, |_| Msg::ClearCompleted),
                ]
    ...
    ```

    _Note:_ We could make the filter pipeline nicer with the help of [apply](https://crates.io/crates/apply) or a custom BTreeMap-[extending trait](http://xion.io/post/code/rust-extension-traits.html), but let's wait for [BTreeMap::drain_filer](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html#method.drain_filter) stabilization.

1. `Msg::CheckOrUncheckAll`

    ```rust
    fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
        match msg {
            ...
            Msg::CheckOrUncheckAll => {
                let all_checked = model.todos.values().all(|todo| todo.completed);
                for todo in model.todos.values_mut() {
                    todo.completed = not(all_checked);
                }
            }
    ...

    fn view_toggle_all(todos: &BTreeMap<Ulid, Todo>) -> Vec<Node<Msg>> {
        let all_completed = todos.values().all(|todo| todo.completed);
        ...
            input![C!["toggle-all"], 
                ...
                ev(Ev::Change, |_| Msg::CheckOrUncheckAll),
            ],
    ...
    ```

    _Note:_ You may be tempted to pass `all_completed` along the message to replace `all_checked` with it to eliminate one loop. Don't do it. `view` often contains old data and you may accidentally introduce a hard-to-debug bug.

1. `Msg::SelectTodo(Option<Ulid>)`

    >### Item
    >
    >A todo item has three possible interactions:
    >
    >1. Clicking the checkbox marks the todo as complete by updating its `completed` value and toggling the class `completed` on its parent `<li>`
    >
    >2. Double-clicking the `<label>` activates editing mode, by toggling the `.editing` class on its `<li>`
    >
    >3. Hovering over the todo shows the remove button (`.destroy`)
    >
    >### Editing
    >
    >When editing mode is activated it will hide the other controls and bring forward an input that contains the todo title, which should be focused (`.focus()`). The edit should be saved on both blur and enter, and the `editing` class should be removed. Make sure to `.trim()` the input and then check that it's not empty. If it's empty the todo should instead be destroyed. If escape is pressed during the edit, the edit state should be left and any changes be discarded.

    ```rust
    use std::mem;
    use std::convert::TryFrom;
    ...

    const ENTER_KEY: &str = "Enter";
    const ESCAPE_KEY: &str = "Escape";
    ...

    fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
        match msg {
            ...
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
    ...

    fn view_todo_list(todos: &BTreeMap<Ulid, Todo>, selected_todo: Option<&SelectedTodo>) -> Node<Msg> {
        ...
                        label![
                            ...
                            ev(Ev::DblClick, move |_| Msg::SelectTodo(Some(id))),
                        ],
        ...
                        input![C!["edit"], 
                            ...
                            keyboard_ev(Ev::KeyDown, |keyboard_event| {
                                IF!(keyboard_event.key() == ESCAPE_KEY => Msg::SelectTodo(None))
                            }),
    ...
    ```
    _Notes_:
    
    - `orders.after_next_render` registers a callback that is invoked after the next `view` invocation. The callback receives [RenderInfo](https://github.com/seed-rs/seed/blob/master/src/app/render_info.rs) - it's useful for animations but we don't need it here (see example [animation](https://github.com/seed-rs/seed/blob/d514b2131a9e94f5ffe965f3d0ac74763a11aeb6/examples/animation/src/lib.rs#L81-L93)).

    - `input_element.get()` returns `Option<E>` where `E` is a specific DOM element reference like `web_sys::HtmlInputElement`. It returns `None` when the element doesn't exists in the DOM or has an incompatible type => all [ElRef](https://github.com/seed-rs/seed/blob/0a538f03d6aeb56b00d997c80a666e388279a727/src/virtual_dom/el_ref.rs) methods are safe to use.

    - There are many `.expect(..)` calls because DOM operations are dangerous - any JS library or browser extension can modify the DOM "under our hands", browsers have bugs and don't support all features in official specs, etc. So we want to get as much information as possible when our app panics for one of these reasons. Descriptions inside `expect` calls help with readability.

    - [as](https://doc.rust-lang.org/beta/std/keyword.as.html) for casting is an anti-pattern in most cases. You should write `xx::from(yy)` or `xx::try_from(yy)` instead. E.g.
        ```rust
        u32::try_from(todo.title.len()))
        ```
        Alternatives are `xx = yy.into()` and `xx = yy.try_into()` - they are as safe as their `(Try)From` counterparts however they make the code LESS READABLE because you often have to guess the type.

1. `Msg::SelectedTodoTitleChanged(String)`

    ```rust
    fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
        match msg {
            ...
            Msg::SelectedTodoTitleChanged(title) => {
                if let Some(selected_todo) = &mut model.selected_todo {
                    selected_todo.title = title;
                }
            },
    ...

    fn view_todo_list(todos: &BTreeMap<Ulid, Todo>, selected_todo: Option<&SelectedTodo>) -> Node<Msg> {
        ...
                        input![C!["edit"], 
                            ...
                            input_ev(Ev::Input, Msg::SelectedTodoTitleChanged),
                        ]
    ...
    ```

1. `Msg::SaveSelectedTodo`

    >### Editing
    >
    >When editing mode is activated it will hide the other controls and bring forward an input that contains the todo title, which should be focused (`.focus()`). The edit should be saved on both blur and enter, and the `editing` class should be removed. Make sure to `.trim()` the input and then check that it's not empty. If it's empty the todo should instead be destroyed. If escape is pressed during the edit, the edit state should be left and any changes be discarded.

    ```rust
    fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
        match msg {
            ...
            Msg::SaveSelectedTodo => {
                if let Some(selected_todo) = model.selected_todo.take() {
                    let title = selected_todo.title.trim();
                    if title.is_empty() {
                        model.todos.remove(&selected_todo.id);
                    } else {
                        if let Some(todo) = model.todos.get_mut(&selected_todo.id) {
                            todo.title = title.to_owned();
                        }
                    }
                }
            }
    ...

    fn view_todo_list(todos: &BTreeMap<Ulid, Todo>, selected_todo: Option<&SelectedTodo>) -> Node<Msg> {
        ...
                            keyboard_ev(Ev::KeyDown, |keyboard_event| {
                                Some(match keyboard_event.key().as_str() {
                                    ESCAPE_KEY => Msg::SelectTodo(None),
                                    ENTER_KEY => Msg::SaveSelectedTodo,
                                    _ => return None
                                })
                            }),
                            ev(Ev::Blur, |_| Msg::SaveSelectedTodo),
                        ]
    ...
    ```
    _Notes:_
    
    - `selected_todo.take()` - it's "a common trick" how to take ownership of the chosen variable. It's basically equivalent to `mem::take` however you can be sure that creating a default value is cheap and it's idiomatic Rust. It has two benefits in our case: It implicitly deselects the todo and we don't have to clone anything.

    - `Some(match...` - `Some` is used as a wrapper here so we don't have to wrap all `Msg`s in the `match` arms - e.g. `Some(Msg::SelectTodo(None))`. 

1. `Msg::UrlChanged(subs::UrlChanged)`
   - It's the last one but we'll implement it once we know routing.

1. We no longer need the method `Model::add_mock_data`.
    - Delete it and remove the `.add_mock_data()` call from your `init` function, too.

---

That's it! We'll store todos in `LocalStorage` during the next chapter and the app is almost complete!
