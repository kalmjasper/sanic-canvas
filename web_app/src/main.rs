use leptos::task::spawn_local;
use leptos::*;
use leptos::{html::Canvas, prelude::*};
use shared::run_app;

#[component]
fn App() -> impl IntoView {
    let canvas_ref: NodeRef<Canvas> = NodeRef::new();

    Effect::new(move |_| {
        if let Some(canvas) = canvas_ref.get() {
            canvas.set_id("sanic");

            spawn_local(async move {
                run_app(Some("#sanic")).await;
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
    leptos::mount::mount_to_body(|| {
        view! { <App/> }
    });
}
