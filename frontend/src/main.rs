#![allow(non_snake_case)]

use dioxus::prelude::*;

fn main() {
    println!("Frontend!");
    dioxus_web::launch(App);
}


fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        ul {
            li {
                "Create Todos"
            },
            li {
                "Delete Todos"
            }, 
            li {
                "Views Todos"
            }
        }
    })
}
