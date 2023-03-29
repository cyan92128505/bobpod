#![allow(non_snake_case)]
// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use crate::components::navbar::Navbar;
use crate::pages::home::HomePage;
use crate::pages::page_not_found::NotFoundPage;
use dioxus::prelude::*;
use dioxus_router::*;

mod components;
mod pages;

fn main() {
    // launch the web app
    dioxus_web::launch(App);
}

// create a component that renders a div with the text "Hello, world!"
fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        style { include_str!("assets/tailwind.min.css") },
        Router {
            Navbar {} // NEW
            Route { to: "/", HomePage {}}
            Route { to: "", NotFoundPage {}}
        }
    })
}
