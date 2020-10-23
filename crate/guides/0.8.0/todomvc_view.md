# TodoMVC - View

Our `view` function looks like this now:
```rust
fn view(model: &Model) -> Node<Msg> {
    div![
        "I'm a placeholder"
    ]
}
```
and are goal is something like:

![TodoMVC screen](/static/images/todomvc_screen.png)

Fortunately official TodoMVC project contains [HTML template](https://github.com/tastejs/todomvc-app-template/blob/master/index.html):

<details>
<summary>Template <code>index.html</code></summary>

```html
<!doctype html>
<html lang="en">
	<head>
		<meta charset="utf-8">
		<meta name="viewport" content="width=device-width, initial-scale=1">
		<title>Template • TodoMVC</title>
		<link rel="stylesheet" href="node_modules/todomvc-common/base.css">
		<link rel="stylesheet" href="node_modules/todomvc-app-css/index.css">
		<!-- CSS overrides - remove if you don't need it -->
		<link rel="stylesheet" href="css/app.css">
	</head>
	<body>
		<section class="todoapp">
			<header class="header">
				<h1>todos</h1>
				<input class="new-todo" placeholder="What needs to be done?" autofocus>
			</header>
			<!-- This section should be hidden by default and shown when there are todos -->
			<section class="main">
				<input id="toggle-all" class="toggle-all" type="checkbox">
				<label for="toggle-all">Mark all as complete</label>
				<ul class="todo-list">
					<!-- These are here just to show the structure of the list items -->
					<!-- List items should get the class `editing` when editing and `completed` when marked as completed -->
					<li class="completed">
						<div class="view">
							<input class="toggle" type="checkbox" checked>
							<label>Taste JavaScript</label>
							<button class="destroy"></button>
						</div>
						<input class="edit" value="Create a TodoMVC template">
					</li>
					<li>
						<div class="view">
							<input class="toggle" type="checkbox">
							<label>Buy a unicorn</label>
							<button class="destroy"></button>
						</div>
						<input class="edit" value="Rule the web">
					</li>
				</ul>
			</section>
			<!-- This footer should hidden by default and shown when there are todos -->
			<footer class="footer">
				<!-- This should be `0 items left` by default -->
				<span class="todo-count"><strong>0</strong> item left</span>
				<!-- Remove this if you don't implement routing -->
				<ul class="filters">
					<li>
						<a class="selected" href="#/">All</a>
					</li>
					<li>
						<a href="#/active">Active</a>
					</li>
					<li>
						<a href="#/completed">Completed</a>
					</li>
				</ul>
				<!-- Hidden if no completed items are left ↓ -->
				<button class="clear-completed">Clear completed</button>
			</footer>
		</section>
		<footer class="info">
			<p>Double-click to edit a todo</p>
			<!-- Remove the below line ↓ -->
			<p>Template by <a href="http://sindresorhus.com">Sindre Sorhus</a></p>
			<!-- Change this out with your name and url ↓ -->
			<p>Created by <a href="http://todomvc.com">you</a></p>
			<p>Part of <a href="http://todomvc.com">TodoMVC</a></p>
		</footer>
		<!-- Scripts here. Don't remove ↓ -->
		<script src="node_modules/todomvc-common/base.js"></script>
		<script src="js/app.js"></script>
	</body>
</html>
```

</details>

Let's integrate it into our app!

1. Update our `index.html` - `head`, the app root element and `footer` - according the HTML template. We can't include [base.css](https://github.com/tastejs/todomvc-common/blob/master/base.css) and [index.css](https://github.com/tastejs/todomvc-app-css/blob/master/index.css) from `node_modules` so we have to download/copy them into our new `/css` folder.

    <details>
    <summary>New <code>css/base.css</code></summary>

    ```css
    hr {
        margin: 20px 0;
        border: 0;
        border-top: 1px dashed #c5c5c5;
        border-bottom: 1px dashed #f7f7f7;
    }

    .learn a {
        font-weight: normal;
        text-decoration: none;
        color: #b83f45;
    }

    .learn a:hover {
        text-decoration: underline;
        color: #787e7e;
    }

    .learn h3,
    .learn h4,
    .learn h5 {
        margin: 10px 0;
        font-weight: 500;
        line-height: 1.2;
        color: #000;
    }

    .learn h3 {
        font-size: 24px;
    }

    .learn h4 {
        font-size: 18px;
    }

    .learn h5 {
        margin-bottom: 0;
        font-size: 14px;
    }

    .learn ul {
        padding: 0;
        margin: 0 0 30px 25px;
    }

    .learn li {
        line-height: 20px;
    }

    .learn p {
        font-size: 15px;
        font-weight: 300;
        line-height: 1.3;
        margin-top: 0;
        margin-bottom: 0;
    }

    #issue-count {
        display: none;
    }

    .quote {
        border: none;
        margin: 20px 0 60px 0;
    }

    .quote p {
        font-style: italic;
    }

    .quote p:before {
        content: '“';
        font-size: 50px;
        opacity: .15;
        position: absolute;
        top: -20px;
        left: 3px;
    }

    .quote p:after {
        content: '”';
        font-size: 50px;
        opacity: .15;
        position: absolute;
        bottom: -42px;
        right: 3px;
    }

    .quote footer {
        position: absolute;
        bottom: -40px;
        right: 0;
    }

    .quote footer img {
        border-radius: 3px;
    }

    .quote footer a {
        margin-left: 5px;
        vertical-align: middle;
    }

    .speech-bubble {
        position: relative;
        padding: 10px;
        background: rgba(0, 0, 0, .04);
        border-radius: 5px;
    }

    .speech-bubble:after {
        content: '';
        position: absolute;
        top: 100%;
        right: 30px;
        border: 13px solid transparent;
        border-top-color: rgba(0, 0, 0, .04);
    }

    .learn-bar > .learn {
        position: absolute;
        width: 272px;
        top: 8px;
        left: -300px;
        padding: 10px;
        border-radius: 5px;
        background-color: rgba(255, 255, 255, .6);
        transition-property: left;
        transition-duration: 500ms;
    }

    @media (min-width: 899px) {
        .learn-bar {
            width: auto;
            padding-left: 300px;
        }

        .learn-bar > .learn {
            left: 8px;
        }
    }
    ```
    </details>

    <details>
    <summary>New <code>css/index.css</code></summary>

    ```css
    html,
    body {
        margin: 0;
        padding: 0;
    }

    button {
        margin: 0;
        padding: 0;
        border: 0;
        background: none;
        font-size: 100%;
        vertical-align: baseline;
        font-family: inherit;
        font-weight: inherit;
        color: inherit;
        -webkit-appearance: none;
        appearance: none;
        -webkit-font-smoothing: antialiased;
        -moz-osx-font-smoothing: grayscale;
    }

    body {
        font: 14px 'Helvetica Neue', Helvetica, Arial, sans-serif;
        line-height: 1.4em;
        background: #f5f5f5;
        color: #111111;
        min-width: 230px;
        max-width: 550px;
        margin: 0 auto;
        -webkit-font-smoothing: antialiased;
        -moz-osx-font-smoothing: grayscale;
        font-weight: 300;
    }

    :focus {
        outline: 0;
    }

    .hidden {
        display: none;
    }

    .todoapp {
        background: #fff;
        margin: 130px 0 40px 0;
        position: relative;
        box-shadow: 0 2px 4px 0 rgba(0, 0, 0, 0.2),
                    0 25px 50px 0 rgba(0, 0, 0, 0.1);
    }

    .todoapp input::-webkit-input-placeholder {
        font-style: italic;
        font-weight: 300;
        color: rgba(0, 0, 0, 0.4);
    }

    .todoapp input::-moz-placeholder {
        font-style: italic;
        font-weight: 300;
        color: rgba(0, 0, 0, 0.4);
    }

    .todoapp input::input-placeholder {
        font-style: italic;
        font-weight: 300;
        color: rgba(0, 0, 0, 0.4);
    }

    .todoapp h1 {
        position: absolute;
        top: -140px;
        width: 100%;
        font-size: 80px;
        font-weight: 200;
        text-align: center;
        color: #b83f45;
        -webkit-text-rendering: optimizeLegibility;
        -moz-text-rendering: optimizeLegibility;
        text-rendering: optimizeLegibility;
    }

    .new-todo,
    .edit {
        position: relative;
        margin: 0;
        width: 100%;
        font-size: 24px;
        font-family: inherit;
        font-weight: inherit;
        line-height: 1.4em;
        color: inherit;
        padding: 6px;
        border: 1px solid #999;
        box-shadow: inset 0 -1px 5px 0 rgba(0, 0, 0, 0.2);
        box-sizing: border-box;
        -webkit-font-smoothing: antialiased;
        -moz-osx-font-smoothing: grayscale;
    }

    .new-todo {
        padding: 16px 16px 16px 60px;
        border: none;
        background: rgba(0, 0, 0, 0.003);
        box-shadow: inset 0 -2px 1px rgba(0,0,0,0.03);
    }

    .main {
        position: relative;
        z-index: 2;
        border-top: 1px solid #e6e6e6;
    }

    .toggle-all {
        width: 1px;
        height: 1px;
        border: none; /* Mobile Safari */
        opacity: 0;
        position: absolute;
        right: 100%;
        bottom: 100%;
    }

    .toggle-all + label {
        width: 60px;
        height: 34px;
        font-size: 0;
        position: absolute;
        top: -52px;
        left: -13px;
        -webkit-transform: rotate(90deg);
        transform: rotate(90deg);
    }

    .toggle-all + label:before {
        content: '❯';
        font-size: 22px;
        color: #e6e6e6;
        padding: 10px 27px 10px 27px;
    }

    .toggle-all:checked + label:before {
        color: #737373;
    }

    .todo-list {
        margin: 0;
        padding: 0;
        list-style: none;
    }

    .todo-list li {
        position: relative;
        font-size: 24px;
        border-bottom: 1px solid #ededed;
    }

    .todo-list li:last-child {
        border-bottom: none;
    }

    .todo-list li.editing {
        border-bottom: none;
        padding: 0;
    }

    .todo-list li.editing .edit {
        display: block;
        width: calc(100% - 43px);
        padding: 12px 16px;
        margin: 0 0 0 43px;
    }

    .todo-list li.editing .view {
        display: none;
    }

    .todo-list li .toggle {
        text-align: center;
        width: 40px;
        /* auto, since non-WebKit browsers doesn't support input styling */
        height: auto;
        position: absolute;
        top: 0;
        bottom: 0;
        margin: auto 0;
        border: none; /* Mobile Safari */
        -webkit-appearance: none;
        appearance: none;
    }

    .todo-list li .toggle {
        opacity: 0;
    }

    .todo-list li .toggle + label {
        /*
            Firefox requires `#` to be escaped - https://bugzilla.mozilla.org/show_bug.cgi?id=922433
            IE and Edge requires *everything* to be escaped to render, so we do that instead of just the `#` - https://developer.microsoft.com/en-us/microsoft-edge/platform/issues/7157459/
        */
        background-image: url('data:image/svg+xml;utf8,%3Csvg%20xmlns%3D%22http%3A//www.w3.org/2000/svg%22%20width%3D%2240%22%20height%3D%2240%22%20viewBox%3D%22-10%20-18%20100%20135%22%3E%3Ccircle%20cx%3D%2250%22%20cy%3D%2250%22%20r%3D%2250%22%20fill%3D%22none%22%20stroke%3D%22%23ededed%22%20stroke-width%3D%223%22/%3E%3C/svg%3E');
        background-repeat: no-repeat;
        background-position: center left;
    }

    .todo-list li .toggle:checked + label {
        background-image: url('data:image/svg+xml;utf8,%3Csvg%20xmlns%3D%22http%3A//www.w3.org/2000/svg%22%20width%3D%2240%22%20height%3D%2240%22%20viewBox%3D%22-10%20-18%20100%20135%22%3E%3Ccircle%20cx%3D%2250%22%20cy%3D%2250%22%20r%3D%2250%22%20fill%3D%22none%22%20stroke%3D%22%23bddad5%22%20stroke-width%3D%223%22/%3E%3Cpath%20fill%3D%22%235dc2af%22%20d%3D%22M72%2025L42%2071%2027%2056l-4%204%2020%2020%2034-52z%22/%3E%3C/svg%3E');
    }

    .todo-list li label {
        word-break: break-all;
        padding: 15px 15px 15px 60px;
        display: block;
        line-height: 1.2;
        transition: color 0.4s;
        font-weight: 400;
        color: #4d4d4d;
    }

    .todo-list li.completed label {
        color: #cdcdcd;
        text-decoration: line-through;
    }

    .todo-list li .destroy {
        display: none;
        position: absolute;
        top: 0;
        right: 10px;
        bottom: 0;
        width: 40px;
        height: 40px;
        margin: auto 0;
        font-size: 30px;
        color: #cc9a9a;
        margin-bottom: 11px;
        transition: color 0.2s ease-out;
    }

    .todo-list li .destroy:hover {
        color: #af5b5e;
    }

    .todo-list li .destroy:after {
        content: '×';
    }

    .todo-list li:hover .destroy {
        display: block;
    }

    .todo-list li .edit {
        display: none;
    }

    .todo-list li.editing:last-child {
        margin-bottom: -1px;
    }

    .footer {
        padding: 10px 15px;
        height: 20px;
        text-align: center;
        font-size: 15px;
        border-top: 1px solid #e6e6e6;
    }

    .footer:before {
        content: '';
        position: absolute;
        right: 0;
        bottom: 0;
        left: 0;
        height: 50px;
        overflow: hidden;
        box-shadow: 0 1px 1px rgba(0, 0, 0, 0.2),
                    0 8px 0 -3px #f6f6f6,
                    0 9px 1px -3px rgba(0, 0, 0, 0.2),
                    0 16px 0 -6px #f6f6f6,
                    0 17px 2px -6px rgba(0, 0, 0, 0.2);
    }

    .todo-count {
        float: left;
        text-align: left;
    }

    .todo-count strong {
        font-weight: 300;
    }

    .filters {
        margin: 0;
        padding: 0;
        list-style: none;
        position: absolute;
        right: 0;
        left: 0;
    }

    .filters li {
        display: inline;
    }

    .filters li a {
        color: inherit;
        margin: 3px;
        padding: 3px 7px;
        text-decoration: none;
        border: 1px solid transparent;
        border-radius: 3px;
    }

    .filters li a:hover {
        border-color: rgba(175, 47, 47, 0.1);
    }

    .filters li a.selected {
        border-color: rgba(175, 47, 47, 0.2);
    }

    .clear-completed,
    html .clear-completed:active {
        float: right;
        position: relative;
        line-height: 20px;
        text-decoration: none;
        cursor: pointer;
    }

    .clear-completed:hover {
        text-decoration: underline;
    }

    .info {
        margin: 65px auto 0;
        color: #4d4d4d;
        font-size: 11px;
        text-shadow: 0 1px 0 rgba(255, 255, 255, 0.5);
        text-align: center;
    }

    .info p {
        line-height: 1;
    }

    .info a {
        color: inherit;
        text-decoration: none;
        font-weight: 400;
    }

    .info a:hover {
        text-decoration: underline;
    }

    /*
        Hack to remove background from Mobile Safari.
        Can't use it globally since it destroys checkboxes in Firefox
    */
    @media screen and (-webkit-min-device-pixel-ratio:0) {
        .toggle-all,
        .todo-list li .toggle {
            background: none;
        }

        .todo-list li .toggle {
            height: 40px;
        }
    }

    @media (max-width: 430px) {
        .footer {
            height: 50px;
        }

        .filters {
            bottom: 10px;
        }
    }
    ```
    </details>

    Updated `index.html`:

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

1. If you try to run the app now, it panics because it can't find an element with id `app`. That's why we have to update our `start` function in `lib.rs`. 

    Our root element:

    ```html
    <section class="todoapp"></section>
    ```

    And updated `start`:
    ```rust
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

    _Note:_ Best practice is to add an element id, but we want to respect predefined template HTML in this case.

