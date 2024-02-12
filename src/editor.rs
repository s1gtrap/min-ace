use std::collections::{HashMap, HashSet};

use dioxus::prelude::*;
use wasm_bindgen::prelude::*;

#[derive(Clone, Debug, Eq, Hash, PartialEq, serde::Serialize)]
pub struct Annotation {
    pub row: usize,
    pub column: usize,
    pub text: String,
    #[serde(rename = "type")]
    pub ty: String,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Marker {
    pub start: (usize, usize),
    pub stop: (usize, usize),
    pub class: String,
    pub ty: String,
    pub inFront: bool,
}

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
    fn setValue(this: &EditorSession, val: String);

    #[wasm_bindgen(method)]
    fn getValue(this: &EditorSession) -> String;

    #[wasm_bindgen(method)]
    fn setAnnotations(this: &EditorSession, annots: JsValue);

    #[wasm_bindgen(method)]
    fn addMarker(
        this: &EditorSession,
        range: JsValue,
        class: String,
        ty: String,
        inFront: bool,
    ) -> isize;

    #[wasm_bindgen(method)]
    fn removeMarker(this: &EditorSession, id: isize);

    #[wasm_bindgen(method)]
    fn on(this: &EditorSession, ev: &str, func: &Closure<dyn FnMut()>);
}

#[component]
pub fn Editor(
    annotations: HashSet<Annotation>,
    markers: HashSet<Marker>,
    onchange: EventHandler<String>,
) -> Element {
    use gloo_utils::format::JsValueSerdeExt;
    let mut editor = use_signal(|| None);
    use_effect({
        move || {
            if editor.read().is_none() {
                let instance: EditorInstance = edit("editor");
                instance.getSession().setValue(
                    r#"define i32 @main() {
    %0 = add i32 1, 2
    ret i32 %0
}"#
                    .into(),
                );
                let closure = Closure::new({
                    let onchange = onchange.clone();
                    let session = instance.getSession();
                    move || {
                        use mullvm_parser::PestParser;
                        let str = session.getValue();
                        onchange.call(str);
                    }
                });
                instance.getSession().on("change", &closure);
                closure.forget();
                *editor.write() = Some(instance);
            }
        }
    });
    use_effect(move || {
        if let Some(editor) = &*editor.read() {
            let session = editor.getSession();
            session.setAnnotations(
                JsValue::from_serde(&annotations.iter().collect::<Vec<_>>()).unwrap(),
            );
        }
    });
    let mut marker_ids = use_signal(|| HashMap::<Marker, isize>::new());
    use_effect(move || {
        if let Some(editor) = &*editor.read() {
            let session = editor.getSession();

            let add_markers = markers
                .iter()
                .filter(|marker| !marker_ids.read().contains_key(*marker));

            for marker in add_markers {
                log::info!(
                    "addMarker(new Range({}, {}, {}, {}), {:?}, {:?})",
                    marker.start.0,
                    marker.start.1,
                    marker.stop.0,
                    marker.stop.1,
                    marker.ty,
                    marker.inFront
                );
                let range =
                    js_sys::Reflect::get(&require("ace/range"), &JsValue::from_str("Range"))
                        .unwrap();
                let args = js_sys::Array::new();
                args.push(&marker.start.0.into());
                args.push(&marker.start.1.into());
                args.push(&marker.stop.0.into());
                args.push(&marker.stop.1.into());
                let range = js_sys::Reflect::construct(range.dyn_ref().unwrap(), &args).unwrap();
                let id = session.addMarker(
                    range,
                    marker.class.clone(),
                    marker.ty.clone(),
                    marker.inFront,
                );
                // FIXME: make this work
                // marker_ids.with_mut(|markers| {
                //     markers.insert(marker.clone(), id);
                // });
            }

            // FIXME: make this work
            // let remove_markers = marker_ids
            //     .read()
            //     .iter()
            //     .filter(|(marker, _)| !markers.contains(marker));
            //
            // for (marker, id) in remove_markers {
            //     log::info!("removeMarker({})", marker_ids.read().get(marker).unwrap());
            //     // FIXME: make this work
            //     // marker_ids.with_mut(|markers| {
            //     //     session.removeMarker(*id);
            //     //     markers.remove(marker);
            //     // });
            // }
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
