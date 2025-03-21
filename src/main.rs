// native app entry_point

// use leptos::prelude::{Effect, Get, IntoView, NodeRef, NodeRefAttribute};
use leptos::task::spawn_local;
use leptos::*;
use leptos::{html::Canvas, prelude::*};
use leptos_use::{use_mutation_observer_with_options, UseMutationObserverOptions};
use sketch::run_app;
use web_sys::console;

mod sketch;

#[component]
fn App() -> impl IntoView {
    let canvas_ref: NodeRef<Canvas> = NodeRef::new();

    let target_style = "width: 100%; height: 100%;";

    use_mutation_observer_with_options(
        canvas_ref,
        move |_entries, _observer| {
            if let Some(canvas) = canvas_ref.get() {
                if let Some(attr) = canvas.get_attribute("style") {
                    if attr != target_style {
                        canvas.set_attribute("style", &target_style).unwrap();
                    }
                }
            }
        },
        UseMutationObserverOptions::default().attributes(true),
    );

    Effect::new(move |_| {
        if let Some(canvas) = canvas_ref.get() {
            canvas.set_id("nannou");

            spawn_local(async move {
                run_app().await;
            });
        }
    });

    view! {
        <canvas
            node_ref=canvas_ref
            // id="nannou"
            style="width: 990px !important; height: 560px !important;"
        />
    }
    // view! {
    //     <div style="width: 100%; height: 100vh; display: flex; justify-content: center; align-items: center; user-select: none; -webkit-user-select: none; -moz-user-select: none; -ms-user-select: none;">
    //         <div style="display: flex; flex-direction: column; align-items: center;">
    //             <h1 style="color: white; margin-bottom: 10px;">HALLO</h1>
    //             <canvas
    //                 node_ref=canvas_ref
    //                 id="nannou"
    //                 style="width: 990px !important; height: 560px !important;"
    //             />
    //         </div>
    //     </div>
    // }
}

fn main() {
    leptos::mount::mount_to_body(|| view! { <App /> });
}
