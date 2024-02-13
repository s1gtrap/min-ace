#![allow(non_snake_case)]

use std::collections::HashSet;

use dioxus::prelude::*;
use dioxus_router::prelude::*;
use log::LevelFilter;

mod editor;
mod error;

fn main() {
    // Init debug
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    console_error_panic_hook::set_once();

    log::info!("starting app");
    launch(app);
}

fn app() -> Element {
    let mut annotations = use_signal(|| {
        HashSet::from([editor::Annotation {
            row: 1,
            column: 1,
            text: "hello world".into(),
            ty: "error".into(),
        }])
    });
    let mut markers = use_signal(HashSet::new);
    //let mut error = use_signal(|| Option::None);

    rsx! {
        div {
            class: "container",
            div {
                class: "grid",
                button {
                    onclick: move |_| {
                        let new_annotations = {
                            annotations.read().iter().cloned().chain([
                                editor::Annotation {
                                    row: annotations.read().len(),
                                    column: 1,
                                    text: format!("this is line {}", annotations.read().len()),
                                    ty: "error".into(),
                                }
                            ]).collect()
                        };
                        *annotations.write() = new_annotations;
                    },
                    "add annotation"
                }
                button {
                    onclick: move |_| {
                        use rand::Rng;
                        let mut rng = rand::thread_rng();
                        let l1: usize = rng.gen_range(0..2);
                        let l2: usize = rng.gen_range((l1+1)..3);
                        let new_markers = {
                            markers.read().iter().cloned().chain([
                                editor::Marker {
                                    start: (l1, rng.gen_range(0..20)),
                                    stop: (l2+1, rng.gen_range(0..20)),
                                    class: "error".to_owned(),
                                    ty: "text".to_owned(),
                                    inFront: false,
                                }
                            ]).collect()
                        };
                        *markers.write() = new_markers;
                    },
                    "add marker"
                }
                button {
                    onclick: move |_| {
                        use rand::prelude::*;
                        let mut rng = rand::thread_rng();
                        let c = markers.read().iter().cloned().choose(&mut rng).clone();
                        if let Some(marker) = c {
                            markers.with_mut(|markers| {
                                markers.remove(&marker);
                            });
                        }
                    },
                    "remove marker"
                }
                div {
                    class: "h-screen",
                    editor::Editor {
                        annotations: annotations,
                        markers: markers,
                        onchange: move |s: String| {
                            use mullvm_parser::PestParser;
                            log::info!("{s:?}");
                            match mullvm_parser::LLVMParser::parse(mullvm_parser::Rule::module, &s) {
                                Ok(_) => {
                                    log::info!("ok!");
                                    *annotations.write() = HashSet::new();
                                }
                                Err(e) => {
                                    log::error!("{e}");
                                    *annotations.write() = HashSet::from([
                                        match e.line_col {
                                            mullvm_parser::LineColLocation::Pos((row, column)) |
                                            mullvm_parser::LineColLocation::Span((row, column), _) => {
                                                log::info!("{row} {column}");
                                                editor::Annotation {
                                                        row,
                                                        column,
                                                        text: format!("{}", e.variant.message()),
                                                        ty: "error".into(),
                                                    }
                                                }
                                            }
                                    ])
                                }
                            }
                        }
                    }
                }
                Router::<Route> {}
            }
        }
    }
}

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}

#[component]
fn Blog(id: i32) -> Element {
    rsx! {
        Link { to: Route::Home {}, "Go to counter" }
        "Blog post {id}"
    }
}

#[component]
fn Home() -> Element {
    let mut count = use_signal(|| 0);

    rsx! {
        Link {
            to: Route::Blog {
                id: *count.read()
            },
            "Go to blog"
        }
        div {
            h1 { "High-Five counter: {count}" }
            button { onclick: move |_| count += 1, "Up high!" }
            button { onclick: move |_| count -= 1, "Down low!" }
        }
    }
}
