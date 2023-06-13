#![allow(non_snake_case)]

// https://github.com/DioxusLabs/dioxus/blob/master/examples/router.rs

use dioxus::prelude::*;
use dioxus_free_icons::{icons::io_icons::IoAirplane, Icon};
use dioxus_router::{Link, Route, Router};

#[derive(PartialEq, Props)]
pub struct DockerProps {
    pub container: u32,
    pub image: u32,
    pub os: u32,
}

pub fn Dockerizer(cx: Scope<DockerProps>) -> Element {
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

pub fn TodoPage(cx:Scope) -> Element {
    cx.render(rsx! {
        Router {
            ul {
                Link {to: "/", li {"Home"}}
                Link {to: "/users", li {"Users"}}
                // pages of users 0 to 3 should show todo!()  --6/13/23
                Link {to: "/users/0", li {"user 0"}}
                Link {to: "/users/1", li {"user 1"}}
                Link {to: "/users/2", li {"user 2"}}
                Link {to: "/users/3", li {"user 3"}}
            },
            Route {to: "/", "Home"}
            Route {to: "/users", "List of Users"}
            Route {to: "/users/:name", User {}}
            Route {to: "", "404 Error"}
        }
    })
}

pub fn User(cx:Scope) -> Element {
    cx.render(rsx! {
        h1 {"Todo!()"}
    })
}
