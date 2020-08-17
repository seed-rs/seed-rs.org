# Backend

We've already integrated [Auth0](https://auth0.com/) in the previous chapters, now it's time to write our backend APIs. We've decided (in the previous chapters) to use [Slash GraphQL](https://dgraph.io/slash-graphql) to manage and store our entities.

Our plan:
1. Create a new `Slash GraphQL` _backend_.
1. Define our backend entities and GraphQL endpoint by writing [GraphQL schema](https://graphql.org/learn/schema/).
1. Add test data.
1. Test our endpoint by simple read queries.
1. Implement communication between the frontend and backend through GraphQL protocol.
1. Authorization - each user can access only his data.

## 1. New `Slash GraphQL` _backend_

1. Slash Graphql is currently _developer preview_. So it's free, however you need to get invitation. Try my [invitation page](https://slash.dgraph.io/invite.html?id=0x345e) or if it doesn't work write me your email (on [chat](https://discord.gg/JHHcHp5) or `martin@kavik.cz`) and I'll invite you manually. Or you can request an invitation on their homapage - https://dgraph.io/slash-graphql.

1. Log in to Slash administration and you should see somehing like:

    ![Slash Dashboard](/static/images/time_tracker_slash_dashboard.png)

1. Click the button `Create a free Backend` or click the dropdown at the top with the value `None` and then on `Create a Backend`.

1. Fill the form:
    1. Name: `Time Tracker`
    2. Subdomain: Something like `time-tracker` that doesn't exist yet.
    3. Pick the `Provider` and its `Zone`.
    4. Confirm by `Create New Backend`.

    ![Slash Create a new Backend](/static/images/time_tracker_slash_create_a_backend.png)

1. Click `Create your Schema`:

    ![Slash Backend is live](/static/images/time_tracker_slash_backend_is_live.png)

## 2. GraphQL schema

The schema is basically our Rust `Model`s rewritten to GraphQL with some Slash GraphQL extensions.

```graphql

type Client {
    id: String! @id
    name: String!
    projects: [Project!]! @hasInverse(field: client)
    time_blocks: [TimeBlock!]! @hasInverse(field: client)
    user: String!
}

type Project {
    id: String! @id
    name: String!
    time_entries: [TimeEntry!]! @hasInverse(field: project)
    client: Client!
}

type TimeEntry {
    id: String! @id
    name: String!
    started: DateTime!
    stopped: DateTime
    project: Project!
}

type TimeBlock {
    id: String! @id
    name: String!
    status: TimeBlockStatus!
    duration: Int!
    invoice: Invoice @hasInverse(field: time_block)
    client: Client!
}

enum TimeBlockStatus {
  NON_BILLABLE
  UNPAID
  PAID
}

type Invoice {
  id: String! @id
  custom_id: String
  url: String
  time_block: TimeBlock!
}

```

- `Client` field `user` represents Auth0 User id. It will be used during [authorization](https://graphql.dgraph.io/doc/authorization/directive) as soon as we integrate it.
- Directive `@hasInverse` [docs](https://graphql.dgraph.io/doc/schema/graph-links).
- Directive `@id` [docs](https://graphql.dgraph.io/doc/schema/ids). We can't use `ID` because we want to define ids in our app and it'll also make creating test data easier as a nice side-effect.

1. Write the schema into the textarea on the page `Schema`:
    - _Note_: Therer is a bug at the time of writing in the Slash's administration - `Schema Builer` doesn't work, probably because it fails while it's parsing `enum`.

    ![Slash Backend is live](/static/images/time_tracker_slash_schema.png)

1. Save it by `Update Schema`.

## 3. Test data

1. Click `API Explorer` (either in the `Success` dialog or in the main side menu).

1. Insert the `mutation`. Generate ids by [ulidgenerator.com](https://ulidgenerator.com/):
    - _Note_: You can press `Ctrl + Space` to show autocomplete suggestions. And `Ctrl + /` to comment/uncomment selected text.

    ```graphql
    mutation {
      addClient(input: [{
        id: "01EG14HC157VEJ8DWMSKBEDGY5",
        name: "Client A",
        projects: [],
        time_blocks: [],
        user: "HARDCODED_USER_ID",
      }]) {
        numUids
      }
      
      addProject(input: [{
        id: "01EG14HRYJ6SRC6NNK8AV4J7NC",
        name: "Project 1",
        time_entries: [],
        client: { id: "01EG14HC157VEJ8DWMSKBEDGY5" },
      }, {
        id: "01EG14J2MW4N92WGBRAE79S134",
        name: "Project 2",
        time_entries: [],
        client: { id: "01EG14HC157VEJ8DWMSKBEDGY5" },
      }]) {
        numUids
      }
      
      addTimeEntry(input: [{
        id: "01EG14JB1NR008D032E2GHXD6G",
        name: "Time Entry X",
        started: "2020-01-15T15:53:39Z",
        stopped: null,
        project: { id: "01EG14HRYJ6SRC6NNK8AV4J7NC" },
      }, {
        id: "01EG14JJGVEVKM9K5ET99W40R9",
        name: "Time Entry Y",
        started: "2020-01-15T16:58:20Z",
        stopped: "2020-01-15T17:25:23Z",
        project: { id: "01EG14J2MW4N92WGBRAE79S134" },
      }]) {
        numUids
      }
      
      addTimeBlock(input: [{
        id: "01EG14JTR0VP8J2FN45RZ92VEX",
        name: "Time Block X",
        status: UNPAID,
        duration: 72000, # 20h * 3600 
        invoice: null,
        client: { id: "01EG14HC157VEJ8DWMSKBEDGY5" },
      }, {
        id: "01EG14JZ9AHFDNXQ48TKY721YA",
        name: "Time Block Y",
        status: UNPAID,
        duration: 72000, # 20h * 3600 
        invoice: null,
        client: { id: "01EG14HC157VEJ8DWMSKBEDGY5" },
      }]) {
        numUids
      }
      
      addInvoice(input: [{
        id: "01EG14K58V0YS7K48B7VS6345R",
        custom_id: "5-2020",
        url: "https://example.com/my_invoice.pdf",
        time_block: { id: "01EG14JZ9AHFDNXQ48TKY721YA" },
      }]) {
        numUids
      }
    }
    ```

1. Click the "play" button or press `Ctrl + Enter`:

    ![Slash Explorer Created entities](/static/images/time_tracker_slash_explorer_created_entities.png)

## 4. Simple `query`

Insert into the API Explorer and press the "play" button again:

```graphql
{
  queryClient() {
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
    user
  }
}
```

![Slash Explorer Simple query](/static/images/time_tracker_slash_explorer_simple_query.png)

---

We have a working GraphQL endpoint!

However two steps still remain:
1. Implement communication between the frontend and backend through GraphQL protocol.
1. Authorization - each user can access only his data.

We'll implement communication in the next chapter and we'll focus on authorization later because it would unnecessary complicate development now.

Before you move to the next chapter, please note your endpoint url:
 - Example: `https://time-tracker.eu-central-1.aws.cloud.dgraph.io/graphql`
 - You can find it on your dashboard:

    ![Slash Explorer Simple query](/static/images/time_tracker_slash_dashboard_endpoint.png)

