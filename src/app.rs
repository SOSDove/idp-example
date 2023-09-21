use crate::app::invite_manager::InviteManager;
use leptos::ev::MouseEvent;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use serde_derive::{Deserialize, Serialize};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, Once};

mod invite_manager;

static INIT: Once = Once::new();
static GET_INVITES_CALL_AMOUNT: AtomicUsize = AtomicUsize::new(0);
static mut INVITE_MANAGER: Option<Arc<Mutex<InviteManager>>> = None;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! { cx,
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>
        <link href="https://cdnjs.cloudflare.com/ajax/libs/flowbite/1.8.1/flowbite.min.css" rel="stylesheet" />
        <script src="https://cdnjs.cloudflare.com/ajax/libs/flowbite/1.8.1/flowbite.min.js"></script>

        // sets the document title
        <Title text="SOS Deployment Helper"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="mock-case-portal" view=MockCasePortal/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

///Site Components
#[component]
fn MockCasePortal(cx: Scope) -> impl IntoView {
    view! {cx,
        <h1 class="text-center mb-4 text-4xl font-extrabold">"SOS Mock Case Portal"</h1>
        <div id="mainContainer" class="flex flex-col items-center mx-[10vw] my-[5vh]">
    <div id="infoSection" class="mx-[10vw] w-[80vw] h-2/8 max-h-[800px] bg-gray-100 p-4 mb-8">
      <MainContainer/>
    </div>
    <div id="infoSection2" class="mx-[10vw] w-[80vw] h-4/8 max-h-[800px] bg-gray-100 p-4 mb-8">
      <ParentContainer/>
    </div>
    <div id="infoSection3" class="mx-[10vw] w-[80vw] h-2/8 max-h-[800px] bg-gray-100 p-4">
      <MainContainer/>
    </div>
        </div>
            }
}

/// Renders the home page of your application.
#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    // Creates a reactive value to update the button

    view! { cx,
        <div class="flex flex-col h-screen bg-white">
            <h1 class="text-center text-4xl font-extrabold text-red-600 bg-gray-100 flex-shrink-0 h-24">"Welcome to the SOS Deployment Helper!"</h1>
            <div id="overallContainer" class="flex flex-grow">
                <div id="sideBar" class="w-1/8 h-full bg-gray-100 p-4 border-r border-gray-300">
                    <SideBar/>
                </div>
                <div id="mainContainer" class="flex-grow w-6/8 h-1/2 bg-white p-4 border-l border-gray-300">
                    <MainContainer/>
                </div>
            </div>
        </div>
    }
}




/// 404 - Not Found
#[component]
fn NotFound(cx: Scope) -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>(cx);
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! { cx,
        <h1>"Not Found"</h1>
    }
}

///Container Components
#[component]
fn SideBar(cx: Scope) -> impl IntoView {
    view! { cx,
    <div class="text-xl font-bold mb-2 text-red-600">Deploy a new App</div>

    <div class="ml-4 mb-4 text-black">
      <div class="mb-1">Read Dependencies</div>
      <div class="mb-1">Deploy App</div>
      <div class="mb-1">Generate Pipelines</div>
    </div>

    <div class="text-xl font-bold mb-2 text-red-600">Generate a new App</div>

    <div class="ml-4 text-black">
      <div class="mb-1">Generate App</div>
      <div class="ml-4 mb-1">Generate Quarkus App</div>
      <div class="mb-1">Generate Deployment For New App</div>
    </div>
      }
}

