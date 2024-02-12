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
    fn edit(s: &str) -> EditorInstance;

    #[wasm_bindgen(js_namespace = ace)]
    fn require(s: &str) -> JsValue;

    #[wasm_bindgen(method)]
    fn setValue(this: &EditorSession, val: String);

    #[wasm_bindgen(method)]
    fn getValue(this: &EditorSession) -> String;

    #[wasm_bindgen(method)]
    fn setAnnotations(this: &EditorSession, annots: JsValue);

    #[wasm_bindgen(method)]
    fn getSession(this: &EditorInstance) -> EditorSession;

    #[wasm_bindgen(method)]
    fn on(this: &EditorSession, ev: &str, func: &Closure<dyn FnMut()>);
}

#[derive(Props)]
pub struct FancyButtonProps<'a> {
    annotations: HashSet<Annotation>,
    markers: HashSet<Marker>,
    onchange: EventHandler<'a, MouseEvent>,
}

#[component]
pub fn Editor<'a>(cx: Scope<'a, FancyButtonProps>) -> Element<'a> {
    use gloo_utils::format::JsValueSerdeExt;
    let editor = use_state(cx, || None);
    use_effect(cx, (editor,), |(editor,)| async move {
        if editor.is_none() {
            let instance: EditorInstance = edit("editor");
            instance.getSession().setValue(
                r#"define i32 @main() {
    %0 = add i32 1, 2
    ret i32 %0
}"#
                .into(),
            );
            let closure = Closure::new({
                let session = instance.getSession();
                move || {
                    use mullvm_parser::PestParser;
                    let str = session.getValue();
                    log::info!("{:?}", str);
                }
            });
            instance.getSession().on("change", &closure);
            closure.forget();
            editor.set(Some(instance));
        }
    });
    use_effect(
        cx,
        (editor, &cx.props.annotations),
        |(editor, annotations)| async move {
            if let Some(editor) = editor.get() {
                let session = editor.getSession();
                session.setAnnotations(
                    JsValue::from_serde(&annotations.iter().collect::<Vec<_>>()).unwrap(),
                );
            }
        },
    );
    let marker_ids = use_state(cx, || HashMap::new());
    use_effect(
        cx,
        (editor, marker_ids, &cx.props.markers),
        |(editor, marker_ids, markers)| async move {
            let add_markers = markers
                .iter()
                .filter(|marker| !marker_ids.contains_key(*marker));

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
                marker_ids.with_mut(|markers| {
                    markers.insert(marker.clone(), 0);
                });
            }

            let remove_markers = marker_ids
                .iter()
                .filter(|(marker, _)| !markers.contains(marker));

            for (marker, _) in remove_markers {
                log::info!("removeMarker({})", marker_ids.get().get(marker).unwrap());
                marker_ids.with_mut(|markers| {
                    markers.remove(marker);
                });
            }
        },
    );

    cx.render(rsx! {
        div {
            id: "editor",
            class: "h-full",
            ""
        }
    })
}
