// native app entry_point

use async_std::task::block_on;

// use leptos::prelude::{Effect, Get, IntoView, NodeRef, NodeRefAttribute};
use leptos::task::spawn_local;
use leptos::wasm_bindgen::JsCast;
use leptos::*;
use leptos::{html::Canvas, prelude::*};
use nannou::glam::vec3;
use sketch::{run_app, Model};

mod sketch;

use leptos::html::{Div, Html};
use web_sys::HtmlCanvasElement;

// #[component]
// pub fn NannouCanvas() -> impl IntoView {
//     let div_ref: NodeRef<Div> = NodeRef::new();
//
//     Effect::new(move |_| {
//         if let Some(div) = div_ref.get() {
//             // Initialize Nannou app when the div is mounted
//             spawn_local(async move {
//                 let model = Model {};
//                 run_app(model).await;
//             });
//         }
//     });
//
//     view! {
//         <div
//             node_ref=div_ref
//             style="width: 100%; height: 100vh; position: relative;"
//         >
//             <canvas
//                 id="nannou"
//                 style="width: 100%; height: 100%;"
//             />
//         </div>
//     }
// }

#[component]
fn App() -> impl IntoView {
    let canvas_ref: NodeRef<Canvas> = NodeRef::new();

    Effect::new(move |_| {
        if let Some(canvas) = canvas_ref.get() {
            canvas.set_id("nannou");

            canvas.set_height(1000);
            canvas.set_width(1000);

            // Set important CSS directly on the element
            // canvas
            //     .style()
            //     .set_property("width", "100% !important")
            //     .unwrap();
            // canvas
            //     .style()
            //     .set_property("height", "100% !important")
            //     .unwrap();
            //
            spawn_local(async move {
                let model = Model {};
                run_app(model).await;
            });
        }

        let canvas: HtmlCanvasElement = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .query_selector("canvas")
            .unwrap()
            .unwrap()
            .unchecked_into();

        canvas.set_width(990);
        canvas.set_height(560);
    });

    view! {
        <div style="width: 100%; height: 100vh; display: flex; justify-content: center; align-items: center; user-select: none; -webkit-user-select: none; -moz-user-select: none; -ms-user-select: none;">
            <canvas
                node_ref=canvas_ref
                id="nannou"
                style="width: 990px !important; height: 560px !important;"
            />
        </div>
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