1. Let's include the remaining template HTML into our `view` function to test if everything works => Create a new file `/template.html` and include it during compilation into our `view` body.

    <details>
    <summary><code>template.html</code></summary>

    ```html
    <header class="header">
        <h1>todos</h1>
        <input class="new-todo" placeholder="What needs to be done?" autofocus>
    </header>
    <!-- This section should be hidden by default and shown when there are todos -->
    <section class="main">
        <input id="toggle-all" class="toggle-all" type="checkbox">
        <label for="toggle-all">Mark all as complete</label>
        <ul class="todo-list">
            <!-- These are here just to show the structure of the list items -->
            <!-- List items should get the class `editing` when editing and `completed` when marked as completed -->
            <li class="completed">
                <div class="view">
                    <input class="toggle" type="checkbox" checked>
                    <label>Taste JavaScript</label>
                    <button class="destroy"></button>
                </div>
                <input class="edit" value="Create a TodoMVC template">
            </li>
            <li>
                <div class="view">
                    <input class="toggle" type="checkbox">
                    <label>Buy a unicorn</label>
                    <button class="destroy"></button>
                </div>
                <input class="edit" value="Rule the web">
            </li>
        </ul>
    </section>
    <!-- This footer should hidden by default and shown when there are todos -->
    <footer class="footer">
        <!-- This should be `0 items left` by default -->
        <span class="todo-count"><strong>0</strong> item left</span>
        <!-- Remove this if you don't implement routing -->
        <ul class="filters">
            <li>
                <a class="selected" href="#/">All</a>
            </li>
            <li>
                <a href="#/active">Active</a>
            </li>
            <li>
                <a href="#/completed">Completed</a>
            </li>
        </ul>
        <!-- Hidden if no completed items are left ↓ -->
        <button class="clear-completed">Clear completed</button>
    </footer>
    </section>
    ```
    </details>

    `view` function:

    ```rust
    fn view(model: &Model) -> Vec<Node<Msg>> {
        raw![include_str!("../template.html")]
    }
    ```
    _Note:_ Notice change `Node<Msg>` to `Vec<Node<Msg>>`. We can't predict if `raw!` (or `md!`) contains one or multiple elements, so it always returns `Vec`.

