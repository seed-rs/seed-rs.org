# GraphQL Mappers

Let's integrate prepared GraphQL helpers and queries from the previous chapter to our pages.

We can start with the simplest page - `src/page/clients_and_projects.rs`.

## Page `clients_and_projects`

1. Import required items:

    ```rust
    use ulid::Ulid;

    use cynic::QueryFragment; // <-- New

    use std::collections::BTreeMap;
    use std::convert::identity;  // <-- New

    use crate::graphql;  // <-- New

    type ClientId = Ulid;
    ```

    - We'll need trait `cynic::QueryFragment` to call the `fragment` function in `send_query(MyQuery::fragment(())`.
    - [std::convert::identity](https://doc.rust-lang.org/std/convert/fn.identity.html) is basically a named closure`|x| x`, however I recommend to read the docs to know the differences and where it's useful.

1. Change `FetchError` to `GraphQLError` in `errors`:

    ```rust
    pub struct Model {
        ...
        errors: Vec<graphql::GraphQLError>,
    ```

1. Change `fetch::Result` to `graphql::Result` in `Msg::ClientsFetched`. And we want to log `clients` and update `Model` on fetch:

    ```rust
    pub enum Msg {
        ClientsFetched(graphql::Result<BTreeMap<ClientId, Client>>),

    ...

    pub fn update ... {
        match msg {
            Msg::ClientsFetched(Ok(clients)) => {
                log!("Msg::ClientsFetched", clients);
                model.clients = RemoteData::Loaded(clients);
            },
            Msg::ClientsFetched(Err(graphql_error)) => {
                model.errors.push(graphql_error);
            },
    ```

1. And we have to derive `Debug` for some items because of our new `log!` call:

    ```rust
    #[derive(Debug)]
    pub struct Client { 
    ...

    #[derive(Debug)]
    struct Project {
    ...
    ```

1. Send GraphQL query on `init` and set `client` state to `Loading`:

    ```rust
    pub fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
        orders.perform_cmd(async { Msg::ClientsFetched(request_clients().await) });

        Model {
            ...
            clients: RemoteData::Loading,
    ```

1. And the last and most important thing just below the `init` function - `request_clients`:

    ```rust
    async fn request_clients() -> graphql::Result<BTreeMap<ClientId, Client>> {
        use graphql::queries::clients_with_projects as query_mod;

        let project_mapper = |project: query_mod::Project| (
            project.id.parse().expect("parse project Ulid"), 
            Project { name: project.name }
        );

        let client_mapper = |client: query_mod::Client| (
            client.id.parse().expect("parse client Ulid"),
            Client {
                name: client.name,
                projects: client.projects.into_iter().map(project_mapper).collect()
            }
        );

        Ok(
            graphql::send_query(query_mod::Query::fragment(()))
                .await?
                .query_client
                .expect("get clients")
                .into_iter()
                .filter_map(identity)
                .map(client_mapper)
                .collect()
        )
    }
    ```
    - The purpose of this function is to send a GraphQL request and then return response data to fill our `Model` later. However there is a problem - we can't just move the response data directly to our `Model` because they have different types. So we have to transform response data to `Model` data by `*_mapper` closures.
      - If you have tendency to delete mappers and use response data directly to remove some boilerplate, please fight the urge. It would mix two different application parts - business core and IO - and it usually doesn't end well. I recommend to read about [hexagonal architecture](https://madewithlove.com/hexagonal-architecture-demystified/).

    - `query_mod::Query::fragment(())` creates a [SelectionSet](https://docs.rs/cynic/0.8.0/cynic/selection_set/struct.SelectionSet.html) with no [Arguments](https://docs.rs/cynic/0.8.0/cynic/struct.Argument.html) (represented by [unit](https://doc.rust-lang.org/std/primitive.unit.html) `()`).

    - We need to call `.expect("get clients")` because `query_client` is `Option`. And it's `Option` because Slash GraphQL generated the function `queryClient` with optional array of `Client`s and then `cynic` forced us to respect that because of the `schema.graphql`.

    - We need to call `filter_map(identity)` to remove potential `None` values from `query_client` list. It's also caused by the generated `queryClient` function type.

    - We've written mappers as closures inside `request_clients()` body to not pollute the file by functions that are used only in once place. Also it plays nicely with our alias `use graphql::queries::clients_with_projects as query_mod`. We can always refactor it to make mappers reusable and to respect the rule _"children below the parent"_.

## Page `time_tracker`

It's very similar to the previous page.

1. Import required items:

    ```rust
    use ulid::Ulid;

    use cynic::QueryFragment; // <-- New

    use std::collections::BTreeMap;
    use std::convert::identity;  // <-- New

    use crate::graphql;  // <-- New

    type ClientId = Ulid;
    ```

1. Change `FetchError` to `GraphQLError` in `errors`:

    ```rust
    pub struct Model {
        ...
        errors: Vec<graphql::GraphQLError>,
    ```

1. Change `fetch::Result` to `graphql::Result` in `Msg::ClientsFetched`. And we want to log `clients` and update `Model` on fetch:

    ```rust
    pub enum Msg {
        ClientsFetched(graphql::Result<BTreeMap<ClientId, Client>>),

    ...

    pub fn update ... {
        match msg {
            Msg::ClientsFetched(Ok(clients)) => {
                log!("Msg::ClientsFetched", clients);
                model.clients = RemoteData::Loaded(clients);
            },
            Msg::ClientsFetched(Err(graphql_error)) => {
                model.errors.push(graphql_error);
            },
    ```

1. And we have to derive `Debug` for some items because of our new `log!` call:

    ```rust
    #[derive(Debug)]
    pub struct Client { 
    ...

    #[derive(Debug)]
    struct Project {
    ...

    #[derive(Debug)]
    struct TimeEntry {
    ...
    ```

1. Send GraphQL query on `init` and set `client` state to `Loading`:

    ```rust
    pub fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
        orders.perform_cmd(async { Msg::ClientsFetched(request_clients().await) });

        Model {
            ...
            clients: RemoteData::Loading,
    ```

1. And the last and most important thing just below the `init` function - `request_clients`:

    ```rust
    async fn request_clients() -> graphql::Result<BTreeMap<ClientId, Client>> {
        use graphql::queries::clients_with_projects_with_time_entries as query_mod;

        let time_entry_mapper = |time_entry: query_mod::TimeEntry| (
            time_entry.id.parse().expect("parse time_entry Ulid"),
            TimeEntry {
                name: time_entry.name,
                started: time_entry.started.0.parse().expect("parse time_entry started time"),
                stopped: time_entry.stopped.map(|time| time.0.parse().expect("parse time_entry started time")),
            }
        );

        let project_mapper = |project: query_mod::Project| (
            project.id.parse().expect("parse project Ulid"), 
            Project { 
                name: project.name, 
                time_entries: project.time_entries.into_iter().map(time_entry_mapper).collect()
            },
        );

        let client_mapper = |client: query_mod::Client| (
            client.id.parse().expect("parse client Ulid"),
            Client {
                name: client.name,
                projects: client.projects.into_iter().map(project_mapper).collect()
            }
        );

        Ok(
            graphql::send_query(query_mod::Query::fragment(()))
                .await?
                .query_client
                .expect("get clients")
                .into_iter()
                .filter_map(identity)
                .map(client_mapper)
                .collect()
        )
    }
    ```

## Page `time_blocks`

1. Import required items:

    ```rust
    use ulid::Ulid;

    use cynic::QueryFragment; // <-- New

    use std::collections::BTreeMap;
    use std::convert::identity;  // <-- New
    use std::ops::Add; // <-- New

    use crate::graphql;  // <-- New

    type ClientId = Ulid;
    ```
    - When we import the trait [std::ops::Add](https://doc.rust-lang.org/std/ops/trait.Add.html), it allows us to pass the function `MyType::add` instead of a closure `|x| x + x` to a function that expects a function as an argument. It'll make our code more readable and declarative.

1. Change `FetchError` to `GraphQLError` in `errors`:

    ```rust
    pub struct Model {
        ...
        errors: Vec<graphql::GraphQLError>,
    ```

1. Change `fetch::Result` to `graphql::Result` in `Msg::ClientsFetched`. And we want to log `clients` and update `Model` on fetch:

    ```rust
    pub enum Msg {
        ClientsFetched(graphql::Result<BTreeMap<ClientId, Client>>),

    ...

    pub fn update ... {
        match msg {
            Msg::ClientsFetched(Ok(clients)) => {
                log!("Msg::ClientsFetched", clients);
                model.clients = RemoteData::Loaded(clients);
            },
            Msg::ClientsFetched(Err(graphql_error)) => {
                model.errors.push(graphql_error);
            },
    ```

1. And we have to derive `Debug` for some items because of our new `log!` call:

    ```rust
    #[derive(Debug)]
    pub struct Client { 
    ...

    #[derive(Debug)]
    struct TimeBlock {
    ...

    #[derive(Debug)]
    pub enum TimeBlockStatus {
    ...

    #[derive(Debug)]
    struct Invoice {
    ...
    ```

1. Send GraphQL query on `init` and set `client` state to `Loading`:

    ```rust
    pub fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
        orders.perform_cmd(async { Msg::ClientsFetched(request_clients().await) });

        Model {
            ...
            clients: RemoteData::Loading,
    ```

1. And the last and most important thing just below the `init` function - `request_clients`:

    ```rust
    async fn request_clients() -> graphql::Result<BTreeMap<ClientId, Client>> {
        use graphql::queries::clients_with_time_blocks_and_time_entries as query_mod;

        let invoice_mapper = |invoice: query_mod::Invoice| {
            Invoice {
                custom_id: invoice.custom_id,
                url: invoice.url,
            }
        };

        let status_mapper = |status: query_mod::TimeBlockStatus| {
            match status {
                query_mod::TimeBlockStatus::NON_BILLABLE => TimeBlockStatus::NonBillable,
                query_mod::TimeBlockStatus::UNPAID => TimeBlockStatus::Unpaid,
                query_mod::TimeBlockStatus::PAID => TimeBlockStatus::Paid,
            }
        };

        let time_block_mapper = |time_block: query_mod::TimeBlock| (
            time_block.id.parse().expect("parse time_block Ulid"), 
            TimeBlock { 
                name: time_block.name,
                status: status_mapper(time_block.status),
                duration: Duration::seconds(i64::from(time_block.duration)),
                invoice: time_block.invoice.map(invoice_mapper),
            }
        );

        let compute_tracked_time = |projects: Vec<query_mod::Project>| {
            projects
                .into_iter()
                .flat_map(|project| project.time_entries)
                .map(|time_entry| {
                    let started: DateTime<Local> = 
                        time_entry.started.0.parse().expect("parse time_entry started");
                    
                    let stopped: DateTime<Local> = if let Some(stopped) = time_entry.stopped {
                        stopped.0.parse().expect("parse time_entry stopped")
                    } else {
                        chrono::Local::now()
                    };
                    
                    stopped - started
                })
                .fold(Duration::seconds(0), Duration::add)
        };

        let client_mapper = |client: query_mod::Client| (
            client.id.parse().expect("parse client Ulid"),
            Client {
                name: client.name,
                time_blocks: client.time_blocks.into_iter().map(time_block_mapper).collect(),
                tracked: compute_tracked_time(client.projects),
            }
        );

        Ok(
            graphql::send_query(query_mod::Query::fragment(()))
                .await?
                .query_client
                .expect("get clients")
                .into_iter()
                .filter_map(identity)
                .map(client_mapper)
                .collect()
        )
    }
    ```
    - There are some useful `Iterator` methods in the closure `compute_tracked_time`. I recommend to read their docs if you are not an experienced functional programmer:
      - [flat_map](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.flat_map)
      - [filter_map](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.filter_map)
      - [fold](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.fold)
        - _Note:_ Our `fold` could be replaced by [sum](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.sum), but `Duration` doesn't implement the trait [Sum](https://doc.rust-lang.org/std/iter/trait.Sum.html).

---

Let's finally test how our app sends and decodes queries:

![Time Tracker GraphQL queries](/static/images/time_tracker_graphql_queries.gif)

Nice! We can start to write `view` functions in the next chapter.



