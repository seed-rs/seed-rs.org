# GraphQL

Our GraphQL backend is ready, let's fire some requests from our Seed app!

## Libraries

I've found 3 GraphQL clients on [crates.io](https://crates.io/) before I've started writing this chapter:

- [artemis](https://github.com/wingertge/artemis/tree/master/artemis)
  - It looks more like an interesting experiment than a serious library now, with features like cache, deduplication and schema downloading. 
  - The latest version at the time of writing is `0.1.0-alpha.1` from `Apr 26, 2020` ; All time downloads: `97`.

- [cynic](https://github.com/obmarg/cynic)
   - It's also a pretty new project. Author draws inspiration from the Elm world and `graphql-client` (see below) and tries to write a less "magical" API - the library allows you to write own items that will be used in queries and validates them later against the given GraphQL scheme. Other libraries usually generate all the structures for you by themselves.
   - The latest version at the time of writing is `0.8.0` from `Aug 16, 2020` ; All time downloads: `720`.

- [graphql-client](https://github.com/graphql-rust/graphql-client)
  - It's the oldest and the most mature library.
  - Our [GraphQL example](https://github.com/seed-rs/seed/tree/5e0a5a39c8c34ad4b746eb21a5b701474a7128e2/examples/graphql) is based on this lib.
  - The latest version at the time of writing is `0.9.0` from `Mar 13, 2020` ; All time downloads: `47_092`.

I like the `cynic`'s approach because I was a little bit confused while I was writing that GraphQL example mentioned above (with `graphql-client`) because of many auto-generated items. Also I think `cynic` has the [best documentation](https://cynic-rs.dev/) (although it isn't complete yet).

`artemis`'s README contains a bold warning: _THIS IS SUPER DUPER WORK IN PROGRESS! IT WILL PROBABLY NOT COMPILE WHEN YOU READ THIS!_ so I wouldn't recommend to use it now. 

So let's choose `cynic` and write some code!

## GraphQL schema

All libraries need `scheme.graphql` or `schema.json` to generate or validate Rust items for our queries.

I haven't found a way how to read/download a complete schema from Slash GraphQL and `cynic` also can't do it by itself. (I'll probably write some feedbacks to change it.) 

So we have to use an external tool. The simplest one is probably [get-graphql-schema](https://github.com/prisma-labs/get-graphql-schema) or maybe [graphql_client_cli](https://github.com/graphql-rust/graphql-client/tree/master/graphql_client_cli) would be also useful.

If you want to use `get-graphql-schema`, run these commands in the Time Tracker project root:
```bash
npm install -g get-graphql-schema # or: yarn add global get-graphql-schema
get-graphql-schema [YOUR_NOTED_GRAPHQL_ENDPOINT] > schema.graphql
```

Make sure the file `/schema.graphql` has been created. It should look like:

<details>
<summary>schema.graphql</summary>

```graphql
directive @id on FIELD_DEFINITION

directive @withSubscription on OBJECT | INTERFACE

directive @auth(query: AuthRule, add: AuthRule, update: AuthRule, delete: AuthRule) on OBJECT

directive @remote on OBJECT | INTERFACE

directive @hasInverse(field: String!) on FIELD_DEFINITION

directive @cascade on FIELD

directive @search(by: [DgraphIndex!]) on FIELD_DEFINITION

directive @dgraph(type: String, pred: String) on OBJECT | INTERFACE | FIELD_DEFINITION

directive @secret(field: String!, pred: String) on OBJECT | INTERFACE

directive @custom(http: CustomHTTP, dql: String) on FIELD_DEFINITION

input AddClientInput {
  id: String!
  name: String!
  projects: [ProjectRef!]!
  time_blocks: [TimeBlockRef!]!
  user: String!
}

type AddClientPayload {
  client(filter: ClientFilter, order: ClientOrder, first: Int, offset: Int): [Client]
  numUids: Int
}

input AddInvoiceInput {
  id: String!
  custom_id: String
  url: String
  time_block: TimeBlockRef!
}

type AddInvoicePayload {
  invoice(filter: InvoiceFilter, order: InvoiceOrder, first: Int, offset: Int): [Invoice]
  numUids: Int
}

input AddProjectInput {
  id: String!
  name: String!
  time_entries: [TimeEntryRef!]!
  client: ClientRef!
}

type AddProjectPayload {
  project(filter: ProjectFilter, order: ProjectOrder, first: Int, offset: Int): [Project]
  numUids: Int
}

input AddTimeBlockInput {
  id: String!
  name: String!
  status: TimeBlockStatus!
  duration: Int!
  invoice: InvoiceRef
  client: ClientRef!
}

type AddTimeBlockPayload {
  timeBlock(filter: TimeBlockFilter, order: TimeBlockOrder, first: Int, offset: Int): [TimeBlock]
  numUids: Int
}

input AddTimeEntryInput {
  id: String!
  name: String!
  started: DateTime!
  stopped: DateTime
  project: ProjectRef!
}

type AddTimeEntryPayload {
  timeEntry(filter: TimeEntryFilter, order: TimeEntryOrder, first: Int, offset: Int): [TimeEntry]
  numUids: Int
}

input AuthRule {
  and: [AuthRule]
  or: [AuthRule]
  not: AuthRule
  rule: String
}

type Client {
  id: String!
  name: String!
  projects(filter: ProjectFilter, order: ProjectOrder, first: Int, offset: Int): [Project!]!
  time_blocks(filter: TimeBlockFilter, order: TimeBlockOrder, first: Int, offset: Int): [TimeBlock!]!
  user: String!
}

input ClientFilter {
  id: StringHashFilter
  and: ClientFilter
  or: ClientFilter
  not: ClientFilter
}

input ClientOrder {
  asc: ClientOrderable
  desc: ClientOrderable
  then: ClientOrder
}

enum ClientOrderable {
  id
  name
  user
}

input ClientPatch {
  name: String
  projects: [ProjectRef!]
  time_blocks: [TimeBlockRef!]
  user: String
}

input ClientRef {
  id: String
  name: String
  projects: [ProjectRef!]
  time_blocks: [TimeBlockRef!]
  user: String
}

input CustomHTTP {
  url: String!
  method: HTTPMethod!
  body: String
  graphql: String
  mode: Mode
  forwardHeaders: [String!]
  secretHeaders: [String!]
  introspectionHeaders: [String!]
  skipIntrospection: Boolean
}

scalar DateTime

input DateTimeFilter {
  eq: DateTime
  le: DateTime
  lt: DateTime
  ge: DateTime
  gt: DateTime
}

type DeleteClientPayload {
  client(filter: ClientFilter, order: ClientOrder, first: Int, offset: Int): [Client]
  msg: String
  numUids: Int
}

type DeleteInvoicePayload {
  invoice(filter: InvoiceFilter, order: InvoiceOrder, first: Int, offset: Int): [Invoice]
  msg: String
  numUids: Int
}

type DeleteProjectPayload {
  project(filter: ProjectFilter, order: ProjectOrder, first: Int, offset: Int): [Project]
  msg: String
  numUids: Int
}

type DeleteTimeBlockPayload {
  timeBlock(filter: TimeBlockFilter, order: TimeBlockOrder, first: Int, offset: Int): [TimeBlock]
  msg: String
  numUids: Int
}

type DeleteTimeEntryPayload {
  timeEntry(filter: TimeEntryFilter, order: TimeEntryOrder, first: Int, offset: Int): [TimeEntry]
  msg: String
  numUids: Int
}

enum DgraphIndex {
  int
  float
  bool
  hash
  exact
  term
  fulltext
  trigram
  regexp
  year
  month
  day
  hour
}

input FloatFilter {
  eq: Float
  le: Float
  lt: Float
  ge: Float
  gt: Float
}

enum HTTPMethod {
  GET
  POST
  PUT
  PATCH
  DELETE
}

input IntFilter {
  eq: Int
  le: Int
  lt: Int
  ge: Int
  gt: Int
}

type Invoice {
  id: String!
  custom_id: String
  url: String
  time_block(filter: TimeBlockFilter): TimeBlock!
}

input InvoiceFilter {
  id: StringHashFilter
  and: InvoiceFilter
  or: InvoiceFilter
  not: InvoiceFilter
}

input InvoiceOrder {
  asc: InvoiceOrderable
  desc: InvoiceOrderable
  then: InvoiceOrder
}

enum InvoiceOrderable {
  id
  custom_id
  url
}

input InvoicePatch {
  custom_id: String
  url: String
  time_block: TimeBlockRef
}

input InvoiceRef {
  id: String
  custom_id: String
  url: String
  time_block: TimeBlockRef
}

enum Mode {
  BATCH
  SINGLE
}

type Mutation {
  addClient(input: [AddClientInput!]!): AddClientPayload
  updateClient(input: UpdateClientInput!): UpdateClientPayload
  deleteClient(filter: ClientFilter!): DeleteClientPayload
  addProject(input: [AddProjectInput!]!): AddProjectPayload
  updateProject(input: UpdateProjectInput!): UpdateProjectPayload
  deleteProject(filter: ProjectFilter!): DeleteProjectPayload
  addTimeEntry(input: [AddTimeEntryInput!]!): AddTimeEntryPayload
  updateTimeEntry(input: UpdateTimeEntryInput!): UpdateTimeEntryPayload
  deleteTimeEntry(filter: TimeEntryFilter!): DeleteTimeEntryPayload
  addTimeBlock(input: [AddTimeBlockInput!]!): AddTimeBlockPayload
  updateTimeBlock(input: UpdateTimeBlockInput!): UpdateTimeBlockPayload
  deleteTimeBlock(filter: TimeBlockFilter!): DeleteTimeBlockPayload
  addInvoice(input: [AddInvoiceInput!]!): AddInvoicePayload
  updateInvoice(input: UpdateInvoiceInput!): UpdateInvoicePayload
  deleteInvoice(filter: InvoiceFilter!): DeleteInvoicePayload
}

type Project {
  id: String!
  name: String!
  time_entries(filter: TimeEntryFilter, order: TimeEntryOrder, first: Int, offset: Int): [TimeEntry!]!
  client(filter: ClientFilter): Client!
}

input ProjectFilter {
  id: StringHashFilter
  and: ProjectFilter
  or: ProjectFilter
  not: ProjectFilter
}

input ProjectOrder {
  asc: ProjectOrderable
  desc: ProjectOrderable
  then: ProjectOrder
}

enum ProjectOrderable {
  id
  name
}

input ProjectPatch {
  name: String
  time_entries: [TimeEntryRef!]
  client: ClientRef
}

input ProjectRef {
  id: String
  name: String
  time_entries: [TimeEntryRef!]
  client: ClientRef
}

type Query {
  getClient(id: String!): Client
  queryClient(filter: ClientFilter, order: ClientOrder, first: Int, offset: Int): [Client]
  getProject(id: String!): Project
  queryProject(filter: ProjectFilter, order: ProjectOrder, first: Int, offset: Int): [Project]
  getTimeEntry(id: String!): TimeEntry
  queryTimeEntry(filter: TimeEntryFilter, order: TimeEntryOrder, first: Int, offset: Int): [TimeEntry]
  getTimeBlock(id: String!): TimeBlock
  queryTimeBlock(filter: TimeBlockFilter, order: TimeBlockOrder, first: Int, offset: Int): [TimeBlock]
  getInvoice(id: String!): Invoice
  queryInvoice(filter: InvoiceFilter, order: InvoiceOrder, first: Int, offset: Int): [Invoice]
}

input StringExactFilter {
  eq: String
  le: String
  lt: String
  ge: String
  gt: String
}

input StringFullTextFilter {
  alloftext: String
  anyoftext: String
}

input StringHashFilter {
  eq: String
}

input StringRegExpFilter {
  regexp: String
}

input StringTermFilter {
  allofterms: String
  anyofterms: String
}

type TimeBlock {
  id: String!
  name: String!
  status: TimeBlockStatus!
  duration: Int!
  invoice(filter: InvoiceFilter): Invoice
  client(filter: ClientFilter): Client!
}

input TimeBlockFilter {
  id: StringHashFilter
  and: TimeBlockFilter
  or: TimeBlockFilter
  not: TimeBlockFilter
}

input TimeBlockOrder {
  asc: TimeBlockOrderable
  desc: TimeBlockOrderable
  then: TimeBlockOrder
}

enum TimeBlockOrderable {
  id
  name
  duration
}

input TimeBlockPatch {
  name: String
  status: TimeBlockStatus
  duration: Int
  invoice: InvoiceRef
  client: ClientRef
}

input TimeBlockRef {
  id: String
  name: String
  status: TimeBlockStatus
  duration: Int
  invoice: InvoiceRef
  client: ClientRef
}

enum TimeBlockStatus {
  NON_BILLABLE
  UNPAID
  PAID
}

type TimeEntry {
  id: String!
  name: String!
  started: DateTime!
  stopped: DateTime
  project(filter: ProjectFilter): Project!
}

input TimeEntryFilter {
  id: StringHashFilter
  and: TimeEntryFilter
  or: TimeEntryFilter
  not: TimeEntryFilter
}

input TimeEntryOrder {
  asc: TimeEntryOrderable
  desc: TimeEntryOrderable
  then: TimeEntryOrder
}

enum TimeEntryOrderable {
  id
  name
  started
  stopped
}

input TimeEntryPatch {
  name: String
  started: DateTime
  stopped: DateTime
  project: ProjectRef
}

input TimeEntryRef {
  id: String
  name: String
  started: DateTime
  stopped: DateTime
  project: ProjectRef
}

input UpdateClientInput {
  filter: ClientFilter!
  set: ClientPatch
  remove: ClientPatch
}

type UpdateClientPayload {
  client(filter: ClientFilter, order: ClientOrder, first: Int, offset: Int): [Client]
  numUids: Int
}

input UpdateInvoiceInput {
  filter: InvoiceFilter!
  set: InvoicePatch
  remove: InvoicePatch
}

type UpdateInvoicePayload {
  invoice(filter: InvoiceFilter, order: InvoiceOrder, first: Int, offset: Int): [Invoice]
  numUids: Int
}

input UpdateProjectInput {
  filter: ProjectFilter!
  set: ProjectPatch
  remove: ProjectPatch
}

type UpdateProjectPayload {
  project(filter: ProjectFilter, order: ProjectOrder, first: Int, offset: Int): [Project]
  numUids: Int
}

input UpdateTimeBlockInput {
  filter: TimeBlockFilter!
  set: TimeBlockPatch
  remove: TimeBlockPatch
}

type UpdateTimeBlockPayload {
  timeBlock(filter: TimeBlockFilter, order: TimeBlockOrder, first: Int, offset: Int): [TimeBlock]
  numUids: Int
}

input UpdateTimeEntryInput {
  filter: TimeEntryFilter!
  set: TimeEntryPatch
  remove: TimeEntryPatch
}

type UpdateTimeEntryPayload {
  timeEntry(filter: TimeEntryFilter, order: TimeEntryOrder, first: Int, offset: Int): [TimeEntry]
  numUids: Int
}


```

</details>

## `cynic` integration

1. Add required dependencies to `Cargo.toml`:

    ```toml
    serde-wasm-bindgen ...
    cynic = "0.11.0"
    ```

1. Create a new empty file `/src/graphql.rs`. This module will contain our GraphQL queries.

1. And include it as a new module in `/src/lib.rs`:
    ```rust
    mod page;
    mod graphql;
    ```

## `send_query` & `graphql::Result`

The code below is everything we need to send GraphQL queries to our backend.
Let's read it all and then we'll explain its parts.

_Note:_ If the code snippet below looks a little bit too generic to you, you aren't alone - maybe we should wrap it into a new GraphQL Seed service. Please write your opinions in [this issue](https://github.com/seed-rs/seed/issues/519).


```rust
use seed::{prelude::*};

use cynic;

pub type Result<T> = std::result::Result<T, GraphQLError>;

pub async fn send_operation<'a, ResponseData: 'a>(
    operation: cynic::Operation<'a, ResponseData>
) -> Result<ResponseData> {
    let graphql_response = 
        // @TODO: Move url to a config file.
        Request::new("https://time-tracker.eu-central-1.aws.cloud.dgraph.io/graphql")
            .method(Method::Post)
            .json(&operation)?
            .fetch()
            .await?
            .check_status()?
            .json()
            .await?;

    let response_data = operation.decode_response(graphql_response)?;
    if let Some(errors) = response_data.errors {
        Err(errors)?
    }
    Ok(response_data.data.expect("response data"))
}

// ------ Error ------

#[derive(Debug)]
pub enum GraphQLError {
    FetchError(FetchError),
    ResponseErrors(Vec<cynic::GraphQLError>),
    DecodeError(cynic::DecodeError)
}

impl From<FetchError> for GraphQLError {
    fn from(fetch_error: FetchError) -> Self {
        Self::FetchError(fetch_error)
    }
}

impl From<Vec<cynic::GraphQLError>> for GraphQLError {
    fn from(response_errors: Vec<cynic::GraphQLError>) -> Self {
        Self::ResponseErrors(response_errors)
    }
}

impl From<cynic::DecodeError> for GraphQLError {
    fn from(decode_error: cynic::DecodeError) -> Self {
        Self::DecodeError(decode_error)
    }
}
```

1. Let's start with the type alias `Result`:

    ```rust
    pub type Result<T> = std::result::Result<T, GraphQLError>;
    ```
    - It's basically an alternative to `fetch::Result`. However GraphQL request may fail because of some other reasons than a simple `fetch` request so we have to use different type for `Err` (`GraphQLError` instead of `FetchError`) - which means we need to introduce a new type alias.

1. `GraphQLError` which is used in the `Result` alias:

    ```rust
    #[derive(Debug)]
    pub enum GraphQLError {
        FetchError(FetchError),
        ResponseErrors(Vec<cynic::GraphQLError>),
        DecodeError(cynic::DecodeError)
    }
    ```
    - `ResponseErrors` means the GraphQL response's `errors` isn't an empty array. _Note:_ Once we need to read `data` even if there are errors, we will need something like [cynic::GraphQLResult](https://docs.rs/cynic/0.10.0/cynic/type.GraphQLResult.html).
    - `DecodeError` means the response is probably malformed and can't be deserialized to prepared Rust items.

1. [From](https://doc.rust-lang.org/std/convert/trait.From.html) implementations for `GraphQLError`:
    ```rust
    impl From<*> for GraphQLError {
        fn from(*: *) -> Self {
            Self::*(*)
        }
    }
    ```
    - The only purpose is to allow to use early returns (like `Err(error)?` or `.await?`) in functions that returns `graphql::Result<T>` - e.g. `send_query`.

1. And finally `send_operation`:
    ```rust
    pub async fn send_operation<'a, ResponseData: 'a>(
        operation: cynic::Operation<'a, ResponseData>
    ) -> Result<ResponseData>
    ```
    - It looks a bit scary but those generic parameters allow us to pass all future queries into the function this way:
        ```rust
        graphql::send_operation(MyQuery::build(()))
        ```
        And you can read about `cynic` types on [docs.rs](https://docs.rs/cynic/0.11.0/cynic/) or [cynic-rs.dev](https://cynic-rs.dev/).

    - `send_operation`'s body isn't very interesting - just one `POST` fetch request with basic error handling and some `cynic`-related calls that I've found in `cynic`'s docs.

## Queries

We will need 3 queries for our 3 main pages:

1. For the page `clients_and_projects`:

    ```graphql
    {
        queryClient {
            id
            name
            projects {
                id
                name
            }
        }
    }
    ```

1. For the page `time_tracker`:

    ```graphql
    {
        queryClient {
            id
            name
            projects {
                id
                name
                time_entries {
                    id
                    name
                    started
                    stopped
                }
            }
        }
    }
    ```

1. For the page `time_blocks`:

    ```graphql
    {
        queryClient {
            id
            name
            time_blocks {
                id
                name
                status
                duration
                invoice {
                    id
                    custom_id
                    url
                }
            }
            projects {
                time_entries {
                    started
                    stopped
                }
            }
        }
    }
    ```
    - _Note_: I haven't found a simple way to compute total tracked time on the backend (I assume I'm overlooking something in Slash GraphQL docs or they are working on it right now.) So we'll request all time entries and compute tracked time manually on the frontend.

## GraphQL items

Let's do a magic trick. Go to [generator.cynic-rs.dev](https://generator.cynic-rs.dev/), write your GraphQL endpoint url or paste your schema and then insert one of the queries above into the query builder window:

![Cynic Generator](/static/images/time_tracker_cynic_generator.png)

Notice especially the right panel "GENERATED RUST".

Unfortunately the generator isn't so clever (yet) to resolve name conflicts when your enter multiple queries and handle all valid inputs, however it's a very good start.

So when you play with the generator and all 3 queries and refactor a bit, you'll end up with something like:

```rust
// ------ ------
// GraphQL items
// ------ ------

pub mod queries {
    #[cynic::query_module(
        schema_path = "schema.graphql",
        query_module = "query_dsl",
    )]
    pub mod clients_with_projects {
        use crate::graphql::query_dsl;

        ///```graphql
        ///{
        ///    queryClient {
        ///        id
        ///        name
        ///        projects {
        ///            id
        ///            name
        ///        }
        ///    }
        ///}
        ///```
        #[derive(cynic::QueryFragment, Debug)]
        #[cynic(graphql_type = "Query")]
        pub struct Query {
            pub query_client: Option<Vec<Option<Client>>>,
        }

        #[derive(cynic::QueryFragment, Debug)]
        #[cynic(graphql_type = "Client")]
        pub struct Client {
            pub id: String,
            pub name: String,
            pub projects: Vec<Project>,
        }

        #[derive(cynic::QueryFragment, Debug)]
        #[cynic(graphql_type = "Project")]
        pub struct Project {
            pub id: String,
            pub name: String,
        }
    }

    #[cynic::query_module(
        schema_path = "schema.graphql",
        query_module = "query_dsl",
    )]
    pub mod clients_with_projects_with_time_entries {
        use crate::graphql::{query_dsl, types::*};

        ///```graphql
        ///{
        ///    queryClient {
        ///        id
        ///        name
        ///        projects {
        ///            id
        ///            name
        ///            time_entries {
        ///                id
        ///                name
        ///                started
        ///                stopped
        ///            }
        ///        }
        ///    }
        ///}
        ///```
        #[derive(cynic::QueryFragment, Debug)]
        #[cynic(graphql_type = "Query")]
        pub struct Query {
            pub query_client: Option<Vec<Option<Client>>>,
        }

        #[derive(cynic::QueryFragment, Debug)]
        #[cynic(graphql_type = "Client")]
        pub struct Client {
            pub id: String,
            pub name: String,
            pub projects: Vec<Project>,
        }

        #[derive(cynic::QueryFragment, Debug)]
        #[cynic(graphql_type = "Project")]
        pub struct Project {
            pub id: String,
            pub name: String,
            pub time_entries: Vec<TimeEntry>,
        }

        #[derive(cynic::QueryFragment, Debug)]
        #[cynic(graphql_type = "TimeEntry")]
        pub struct TimeEntry {
            pub id: String,
            pub name: String,
            pub started: DateTime,
            pub stopped: Option<DateTime>,
        }
    }

    #[cynic::query_module(
        schema_path = "schema.graphql",
        query_module = "query_dsl",
    )]
    pub mod clients_with_time_blocks_and_time_entries {
        use crate::graphql::{query_dsl, types::*};

        ///```graphql
        ///{
        ///    queryClient {
        ///        id
        ///        name
        ///        time_blocks {
        ///            id
        ///            name
        ///            status
        ///            duration
        ///            invoice {
        ///                id
        ///                custom_id
        ///                url
        ///            }
        ///        }
        ///        projects {
        ///            time_entries {
        ///                started
        ///                stopped
        ///            }
        ///        }
        ///    }
        ///}
        ///```
        #[derive(cynic::QueryFragment, Debug)]
        #[cynic(graphql_type = "Query")]
        pub struct Query {
            pub query_client: Option<Vec<Option<Client>>>,
        }

        #[derive(cynic::QueryFragment, Debug)]
        #[cynic(graphql_type = "Client")]
        pub struct Client {
            pub id: String,
            pub name: String,
            pub time_blocks: Vec<TimeBlock>,
            pub projects: Vec<Project>,
        }

        #[derive(cynic::QueryFragment, Debug)]
        #[cynic(graphql_type = "TimeBlock")]
        pub struct TimeBlock {
            pub id: String,
            pub name: String,
            pub status: TimeBlockStatus,
            pub duration: i32,
            pub invoice: Option<Invoice>,
        }

        #[derive(cynic::Enum, Debug, Copy, Clone)]
        #[cynic(graphql_type = "TimeBlockStatus", rename_all = "SCREAMING_SNAKE_CASE")]
        pub enum TimeBlockStatus {
            NonBillable,
            Unpaid,
            Paid,
        }

        #[derive(cynic::QueryFragment, Debug)]
        #[cynic(graphql_type = "Invoice")]
        pub struct Invoice {
            pub id: String,
            pub custom_id: Option<String>,
            pub url: Option<String>,
        }

        #[derive(cynic::QueryFragment, Debug)]
        #[cynic(graphql_type = "Project")]
        pub struct Project {
            pub time_entries: Vec<TimeEntry>,
        }

        #[derive(cynic::QueryFragment, Debug)]
        #[cynic(graphql_type = "TimeEntry")]
        pub struct TimeEntry {
            pub started: DateTime,
            pub stopped: Option<DateTime>,
        }
    }
}

mod types {
    #[derive(cynic::Scalar, Debug)]
    pub struct DateTime(pub String);
}

mod query_dsl {
    use super::types::*;
    cynic::query_dsl!("schema.graphql");
}
```
_Note_: The hardest part is (as always) naming... do you have an idea for better module names?

Append the code above to your `graphql.rs` file. 

All those structs and enums are verified against the schema during compilation. And you can use them directly in your business code if you want, which is nice. There are also other ways how to define queries in `cynic` - consult its docs for more info.

---

We are ready to send GraphQL requests, however we can't transform their responses to match our `Model` types yet. Let's fix it in the next chapter and finally send them!


