use leptos::{component, create_effect, create_signal, expect_context, IntoView, provide_context, Scope, SignalGet, spawn_local, view};
use leptos::*;
use crate::app::GlobalState;


/// Renders the home page of your application.
#[component]
pub fn HomePage(cx: Scope) -> impl IntoView {

    let state = use_context::<RwSignal<GlobalState>>(cx).expect("state to have been provided");

    let (guide_set, set_guide_set) = create_slice(
        cx,
        // we take a slice *from* `state`
        state,
        // our getter returns a "slice" of the data
        |state| state.guide_set,
        // our setter describes how to mutate that slice, given a new value
        |state, n| state.guide_set = n,
    );

    view! { cx,
        <div class="flex flex-col w-[90%] h-[90%] bg-white">
            <h1 class="text-center text-4xl font-extrabold text-red-600 bg-gray-100 flex-shrink-0 h-24 border-b border-gray-700">
                "Welcome to the SOS Internal Developer Portal"
            </h1>
         <button
            class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
            on:click=move |_| {
                set_guide_set(!guide_set.get());
            }
        >

            "Show Me Why You Matter"
        </button>
            <GuidePopup/>
            <div id="overallContainer" class="flex flex-grow">
                // <div id="sideBar" class="w-1/8 h-full bg-gray-100 p-4 border-r border-gray-300">
                //     <SideBar/>
                // </div>
                <div id="mainContainer" class="w-full h-full bg-white p-4 border-l border-gray-300">
                    <MainContainer/>
                </div>
            </div>
        </div>
    }
}



/// 404 - Not Found
#[component]
pub fn NotFound(cx: Scope) -> impl IntoView {
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

    view! { cx, <h1>"Not Found"</h1> }
}

#[component]
pub fn MainContainer(cx: Scope) -> impl IntoView {

    let state = use_context::<RwSignal<GlobalState>>(cx).expect("state to have been provided");
    let (card_class, set_card_class) = create_signal(cx, "m-4 group relative bg-gray-100 p-4 rounded-none shadow-lg w-64 h-64 hover:bg-gray-200 hover:shadow-xl transition duration-300".to_string());

    let (guide_set, set_guide_set) = create_slice(
        cx,
        // we take a slice *from* `state`
        state,
        // our getter returns a "slice" of the data
        |state| state.guide_set,
        // our setter describes how to mutate that slice, given a new value
        |state, n| state.guide_set = n,
    );

    create_effect(cx, move |_| {
        if guide_set() {
            set_card_class("m-4 group relative bg-red-100 p-4 rounded-none shadow-lg w-64 h-64 hover:bg-red-200 hover:shadow-xl transition duration-300".to_string());
        } else {
            set_card_class("m-4 group relative bg-gray-100 p-4 rounded-none shadow-lg w-64 h-64 hover:bg-gray-200 hover:shadow-xl transition duration-300".to_string());
        }
    }, /* f */);

    let microservice_guide_components = GuideProps {
        title: "Create a New Microservice".to_string(),
        content: "Hover to see choices.".to_string(),
    };

    view! { cx,
        <h1 class="text-2xl font-bold mb-4 text-red-600 text-center">Available Actions are listed on the cards</h1>

        <p class="text-1xl font-bold mb-4 text-red-600 text-center">Select an Action from the CARDS below.</p>

            <div class="flex flex-wrap justify-between">

            <div class={card_class}>
                <GuideComponent props=microservice_guide_components/>
                <h2 class="text-xl font-semibold mb-2 text-red-600">Create a New Microservice</h2>
                <p class="text-black">Hover to see choices.</p>
                <div class="absolute left-0 bottom-0 w-full bg-white opacity-0 group-hover:opacity-100 transition duration-300">
                    {move || {
                        if guide_set() {
                            view! { cx,
                                <a href="/quarkus?guide_started=true" class="block p-2">
                                    "Quarkus"
                                </a>
                            }
                        } else {
                            view! { cx,
                                <a href="/quarkus" class="block p-2">
                                    "Quarkus"
                                </a>
                            }
                        }
                    }}
                    <a href="#" class="block p-2">
                        Spring-Boot
                    </a> <a href="#" class="block p-2">
                        Rust
                    </a>
                </div>
            </div>

            <div class="m-4 group relative bg-gray-100 p-4 rounded-none shadow-lg w-64 h-64 hover:bg-gray-200 hover:shadow-xl transition duration-300">
                <h2 class="text-xl font-semibold mb-2 text-red-600">Request a Database</h2>
                <p class="text-black">Lorem ipsum dolor sit.</p>
                <div class="absolute left-0 bottom-0 w-full bg-white opacity-0 group-hover:opacity-100 transition duration-300">
                    <a href="/mssql" class="block p-2">
                        MSSQL
                    </a>
                    <a href="#" class="block p-2">
                        Postgres
                    </a>
                </div>
            </div>

            <div class="m-4 group relative bg-gray-100 p-4 rounded-none shadow-lg w-64 h-64 hover:bg-gray-200 hover:shadow-xl transition duration-300">
                <h2 class="text-xl font-semibold mb-2 text-red-600">Add To Existing Service</h2>
                <p class="text-black">Lorem ipsum dolor sit.</p>
                <div class="absolute left-0 bottom-0 w-full bg-white opacity-0 group-hover:opacity-100 transition duration-300">
                    <a href="#" class="block p-2">
                        AMQ
                    </a>
                    <a href="#" class="block p-2">
                        Outbound
                    </a>
                    <a href="#" class="block p-2">
                        Keycloak
                    </a>
                </div>
            </div>

        <div class="m-4 group relative bg-gray-100 p-4 rounded-none shadow-lg w-64 h-64 hover:bg-gray-200 hover:shadow-xl transition duration-300">
    <h2 class="text-xl font-semibold mb-2 text-red-600">API Documentation</h2>
    <p class="text-black">Find API specs and examples.</p>
    <div class="absolute left-0 bottom-0 w-full bg-white opacity-0 group-hover:opacity-100 transition duration-300">
        <a href="#" class="block p-2">
            REST APIs
        </a>
        <a href="#" class="block p-2">
            GraphQL APIs
        </a>
    </div>
</div>

        // Code Samples Card
<div class="m-4 group relative bg-gray-100 p-4 rounded-none shadow-lg w-64 h-64 hover:bg-gray-200 hover:shadow-xl transition duration-300">
    <h2 class="text-xl font-semibold mb-2 text-red-600">Code Samples</h2>
    <p class="text-black">Get started with code snippets.</p>
    <div class="absolute left-0 bottom-0 w-full bg-white opacity-0 group-hover:opacity-100 transition duration-300">
        <a href="#" class="block p-2">
            Java
        </a>
        <a href="#" class="block p-2">
            Python
        </a>
    </div>
</div>

        <div class="m-4 group relative bg-gray-100 p-4 rounded-none shadow-lg w-64 h-64 hover:bg-gray-200 hover:shadow-xl transition duration-300">
    <h2 class="text-xl font-semibold mb-2 text-red-600">Developer Tools</h2>
    <p class="text-black">Explore dev tools and libraries.</p>
    <div class="absolute left-0 bottom-0 w-full bg-white opacity-0 group-hover:opacity-100 transition duration-300">
        <a href="#" class="block p-2">
            CI/CD Pipelines
        </a>
        <a href="#" class="block p-2">
            Monitoring Tools
        </a>
    </div>
</div>

        </div>
    }
}

