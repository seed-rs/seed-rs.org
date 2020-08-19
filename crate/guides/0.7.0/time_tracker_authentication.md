# Authentication

Let's integrate [Auth0](https://auth0.com/) into our app!

Our goal:

![Final result](/static/images/time_tracker_authentication_final.gif)

There isn't a Rust Auth0 SDK and we don't want to write it because it would take a lot of time and there is a big change to write security bugs. Fortunately there is an official [Javascript SDK for SPAs](https://auth0.com/docs/libraries/auth0-single-page-app-sdk) that we can use through `wasm-bindgen` JS/Rust "bridge".

## Flows

- Log in:
    1. The Seed app starts.
    1. Fetch the Auth0 configration from the API endpoint or file `/auth_config.json`.
    1. Initiate the Auth0 client with the fetched configuration.
    1. The client finds out that the user isn't logged in.
    1. The user clicks the button `Log in`.
    1. The user is redirected to the Auth0 login form.
    1. The user is redirected back to the app after the successful login with some special query parameters in the url.
    1. The app is started again (because of redirecting).
    1. The Auth0 client is initiated again.
    1. The client handles special url query parameters and passes the user data to the app.
    1. The app saves the user data to `model.ctx.user` and removes the query parameters from the url.
    1. The user see buttons `[nickname]` and `Log out` instead of `Sign up` and `Log in`.

- Sign up flow is almost identical to log in flow, only the sign up form is preselected on the Auth0 hosted website.

- Log out:
   1. The user clicks the button `Log out`.
   1. The user is redirected to Auth0 platform and immediatelly back to the app.

As you can see we don't have to think about tokens, storages, cookies, emails, backend APIs and other "low-level" things thanks to the SDK.

Auth0 uses a combination of memory and cookies instead of `LocalStorage` for managing tokens by default, the log in and sign up process is almost completely orchestrated on their platform (even SDK script is hosted on their CDN) so it should be secure enough.
The trade-off could be the worse UX because of redirections and inconsistent UI in the app and among emails the user receives.  


## 1. Auth0 account configuration

1. Create a new [Auth0](https://auth0.com/) account or log in if you already have one.

1. Edit the `Default App` or create a new app in Auth0 administration (menu item `Applications`, tab `Settings`):

   ![Auth0 App](/static/images/time_tracker_auth0_app.png)

   1. Name: `Time Tracker`
   1. Application Type: `Single Page Application`
   1. Allowed Callback Urls: `http://localhost:8000`
   1. Allowed Logout URLs: `http://localhost:8000`
   1. Allowed Web Origins: `http://localhost:8000`
   1. Click the button `SAVE CHANGES`

1. Note somewhere the `Domain` and `Client ID` values from the same tab.

1. Edit universal login (menu item `Universal Login`, tab `Settings`):

   ![Auth0 Experience](/static/images/time_tracker_auth0_experience.png)

   1. Switch `Experience` to `New`.
   1. Confirm with `SAVE CHANGES`

## 2. Auth0 SDK initialization

We'll move the app initialization script from `index.html` to `index.js`, fetch auth data and pass user data from SDK's JS Auth0 client to the Rust part of the app.

_Note:_ I was drawing inspiration from the official [Auth0 tutorial](https://auth0.com/docs/quickstart/spa/vanillajs) for writing SPA in vanilla JS and from [SDK docs](https://auth0.com/docs/libraries/auth0-single-page-app-sdk) during the code writing.

1. Add a new file `/index.js` with the app and auth initialization script:
    ```js
    import init from '/pkg/package.js';

    let auth0 = null;

    window.init_auth = async (domain, client_id) => {
        auth0 = await createAuth0Client({
            domain,
            client_id,
        });

        const query = window.location.search;
        if (query.includes("code=") && query.includes("state=")) {
            await auth0.handleRedirectCallback();
        }

        if (await auth0.isAuthenticated()) {
            return await auth0.getUser();
        }
    }

    init('/pkg/package_bg.wasm');
    ```
    - _Note_: We only read url query parameters here - we'll remove them from the history later, in the Rust code.

1. Add SDK and `index.js` into our `/index.html` (remove the old initialization script):
    ```html
    ...
    <body>
        <section id="app"></section>

        <script src="https://cdn.auth0.com/js/auth0-spa-js/1.9/auth0-spa-js.production.js"></script>
        <script src="/index.js" type="module"></script>
    </body>
    ...
    ```

1. Create a new file `/auth_config.json` with the content:
    ```json
    {
        "domain": "YOUR_DOMAIN",
        "client_id": "YOUR_CLIENT_ID"
    }
    ```
    - _Note:_ This file would be modified during the deploy for each environment (e.g. dev, test, prod) or replaced by the API endpoint.

1. Add new dependencies to `Cargo.toml`:
    ```toml
    [dependencies]
    ...
    serde = "1.0.115"
    wasm-bindgen-futures = "0.4.17"
    serde-wasm-bindgen = "0.1.3"
    ```
    - [serde](https://serde.rs/) is required for `AuthCofing` and `User` deserialization.
    - [wasm-bindgen-futures](https://rustwasm.github.io/wasm-bindgen/api/wasm_bindgen_futures/) is required for calling async JS functions from the Rust code.
    - [serde-wasm-bindgen](https://crates.io/crates/serde-wasm-bindgen) can directly (i.e. without `JSON.stringify`) transform JS User object to the Rust `User` struct.

1. Fetch `auth_config.json` content to a new `Model` field `auth_config` on app start in `lib.rs`:
    ```rust
    use seed::{prelude::*, *};
    use serde::Deserialize;

    ...

    fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
        orders
            ....
            .stream(...)
            .perform_cmd(async { 
                Msg::AuthConfigFetched(
                    async { fetch("/auth_config.json").await?.check_status()?.json().await }.await
                )
            });

        Model {
            ...
            auth_config: None,
        }
    }

    ...

    // ------ AuthConfig ------

    #[derive(Deserialize)]
    struct AuthConfig {
        domain: String,
        client_id: String,
    }

    // ------ ------
    //     Urls
    // ------ ------

    ...

    enum Msg {
        ...
        HideMenu,
        AuthConfigFetched(fetch::Result<AuthConfig>),
        ...
    }

    ...

    fn update(...) {
        match msg {
            ...
            Msg::HideMenu => {
                ...
            },
            Msg::AuthConfigFetched(Ok(auth_config)) => model.auth_config = Some(auth_config),
            Msg::AuthConfigFetched(Err(fetch_error)) => error!("AuthConfig fetch failed!", fetch_error),
            ...
        }
    ```
    - _Notes:_ 
        - You'll learn more about fetching in the next chapter.
        - There should be a better error handling than a simple `error!` call in the production app - we should at least show the user-friendly message on the website and describe user-friendly steps how to resolve the problem.
        - We don't really need to store `AuthConfig` in our `Model` because it will be passed to a new JS Auth0 client, but in this development phase it's useful for debugging and maybe we'll need it later. 

1. Remove mocked `User` and update `User` fields according the data that will be sent from `auth0.getUser()`:
    ```rust
    fn init... {
        ...
        Model {
            ctx: Context {
                user: None,

    ...

    #[derive(Deserialize)]
    struct User {
        nickname: String,
        name: String,
        picture: String,
        updated_at: String,
        email: String,
        email_verified: bool,
        sub: String,
    }

    // ------ Page ------
    ```
    - _Note:_ I've defined the `User` fields according the data coming from `auth0.getUser()`. However I assume all possible fields are listed in the `/userinfo` [endpoint docs](https://auth0.com/docs/api/authentication#get-user-info). 

1. Add "bridge" between the JS and Rust world:
    ```rust
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(catch)]
        async fn init_auth(domain: String, client_id: String) -> Result<JsValue, JsValue>;
    }

    // ------ ------
    //     View
    // ------ ------
    ```

    - _Notes:_
        - `#[wasm_bindgen(catch)]` catches the eventual `init_auth` exception and returns it as the second `JsValue` in `-> Result<JsValue, JsValue>`
        - See more about JS async functions handling in [this wasm-bindgen article](https://rustwasm.github.io/wasm-bindgen/reference/js-promises-and-rust-futures.html).
        - We have to return `JsValue` (the first one) instead of `Option<User>` and convert it manually later - it's the current `wasm-bindgen` limitation.

1. Call `init_auth` once `AuthConfig` is fetched and handle the resolved `promise`/`Future` in the `match` arm `Msg::AuthInitialized(Ok(user)) => {...}`:
    ```rust
    enum Msg {
        ...
        AuthConfigFetched(...),
        AuthInitialized(Result<JsValue, JsValue>),

    ...

    fn update(...) {
        match msg {
            ...
            Msg::AuthConfigFetched(Ok(auth_config)) => {
                let domain = auth_config.domain.clone();
                let client_id = auth_config.client_id.clone();

                orders.perform_cmd(async { Msg::AuthInitialized(
                    init_auth(domain, client_id).await
                )});
                model.auth_config = Some(auth_config);
            },
            Msg::AuthConfigFetched(Err(fetch_error)) => error!("AuthConfig fetch failed!", fetch_error),
            Msg::AuthInitialized(Ok(user)) => {
                if not(user.is_undefined()) {
                    match serde_wasm_bindgen::from_value(user) {
                        Ok(user) => model.ctx.user = Some(user),
                        Err(error) => error!("User deserialization failed!", error),
                    }
                }

                let search = model.base_url.search_mut();
                if search.remove("code").is_some() && search.remove("state").is_some() {        
                    model.base_url.go_and_replace();
                }
            }
            Msg::AuthInitialized(Err(error)) => {
                error!("Auth initialization failed!", error);
            }
        }
    ```
    - We also have to remove special auth query parameters `code` and `state` from our `base_url` and from the browser history (by calling `go_and_replace` on the modified `Url`). Otherwise the Auth0 client would fail on the app reload because the `code` is valid only once and the ugly url in the browser bar would reduce UX.

## 3. Sign up, Log in, Log out

Let's make our header buttons useable by connecting them with the Auth0 client.

1. Add one function for each feature in `index.js`:
    ```js
    ...
    window.redirect_to_sign_up = async () => {
        await auth0.loginWithRedirect({
            redirect_uri: window.location.origin,
            screen_hint: "signup"
        });
    }

    window.redirect_to_log_in = async () => {
        await auth0.loginWithRedirect({
            redirect_uri: window.location.origin,
        });
    }

    window.logout = () => {
        auth0.logout({
            returnTo: window.location.origin
        });
    }

    init('/pkg/package_bg.wasm');
    ```

1. And then add corresponding external imports in `lib.rs`:
    ```rust
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(catch)]
        async fn init_auth...;

        #[wasm_bindgen(catch)]
        async fn redirect_to_sign_up() -> Result<(), JsValue>;

        #[wasm_bindgen(catch)]
        async fn redirect_to_log_in() -> Result<(), JsValue>;

        #[wasm_bindgen(catch)]
        fn logout() -> Result<(), JsValue>;
    }
    ```

1. And call them in `update` function:
    ```rust
    enum Msg {
        AuthInitialized...,
        SignUp,
        LogIn,
        LogOut,
        RedirectingToSignUp(Result<(), JsValue>),
        RedirectingToLogIn(Result<(), JsValue>),

    ...

    fn update(...) {
        match msg {
            ...
            Msg::AuthInitialized...
            Msg::SignUp => {
                orders.perform_cmd(async { Msg::RedirectingToSignUp(
                    redirect_to_sign_up().await
                )});
            },
            Msg::LogIn => {
                orders.perform_cmd(async { Msg::RedirectingToLogIn(
                    redirect_to_log_in().await
                )});
            },
            Msg::RedirectingToSignUp(result) => {
                if let Err(error) = result {
                    error!("Redirect to sign up failed!", error);
                }
            },
            Msg::RedirectingToLogIn(result) => {
                if let Err(error) = result {
                    error!("Redirect to log in failed!", error);
                }
            }
            Msg::LogOut => {
                if let Err(error) = logout() {
                    error!("Cannot log out!", error);
                } else {
                    model.ctx.user = None;
                }
            },
    ```

1. Let's fire new `Msg`s from our header buttons and we also have to rename a `User` field reference because we've updated the `User` struct:
    ```rust
    fn view_buttons_for_logged_in_user(...) -> Vec<Node<Msg>> {
        vec![
            a![
                C!["button", "is-primary"],
                ...
                strong![&user.nickname],
            ],
            a![
                C!["button", "is-light"],
                "Log out",
                ev(Ev::Click, |_| Msg::LogOut),
            ]
        ]
    }

    ...

    fn view_buttons_for_anonymous_user() -> Vec<Node<Msg>> {
    vec![
            a![
                C!["button", "is-primary"],
                strong!["Sign up"],
                ev(Ev::Click, |_| Msg::SignUp),
            ],
            a![
                C!["button", "is-light"],
                "Log in",
                ev(Ev::Click, |_| Msg::LogIn),
            ]
        ]
    }
    ```

---

And that's it! We've successfully integrated secure authentication. 

There are still some things that should be polished - e.g. show a loading spinner instead of `Sign up` and `Log in` buttons while the Auth0 client is initializing - but it's good enough for now.

The next chapter should explain how fetching works. And then we'll explore [Slash GrahpQL](https://dgraph.io/slash-graphql) and try to implement some backend APIs.



