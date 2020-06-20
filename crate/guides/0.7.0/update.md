# Update

Counter example part:

```rust
// ------ ------
//    Update
// ------ ------

// enum Msg ...

// `update` describes how to handle each `Msg`.
fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::Increment => *model += 1,
    }
}
```

<details>
<summary>Example from a production app (this website)</summary>

```rust
pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::UrlChanged(subs::UrlChanged(url)) => {
            model.page = Page::init(
                url,
                &model.guides,
                &mut model.selected_seed_version,
            );

            let title = match model.page {
                Page::Guide {
                    guide,
                    ..
                } => format!("{} - {}", guide.menu_title, TITLE_SUFFIX),
                Page::NotFound => format!("404 - {}", TITLE_SUFFIX),
            };
            document().set_title(&title);

            orders.send_msg(Msg::ScrollToTop);
        },
        Msg::ScrollToTop => window().scroll_to_with_scroll_to_options(
            web_sys::ScrollToOptions::new().top(0.),
        ),
        Msg::ToggleGuideList => model.guide_list_visibility.toggle(),
        Msg::HideGuideList => {
            model.guide_list_visibility = Hidden;
        },
        Msg::ToggleMenu => model.menu_visibility.toggle(),
        Msg::HideMenu => {
            model.menu_visibility = Hidden;
        },
        Msg::SearchQueryChanged(query) => {
            model.matched_guides = search(&model.guides, &query);
            model.search_query = query;
        },
        Msg::ToggleMode => {
            model.mode.toggle();

            let config = Config {
                mode: model.mode,
            };
            LocalStorage::insert(STORAGE_KEY, &config)
                .expect("insert to local storage");
        },
        Msg::SwitchVersion(version) => {
            orders
                .notify(subs::UrlRequested::new(
                    model
                        .base_url
                        .clone()
                        .add_path_part(version.version())
                        .add_path_part(DEFAULT_GUIDE_SLUG),
                ))
                .skip();
        },
    }
}
```

</details>

---

- `update` is the function where you should handle `Msg`s.

- It's the only place where you should mutate your business data.

- `update` is invoked by Seed when it receives a new `Msg` instance.

## How to write a good `update`

- It's basically just one `match` - keep it simple.

- It will be probably the longest function in your app. The most attempts to shorten it fail and make the code worse. Make sure that your `update` helpers are really necessary.

- When you need to write some helpers, respect the rule *"children below the parent"* as always.

- Don't write [catch-all](https://doc.rust-lang.org/book/ch18-03-pattern-syntax.html#ignoring-values-in-a-pattern) `match` arm. (It's a general rule for the entire code-base; you would regret it sooner or later.)

- It's often useful to handle one `Msg` by multiple `match` arms. Especially if the `Msg` variant contains [Result](https://doc.rust-lang.org/std/result/enum.Result.html) or [Option](https://doc.rust-lang.org/std/option/enum.Option.html). It eliminates nesting and boilerplate in arm bodies. Example:
```rust
enum Msg {
    Fetched(fetch::Result<MyData>)
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::Fetched(Ok(response_data)) => {
            model.response_data = Some(response_data);
        },
        Msg::Fetched(Err(fetch_error)) => {
            error!("Cannot fetch data:", fetch_error);
        }
    }
}
```

- When there are many `Msg` variants, split them visually into groups and divide them by comments to improve scannability. Example:
```rust
enum Msg {
    // ------ A ------
    A1,
    A2,
    // ------ B ------
    B1,
    B2,
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        // ------ A ------
        Msg::A1 => { .. },
        Msg::A1 => { .. },
        // ------ B ------
        Msg::B1 => { .. },
        Msg::B2 => { .. },
    }
}
```


