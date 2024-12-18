use gloo::events::EventListener;
use gloo::utils::window;
use wasm_bindgen::JsValue;
use web_sys::{Element, ScrollBehavior, ScrollToOptions};
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub enum Behavior {
    Auto,
    Instant,
    Smooth,
}

/// Default CSS style for the scroll-to-top button.
const SCROLL_TO_TOP_STYLE: &'static str =
    "position: fixed; bottom: 1rem; right: 1rem; background-color: #3b82f6; color: #ffffff; padding: 0.75rem; border-radius: 50%; cursor: pointer; transition: background-color 300ms ease-in-out;";

/// Properties for configuring the `Scroll` component.
///
/// This component provides a scroll-to-top button with customizable styles, behavior,
/// and functionality. It supports scrolling to specific positions, managing visibility
/// based on scroll position, and triggering callbacks for scroll events.
#[derive(Properties, Clone, PartialEq)]
pub struct ScrollProps {
    /// Custom inline styles for the scroll-to-top button.
    ///
    /// Accepts a `&'static str` to define CSS properties for the button.
    /// Defaults to the built-in `SCROLL_TO_TOP_STYLE`.
    #[prop_or(SCROLL_TO_TOP_STYLE)]
    pub style: &'static str,

    /// Custom CSS classes for the scroll-to-top button.
    ///
    /// Accepts a `&'static str` for additional styling using class selectors.
    /// Defaults to an empty string.
    #[prop_or_default]
    pub class: &'static str,

    /// Custom content for the scroll button.
    ///
    /// This can be an SVG, HTML, or any valid Yew `Html` type to define the
    /// button's visual representation. Defaults to an internal SVG icon.
    #[prop_or_else(default_svg)]
    pub content: Html,

    /// Behavior of the scroll action.
    ///
    /// Defines how the scroll will occur (e.g., `smooth` or `instant`).
    /// Defaults to `Behavior::Smooth`.
    #[prop_or(Behavior::Smooth)]
    pub behavior: Behavior,

    /// Vertical scroll target position in pixels.
    ///
    /// Specifies the top position to scroll to. Defaults to `0.0`.
    #[prop_or(0.0)]
    pub top: f64,

    /// Horizontal scroll target position in pixels.
    ///
    /// Specifies the left position to scroll to (for horizontal scrolling).
    /// Defaults to `0.0`.
    #[prop_or(0.0)]
    pub left: f64,

    /// Additional offset in pixels for the scroll target.
    ///
    /// Useful for adjusting the target position to account for fixed headers
    /// or other elements. Defaults to `0.0`.
    #[prop_or(0.0)]
    pub offset: f64,

    /// Delay before initiating the scroll action, in milliseconds.
    ///
    /// This allows a pause before the scrolling begins. Defaults to `0`.
    #[prop_or(0)]
    pub delay: u32,

    /// Enable or disable automatic visibility based on scroll position.
    ///
    /// When `true`, the scroll button will automatically appear or hide
    /// based on the user's current scroll position. Defaults to `true`.
    #[prop_or(true)]
    pub auto_hide: bool,

    /// Scroll threshold in pixels for button visibility.
    ///
    /// Defines the vertical scroll position after which the scroll-to-top
    /// button becomes visible. Defaults to `20.0`.
    #[prop_or(20.0)]
    pub threshold: f64,

    /// Callback triggered when scrolling begins.
    ///
    /// Use this to handle actions like logging, animations, or UI updates
    /// when the scrolling starts. Defaults to no-op.
    #[prop_or_default]
    pub on_begin: Callback<()>,

    /// Callback triggered when scrolling ends.
    ///
    /// Use this to handle actions like resetting states, analytics, or
    /// displaying notifications when the scrolling completes. Defaults to no-op.
    #[prop_or_default]
    pub on_end: Callback<()>,

