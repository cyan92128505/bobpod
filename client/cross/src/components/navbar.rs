#![allow(non_snake_case)]
// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::prelude::*;
use dioxus_router::*;

pub fn Navbar(cx: Scope) -> Element {
    cx.render(rsx! {
        ul {
            Link {
                to: "/",
                "Home"
            }
            br {}
            Link {
                to: "/blog",
                "Blog"
            }
        }
    })
}
