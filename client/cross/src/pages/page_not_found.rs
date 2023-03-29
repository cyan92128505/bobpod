#![allow(non_snake_case)]
use dioxus::prelude::*;

pub fn NotFoundPage(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            style: r#"
            font-family: system-ui, 'Segoe UI', Roboto, Helvetica, Arial, sans-serif,'Apple Color Emoji', 'Segoe UI Emoji';
            height: 100vh;
            text-align: center;
            display: flex;
            flex-direction: column;
            align-items: center;
            justify-content: center;
            "#,
            div {
                style { include_str!("../assets/404.css") },
                h1 {
                    class: "next-error-h1",
                    style: r#"
                        display: inline-block;
                        margin: 0 20px 0 0;
                        padding-right: 23px;
                        font-size: 24px;
                        font-weight: 500;
                        vertical-align: top;
                        line-height: 49px;
                    "#,
                    "404"
                },
                div {
                    style: r#"display: inline-block; text-align: left"#,
                    h2 {
                        style: r#"
                            font-size: 14px;
                            font-weight: 400;
                            line-height: 49px;
                            margin: 0;                        
                        "#,
                        "This page could not be found ."
                    }
                }
            }
        }
    })
}
