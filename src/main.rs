#![allow(non_snake_case)]

use std::collections::HashSet;

use dioxus::prelude::*;
use log::LevelFilter;
use wasm_bindgen::prelude::*;

#[derive(Clone, Debug, Eq, Hash, PartialEq, serde::Serialize)]
pub struct Annotation {}

#[wasm_bindgen]
extern "C" {
    type EditorInstance;
    type EditorSession;
    #[wasm_bindgen(js_namespace = ace)]
    fn require(s: &str) -> JsValue;
    #[wasm_bindgen(js_namespace = ace)]
    fn edit(s: &str) -> EditorInstance;
    #[wasm_bindgen(method)]
    fn getSession(this: &EditorInstance) -> EditorSession;
    #[wasm_bindgen(method)]
    fn setAnnotations(this: &EditorSession, annots: JsValue);
    #[wasm_bindgen(method)]
    fn on(this: &EditorSession, ev: &str, func: &Closure<dyn FnMut()>);
}

#[component]
pub fn Editor(annotations: Signal<HashSet<Annotation>>, onchange: EventHandler<String>) -> Element {
    use gloo_utils::format::JsValueSerdeExt;
    let mut editor = use_signal(|| None);
    use_effect({
        move || {
            log::info!("1st effect");
            if editor.read().is_none() {
                let instance: EditorInstance = edit("editor");
                let closure = Closure::new({
                    let onchange = onchange.clone();
                    move || {
                        log::info!("change emitted by ace");
                        //onchange.call(String::new());
                        *annotations.write() = HashSet::new();
                        log::info!("change in ace finished");
                    }
                });
                instance.getSession().on("change", &closure);
                closure.forget();
                *editor.write() = Some(instance);
            }
            log::warn!("1st effect finished");
        }
    });
    use_effect(move || {
        log::info!("2nd effect");
        if let Some(editor) = &*editor.read() {
            let session = editor.getSession();
            session.setAnnotations(
                JsValue::from_serde(&annotations.read().iter().collect::<Vec<_>>()).unwrap(),
            );
            log::warn!("2nd effect finished");
        }
    });
    rsx! {
        div {
            id: "editor",
            class: "h-full",
            ""
        }
    }
}

fn app() -> Element {
    let mut annotations = use_signal(HashSet::new);
    rsx! {
        div {
            div {
                class: "h-screen",
                Editor {
                    annotations: annotations,
                    onchange: move |s: String| {
                        log::info!("change emitted by dx");
                        *annotations.write() = HashSet::new();
                        log::info!("change in dx finished");
                    }
                }
            }
        }
    }
}

fn main() {
    dioxus_logger::init(LevelFilter::Trace).expect("failed to init logger");
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default_with_config(
        tracing_wasm::WASMLayerConfigBuilder::default()
            .set_max_level(tracing::Level::TRACE)
            .build(),
    );
    launch(app);
}
