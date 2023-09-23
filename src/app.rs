use crate::app::invite_manager::InviteManager;
use leptos::ev::MouseEvent;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use serde_derive::{Deserialize, Serialize};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, Once};

mod invite_manager;
mod components;
mod server_functions;

use components::*;

static INIT: Once = Once::new();
static GET_INVITES_CALL_AMOUNT: AtomicUsize = AtomicUsize::new(0);
static mut INVITE_MANAGER: Option<Arc<Mutex<InviteManager>>> = None;

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
