use dioxus::prelude::*;

mod icons;
use icons::{Icon, mdi, material_symbols};

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    //#[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/settings")]
    Settings {},
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Router::<Route> {}
    }
}

/// Home page
#[component]
fn Home() -> Element {
    rsx! {
        div {
            class: "home-page",
            div {
                class: "settings-bar",
                Link {
                    to: Route::Settings {},
                    Icon { data: material_symbols::Settings, size: "24" }
                }
            }
            div {
                class: "budget-display",
                "$38"
            }
            div {
                class: "input-display",
                "-14"
            }
            div {
                class: "keypad",
                button { "7" }
                button { "8" }
                button { "9" }
                button { "4" }
                button { "5" }
                button { "6" }
                button { "1" }
                button { "2" }
                button { "3" }
                button { "C" }
                button { "0" }
                button { "=" }                
            }
        }
    }
}

/// Blog page
#[component]
pub fn Settings() -> Element {
    rsx! {
        div {
            div {
                class: "settings-bar",
                Link {
                    to: Route::Home {},
                    Icon { data: mdi::Home, size: "24" }
                }
            }
        }
    }
}

/// Shared navbar component.
#[component]
fn Navbar() -> Element {
    rsx! {
        div {
            id: "navbar",
            Link {
                to: Route::Home {},
                "Home"
            }
            Link {
                to: Route::Settings {},
                "Settings"
            }
        }

        Outlet::<Route> {}
    }
}