#[component]
fn MainContainer(cx: Scope) -> impl IntoView {
    view! {cx,
    <h1 class="text-2xl font-bold mb-4 text-red-600 text-center">Main Content</h1>

    <p class="mb-4 text-black">
      Select an Action from the CARDS below.
    </p>

    <div class="grid grid-cols-1 md:grid-cols-3 gap-4">

      <div class="group relative bg-gray-100 p-4 rounded-none shadow-lg w-64 h-64 hover:bg-gray-200 hover:shadow-xl transition duration-300">
        <h2 class="text-xl font-semibold mb-2 text-red-600">Create a New Microservice</h2>
        <p class="text-black">Hover to see choices.</p>
        <div class="absolute left-0 bottom-0 w-full bg-white opacity-0 group-hover:opacity-100 transition duration-300">
          <a href="#" class="block p-2">Quarkus</a>
          <a href="#" class="block p-2">Spring-Boot</a>
          <a href="#" class="block p-2">Rust</a>
        </div>
      </div>


      <div class="group relative bg-gray-100 p-4 rounded-none shadow-lg w-64 h-64 hover:bg-gray-200 hover:shadow-xl transition duration-300">
        <h2 class="text-xl font-semibold mb-2 text-red-600">Request a Database</h2>
        <p class="text-black">Lorem ipsum dolor sit.</p>
        <div class="absolute left-0 bottom-0 w-full bg-white opacity-0 group-hover:opacity-100 transition duration-300">
          <a href="#" class="block p-2">MSSQL</a>
          <a href="#" class="block p-2">Postgres</a>
        </div>
      </div>

      <div class="group relative bg-gray-100 p-4 rounded-none shadow-lg w-64 h-64 hover:bg-gray-200 hover:shadow-xl transition duration-300">
        <h2 class="text-xl font-semibold mb-2 text-red-600">Add To Existing Service</h2>
        <p class="text-black">Lorem ipsum dolor sit.</p>
        <div class="absolute left-0 bottom-0 w-full bg-white opacity-0 group-hover:opacity-100 transition duration-300">
          <a href="#" class="block p-2">AMQ</a>
          <a href="#" class="block p-2">Outbound</a>
          <a href="#" class="block p-2">Keycloak</a>
        </div>
      </div>

    </div>
      }
}


#[component]
fn InfoContainer(cx: Scope) -> impl IntoView {
    view! {cx,

    }
}

#[component]
fn SearchComponent(cx: Scope) -> impl IntoView {
    let (query, set_query) = create_signal(cx, String::new());

    view! { cx,
        <div id="searchComponent" class="flex items-center justify-center mb-4 p-4 bg-white rounded shadow-lg">
            <input type="text"
                   class="border-2 border-gray-300 p-2 rounded"
                   placeholder="Search..."
                   on:input=move |ev| {
                       set_query(event_target_value(&ev));
                   }
            />
            <button class="ml-2 bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
                    on:click=move |_| {
                let query_value = query.get().to_string();
                        // Your search logic here
                spawn_local(async {

                log_this(query_value).await;
            });                    }
            >
                "Search"
            </button>
        </div>
    }
}

#[component]
fn ParentContainer(cx: Scope) -> impl IntoView {
    // 1. Signal to hold the fetched invites
    let (invites, set_invites) = create_signal(cx, vec![]);
    let (should_update_invites, set_should_update_invites) = create_signal(cx, false);

    provide_context(cx, set_invites);

    // Fetch invites on mount
    create_effect(cx, move |_| {
        spawn_local(async move {
            let fetched_invites = get_invites().await.unwrap();
            set_invites(fetched_invites);
        });
    });

    create_effect(cx, move |_| {
        let should_update = should_update_invites.get();
        spawn_local(async move {
            if should_update {
                let fetched_invites = get_invites().await.unwrap();
                set_invites(fetched_invites);
            }
        });
    });

    view! { cx,
        <div id="parentContainer" class="flex flex-col items-center justify-center space-y-4">
            <SearchComponent/>
        <HorizontalValuesBar/>
        <For
            each=invites
            key=|invite: &Invite| invite.invite_id
            view= move |cx, invite: Invite| {
            view! { cx,
                <HorizontalInfoComponent invite={invite}
                on_decline_invite=move |mouseEvent, invite_id: i32| {
                      spawn_local(async move {
                        let invite_status = decline_invite(invite_id).await;
                        set_should_update_invites(true);
                    })
                }
                on_accept_invite=move |mouseEvent, invite_id: i32| {
                    spawn_local(async move {
                        let invite_status = accept_invite(invite_id).await;
                        set_should_update_invites(true);
                    })
                    }/>
                }
            }
        />
        <div id="pagination" class="flex items-center justify-center mt-4">
                <button class="mr-2 bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded">"Prev"</button>
                <span class="mx-2">"Page 1 of 3"</span>
                <button class="ml-2 bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded">"Next"</button>
            </div>
        </div>
    }
}

