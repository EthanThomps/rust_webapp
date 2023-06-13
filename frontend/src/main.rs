#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_router::{Link, Router, Route};

fn main() {
    dioxus_web::launch(App);
}

fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        Header {}
    })
}

fn Header(cx: Scope) -> Element {
    cx.render(rsx! {
        Router {
            Route {to: "/create-todos",   TodoCard {}}
        },
        div {
            class: "bg-slate-950 p-2 flex",
            div {
                class: "p-5 justify-start text-base",
                "Welcome to app"
            },
            ul {
                class: "justify-center",
                Link {to: "/create-todos", "Create Todos"}
            }
        }
    })
}

fn InputBox(cx:Scope) -> Element {
    let item = use_state(&cx, || "".to_string());
    cx.render(rsx! {
        input {
            class: "inputBox my-4 justify-center",
            value: "{item}",
            oninput: move |evt| item.set(evt.value.clone())
        }
    })
}

fn TodoCard(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "card",
            div {
                class: "card bg-slate-300",
                h1 {
                    class: "p-5 text-2xl justify-center",
                    "Create Todos"
                },
                InputBox {},
                InputBox {}
            }
        }
    })
}
