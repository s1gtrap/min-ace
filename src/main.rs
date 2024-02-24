#![allow(non_snake_case)]

use dioxus::prelude::*;
use wasm_bindgen::prelude::*;

#[component]
pub fn Editor(state: Signal<()>) -> Element {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let mut editor = use_signal(|| None);
    use_effect({
        move || {
            log::info!("enter 1st effect");
            if editor.read().is_none() {
                let elem = document.get_element_by_id("editor").unwrap();
                let closure = Closure::new({
                    move || {
                        log::info!("enter event handler");
                        *state.write() = ();
                        log::info!("exit event handler");
                    }
                });
                elem.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
                    .unwrap();
                closure.forget();
                *editor.write() = Some(elem);
            }
            log::warn!("exit 1st effect");
        }
    });
    use_effect(move || {
        log::info!("enter 2nd effect");
        if let Some(editor) = &*editor.read() {
            editor.set_inner_html(&format!("{:?}", (&state))); // THIS
        }
        log::warn!("exit 2nd effect");
    });
    rsx! {
        div {
            id: "editor",
            class: "h-full bg-slate-100",
            ""
        }
    }
}

fn app() -> Element {
    let state = use_signal(|| ());
    rsx! {
        div {
            div {
                class: "h-screen",
                Editor {
                    state,
                }
            }
        }
    }
}

fn main() {
    dioxus_logger::init(log::LevelFilter::Trace).expect("failed to init logger");
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default_with_config(
        tracing_wasm::WASMLayerConfigBuilder::default()
            .set_max_level(tracing::Level::TRACE)
            .build(),
    );
    launch(app);
}
