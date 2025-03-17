// native app entry_point

use async_std::task::block_on;

use leptos::prelude::{Effect, Get, IntoView, NodeRef, NodeRefAttribute};
use leptos::task::spawn_local;
use leptos::*;
use nannou::glam::vec3;
use sketch::{run_app, Model};

mod sketch;

use leptos::html::{Div, Html};

#[component]
pub fn NannouCanvas() -> impl IntoView {
    let div_ref: NodeRef<Div> = NodeRef::new();

    Effect::new(move |_| {
        if let Some(div) = div_ref.get() {
            // Initialize Nannou app when the div is mounted
            spawn_local(async move {
                let model = Model {};
                run_app(model).await;
            });
        }
    });

    view! {
        <div node_ref=div_ref></div>
    }
    // style="width: 100%; height: 100%;">
}

#[component]
fn App() -> impl IntoView {
    view! {
        <NannouCanvas />
    }
}

fn main() {
    leptos::mount::mount_to_body(|| view! { <App /> });
}

// fn main() {
//     let mut p = vec![];
//     for x in -20..20 {
//         for y in -20..20 {
//             p.push(vec3(x as f32, y as f32, 0.0));
//         }
//     }
//     let model = Model {};
//     block_on(async {
//         run_app(model).await;
//     });
// }
//
