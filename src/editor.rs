use dioxus::prelude::*;
use wasm_bindgen::prelude::*;

#[derive(serde::Serialize)]
struct Annotation {
    pub row: usize,
    pub column: usize,
    pub text: String,
    #[serde(rename = "type")]
    pub ty: String,
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
                    match mullvm_parser::LLVMParser::parse(mullvm_parser::Rule::module, &str) {
                        Ok(_) => {
                            log::info!("ok!");
                            session.setAnnotations(
                                JsValue::from_serde(&Vec::<Annotation>::new()).unwrap(),
                            );
                        }
                        Err(e) => {
                            log::error!("{e}");
                            match e.line_col {
                                mullvm_parser::LineColLocation::Pos((l, c))
                                | mullvm_parser::LineColLocation::Span((l, c), _) => {
                                    session.setAnnotations(
                                        JsValue::from_serde(&vec![Annotation {
                                            row: l - 1,
                                            column: c - 1,
                                            text: e.variant.message().into(),
                                            ty: "error".into(),
                                        }])
                                        .unwrap(),
                                    );
                                }
                            }
                        }
                    }
                }
            });
            instance.getSession().on("change", &closure);
            closure.forget();
            editor.set(Some(instance));
        }
    });

    cx.render(rsx! {
        div {
            id: "editor",
            class: "h-full",
            ""
        }
    })
}
