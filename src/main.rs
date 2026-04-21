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
            span {
                class: "settings-bar",
                Link {
                    to: Route::Settings {},
                    Icon { data: mdi::Home, size: "24" }
                }
            }
        }
    }
}

/// Blog page
#[component]
pub fn Settings() -> Element {
    rsx! {
        div {
            span {
                class: "settings-bar",
                Link {
                    to: Route::Home {},
                    Icon { data: material_symbols::Settings, size: "24" }
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