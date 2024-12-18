use crate::common::{Behavior, SCROLL_TO_TOP_STYLE};
use dioxus::prelude::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::{window, ScrollBehavior, ScrollToOptions};

/// Properties for configuring the `Scroll` component.
///
/// This component provides a scroll-to-top button with customizable styles, behavior,
/// and functionality. It supports scrolling to specific positions, managing visibility
/// based on scroll position, and triggering callbacks for scroll events.
#[derive(Props, Clone, PartialEq)]
pub struct ScrollProps {
    /// Custom inline styles for the scroll-to-top button.
    ///
    /// Accepts a `&'static str` to define CSS properties for the button.
    /// Defaults to the built-in `SCROLL_TO_TOP_STYLE`.
    #[props(default = SCROLL_TO_TOP_STYLE)]
    pub style: &'static str,

    /// Custom CSS classes for the scroll-to-top button.
    ///
    /// Accepts a `&'static str` for additional styling using class selectors.
    /// Defaults to an empty string.
    #[props(default = "")]
    pub class: &'static str,

    /// Custom icon for the scroll button.
    ///
    /// This can be an SVG, HTML, or any valid Dioxus `Element` type to define the
    /// button's visual representation. Defaults to an internal SVG icon.
    #[props(default = default_svg())]
    pub icon: Element,

    /// Behavior of the scroll action.
    ///
    /// Defines how the scroll will occur (e.g., `smooth` or `instant`).
    /// Defaults to `Behavior::Smooth`.
    #[props(default = Behavior::Smooth)]
    pub behavior: Behavior,

    /// Vertical scroll target position in pixels.
    ///
    /// Specifies the top position to scroll to. Defaults to `0.0`.
    #[props(default = 0.0)]
    pub top: f64,

    /// Horizontal scroll target position in pixels.
    ///
    /// Specifies the left position to scroll to (for horizontal scrolling).
    /// Defaults to `0.0`.
    #[props(default = 0.0)]
    pub left: f64,

    /// Additional offset in pixels for the scroll target.
    ///
    /// Useful for adjusting the target position to account for fixed headers
    /// or other elements. Defaults to `0.0`.
    #[props(default = 0.0)]
    pub offset: f64,

    /// Delay before initiating the scroll action, in milliseconds.
    ///
    /// This allows a pause before the scrolling begins. Defaults to `0`.
    #[props(default = 0)]
    pub delay: u32,

    /// Enable or disable automatic visibility based on scroll position.
    ///
    /// When `true`, the scroll button will automatically appear or hide
    /// based on the user's current scroll position. Defaults to `true`.
    #[props(default = true)]
    pub auto_hide: bool,

    /// Scroll threshold in pixels for button visibility.
    ///
    /// Defines the vertical scroll position after which the scroll-to-top
    /// button becomes visible. Defaults to `20.0`.
    #[props(default = 20.0)]
    pub threshold: f64,

    /// Callback triggered when scrolling begins.
    ///
    /// Use this to handle actions like logging, animations, or UI updates
    /// when the scrolling starts. Defaults to no-op.
    #[props(default = Callback::default())]
    pub on_begin: Callback<(), ()>,

    /// Callback triggered when scrolling ends.
    ///
    /// Use this to handle actions like resetting states, analytics, or
    /// displaying notifications when the scrolling completes. Defaults to no-op.
    #[props(default = Callback::default())]
    pub on_end: Callback<(), ()>,

    /// Update the URL hash during scrolling.
    ///
    /// When `true`, the browser's URL hash will be updated to reflect the
    /// scroll target. Defaults to `true`.
    #[props(default = true)]
    pub update_hash: bool,

    /// Target container ID for displaying the scroll button.
    ///
    /// When specified, the button will only be displayed if the scroll
    /// position of the container with the given ID equals to the current scroll y position.
    /// Defaults to an empty string.
    #[props(default = "")]
    pub show_id: &'static str,

    /// Target container ID for scrolling actions.
    ///
    /// When specified, the scrolling will be applied to the container with
    /// the given ID, instead of the default scrolling context (e.g. Scrolling to the top). Defaults to an empty string.
    #[props(default = "")]
    pub scroll_id: &'static str,
}