    /// Update the URL hash during scrolling.
    ///
    /// When `true`, the browser's URL hash will be updated to reflect the
    /// scroll target. Defaults to `true`.
    #[prop_or(true)]
    pub update_hash: bool,

    /// Target container ID for displaying the scroll button.
    ///
    /// When specified, the button will only be displayed if the scroll
    /// position of the container with the given ID equals to the current scroll y position.
    /// Defaults to an empty string.
    #[prop_or_default]
    pub show_id: &'static str,

    /// Target container ID for scrolling actions.
    ///
    /// When specified, the scrolling will be applied to the container with
    /// the given ID, instead of the default scrolling context (e.g. Scrolling to the top). Defaults to an empty string.
    #[prop_or_default]
    pub scroll_id: &'static str,
}

/// Scroll Component
///
/// A Yew component for creating customizable scroll buttons with advanced scrolling functionality.
/// The `Scroll` component supports features like smooth or instant scrolling, visibility based on scroll
/// position, custom content, and callbacks for scroll events. It is highly configurable and can be styled
/// to fit different use cases.
///
/// # Properties
/// The component uses the `ScrollProps` struct for its properties. Key properties include:
///
/// - **style**: Inline styles for the scroll button (`&'static str`). Default: `SCROLL_TO_TOP_STYLE`.
/// - **class**: CSS classes for styling the button (`&'static str`). Default: `""`.
/// - **content**: Custom content for the scroll button (`Html`). Default: An SVG icon.
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
/// use scroll_rs::yew::{Scroll, ScrollProps};
/// use yew::prelude::*;
///
/// #[function_component(MyComponent)]
/// pub fn my_component() -> Html {
///     html! {
///         <>
///             <div id="top" class="h-96 bg-gray-700 text-center">
///                 <h2>{"Top of the Page"}</h2>
///             </div>
///             <div class="h-[2000px]" />
///             <Scroll />
///         </>
///     }
/// }
/// ```
///
/// ## Custom Content and Style
/// ```rust
/// use scroll_rs::yew::{Scroll, Behavior};
/// use yew::prelude::*;
///
/// #[function_component(CustomScrollButton)]
/// pub fn custom_scroll_button() -> Html {
///     html! {
///         <Scroll
///             style="position: fixed; bottom: 2rem; right: 2rem; background-color: #10B981; border-radius: 50%; padding: 1rem; box-shadow: 0 4px 12px rgba(0,0,0,0.4);"
///             content={html! {
///                 <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor" width="24" height="24">
///                     <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 10l7-7 7 7M5 19l7-7 7 7" />
///                 </svg>
///             }}
///             behavior={Behavior::Smooth}
///         />
///     }
/// }
/// ```
///
/// ## Scroll to Specific Target
/// ```rust
/// use scroll_rs::yew::{Scroll, ScrollProps};
/// use yew::prelude::*;
///
/// #[function_component(ScrollToSection)]
/// pub fn scroll_to_section() -> Html {
///     html! {
///         <>
///             <div id="section1" class="h-96 bg-gray-700 text-center">
///                 <h2>{"Section 1"}</h2>
///             </div>
///             <div class="h-96 bg-gray-800 text-center">
///                 <h2>{"Section 2"}</h2>
///             </div>
///             <Scroll
///                 scroll_id="section1"
///                 style="position: fixed; bottom: 4rem; right: 3rem; background-color: #6B7280; color: #FFFFFF; padding: 1rem; border-radius: 50%;"
///                 content={html! {
///                     <span>{"Scroll to Section 1"}</span>
///                 }}
///             />
///         </>
///     }
/// }
/// ```
///
/// # Behavior
/// - The component uses an internal `use_state` hook to manage the visibility of the scroll button.
/// - It attaches a scroll event listener to monitor the current scroll position and determines visibility based on
///   the `threshold` or `show_id` container position.
/// - Clicking the button triggers the scroll action, which can optionally include a delay and emit the `on_begin`
///   and `on_end` callbacks.
///
/// # Notes
/// - Ensure that `scroll_id` and `show_id` refer to valid element IDs in your DOM.
#[function_component(Scroll)]
pub fn scroll(props: &ScrollProps) -> Html {
    let visible_handle = use_state(|| false);
    let is_visible = *visible_handle;

    let behavior = props.behavior.clone();
    let top = props.top;
    let left = props.left;
    let offset = props.offset;
    let delay = props.delay;
    let on_begin = props.on_begin.clone();
    let on_end = props.on_end.clone();
    let update_hash = props.update_hash;
    let threshold = props.threshold;
    let show_id = props.show_id;
    let scroll_id = props.scroll_id;
    let auto_hide = props.auto_hide;

    let container_element: Option<Element> =
        window().document().unwrap().get_element_by_id(show_id);

    use_effect_with((), move |_| {
        let listener = if auto_hide {
            Some(EventListener::new(&window(), "scroll", move |_| {
                if let Some(container) = &container_element {
                    let container_position = container.get_bounding_client_rect().top();
                    let scroll_position = window().scroll_y().unwrap_or(0.0);
                    visible_handle.set(scroll_position > container_position);
                } else {
                    let scroll_position = window().scroll_y().unwrap_or(0.0);
                    visible_handle.set(scroll_position > threshold);
                };
            }))
        } else {
            None
        };
        move || {
            drop(listener);
        }
    });

    let on_click = {
        Callback::from(move |_| {
            if delay > 0 {
                let on_begin = on_begin.clone();
                let on_end = on_end.clone();
                let behavior = behavior.clone();
                gloo::timers::callback::Timeout::new(delay, move || {
                    on_begin.emit(());
                    scroll_to(
                        top,
                        left,
                        offset,
                        behavior.clone(),
                        update_hash,
                        Some(scroll_id.to_string()),
                    );
                    on_end.emit(());
                })
                .forget();
            } else {
                on_begin.emit(());
                scroll_to(
                    top,
                    left,
                    offset,
                    behavior.clone(),
                    update_hash,
                    Some(scroll_id.to_string()),
                );
                on_end.emit(());
            }
        })
    };

    html! {
        if is_visible {
            <div class={props.class} style={props.style} onclick={on_click}>
                { props.content.clone() }
            </div>
        }
    }
}

