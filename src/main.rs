#![allow(non_snake_case)]

use dioxus::prelude::*;

fn main() {
    // launch the web app
    dioxus_web::launch(App);
}

fn ToggleLight(cx: Scope) -> Element {
    let mut toggle = true;

    cx.render(rsx! {
        button {
            onclick: move |e| !toggle, 
            "Toggle Variablee"
        }

        if toggle {
            rsx!(div { "True"})
            toggle = false
        } else {
            rsx!(div { "False"})
            toggle = true
        }

    })
}

// create a component that renders a div with the text "Hello, world!"
fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        ToggleLight {},
        ToggleLight {}
    }) 
}