/// Scroll Component
///
/// A Dioxus component for creating customizable scroll buttons with advanced scrolling functionality.
/// The `Scroll` component supports features like smooth or instant scrolling, visibility based on scroll
/// position, custom content, and callbacks for scroll events. It is highly configurable and can be styled
/// to fit different use cases.
///
/// # Properties
/// The component uses the `ScrollProps` struct for its properties. Key properties include:
///
/// - **style**: Inline styles for the scroll button (`&'static str`). Default: `SCROLL_TO_TOP_STYLE`.
/// - **class**: CSS classes for styling the button (`&'static str`). Default: `""`.
/// - **icon**: Custom icon for the scroll button (`Element`). Default: An internal SVG icon.
/// - **behavior**: Scrolling behavior, either `Behavior::Smooth` or `Behavior::Instant`. Default: `Smooth`.
/// - **top**: Vertical scroll target position (`f64`). Default: `0.0`.
/// - **left**: Horizontal scroll target position (`f64`). Default: `0.0`.
/// - **offset**: Additional offset for the scroll target (`f64`). Default: `0.0`.
/// - **delay**: Delay before scrolling starts in milliseconds (`u32`). Default: `0`.
/// - **auto_hide**: Whether the button is visible based on scroll position (`bool`). Default: `true`.
/// - **threshold**: Scroll position threshold for visibility (`f64`). Default: `20.0`.
/// - **on_begin**: Callback triggered when scrolling begins (`Callback<()>`). Default: No-op.
/// - **on_end**: Callback triggered when scrolling ends (`Callback<()>`). Default: No-op.
/// - **update_hash**: Whether to update the URL hash during scrolling (`bool`). Default: `true`.
/// - **show_id**: ID of the container that determines the button's visibility (`&'static str`). Default: `""`.
/// - **scroll_id**: ID of the target container for scrolling (`&'static str`). Default: `""`.
///
/// # Features
/// - Automatically hides or shows based on scroll position.
/// - Customizable appearance and content (e.g., SVG or HTML).
/// - Supports smooth and instant scrolling behaviors.
/// - Configurable callbacks for when scrolling begins or ends.
/// - Adjusts scrolling target with offsets and delays.
///
/// # Examples
///
/// ## Basic Usage
/// ```rust
/// use scroll_rs::dioxus::Scroll;
/// use dioxus::prelude::*;
///
/// #[component]
/// pub fn App() -> Element {
///     rsx! {
///         div {
///             class: "h-screen bg-gray-900 text-white",
///             Scroll {}
///         }
///     }
/// }
/// ```
///
/// ## Custom Icon and Style
/// ```rust
/// use scroll_rs::dioxus::Scroll;
/// use scroll_rs::Behavior;
/// use dioxus::prelude::*;
///
/// #[component]
/// pub fn CustomScrollButton() -> Element {
///     rsx! {
///         Scroll {
///             style: "position: fixed; bottom: 2rem; right: 2rem; background-color: #10B981; border-radius: 50%; padding: 1rem; box-shadow: 0 4px 12px rgba(0,0,0,0.4);",
///             icon: rsx! {
///                 svg {
///                     xmlns: "http://www.w3.org/2000/svg",
///                     fill: "none",
///                     view_box: "0 0 24 24",
///                     stroke: "currentColor",
///                     path {
///                         stroke_linecap: "round",
///                         stroke_linejoin: "round",
///                         stroke_width: "2",
///                         d: "M5 10l7-7 7 7M5 19l7-7 7 7"
///                     }
///                 }
///             },
///             behavior: Behavior::Smooth,
///         }
///     }
/// }
/// ```
///
/// ## Scroll to Specific Target
/// ```rust
/// use scroll_rs::dioxus::Scroll;
/// use dioxus::prelude::*;
///
/// #[component]
/// pub fn ScrollToBottom() -> Element {
///     rsx! {
///         Scroll {
///             style: "position: fixed; bottom: 4rem; right: 3rem; background-color: #10B981; color: white; padding: 1rem; border-radius: 50%;",
///             scroll_id: "bottom-scroll",
///             icon: rsx! {
///                 svg {
///                     xmlns: "http://www.w3.org/2000/svg",
///                     fill: "none",
///                     view_box: "0 0 24 24",
///                     stroke: "currentColor",
///                     path {
///                         stroke_linecap: "round",
///                         stroke_linejoin: "round",
///                         stroke_width: "2",
///                         d: "M19 9l-7 7-7-7"
///                     }
///                 }
///             },
///         }
///     }
/// }
/// ```
///
/// ## Scroll to Left
/// ```rust
/// use scroll_rs::dioxus::Scroll;
/// use dioxus::prelude::*;
///
/// #[component]
/// pub fn ScrollLeft() -> Element {
///     rsx! {
///         Scroll {
///             style: "position: fixed; top: 40%; left: 2rem; background-color: #E11D48; color: white; padding: 1rem; border-radius: 50%;",
///             left: -500.0,
///             show_id: "left-scroll",
///             icon: rsx! {
///                 svg {
///                     xmlns: "http://www.w3.org/2000/svg",
///                     fill: "none",
///                     view_box: "0 0 24 24",
///                     stroke: "currentColor",
///                     path {
///                         stroke_linecap: "round",
///                         stroke_linejoin: "round",
///                         stroke_width: "2",
///                         d: "M15 19l-7-7 7-7"
///                     }
///                 }
///             },
///         }
///     }
/// }
/// ```
///
/// ## Scroll Visibility Based on Threshold
/// ```rust
/// use scroll_rs::dioxus::Scroll;
/// use scroll_rs::Behavior;
/// use dioxus::prelude::*;
///
/// #[component]
/// pub fn ScrollVisibleAfterThreshold() -> Element {
///     rsx! {
///         Scroll {
///             style: "position: fixed; bottom: 6rem; left: 6rem; background-color: #F43F5E; color: white; padding: 12px; border-radius: 50%; border: 2px solid #BE123C;",
///             threshold: 400.0,
///             behavior: Behavior::Instant,
///         }
///     }
/// }
/// ```
///
/// # Behavior
/// - The component uses an internal `use_signal` hook to manage the visibility of the scroll button.
/// - It attaches a scroll event listener to monitor the current scroll position and determines visibility based on
///   the `threshold` or `show_id` container position.
/// - Clicking the button triggers the scroll action, which can optionally include a delay and emit the `on_begin`
///   and `on_end` callbacks.
///
/// # Notes
/// - Ensure that `scroll_id` and `show_id` refer to valid element IDs in your DOM.
/// - The `on_begin` and `on_end` callbacks allow you to handle actions when scrolling starts and ends, such as logging
///   or triggering animations.
/// - The button will only be visible when the user has scrolled past the defined threshold or when the `show_id` container
///   is in view.
#[component]
pub fn Scroll(props: ScrollProps) -> Element {
    let mut is_visible = use_signal(|| false);

    let container_element: Option<web_sys::Element> = if props.show_id.is_empty() {
        None
    } else {
        window()
            .expect("window not available")
            .document()
            .unwrap()
            .get_element_by_id(props.show_id)
    };

    use_effect(move || {
        let container_element = container_element.clone();
        let threshold = props.threshold;
        let auto_hide = props.auto_hide;

        if auto_hide {
            let closure = Closure::new({
                move || {
                    let window = window().expect("window not available");
                    let scroll_position = window.scroll_y().unwrap_or(0.0);

                    if let Some(container) = container_element.as_ref() {
                        let container_position = container.get_bounding_client_rect().top();
                        is_visible.set(scroll_position > container_position);
                    } else {
                        is_visible.set(scroll_position > threshold);
                    }
                }
            });
            let window = window().expect("window not available");

            window
                .add_event_listener_with_callback("scroll", closure.as_ref().unchecked_ref())
                .expect("Failed to add scroll event listener");

            closure.forget();
        }

        {
            if auto_hide {
                let window = window().expect("window not available");
                let closure = Closure::wrap(Box::new(|| {}) as Box<dyn FnMut()>);
                window
                    .remove_event_listener_with_callback("scroll", closure.as_ref().unchecked_ref())
                    .expect("Failed to remove scroll event listener");
            }
        }
    });

    let on_click = {
        move |_| {
            if props.delay > 0 {
                let behavior = props.behavior.clone();
                let top = props.top;
                let left = props.left;
                let offset = props.offset;
                let update_hash = props.update_hash;
                let scroll_id = props.scroll_id.to_string();
                let on_begin = props.on_begin;
                let on_end = props.on_end;
                gloo::timers::callback::Timeout::new(props.delay, move || {
                    on_begin.call(());
                    scroll_to(
                        top,
                        left,
                        offset,
                        behavior,
                        update_hash,
                        Some(scroll_id.clone()),
                    );
                    on_end.call(());
                })
                .forget();
            } else {
                props.on_begin.call(());
                scroll_to(
                    props.top,
                    props.left,
                    props.offset,
                    props.behavior.clone(),
                    props.update_hash,
                    Some(props.scroll_id.to_string()),
                );
                props.on_end.call(());
            }
        }
    };

    rsx! {
        if is_visible() {
            div {
                class: props.class,
                style: props.style,
                onclick: on_click,
                {props.icon}
            }
        }
    }
}