1. Refresh you browser and you should see something like:

    ![TodoMVC template screen](/static/images/todomvc_template_screen.png)


1. Let's rewrite `template.html` to Rust! Then you can delete `template.html`. And don't forget to manually test the code in your browser after this step and after all next ones.

    <details>
    <summary>Updated <code>view</code></summary>

    ```rust
    fn view(model: &Model) -> Vec<Node<Msg>> {
        vec![
            header![C!["header"],
                h1!["todos"],
                input![C!["new-todo"],
                    attrs!{At::Placeholder => "What needs to be done?", At::AutoFocus => AtValue::None},
                ]
            ],
            // This section should be hidden by default and shown when there are todos
            section![C!["main"],
                input![C!["toggle-all"], attrs!{At::Id => "toggle-all", At::Type => "checkbox"}],
                label![attrs!{At::For => "toggle-all"}, "Mark all as complete"],
                ul![C!["todo-list"],
                    // These are here just to show the structure of the list items
                    // List items should get the class `editing` when editing and `completed` when marked as completed
                    li![C!["completed"],
                        div![C!["view"],
                            input![C!["toggle"], attrs!{At::Type => "checkbox", At::Checked => AtValue::None}],
                            label!["Taste JavaScript"],
                            button![C!["destroy"]],
                        ],
                        input![C!["edit"], attrs!{At::Value => "Create a TodoMVC template"}]
                    ],
                    li![
                        div![C!["view"],
                            input![C!["toggle"], attrs!{At::Type => "checkbox"}],
                            label!["Buy a unicorn"],
                            button![C!["destroy"]],
                        ],
                        input![C!["edit"], attrs!{At::Value => "Rule the web"}]
                    ]
                ]
            ],
            // This footer should hidden by default and shown when there are todos
            footer![C!["footer"],
                // This should be `0 items left` by default
                span![C!["todo-count"],
                    strong!["0"],
                    " item left",
                ],
                ul![C!["filters"],
                    li![
                        a![C!["selected"],
                            attrs!{At::Href => "#/"},
                            "All",
                        ],
                    ],
                    li![
                        a![
                            attrs!{At::Href => "#/active"},
                            "Active",
                        ],
                    ],
                    li![
                        a![
                            attrs!{At::Href => "#/completed"},
                            "Completed",
                        ],
                    ],
                ],
                // Hidden if no completed items are left ↓
                button![C!["clear-completed"],
                    "Clear completed"
                ]
            ]
        ]
    }
    ```
    </details>

