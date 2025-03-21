// native app entry_point


// use leptos::prelude::{Effect, Get, IntoView, NodeRef, NodeRefAttribute};
use leptos::task::spawn_local;
use leptos::*;
use leptos::{html::Canvas, prelude::*};
use leptos_use::use_resize_observer;
use sketch::{run_app, Model};

mod sketch;


#[component]
fn App() -> impl IntoView {
    let canvas_ref: NodeRef<Canvas> = NodeRef::new();

    Effect::new(move |_| {
        if let Some(canvas) = canvas_ref.get() {
            canvas.set_id("nannou");

            spawn_local(async move {
                let model = Model {};
                run_app(model).await;
            });
        }
    });

    use_resize_observer(canvas_ref, move |entries, _observer| {
        let rect = entries[0].content_rect();
        println!("rect: {:?}", rect);
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