/// Helper function to scroll to a specific position
fn scroll_to(
    top: f64,
    left: f64,
    offset: f64,
    behavior: Behavior,
    update_hash: bool,
    scroll_id: Option<String>,
) {
    let options = ScrollToOptions::new();
    options.set_left(left);
    let window = window().expect("window not available");

    match behavior {
        Behavior::Auto => options.set_behavior(ScrollBehavior::Auto),
        Behavior::Instant => options.set_behavior(ScrollBehavior::Instant),
        Behavior::Smooth => options.set_behavior(ScrollBehavior::Smooth),
    }

    if let Some(container) = scroll_id
        .clone()
        .and_then(|id| window.document().unwrap().get_element_by_id(&id))
    {
        let container_position = container.get_bounding_client_rect().top();
        options.set_top(container_position);
    } else {
        options.set_top(top + offset);
    }

    window.scroll_with_scroll_to_options(&options);

    if update_hash {
        if let Some(hash) = scroll_id {
            let hash = format!("#{}", hash);
            let _ = window
                .history()
                .unwrap()
                .push_state_with_url(&JsValue::NULL, "", Some(&hash));
        }
    }
}

/// Default SVG content
fn default_svg() -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            fill: "none",
            view_box: "0 0 24 24",
            stroke: "currentColor",
            style: "width: 16px; height: 16px;",
            path {
                stroke_linecap: "round",
                stroke_linejoin: "round",
                stroke_width: "2",
                d: "M5 10l7-7m0 0l7 7m-7-7v18",
            }
        }
    }
}
