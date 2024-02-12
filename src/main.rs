#![allow(non_snake_case)]

use std::collections::HashSet;

use dioxus::prelude::*;
use dioxus_router::prelude::*;
use log::LevelFilter;

mod editor;

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
    let mut markers = use_signal(|| HashSet::new());
    use_effect(move || log::info!("{annotations:?}"));
    rsx! {
        div {
            class: "container",
            div {
                class: "grid",
                button {
                    onclick: move |_| {
                        *annotations.write() = annotations.read().iter().cloned().chain([
                            editor::Annotation {
                                row: annotations.read().len(),
                                column: 1,
                                text: format!("this is line {}", annotations.read().len()),
                                ty: "error".into(),
                            }
                        ]).collect();
                    },
                    "add annotation"
                }
                button {
                    onclick: move |_| {
                        use rand::Rng;
                        let mut rng = rand::thread_rng();
                        let l1: usize = rng.gen_range(0..2);
                        let l2: usize = rng.gen_range((l1+1)..3);
                        *markers.write()=markers.read().iter().cloned().chain([
                            editor::Marker {
                                start: (l1, rng.gen_range(0..20)),
                                stop: (l2+1, rng.gen_range(0..20)),
                                class: "error".to_owned(),
                                ty: "text".to_owned(),
                                inFront: false,
                            }
                        ]).collect();
                    },
                    "add marker"
                }
                button {
                    onclick: move |_| {
                        use rand::Rng;
                        use rand::prelude::*;
                        let mut rng = rand::thread_rng();
                        let l1: usize = rng.gen_range(0..2);
                        let l2: usize = rng.gen_range((l1+1)..3);
                        if let Some(marker) = markers.read().iter().choose(&mut rng) {
                            markers.with_mut(|markers| {
                                markers.remove(marker);
                            });
                        }
                    },
                    "remove marker"
                }
                div {
                    class: "h-screen",
                    editor::Editor {
                        annotations: annotations.read().clone(),
                        markers: markers.read().clone(),
                        onchange: |s| log::info!("{s:?}"),
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
    render! {
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
