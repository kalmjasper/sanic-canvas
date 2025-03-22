// native app entry_point

// use leptos::prelude::{Effect, Get, IntoView, NodeRef, NodeRefAttribute};
use leptos::task::spawn_local;
use leptos::*;
use leptos::{html::Canvas, prelude::*};
use leptos_router::{components::*, path};
use sketch::run_app;

mod sketch;

#[component]
fn HelloWorld() -> impl IntoView {
    view! {
        <div style="background-color: white; color: black;">
            <h1>Hello, World!</h1>
        </div>
    }
}

#[component]
fn App() -> impl IntoView {
    let canvas_ref: NodeRef<Canvas> = NodeRef::new();

    Effect::new(move |_| {
        if let Some(canvas) = canvas_ref.get() {
            canvas.set_id("sanic");

            spawn_local(async move {
                run_app().await;
            });
        }
    });

    view! {
        <div style="width: 100%; height: 100%;">
            <canvas
                style="outline: none;"
                node_ref=canvas_ref
            />
        </div>
    }
}

fn main() {
    // Initialize Leptos
    // leptos::mount::mount_to_body(|| {
    //     view! { <Router><Routes fallback=|| "Not found.">
    //         <Route path=path!("/ooo") view=|| view! { <App/> } />
    //         <Route path=path!("/hello") view=|| view! { <HelloWorld/> } />
    //     </Routes></Router> }
    // });

    leptos::mount::mount_to_body(|| {
        view! { <App/> }
    });
}
