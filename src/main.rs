mod components;
mod pages;
mod seo;

use crate::pages::{AboutMain, DioxusTailwindMain};
use crate::seo::{AboutHead, HomeHead, TailwindHead};
use components::{BentoGrid, CallToAction, Footer, HeaderNav, Hero, FAQ};

use dioxus::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/about")]
    About {},
    #[route("/dioxus-tailwind")]
    DioxusTailwind {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const ROCKYPOD: Asset = asset!("/assets/rockypod.svg");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "icon", href: ROCKYPOD }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> { }
    }
}

/// Home page
#[component]
fn Home() -> Element {
    rsx! {
        HomeHead {}
        Hero {}
        BentoGrid {}
        Footer {}
    }
}

/// About page
#[component]
fn About() -> Element {
    rsx! {
        AboutHead {}
        AboutMain {}
        CallToAction {}
        FAQ {}
        Footer {}
    }
}

/// Dioxus Tailwind page
#[component]
pub fn DioxusTailwind() -> Element {
    rsx! {
        TailwindHead {}
        DioxusTailwindMain {}
        Footer {}
    }
}

/// Blog page
#[component]
pub fn Blog(id: i32) -> Element {
    rsx! {
        div { class: "max-w-4xl mx-auto px-6 py-12",
            div { class: "bg-white rounded-xl shadow-lg p-8",
                h1 { class: "text-3xl font-bold text-gray-900 mb-6",
                    "Blog Post #{id}"
                }
                p { class: "text-gray-700 mb-8 text-lg leading-relaxed",
                    "In blog post #{id}, we explore how the Dioxus router works and demonstrate how URL parameters can be seamlessly passed as props to our route components. This showcases the power and flexibility of Dioxus's routing system."
                }

                div { class: "flex items-center justify-between pt-6 border-t border-gray-200",
                    Link {
                        to: Route::Blog { id: id - 1 },
                        class: "flex items-center px-4 py-2 bg-gray-100 hover:bg-gray-200 rounded-lg transition-colors",
                        span { class: "mr-2", "←" }
                        "Previous Post"
                    }
                    Link {
                        to: Route::Blog { id: id + 1 },
                        class: "flex items-center px-4 py-2 bg-bamboo-600 hover:bg-bamboo-500 text-white rounded-lg transition-colors",
                        "Next Post"
                        span { class: "ml-2", "→" }
                    }
                }
            }
        }
        Footer {}
    }
}

/// Shared navbar component
#[component]
fn Navbar() -> Element {
    rsx! {
    HeaderNav{}
        }
}