#[component]
fn GuidePopup(cx: Scope) -> impl IntoView {

    let (guide_step, set_guide_step) = create_signal(cx, 0);
    let state = use_context::<RwSignal<GlobalState>>(cx).expect("state to have been provided");

    let (guide_set, set_guide_set) = create_slice(
        cx,
        // we take a slice *from* `state`
        state,
        // our getter returns a "slice" of the data
        |state| state.guide_set,
        // our setter describes how to mutate that slice, given a new value
        |state, n| state.guide_set = n,
    );


    let point = move || {
        if guide_step.get() == 0 {
            return Some("Hey there, Developer! 👋 Welcome to your new best friend, the Internal Developer Portal. This guide will walk you through why you should care, how to use it, and what you can do here. Ready? Let's go!")
        }
        if guide_step.get() == 1 {
            return Some("First off, why should you care? Well, imagine having all the tools, resources, and documentation you need in one place. No more juggling between tabs or digging through outdated wikis. Sounds like a dream, right? That's what IDP is all about!")
        }
        if guide_step.get() == 2 {
            return Some("Great, you're interested! Now, how does this work? It's simple! See that card that says 'Create a New Microservice'? Hover over it and select 'Quarkus'. You'll see how easy and fast it is to kickstart a new project!")
        }
        if guide_step.get() == 3 {
            return Some("Okay, you've got the basics down. But there's more! From generating pipelines to requesting databases, IDP is your one-stop-shop for all things development. Explore and let your creativity run wild!")
        }
        if guide_step.get() == 3 {
            return Some("Awesome, you're all set to make the most out of IDP! Remember, this is built for you, so if you have any feedback or features you'd like to see, don't hesitate to let us know. Happy coding! 🚀")
        }
        if guide_step.get() >= 4 {
            set_guide_step(0);
            set_guide_set(false);
        }
        return None
    };

    view! { cx,
        {move || {
            if guide_set() {
                view! { cx,
                    <div class="fixed inset-0 flex items-center justify-center z-50 pointer-events-none">
                        <div class="bg-black opacity-50 absolute inset-0"></div>
                        <div
                            class="bg-gray-100 p-6 rounded-lg shadow-lg relative z-10"
                            style="pointer-events: all;"
                        >
                            <h2 class="text-xl font-semibold mb-2">Internal Developer</h2>
                            <p>{point}</p>
                            <button
                                class="mt-4 bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
                                on:click=move |_| {
                                    set_guide_set(true);
                                    set_guide_step(guide_step.get() + 1);
                                }
                            >

                                "Next ->"
                            </button>
                        </div>
                    </div>
                }
            } else {

                view! { cx, <div></div> }
            }
        }}
    }
}

