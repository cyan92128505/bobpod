#![allow(non_snake_case)]
use dioxus::prelude::*;

pub fn HomePage(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            "Home"
        }
    })
}
