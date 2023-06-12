#![allow(non_snake_case)]

use dioxus::prelude::*;
use

fn main() {
    dioxus_web::launch(App);
}

#[derive(PartialEq, Props)]
struct DockerProps {
    container:u32,
    image: u32,
    os: u32
}

fn Dockerizer(cx: Scope<DockerProps>) -> Element {
        cx.render(rsx! {
            div {
                class: "bg-slate-500",
                h1 {
                    class: "p-5 text-2xl",
                    "Container -> {cx.props.container}",
                },
                h1 {
                    class: "p-5 text-2xl",
                    "Image -> {cx.props.image}",
                },
                h1 {
                    class: "p-5 text-2xl",
                    "Os -> {cx.props.os}",
                }
            }
        }) 
}


fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        Dockerizer {
            container: 100,
            image: 100,
            os: 100
        }
    })
}

