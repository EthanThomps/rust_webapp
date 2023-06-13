#![allow(non_snake_case)]

mod test;
mod test2;

use dioxus::prelude::*;

use crate::{test2::*};

fn main() {
    dioxus_web::launch(App);
}


fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        SSREG {}
    })
}

