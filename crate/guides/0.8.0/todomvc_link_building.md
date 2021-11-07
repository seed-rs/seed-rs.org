# TodoMVC - Link Building

There are hard-coded filter links and corresponding urls in our app:

```rust
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

fn view_filters(selected_filter: Filter) -> Node<Msg> {
    ul![C!["filters"],
        Filter::iter().map(|filter| {
            let (link, title) = match filter {
                Filter::All => ("#/", "All"),
                Filter::Active => ("#/active", "Active"),
                Filter::Completed => ("#/completed", "Completed"),
            };
...
```

## 1. `const` path parts

Duplicated literal strings is the one of the worst thing the developer may encounter in an unfamiliar code-base. Let's DRY them.

```rust
const STORAGE_KEY: &str = "todos-seed";

// ------ Url path parts ------
const ACTIVE: &str = "active";
const COMPLETED: &str = "completed";
...

impl From<Url> for Filter {
    ...
            [ACTIVE] => Self::Active,
            [COMPLETED] => Self::Completed,
            _ => Self::All,
...

fn view_filters(selected_filter: Filter) -> Node<Msg> {
   ...
            let (path, title) = match filter {
                Filter::All => ("", "All"),
                Filter::Active => (ACTIVE, "Active"),
                Filter::Completed => (COMPLETED, "Completed"),
            };
            li![
                a![C![IF!(filter == selected_filter => "selected")],
                    attrs!{At::Href => format!("#/{}", path)},
                    title,
                ],
...
```

_Note_: It would be easy now to switch to [hashbang](https://stackoverflow.com/a/10355561) routing (don't modify your app code, please). Specs:
> ... The following routes should be implemented: `#/` (all - default), `#/active` and `#/completed` (`#!/` is also allowed). ...

```rust
impl From<Url> for Filter {
    fn from(mut url: Url) -> Self {
        match url.remaining_hash_path_parts().as_slice() {
            ["!", rest @ ..] => {
                match rest {
                    [ACTIVE] => Self::Active,
                    [COMPLETED] => Self::Completed,
                    _ => Self::All,
                }
            }
            _ => Self::All,
        }
    }
}
...

fn view_filters(selected_filter: Filter) -> Node<Msg> {
    ...
                    attrs!{At::Href => format!("#!/{}", path)},
...
```

## 2. Standard link building

The routing code and links are now good enough.

However it's not a standard way how to create links in Seed apps. Once you have a larger app with nested paths and pages, you don't want to know parent path parts - the only interesting ones are path parts related to the particular page.

Example:
  - Paths:
     - `/admin/statistics/report/daily`
     - `/admin/statistics/report/weekly`
  - Pages: `admin`, `statistics`, `report`
  - The only path parts interesting for `report` page are `daily` and `weekly`.

Yes, you can try to rely on browser base path handling and use relative paths (e.g. `/admin/statistics/report/` + `daily` = `/admin/statistics/report/daily`). However it's not very reliable, especially once you decide to deploy the app to non-root server path and had to explicitly set the base path (you'll learn about base path in other chapters).
And there is no way how to do it for our hash-based paths.

Let's rewrite it to the standard form and then discuss it.

```rust
// ----------------- A) -----------------

fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    ...
    Model {
        base_url: url.to_hash_base_url(),
        ...
        filter: Filter::from(url),
    }
}
...

// ----------------- B) -----------------

struct Model {
    base_url: Url,
    ...
    filter: Filter,
}
...

// ----------------- C) -----------------

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

// ----------------- D) -----------------

fn view(model: &Model) -> Vec<Node<Msg>> {
    ...
            view_footer(&model.todos, model.filter, &model.base_url),
...

fn view_footer(todos: &BTreeMap<Ulid, Todo>, selected_filter: Filter, base_url: &Url) -> Node<Msg> {
    ...
        view_filters(selected_filter, base_url),
...

// ----------------- E) -----------------

fn view_filters(selected_filter: Filter, base_url: &Url) -> Node<Msg> {
    ul![C!["filters"],
        Filter::iter().map(|filter| {
            let urls = Urls::new(base_url);

            let (url, title) = match filter {
                Filter::All => (urls.home(), "All"),
                Filter::Active => (urls.active(), "Active"),
                Filter::Completed => (urls.completed(), "Completed"),
            };

            li![
                a![C![IF!(filter == selected_filter => "selected")],
                    attrs!{At::Href => url},
                    title,
                ],
            ]
...

```

### Block A)

We've moved field `base_url` at the top because:
   - The value is initiated by using `url`, so it has to be placed above `filter: Filter::from(url)` because `Filter::from` consumes `url` (i.e. takes ownership).
   - It's a common part of Seed apps - it allows a bit faster code scanning for experienced Seed users.

`Url` method `to_hash_base_url()` deletes all path parts with index >= `next_hash_path_part_index` in the cloned url. In our case it removes all path parts because `next_hash_path_part_index` is always set to 0 in `url` in `init`.

`to_hash_base_url()` returns the cloned url that will be used as a base url for our links as you'll see later.

### Block B)

`Model` field ordering has been changed to mirror our new field ordering in `init`.

### Block C)

This is the link building itself. We'll talk about it in a standalone section `3. struct_urls!` below.

### Block D)

We just pass `base_url` through `view` functions as necessary.

### Block E)

The most interesting parts in this block are:
```rust
let urls = Urls::new(base_url);
urls.home()
At::Href => url
```

- We don't have to write error-prone string links and be careful to add symbols like `#` anymore - links are created by typed methods now.

- We don't have to think about parent path parts - it allows us to move the code into sub-modules without problems.

- It would be easy to switch from hash-based routing to the standard one.

## 3. `struct_urls!`

Let's look at the content of block C) again:

```rust
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
```

`struct_urls!()` doesn't do anything fancy - it just hides the code to improve readability. You can copy-paste the code from the [macro definition](https://github.com/seed-rs/seed/blob/d514b2131a9e94f5ffe965f3d0ac74763a11aeb6/src/shortcuts.rs#L83-L117), fix paths and it would look like this:

```rust
pub struct Urls<'a> {
    base_url: std::borrow::Cow<'a, Url>,
}

impl<'a> Urls<'a> {
    pub fn new(base_url: impl Into<std::borrow::Cow<'a, Url>>) -> Self {
        Self {
            base_url: base_url.into(),
        }
    }
    pub fn base_url(self) -> Url {
        self.base_url.into_owned()
    }
}

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
```

There are [Cow](https://doc.rust-lang.org/std/borrow/enum.Cow.html)s and [lifetimes](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#validating-references-with-lifetimes) to improve the performance during chaining (i.e. building links from multiple path parts in nested modules). Example from [pages example](https://github.com/seed-rs/seed/blob/d514b2131a9e94f5ffe965f3d0ac74763a11aeb6/examples/pages/src/lib.rs#L62-L70):
```rust
struct_urls!();
impl<'a> Urls<'a> {
    ...
    pub fn admin_urls(self) -> page::admin::Urls<'a> {
        page::admin::Urls::new(self.base_url().add_path_part(ADMIN))
    }
}
```
In that example we pass an owned `Url` into `page::admin::Urls::new`, however we can still pass `base_url` reference into `page::admin:Urls::new` from the inside of `page::admin` module (like we did in our TodoMVC example).

_Note:_ You don't have to use `struct_urls!()` if you don't want to - it's just your helper. However you'll probably appreciate it while you are writing more complex apps.

---

Nice! TodoMVC has a scalable routing and link building, let's finish our app in the next chapter.



