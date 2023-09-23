use leptos::ev::MouseEvent;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use serde_derive::{Deserialize, Serialize};

mod components;

use components::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);
    let state = create_rw_signal(cx, GlobalState::default());
    provide_context(cx, state);

    view! { cx,
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>
        <link
            href="https://cdnjs.cloudflare.com/ajax/libs/flowbite/1.8.1/flowbite.min.css"
            rel="stylesheet"
        />
        <script src="https://cdnjs.cloudflare.com/ajax/libs/flowbite/1.8.1/flowbite.min.js"></script>

        // sets the document title
        <Title text="SOS Deployment Helper"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="quarkus" view=QuarkusMicroService/>
                    <Route path="mssql" view=DatabaseProvisioningPage/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

// Global store
#[derive(Default, Clone, Debug)]
struct GlobalState {
    guide_set: bool
}