#[component]
pub fn QuarkusMicroService(cx: Scope) -> impl IntoView {
    let (name, set_name) = create_signal(cx, String::new());
    let (project_name, set_project_name) = create_signal(cx, String::new());
    let (namespace, set_namespace) = create_signal(cx, String::new());
    let (project_key, set_project_key) = create_signal(cx, String::new());

    let state = use_context::<RwSignal<GlobalState>>(cx).expect("state to have been provided");

    let (guide_set, set_guide_set) = create_slice(
        cx,
        // we take a slice *from* `state`
        state,
        // our getter returns a "slice" of the data
        |state| state.guide_set,
        // our setter describes how to mutate that slice, given a new value
        |state, n| state.guide_set = n,
    );

    view! { cx,
        <div class="p-4">
            <h2 class="text-xl font-semibold mb-4">Create a Quarkus Microservice</h2>
            <form on:submit=move |ev| {
                ev.prevent_default();
            }>
                <div class="mb-4">
                    <label for="name" class="block text-sm font-medium text-gray-600">
                        Name
                    </label>
                    <input
                        id="name"
                        type="text"
                        class="mt-1 p-2 w-full border rounded-md"
                        on:input=move |ev| {
                            set_name(event_target_value(&ev));
                        }
                    />
                </div>
                <div class="mb-4">
                    <label for="projectName" class="block text-sm font-medium text-gray-600">
                        Project Name
                    </label>
                    <input
                        id="projectName"
                        type="text"
                        class="mt-1 p-2 w-full border rounded-md"
                        on:input=move |ev| {
                            set_project_name(event_target_value(&ev));
                        }
                    />
                </div>
                <div class="mb-4">
                    <label for="namespace" class="block text-sm font-medium text-gray-600">
                        Namespace
                    </label>
                    <input
                        id="namespace"
                        type="text"
                        class="mt-1 p-2 w-full border rounded-md"
                        on:input=move |ev| {
                            set_namespace(event_target_value(&ev));
                        }
                    />
                </div>
                <div class="mb-4">
                    <label for="projectKey" class="block text-sm font-medium text-gray-600">
                        Project Key
                    </label>
                    <input
                        id="projectKey"
                        type="text"
                        class="mt-1 p-2 w-full border rounded-md"
                        on:input=move |ev| {
                            set_project_key(event_target_value(&ev));
                        }
                    />
                </div>
                <button
                    type="submit"
                    class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
                >
                    "Submit"
                </button>
            </form>
       {move || if guide_set() {
            view! { cx,
                <div class="fixed inset-0 flex items-center justify-center z-50 pointer-events-none">
                    <div class="bg-black opacity-50 absolute inset-0"></div>
                    <div class="bg-gray-100 p-6 rounded-lg shadow-lg relative z-10">
                        <h2 class="text-xl font-semibold mb-2">Hey there, Developer!</h2>
                        <p>"Welcome to the Quarkus Microservice creation page! 🚀 Here, you can fill out all the nitty-gritty details to get your new microservice up and running. Just complete the form and hit 'Submit'—easy peasy!"</p>
                        <button
                            class="mt-4 bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
                            on:click=move |_| {
                                // Your action here
                            }
                        >
                            "Got it, let's go!"
                        </button>
                    </div>
                </div>
            }
        } else {
            view! { cx, <div></div> }
        }}
    </div>
    }
}

