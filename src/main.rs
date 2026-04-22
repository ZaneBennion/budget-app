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
                class: "top-container",
                div {
                    class: "settings-bar",
                    Link {
                        to: Route::Settings {},
                        Icon { data: material_symbols::Settings, size: "24" }
                    }
                }
                div {
                    class: "money-displays",
                    div {
                        class: "budget-display",
                        "$38"
                    }
                    div {
                        class: "input-display",
                        "-14"
                    }
                }
            }
            div {
                class: "keypad",
                button { class: "keypad-button", "7" }
                button { class: "keypad-button", "8" }
                button { class: "keypad-button", "9" }
                button { class: "keypad-button", "4" }
                button { class: "keypad-button", "5" }
                button { class: "keypad-button", "6" }
                button { class: "keypad-button", "1" }
                button { class: "keypad-button", "2" }
                button { class: "keypad-button", "3" }
                button { class: "keypad-button", "C" }
                button { class: "keypad-button", "0" }
                button { class: "keypad-button", style: "background-color: #A3B18A", "=" }                
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