1. As you can see, our `view` function is pretty big and unreadable, let's refactor it by splitting it into shorter functions.

    <details>
    <summary>Refactored <code>view</code> with "sub-views"</summary>

    ```rust
    fn view(model: &Model) -> Vec<Node<Msg>> {
        vec![
            view_header(),
            // This section should be hidden by default and shown when there are todos
            view_main(),
            // This footer should hidden by default and shown when there are todos
            view_footer(),
        ]
    }

    // ------ header ------

    fn view_header() -> Node<Msg> {
        header![C!["header"],
            h1!["todos"],
            input![C!["new-todo"],
                attrs!{At::Placeholder => "What needs to be done?", At::AutoFocus => AtValue::None},
            ]
        ]
    }

    // ------ main ------

    fn view_main() -> Node<Msg> {
        section![C!["main"],
            view_toggle_all(),
            view_todo_list(),
        ]
    }

    fn view_toggle_all() -> Vec<Node<Msg>> {
        vec![
            input![C!["toggle-all"], attrs!{At::Id => "toggle-all", At::Type => "checkbox"}],
            label![attrs!{At::For => "toggle-all"}, "Mark all as complete"],
        ]
    }

    fn view_todo_list() -> Node<Msg> {
        ul![C!["todo-list"],
            // These are here just to show the structure of the list items
            // List items should get the class `editing` when editing and `completed` when marked as completed
            li![C!["completed"],
                div![C!["view"],
                    input![C!["toggle"], attrs!{At::Type => "checkbox", At::Checked => AtValue::None}],
                    label!["Taste JavaScript"],
                    button![C!["destroy"]],
                ],
                input![C!["edit"], attrs!{At::Value => "Create a TodoMVC template"}]
            ],
            li![
                div![C!["view"],
                    input![C!["toggle"], attrs!{At::Type => "checkbox"}],
                    label!["Buy a unicorn"],
                    button![C!["destroy"]],
                ],
                input![C!["edit"], attrs!{At::Value => "Rule the web"}]
            ]
        ]
    }

    // ------ footer ------

    fn view_footer() -> Node<Msg> {
        footer![C!["footer"],
            // This should be `0 items left` by default
            span![C!["todo-count"],
                strong!["0"],
                " item left",
            ],
            view_filters(),
            // Hidden if no completed items are left ↓
            button![C!["clear-completed"],
                "Clear completed"
            ]
        ]
    }

    fn view_filters() -> Node<Msg> {
        ul![C!["filters"],
            li![
                a![C!["selected"],
                    attrs!{At::Href => "#/"},
                    "All",
                ],
            ],
            li![
                a![
                    attrs!{At::Href => "#/active"},
                    "Active",
                ],
            ],
            li![
                a![
                    attrs!{At::Href => "#/completed"},
                    "Completed",
                ],
            ],
        ]
    }
    ```
    </details>

