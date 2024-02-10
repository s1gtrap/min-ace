use dioxus::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    type EditorInstance;
    type EditorSession;

    #[wasm_bindgen(js_namespace = ace)]
    fn edit(s: &str) -> EditorInstance;

    #[wasm_bindgen(method)]
    fn getValue(this: &EditorSession) -> String;

    #[wasm_bindgen(method)]
    fn getSession(this: &EditorInstance) -> EditorSession;

    #[wasm_bindgen(method)]
    fn on(this: &EditorSession, ev: &str, func: &Closure<dyn FnMut()>);
}

#[component]
pub fn Editor(cx: Scope) -> Element {
    let editor = use_state(cx, || None);
    use_effect(cx, (editor,), |(editor,)| async move {
        if editor.is_none() {
            let instance: EditorInstance = edit("editor");
            let closure = Closure::new({
                let session = instance.getSession();
                move || {
                    log::info!("{:?}", session.getValue());
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
