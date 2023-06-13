#![allow(non_snake_case)]

use dioxus::prelude::*;


pub fn SSREG(cx: Scope) -> Element {
    cx.render(rsx! {
        h1 {"SSR example"}
    })
}

pub fn Task(cx: Scope) -> Element {
    cx.render(rsx! {
        h1 {"Task: 0"}
    })
}