1. We'll finally use `Model` data in our `view` and we'll try to implement some basic logic based on these data. Let's start with `view` function.
    - `main` and `footer` should be hidden by default and shown when there are todos. So we can wrap `view_main()` and `view_footer()` into one condition by `IF!`:  

    ```rust
    fn view(model: &Model) -> Vec<Node<Msg>> {
        vec![
            view_header(),
            IF!(not(model.todos.is_empty()) => vec![
                view_main(), 
                view_footer(),
            ]),
        ]
    }
    ```
    - However it causes compilation errors because the root `vec![...]` expects only `Node<Msg>` as items but our `IF!` returns `Option<Vec<Node<Msg>>>`. Fortunately, there is macro `nodes!` that aligns all types to make the compiler happy:

    ```rust
    fn view(model: &Model) -> Vec<Node<Msg>> {
        nodes![
            view_header(),
            IF!(not(model.todos.is_empty()) => vec![
                view_main(), 
                view_footer(),
            ]),
        ]
    }
    ```

1. Connect `view_header` to `Model` data. 

    ```rust
    fn view(model: &Model) -> Vec<Node<Msg>> {
        ...
            view_header(&model.new_todo_title),
    ...

    fn view_header(new_todo_title: &str) -> Node<Msg> {
        ...
            input![C!["new-todo"],
                attrs!{
                    At::Placeholder => "What needs to be done?", 
                    At::AutoFocus => AtValue::None,
                    At::Value => new_todo_title,
                },
            ]
        ]
    }
    ```

