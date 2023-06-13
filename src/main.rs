#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_free_icons::{icons::io_icons::IoAirplane, Icon};
use dioxus_router::{Link, Route, Router};

fn main() {
    dioxus_web::launch(App);
}

#[derive(PartialEq, Props)]
struct DockerProps {
    container: u32,
    image: u32,
    os: u32,
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
        },
        Icon {
            width: 100,
            height: 100,
            fill: "black",
            icon: IoAirplane
        }
    })
}

fn TodoPage(cx:Scope) -> Element {
    cx.render(rsx! {
        Router {
            ul {
                Link {to: "/", li {"Home"}}
                Link {to: "/users", li {"Users"}}
            },
            Route {to: "/", "Home"}
            Route {to: "/users", "List of Users"}
            Route {to: "/users/:name", User {}}
            Route {to: "", "404 Error"}
        }
    })
}

fn User(cx:Scope) -> Element {
    cx.render(rsx! {
        h1 {"Todo!()"}
    })
}

fn App(cx: Scope) -> Element {

    cx.render(rsx! {
        Dockerizer {
            container: 10,
            image: 10,
            os: 10,
        },
        TodoPage {}
    })
}