#[component]
pub fn DatabaseProvisioningPage(cx: Scope) -> impl IntoView {
    let (name, set_name) = create_signal(cx, String::new());
    let (project_name, set_project_name) = create_signal(cx, String::new());
    let (namespace, set_namespace) = create_signal(cx, String::new());
    let (project_key, set_project_key) = create_signal(cx, String::new());

    let state = use_context::<RwSignal<GlobalState>>(cx).expect("state to have been provided");

    let (guide_set, set_guide_set) = create_slice(
        cx,
        // we take a slice *from* `state`
        state,
        // our getter returns a "slice" of the data
        |state| state.guide_set,
        // our setter describes how to mutate that slice, given a new value
        |state, n| state.guide_set = n,
    );

    view! { cx,
        <div class="p-4">
            <h2 class="text-xl font-semibold mb-4">Request a MSSQL Database</h2>
            <form on:submit=move |ev| {
                ev.prevent_default();
            }>
                <div class="mb-4">
                    <label for="name" class="block text-sm font-medium text-gray-600">
                        Database Name - Format is SVG-DB-NAME
                    </label>
                    <input
                        id="name"
                        type="text"
                        class="mt-1 p-2 w-full border rounded-md"
                        on:input=move |ev| {
                            set_name(event_target_value(&ev));
                        }
                    />
                </div>
                <div class="mb-4">
                    <label for="projectName" class="block text-sm font-medium text-gray-600">
                        Environments - Format is ENV1,ENV2,ENV3
                    </label>
                    <input
                        id="projectName"
                        type="text"
                        class="mt-1 p-2 w-full border rounded-md"
                        on:input=move |ev| {
                            set_project_name(event_target_value(&ev));
                        }
                    />
                </div>
                <div class="mb-4">
                    <label for="namespace" class="block text-sm font-medium text-gray-600">
                        Namespace
                    </label>
                    <input
                        id="namespace"
                        type="text"
                        class="mt-1 p-2 w-full border rounded-md"
                        on:input=move |ev| {
                            set_namespace(event_target_value(&ev));
                        }
                    />
                </div>
                <div class="mb-4">
                    <label for="projectKey" class="block text-sm font-medium text-gray-600">
                        Users - Types are DBO,READER,WRITER, and format is ENV:USER1:DBO,ENV:USER2:READER
                    </label>
                    <input
                        id="projectKey"
                        type="text"
                        class="mt-1 p-2 w-full border rounded-md"
                        on:input=move |ev| {
                            set_project_key(event_target_value(&ev));
                        }
                    />
                </div>
                <button
                    type="submit"
                    class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
                >
                    "Submit"
                </button>

                <button
                    type="other"
                    class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded ml-4"
                >
                    "Generate Standard Setup!"
                </button>
            </form>
       {move || if guide_set() {
            view! { cx,
                <div class="fixed inset-0 flex items-center justify-center z-50 pointer-events-none">
                    <div class="bg-black opacity-50 absolute inset-0"></div>
                    <div class="bg-gray-100 p-6 rounded-lg shadow-lg relative z-10">
                        <h2 class="text-xl font-semibold mb-2">Hey there, Developer!</h2>
                        <p>"Welcome to the MSSQL Request Page! 🚀 Here, you can fill out all the nitty-gritty details to get your new database up and running. Just complete the form and hit 'Submit'—easy peasy!"</p>
                        <button
                            class="mt-4 bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
                            on:click=move |_| {
                                // Your action here
                            }
                        >
                            "Got it, let's go!"
                        </button>
                    </div>
                </div>
            }
        } else {
            view! { cx, <div></div> }
        }}
    </div>
    }
}

#[component]
pub fn GuideComponent(cx: Scope, props: GuideProps) -> impl IntoView {
    let GuideProps { title, content } = props;
    let (show_content, set_show_content) = create_signal(cx, false);
    let (content_class, set_content_class) = create_signal(cx, "hidden".to_string());

    let toggle_content = move |_| {
        set_show_content(!show_content.get());
    };

    create_effect(cx, move |_| {
        if show_content() {
            set_content_class("block absolute top-full left-0 z-10 bg-white p-4 rounded shadow-lg".to_string());
        } else {
            set_content_class("hidden".to_string());
        }
    }, /* f */);

    view! { cx,
        <div class="absolute top-0 right-0 p-2 cursor-pointer hover:bg-red-200 rounded-full">
            <div class="absolute top-0 right-0 p-2 cursor-pointer hover:bg-red-200 rounded-full" on:click={toggle_content}>
                    {move || {
                        if show_content() {
                            view! { cx,
                                "absolute top-0 right-0 p-2 cursor-pointer hover:bg-red-200 rounded-full"
                            }
                        } else {
                            view! { cx,
                                "?"
                            }
                        }
                    }}
            </div>
                            <div class={content_class()}>
                <h3 class="text-lg font-semibold">
                    {title}
                </h3>
                <p>
                    {content}
                </p>
            </div>
        </div>

    }
}

pub struct GuideProps {
    pub title: String,
    pub content: String,
}