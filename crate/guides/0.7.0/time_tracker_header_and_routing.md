# Header & Routing

Let's make the app a little bit more interesting. We'll write the header and then we'll be able to use the header links to switch between our pages.

## Urls

Write to `lib.rs`:

```rust

const CLIENTS_AND_PROJECTS: &str = "clients_and_projects";
const TIME_TRACKER: &str = "time_tracker";
const TIME_BLOCKS: &str = "time_blocks";
const SETTINGS: &str = "settings";

// ------ ------
//     Init
// ------ ------

...

// ------ ------
//     Urls
// ------ ------

struct_urls!();
impl<'a> Urls<'a> {
    fn home(self) -> Url {
        self.base_url()
    }
    fn clients_and_projects(self) -> Url {
        self.base_url().add_path_part(CLIENTS_AND_PROJECTS)
    }
    fn time_tracker(self) -> Url {
        self.base_url().add_path_part(TIME_TRACKER)
    }
    fn time_blocks(self) -> Url {
        self.base_url().add_path_part(TIME_BLOCKS)
    }
    fn settings(self) -> Url {
        self.base_url().add_path_part(SETTINGS)
    }
}

// ------ ------
//    Update
// ------ ------
```

_Note:_ If you want to learn how to create nested routes, see the example [pages](https://github.com/seed-rs/seed/tree/8d04fcde8a22f785fa20d28cb2f1a9b3b2d7e790/examples/pages).

## Header HTML + toggle on hamburger click

<details>
<summary>Code</summary>

```rust
fn init(url: Url, _: &mut impl Orders<Msg>) -> Model {
    Model {
        ...
        menu_visible: false,
    }
}

...

struct Model {
    ...
    menu_visible: bool,
}

...

enum Msg {
    ...
    ToggleMenu,
}

...

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        ...
        Msg::ToggleMenu => model.menu_visible = not(model.menu_visible),
    }
}

...

fn view(model: &Model) -> Node<Msg> {
    view_navbar(model.menu_visible, &model.base_url, model.ctx.user.as_ref())
}

fn view_navbar(menu_visible: bool, base_url: &Url, user: Option<&User>) -> Node<Msg> {
    nav![
        C!["navbar"],
        attrs!{
            At::from("role") => "navigation",
            At::AriaLabel => "main navigation",
        },
        view_brand_and_hamburger(menu_visible, base_url),
        view_navbar_menu(menu_visible, base_url, user),
    ]
}

fn view_brand_and_hamburger(menu_visible: bool, base_url: &Url) -> Node<Msg> {
    div![
        C!["navbar-brand"],
        // ------ Logo ------
        a![
            C!["navbar-item", "has-text-weight-bold", "is-size-3"],
            attrs!{At::Href => Urls::new(base_url).home()},
            "TT"
        ],
        // ------ Hamburger ------
        a![
            C!["navbar-burger", "burger", IF!(menu_visible => "is-active")],
            attrs!{
                At::from("role") => "button",
                At::AriaLabel => "menu",
                At::AriaExpanded => menu_visible,
            },
            ev(Ev::Click, |_| Msg::ToggleMenu),
            span![attrs!{At::AriaHidden => "true"}],
            span![attrs!{At::AriaHidden => "true"}],
            span![attrs!{At::AriaHidden => "true"}],
        ]
    ]
}

fn view_navbar_menu(menu_visible: bool, base_url: &Url, user: Option<&User>) -> Node<Msg> {
    div![
        C!["navbar-menu", IF!(menu_visible => "is-active")],
        view_navbar_menu_start(base_url),
        view_navbar_menu_end(base_url, user),
    ]
}

fn view_navbar_menu_start(base_url: &Url) -> Node<Msg> {
    div![
        C!["navbar-start"],
        a![
            C!["navbar-item"],
            attrs!{At::Href => Urls::new(base_url).time_tracker()},
            "Time Tracker",
        ],
        a![
            C!["navbar-item"],
            attrs!{At::Href => Urls::new(base_url).clients_and_projects()},
            "Clients & Projects",
        ],
        a![
            C!["navbar-item"],
            attrs!{At::Href => Urls::new(base_url).time_blocks()},
            "Time Blocks",
        ],
    ]
}

fn view_navbar_menu_end(base_url: &Url, user: Option<&User>) -> Node<Msg> {
     div![
        C!["navbar-end"],
        div![
            C!["navbar-item"],
            div![
                C!["buttons"],
                if let Some(user) = user {
                    view_buttons_for_logged_in_user(base_url, user)
                } else {
                    view_buttons_for_anonymous_user()
                }
            ]
        ]
    ]
}

fn view_buttons_for_logged_in_user(base_url: &Url, user: &User) -> Vec<Node<Msg>> {
    vec![
        a![
            C!["button", "is-primary"],
            attrs![
                At::Href => Urls::new(base_url).settings(),
            ],
            strong![&user.username],
        ],
        a![
            C!["button", "is-light"],
            attrs![
                // @TODO: Write the correct href.
                At::Href => "/"
            ],
            "Log out",
        ]
    ]
}

fn view_buttons_for_anonymous_user() -> Vec<Node<Msg>> {
    vec![
        a![
            C!["button", "is-primary"],
            attrs![
                // @TODO: Write the correct href.
                At::Href => "/"
            ],
            strong!["Sign up"],
        ],
        a![
            C!["button", "is-light"],
            attrs![
                // @TODO: Write the correct href.
                At::Href => "/"
            ],
            "Log in",
        ]
    ]
}
```

</details>

_Note:_ We've written HTML attribute `role` as a custom attribute - `At::from("role")` because the typed version `At::Role` hasn't been included in the Seed yet. We are constantly improving and adding typed attributes, events and other HTML items.

![Header Anonymous](/static/images/time_tracker_header_anonymous.png)

Update `User` "mock":
```rust
fn init(url: Url, _: &mut impl Orders<Msg>) -> Model {
    Model {
        ctx: Context {
            // user: None,
            user: Some(User {
                username: "John".to_owned(),
                email: "john@email.com".to_owned(),
            }),
            ...
```

![Header Anonymous](/static/images/time_tracker_header_logged_in.png)

## Hide header on click

We need to listen for all clicks on the page so we can hide the menu when necessary:
```rust
fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.stream(streams::window_event(Ev::Click, |_| Msg::HideMenu));
    ...
```

Set `menu_visible` to `false` when the menu is visible:

_Note:_ We don't want to rerender the page when nothing in `Model` has been changed. It's a kind of micro-optimization but the code `orders.skip()` also signals the reader that we really haven't changed anything and it helps with `view` debugging a little bit because you wouldn't see all debug data in the console log twice on click.

```rust
enum Msg {
    ...
    HideMenu,
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        ...
        Msg::HideMenu => {
            if model.menu_visible {
                model.menu_visible = false;
            } else {
                orders.skip();
            }
        },
    }
}
```

We have to stop `click` event [propagation](https://developer.mozilla.org/en-US/docs/Web/API/Event/stopPropagation) from the hamburger button to the `window`. Otherwise when the user clicks the hamburger, `Msg::ToggleMenu` and then `Msg::HideMenu` is fired - the user wouldn't be able to open the menu. An alternative solution would be to compare the event `target` with the button element but it would be error-prone and cumbersome.

```rust
fn view_brand_and_hamburger(menu_visible: bool, base_url: &Url) -> Node<Msg> {
    div![
        ...
        // ------ Hamburger ------
        a![
            ...
            ev(Ev::Click, |event| {
                event.stop_propagation();
                Msg::ToggleMenu
            }),
```

_WARNING_: `event.stop_propagation()` or `event.prevent_default()` in combination with elements like `a` could disable routing for the particular element because Seed intercepts `click` events to provide automatic routing ability.

## Routing + wiring pages

We need to `subscribe` to `subs::UrlChanged(url)`. (`Msg::UrlChanged` expects this type as the only argument so `Msg::UrlChanged` constructor effectively works as a subscription handler.)

And we pick the right page.

```rust
fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders
        .subscribe(Msg::UrlChanged)
        .stream ...

    Model {
        ...
        page: Page::init(url, orders),
        ...
}
```

We `init` a page according to the provided url.


```rust
// ------ Page ------

enum Page {
    ...
}

impl Page {
    fn init(mut url: Url, orders: &mut impl Orders<Msg>) -> Self {
        match url.remaining_path_parts().as_slice() {
            [] => Self::Home,
            [CLIENTS_AND_PROJECTS] => Self::ClientsAndProjects(
                page::clients_and_projects::init(url, &mut orders.proxy(Msg::ClientsAndProjectsMsg))
            ),
            [TIME_TRACKER] => Self::TimeTracker(
                page::time_tracker::init(url, &mut orders.proxy(Msg::TimeTrackerMsg))
            ),
            [TIME_BLOCKS] => Self::TimeBlocks(
                page::time_blocks::init(url, &mut orders.proxy(Msg::TimeBlocksMsg))
            ),
            [SETTINGS] => Self::Settings(
                page::settings::init(url, &mut orders.proxy(Msg::SettingsMsg))
            ),
            _ => Self::NotFound,
        }
    }
}
```

We need to "redirect" messages to the associated pages (to their `update` functions). However there is a problem - our root `Msg` and page `Msg`s are different types. We can't just pass items with their generic type `Ms` set to the root `Msg` directly to the page functions. 

In this case, the problem is `Orders<Msg>`. However we can allow to pass it into pages (aka sub-modules) by calling `orders.proxy(msg_mapper)`, where `msg_mapper` should be the root `Msg` constructor for the variant that contains page's `Msg`.

_Note:_ The method is called `proxy` and not something like `map_msg` because we don't modify the original `orders` at all, we only create a "projection/proxy" that can transform all sub-module's `Msg`s into the parent's `Msg`s.

```rust
fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::UrlChanged(subs::UrlChanged(url)) => model.page = Page::init(url, orders),
        ...

        // ------ pages ------

        Msg::ClientsAndProjectsMsg(msg) => {
            if let Page::ClientsAndProjects(model) = &mut model.page {
                page::clients_and_projects::update(msg, model, &mut orders.proxy(Msg::ClientsAndProjectsMsg))
            }
        }
        Msg::TimeTrackerMsg(msg) => {
            if let Page::TimeTracker(model) = &mut model.page {
                page::time_tracker::update(msg, model, &mut orders.proxy(Msg::TimeTrackerMsg))
            }
        },
        Msg::TimeBlocksMsg(msg) => {
            if let Page::TimeBlocks(model) = &mut  model.page {
                page::time_blocks::update(msg, model, &mut orders.proxy(Msg::TimeBlocksMsg))
            }
        }
        Msg::SettingsMsg(msg) => {
            if let Page::Settings(model) = &mut model.page {
                page::settings::update(msg, model, &mut orders.proxy(Msg::SettingsMsg))
            }
        }
    }
}
```

Just like we "redirected" messages to the corresponding page `update` function, we want to handle the page content rendering by the appropriate page `view` function.

This time we don't call `proxy` but `map_msg`. In this case we simply cast `Node<page::xx::Msg>` to `Node<Msg>`. Then each fired page `Msg` is automatically converted to the root `Msg` according to the `msg_mapper` callback (root `Msg` constructors). 

```rust
fn view(model: &Model) -> Vec<Node<Msg>> {
    vec![
        view_navbar(model.menu_visible, &model.base_url, model.ctx.user.as_ref()),
        view_content(&model.page),
    ]
}

// ----- view_content ------

fn view_content(page: &Page) -> Node<Msg> {
    div![
        C!["container"],
        match page {
            Page::Home => page::home::view(),
            Page::ClientsAndProjects(model) => page::clients_and_projects::view(model).map_msg(Msg::ClientsAndProjectsMsg),
            Page::TimeTracker(model) => page::time_tracker::view(model).map_msg(Msg::TimeTrackerMsg),
            Page::TimeBlocks(model) => page::time_blocks::view(model).map_msg(Msg::TimeBlocksMsg),
            Page::Settings(model) => page::settings::view(model).map_msg(Msg::SettingsMsg),
            Page::NotFound => page::not_found::view(),
        }
    ]
}

// ----- view_navbar ------
```

## Active menu items

We want to highlight the menu item associated to the currently displayed `page`.

We have to pass down `page` from `Model`:

```rust
fn view(model: &Model) -> Vec<Node<Msg>> {
    vec![
        view_navbar(model.menu_visible, &model.base_url, model.ctx.user.as_ref(), &model.page),
        ...
}

...

fn view_navbar(menu_visible: bool, base_url: &Url, user: Option<&User>, page: &Page) -> Node<Msg> {
    nav![
        ...
        view_navbar_menu(menu_visible, base_url, user, page),
    ]
}

...

fn view_navbar_menu(menu_visible: bool, base_url: &Url, user: Option<&User>, page: &Page) -> Node<Msg> {
    div![
        ...
        view_navbar_menu_start(base_url, page),
        ...
    ]
}
```

And then add class `is-tab` so the selected menu item shows a nice underline when it's marked as active by the class `is-active`.

_Note:_ The Rust macro [matches!](https://doc.rust-lang.org/std/macro.matches.html) helps us to find out if the associated page is currently selected.

```rust
fn view_navbar_menu_start(base_url: &Url, page: &Page) -> Node<Msg> {
    div![
        C!["navbar-start"],
        a![
            C!["navbar-item", "is-tab", IF!(matches!(page, Page::TimeTracker(_)) => "is-active"),],
            ...
        ],
        a![
            C!["navbar-item", "is-tab", IF!(matches!(page, Page::ClientsAndProjects(_)) => "is-active"),],
            ...
        ],
        a![
            C!["navbar-item", "is-tab", IF!(matches!(page, Page::TimeBlocks(_)) => "is-active"),],
            ...
        ],
    ]
}

```

## Final result

![Header Final Result](/static/images/time_tracker_final_result.gif)
(Recorded by [ScreenToGif](https://www.screentogif.com/))

---

## Base path/url

Let's experiment a bit and setup a custom base url for our app. It's useful when you want to deploy the app to a non-root path - for instance it will be served from `example.com/ui/` instead of a standard `example.com/`.

Edit `index.html`:
```html
<head>
    <base href="/ui/">/">
    ...
```

Now you website and routing work for both url:
- [localhost:8000/](http://localhost:8000/)
- [localhost:8000/ui](http://localhost:8000/ui)

And other base path like [localhost:8000/foo](http://localhost:8000/foo) would still show `NotFound` page.

_How it works:_ Do you still remember how routing works from the previous TodoMVC example - especially `url.next_path_part()`? I hope so. Seed is searching for a `base` element while the app is starting. When Seed finds the elements, it saves the associated `href` value (aka base path). And then, it compares the base path with the current url on each url change or root `init` call. If the base path is a prefix of the current url, it just basically calls `url.next_path_part` multiple times to "skip" the prefix. After that, the url is passed into the app.

_Note:_ You can remove `base` element from your `index.html` now, we don't need it for the rest of the tutorial.


## Boilerplate

You may get the impression that there is relatively a lot of boilerplate caused by page functions wiring. It's a well-known The Elm architecture trade-off for explicitness and flexibility. Communication between parents and children is provided by simple function calls. And you don't need to define unnecessary `Model`s or `update`s when simpler sub-modules/pages require only a `view` function. Also you can change function arguments or even alter a return value for standard functions like `update` because you don't have to strictly follow any traits.

But we know that TEA isn't a silver bullet for all use-cases. That's why we are integrating [Seed Hooks & Atoms](https://seed-style-hooks.netlify.app/hooks_home) into the Seed so you can mitigate boilerplate when it makes sense - especially when you want to write a component library or when you have to store a lot of non-business variables in your `Model`s.

<details>
<summary>Alex Korban's opinion on TEA boilerplate</summary>

> Alex is the author of the book [Practical Elm for a Busy Developer](https://korban.net/elm/book/). The opinion below is a copy-pasted part of our mail conversation.

There are a couple of problems with boilerplate:
- it's tedious to write 
- it can cause bugs due to large amount of typing and copy/paste errors.

I think the complaints are overwhelmingly caused by the first issue which is actually insignificant. The second one can be more serious in languages like JS, but in Elm it's mitigated to a large extent by the type system. Besides, we can still use functions to remove a lot of duplication. 

It can also be argued that extensive boilerplate slows down development by requiring more time to read and write code. However, in Elm I probably spend more time on satisfying the constraints imposed by the type system (for example) than on the various aspects of boilerplate. So again, I struggle to see it as a significant problem. (I think that in any language in existence, we are forced to specify an absurd amount of minute detail, but that's another topic of discussion.)

Another thing to consider is the price of removing boilerplate. Sure, it can be hidden behind some kind of abstraction, but solid, non-leaky abstractions are notoriously hard to design, and users of the language pay with a higher learning curve and possibly a higher cognitive load when working with code. Can we be sure that developers will achieve fluency no matter how complex the abstractions involved? I'm not sure. On the other hand, I think the general opinion is that it's fairly easy to dive into unfamiliar Elm code bases and figure out what's going on, because all the workings of the code are explicitly laid out with the help of "boilerplate".

</details>

---

And that's it! We have a solid foundation for further development. We'll start to integrate `Auth0` into our app in the next chapter so we can finalize our header.