1. Connect `view_todo_list` to `Model` data. 

    ```rust
    fn view(model: &Model) -> Vec<Node<Msg>> {
        ...
                view_main(&model.todos, model.selected_todo.as_ref()), 
    ...

    fn view_main(todos: &BTreeMap<Ulid, Todo>, selected_todo: Option<&SelectedTodo>) -> Node<Msg> {
        ...
        view_todo_list(todos, selected_todo),
    ]

    ...

    fn view_todo_list(todos: &BTreeMap<Ulid, Todo>, selected_todo: Option<&SelectedTodo>) -> Node<Msg> {
        ul![C!["todo-list"],
            todos.values().map(|todo| {
                let is_selected = Some(todo.id) == selected_todo.map(|selected_todo| selected_todo.id);

                li![C![IF!(todo.completed => "completed"), IF!(is_selected => "editing")],
                    div![C!["view"],
                        input![C!["toggle"], attrs!{At::Type => "checkbox", At::Checked => todo.completed.as_at_value()}],
                        label![&todo.title],
                        button![C!["destroy"]],
                    ],
                    IF!(is_selected => input![C!["edit"], attrs!{At::Value => selected_todo.unwrap().title}]),
                ]
            })
        ]
    }
    ```
    Tips:
    
    - It's often better to pass `Option<&Item>` instead of `&Option<Item>` because the former one plays better with other `Option` methods and especially with combinators like [map](https://doc.rust-lang.org/std/option/enum.Option.html#method.map) and [and_then](https://doc.rust-lang.org/std/option/enum.Option.html#method.and_then). [Option::as_ref](https://doc.rust-lang.org/std/option/enum.Option.html#method.as_ref) does the conversion.
    
    - Use `.unwrap()` (instead of `expect("...")`) in places when you are SURE the code will work but the logic isn't encodable by the Rust type system. In our case - we know that if `is_selected` is `true` then `selected_todo` contains `SelectedTodo`.