/// A helper function to scroll to a specific position.
fn scroll_to(
    top: f64,
    left: f64,
    offset: f64,
    behavior: Behavior,
    update_hash: bool,
    scroll_id: Option<String>,
) {
    let options = ScrollToOptions::new();
    let container_element: Option<Element> = window()
        .document()
        .unwrap()
        .get_element_by_id(&scroll_id.clone().unwrap_or_default());
    options.set_left(left);
    match behavior {
        Behavior::Auto => {
            options.set_behavior(ScrollBehavior::Auto);
        }
        Behavior::Instant => {
            options.set_behavior(ScrollBehavior::Instant);
        }
        Behavior::Smooth => {
            options.set_behavior(ScrollBehavior::Smooth);
        }
    }
    if let Some(container) = &container_element {
        let container_position = container.get_bounding_client_rect().top();
        options.set_top(container_position);
    } else {
        options.set_top(top + offset);
    };
    window().scroll_with_scroll_to_options(&options);

    if update_hash {
        let hash = format!("#{}", scroll_id.unwrap_or_default());
        window()
            .history()
            .unwrap()
            .push_state_with_url(&JsValue::NULL, "", Some(&hash))
            .unwrap();
    }
}

/// Default SVG content for the scroll button.
fn default_svg() -> Html {
    html! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            fill="none"
            viewBox="0 0 24 24"
            stroke="currentColor"
            style="width: 16px; height: 16px;"
        >
            <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M5 10l7-7m0 0l7 7m-7-7v18"
            />
        </svg>
    }
}
