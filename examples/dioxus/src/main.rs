use dioxus::prelude::*;
use dioxus_logger::tracing;
use scroll_rs::dioxus::Scroll;
use scroll_rs::Behavior;

fn main() {
    dioxus_logger::init(tracing::Level::INFO).expect("failed to init logger");
    tracing::info!("starting app");
    launch(app);
}

fn app() -> Element {
    rsx! {
        document::Stylesheet { href: "https://unpkg.com/tailwindcss@2.2.19/dist/tailwind.min.css" },
        document::Script { src: "https://kit.fontawesome.com/8f223ead6e.js" }
        Home {}
    }
}

#[component]
pub fn Home() -> Element {
    rsx! {
        div {
            class: "p-8 bg-gray-900 text-white min-h-screen relative",
            h1 {
                class: "text-4xl font-bold mb-8 text-center",
                "Scroll RS Dioxus Demo"
            }
            // Content for Scrolling
            div {
                id: "top",
                class: "h-96 bg-gray-700 mt-16 p-8 text-center",
                h2 {
                    class: "text-3xl font-bold",
                    "Top of the Page"
                }
                p { "Scroll down to interact with the buttons." }
            }
            div { id: "left-scroll", class: "w-[2000px] h-96 bg-gray-800 mt-16" }
            div {
                id: "bottom-scroll",
                class: "h-96 bg-gray-700 mt-16 p-8 text-center",
                h2 {
                    class: "text-3xl font-bold",
                    "Bottom of the Page"
                }
                p { "You have reached the bottom!" }
            }
            div {
                // Default Scroll Button
                div {
                    title: "Default Scroll Button",
                    Scroll {}
                }
                // Scroll to Bottom
                div {
                    title: "Scroll to Bottom",
                    Scroll {
                        style: "position: fixed; bottom: 4rem; right: 3rem; background-color: #10B981; color: #FFFFFF; padding: 1rem; border-radius: 50%; cursor: pointer; box-shadow: 0px 4px 12px rgba(0, 0, 0, 0.4);",
                        icon: rsx! {
                            svg {
                                xmlns: "http://www.w3.org/2000/svg",
                                fill: "none",
                                view_box: "0 0 24 24",
                                stroke: "currentColor",
                                style: "width: 24px; height: 24px;",
                                path {
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    stroke_width: "2",
                                    d: "M19 9l-7 7-7-7",
                                }
                            }
                        },
                        scroll_id: "bottom-scroll",
                    }
                }
                // Scroll Left
                div {
                    title: "Scroll to the Left",
                    Scroll {
                        style: "position: fixed; top: 40%; left: 2rem; background-color: #E11D48; color: #FFFFFF; padding: 1rem; border-radius: 50%; cursor: pointer; box-shadow: 0px 4px 12px rgba(0, 0, 0, 0.4);",
                        icon: rsx! {
                            svg {
                                xmlns: "http://www.w3.org/2000/svg",
                                fill: "none",
                                view_box: "0 0 24 24",
                                stroke: "currentColor",
                                style: "width: 24px; height: 24px;",
                                path {
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    stroke_width: "2",
                                    d: "M15 19l-7-7 7-7",
                                }
                            }
                        },
                        left: -500.0,
                        show_id: "left-scroll",
                    }
                }
                // Instant Scroll Right
                div {
                    title: "Scroll to the Right",
                    Scroll {
                        style: "position: fixed; top: 40%; right: 2rem; background-color: #F59E0B; color: #FFFFFF; padding: 1rem; border-radius: 50%; cursor: pointer; box-shadow: 0px 4px 12px rgba(0, 0, 0, 0.4);",
                        icon: rsx! {
                            svg {
                                xmlns: "http://www.w3.org/2000/svg",
                                fill: "none",
                                view_box: "0 0 24 24",
                                stroke: "currentColor",
                                style: "width: 24px; height: 24px;",
                                path {
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    stroke_width: "2",
                                    d: "M9 5l7 7-7 7",
                                }
                            }
                        },
                        left: 500.0,
                    }
                }
                // Delayed Scroll to Top
                div {
                    title: "Delayed Scroll to Top",
                    Scroll {
                        style: "position: fixed; bottom: 2rem; left: 6rem; background-color: #6D28D9; color: white; padding: 1rem; border-radius: 50%; cursor: pointer; transition: transform 0.3s ease-in-out;",
                        delay: 2000,
                        show_id: "top",
                    }
                }
                // Instant Scrolling Visibility After Scroll Threshold
                div {
                    title: "Instant Scrolling Visible After Scrolling 400px",
                    Scroll {
                        style: "position: fixed; bottom: 6rem; left: 6rem; background-color: #F43F5E; color: #FFFFFF; padding: 12px; border-radius: 50%; border: 2px solid #BE123C; cursor: pointer; transition: transform 0.3s ease-in-out;",
                        threshold: 400.0,
                        behavior: Behavior::Instant,
                    }
                }
            }
        }
    }
}
