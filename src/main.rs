#![allow(non_snake_case)]

use dioxus::prelude::*;

fn main() {
    // launch the web app
    // dioxus_web::launch(App);

    // We can render VirtualDoms
    let mut vdom = VirtualDom::new(App);
    let _ = vdom.rebuild();
    println!("{}", dioxus_ssr::render(&vdom));

    // Or we can render rsx! calls themselves
    println!(
        "{}",
        dioxus_ssr::render_lazy(rsx! {
            div {
                h1 { "Hello, world!" }
            }
        })
    );

    // We can configure the SSR rendering to add ids for rehydration
    println!("{}", dioxus_ssr::pre_render(&vdom));

    // We can render to a buf directly too
    let mut file = String::new();
    let mut renderer = dioxus_ssr::Renderer::default();
    renderer.render_to(&mut file, &vdom).unwrap();
    println!("{file}");
}

// create a component that renders a div with the text "Hello, world!"
fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class:"bluepf",
            "Dockerized!"
        }
    })
}