#[component]
fn HorizontalInfoComponent<F, Y>(
    cx: Scope,
    invite: Invite,
    on_decline_invite: F,
    on_accept_invite: Y,
) -> impl IntoView
where
    F: Fn(MouseEvent, i32) + 'static,
    Y: Fn(MouseEvent, i32) + 'static,
{
    let (accepted_status, set_accepted_status) = create_signal(cx, "Accept".to_string());
    let (declined_status, set_declined_status) = create_signal(cx, "Decline".to_string());

    view! { cx,
        <div id="horizontalInfoComponent" class="flex-grow-0 flex-shrink-0 flex items-center justify-between mb-4 p-4 bg-white rounded shadow-lg space-x-12">
            <div class="invite_id">{invite.invite_id}</div>
            <div class="name">{invite.name}</div>
            <div class="age">{invite.age}</div>
            <div class="gender">{invite.gender}</div>
            <div class="level_of_idiocy">{invite.is_stupid}</div>
            <div class="height">{invite.height}</div>
            <button class="button1 bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
                   on:click=move |mouse| on_accept_invite(mouse.clone(),invite.invite_id.clone())
            >"Accept"</button>
            <button class="button2 bg-red-500 hover:bg-red-700 text-white font-bold py-2 px-4 rounded"
                           on:click=move |mouse| on_decline_invite(mouse.clone(),invite.invite_id.clone())
            >"Decline"</button>
        </div>
    }
}

#[component]
fn HorizontalValuesBar(cx: Scope) -> impl IntoView {
    view! { cx,
        <div id="horizontalInfoComponent" class="flex-grow-0 flex-shrink-0 flex items-center justify-between mb-4 p-4 bg-white rounded shadow-lg space-x-12">
            <div class="invite_id">"Id"</div>
            <div class="name">"Name"</div>
            <div class="age">"Age"</div>
            <div class="gender">"Gender"</div>
            <div class="level_of_idiocy">"Is Stupid?"</div>
            <div class="height">"Height"</div>
        </div>
    }
}

///Server Functions
#[server(LogThis, "/api")]
pub async fn log_this(message: String) -> Result<(), ServerFnError> {
    println!("This is a {}", message);

    Ok(())
}

#[server(AcceptInvite, "/api")]
pub async fn accept_invite(invite_id: i32) -> Result<String, ServerFnError> {
    println!("Accepting invite with id of {}", invite_id);
    let manager = unsafe { INVITE_MANAGER.as_ref().unwrap().clone() };
    manager.lock().unwrap().remove_invite(invite_id);
    Ok("Accepted".to_string())
}

#[server(DeclineInvite, "/api")]
pub async fn decline_invite(invite_id: i32) -> Result<String, ServerFnError> {
    println!("Declining invite with id of {}", invite_id);
    let manager = unsafe { INVITE_MANAGER.as_ref().unwrap().clone() };
    manager.lock().unwrap().remove_invite(invite_id);
    Ok("Declined".to_string())
}

#[server(GetInvites, "/api")]
pub async fn get_invites() -> Result<Vec<Invite>, ServerFnError> {
    println!("Was told to fetch invites");
    INIT.call_once(|| {
        let manager = InviteManager::new();
        manager.initialize_with_five_invites(); // Your logic to add 5 invites initially
        unsafe {
            INVITE_MANAGER = Some(Arc::new(Mutex::new(manager)));
        }
    });

    let manager = unsafe { INVITE_MANAGER.as_ref().unwrap().clone() };

    let invites;
    {
        let manager = manager.lock().unwrap();
        invites = manager.get_invites().clone();
    }
    Ok(invites.to_vec())
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Invite {
    invite_id: i32,
    name: String,
    age: i32,
    gender: String,
    is_stupid: bool,
    height: String,
}
