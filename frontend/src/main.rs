#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_router::{Link, Redirect, Route, Router};

fn main() {
    dioxus_web::launch(App);
}

fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        Router {
            Route {to: "/", h1 {"Index"}},
            Route {to: "/create-todos", h1 {"Create Todos"}},
            Route {to: "/delete-todos", h1 {"Delete Todos"}},
            Route {to: "/view-todos", h1 {"View Todos"}},
            Redirect{from: "", to:"/" }
        },
        Link { to:"/", "Index"},
        Link { to:"/create-todos", "Create Todos"},
        Link { to:"/delete-todos", "Delete Todos"},
        Link { to:"/view-todos", "View Todos"}
    })
}
