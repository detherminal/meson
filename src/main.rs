#![allow(non_snake_case)]

use dioxus::{html::{style, title}, prelude::*};
use dioxus_desktop::{Config, WindowBuilder};

fn main() {
    dioxus_desktop::launch_cfg(App, Config::default().with_window(WindowBuilder::new().with_title("Photon - Monero Swiss Army Knife")));
}

fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            h1 {
                style {
                    "color": "red",
                }
                "Photon"
            }
        }
    })
}