1. Connect `view_toggle_all` to `Model` data. 

    ```rust
    fn view_main(todos: &BTreeMap<Ulid, Todo>, selected_todo: Option<&SelectedTodo>) -> Node<Msg> {
        ...
            view_toggle_all(todos),
        ...
    }

    fn view_toggle_all(todos: &BTreeMap<Ulid, Todo>) -> Vec<Node<Msg>> {
        let all_completed = todos.values().all(|todo| todo.completed);
        vec![
            input![C!["toggle-all"], 
                attrs!{
                    At::Id => "toggle-all", At::Type => "checkbox", At::Checked => all_completed.as_at_value()
                }
            ],
            label![attrs!{At::For => "toggle-all"}, "Mark all as complete"],
        ]
    }
    ```

1. Connect `view_filters` to `Model` data. 
   
   - We would like to iterate all available filters and render them (like we did with todos). However there is no way how to do that now. The safest way is to use a macro that writes an iterator for us automatically from defined `Filter` variants. There are multiple crates that can do it, but we'll choose the most used one - [strum](https://crates.io/crates/strum). The interesting `strum` part for us is [EnumIter](https://github.com/Peternator7/strum/wiki/Derive-EnumIter).

   - And we would like to compare `Filter` variants among each other so we can find out if the currently iterated variant is the selected one by the user. We don't need an external dependency for it, because Rust can derive the required traits [Eq](https://doc.rust-lang.org/std/cmp/trait.Eq.html) and [PartialEq](https://doc.rust-lang.org/std/cmp/trait.PartialEq.html).

   - While we are deriving traits, we can also add traits [Copy](https://doc.rust-lang.org/std/marker/trait.Copy.html) and [Clone](https://doc.rust-lang.org/std/clone/trait.Clone.html) - it's a best practice and it'll probably make some code parts easier to write.

    Updated `Cargo.toml`:

    ```toml
    [dependencies]
    strum = "0.18.0"
    strum_macros = "0.18.0"
    ...
    ```

    Updated `lib.rs`:

    ```rust
    use seed::{prelude::*, *};

    use std::collections::BTreeMap;

    use strum_macros::EnumIter;
    use strum::IntoEnumIterator;
    use ulid::Ulid;

    ...

    #[derive(Copy, Clone, Eq, PartialEq, EnumIter)]
    enum Filter {
        ...
    ...

    fn view(model: &Model) -> Vec<Node<Msg>> {
        ...
                view_footer(model.filter),
        ...
    }

    ...

    fn view_footer(todos: &BTreeMap<Ulid, Todo>, selected_filter: Filter) -> Node<Msg> {
        ...
            view_filters(selected_filter),
        ...
    }

    ...

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
    ```
    _Note_: We've split our `use` imports into 3 groups: Seed, std, external. 

1. Connect `view_footer` to `Model` data. 

    ```rust
    fn view(model: &Model) -> Vec<Node<Msg>> {
        ...
        view_footer(&model.todos, model.filter),
    ...

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
                    "Clear completed"
                ]
            )
        ]
    }
    ```

    _Note_:
    ```rust
    format!(" item{} left", if active_count == 1 { "" } else { "s" })
    ```
    is simple but too naive solution for natural language problems. Imagine you would like to write it in Czech:
    ```
    0 položek zbývá  (== 0 items left)
    1,2,3 položky zbývají
    4+ položek zbývá
    ```
    I think it's clear it becomes cumbersome quickly. There are localization systems like [Fluent](https://www.projectfluent.org/) that mitigate these problems. And we have [i18n example](https://github.com/seed-rs/seed/tree/0a538f03d6aeb56b00d997c80a666e388279a727/examples/i18n) that demonstrates how to leverage `Fluent`.

1. The last two things left to connect - the first one is `input_element`.

    ```rust
    fn view_todo_list(todos: &BTreeMap<Ulid, Todo>, selected_todo: Option<&SelectedTodo>) -> Node<Msg> {
        ...
                    IF!(is_selected => {
                        let selected_todo = selected_todo.unwrap();
                        input![C!["edit"], 
                            el_ref(&selected_todo.input_element), 
                            attrs!{At::Value => selected_todo.title},
                        ]
                    }),
        ...
    }
    ```

    This code associates the DOM input element with the field `input_element`. It allows us to safety access the DOM element in our `update` function (you'll see how in the next chapters). There are multiple examples leveraging element references, however the best demonstration is in the [canvas example](https://github.com/seed-rs/seed/blob/0a538f03d6aeb56b00d997c80a666e388279a727/examples/canvas/src/lib.rs#L24).

1. And the last thing is _element keys_. They are optional for the most cases but they'll help to optimize rendering and they are required when you want to animate your list items by CSS animations. We have only two lists in our `view` - filters and todos. I would recommend to add keys for todos because they are more "dynamic", their count will be often higher and there is a chance that we'll animate them in the future. Look at [el_key example](https://github.com/seed-rs/seed/tree/0a538f03d6aeb56b00d997c80a666e388279a727/examples/el_key) when you want to know more about element keys.

    ```rust
    fn view_todo_list(todos: &BTreeMap<Ulid, Todo>, selected_todo: Option<&SelectedTodo>) -> Node<Msg> {
        ...
                li![C![...],
                    el_key(&todo.id),
        ...
    }
    ```

---

We've done it! You should see something like:

![TodoMVC after view screen](/static/images/todomvc_after_view_screen.png)

Perhaps you noticed that we haven't written any event handlers yet - we'll be writing them together with `update` function in the next chapter. And we'll finish filtering once we understand subscriptions, routing and link building.
