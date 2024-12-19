use crate::common::SCROLL_TO_TOP_STYLE;
use crate::Behavior;
use leptos::{
    ev::{scroll, MouseEvent},
    prelude::*,
    *,
};
use std::time::Duration;
use wasm_bindgen::JsValue;
use web_sys::{ScrollBehavior, ScrollToOptions};

/// Scroll Component
///
/// A Leptos component for creating customizable scroll buttons with advanced scrolling functionality.
/// The `Scroll` component supports smooth or instant scrolling, visibility based on scroll position,
/// custom icons, and callbacks for scrolling events. It is highly configurable and fits a wide range
/// of use cases.
///
/// # Properties
/// The `Scroll` component accepts a set of properties to customize its appearance and behavior:
///
/// - **style**: Inline CSS styles for the scroll button (`&'static str`). Default: `SCROLL_TO_TOP_STYLE`.
/// - **class**: Additional CSS classes for the scroll button (`&'static str`). Default: `""`.
/// - **icon**: Custom icon for the scroll button (`&'static str`). Default: `"↑"`.
/// - **behavior**: Scrolling behavior (`Behavior`). Options: `Behavior::Smooth` or `Behavior::Instant`. Default: `Smooth`.
/// - **top**: Vertical scroll target position in pixels (`f64`). Default: `0.0`.
/// - **left**: Horizontal scroll target position in pixels (`f64`). Default: `0.0`.
/// - **offset**: Additional offset for the scroll target in pixels (`f64`). Default: `0.0`.
/// - **delay**: Delay before initiating scrolling, in milliseconds (`u64`). Default: `0`.
/// - **auto_hide**: Toggles automatic visibility based on scroll position (`bool`). Default: `true`.
/// - **threshold**: Scroll position threshold for button visibility, in pixels (`f64`). Default: `20.0`.
/// - **on_begin**: Callback triggered when scrolling begins (`Callback<()>`). Default: No-op.
/// - **on_end**: Callback triggered when scrolling ends (`Callback<()>`). Default: No-op.
/// - **update_hash**: Whether to update the URL hash during scrolling (`bool`). Default: `true`.
/// - **show_id**: ID of a container controlling the button's visibility (`&'static str`). Default: `""`.
/// - **scroll_id**: ID of the target container to scroll to (`&'static str`). Default: `""`.
///
/// # Features
/// - Automatically hides or shows based on scroll position.
/// - Smooth or instant scrolling to specific positions.
/// - Customizable content, including icons or HTML.
/// - Configurable callbacks for when scrolling begins and ends.
/// - Adjustable offsets and delays for precise control.
///
/// # Behavior
/// - The component listens to the window's scroll events to manage its visibility based on the `threshold`
///   or `show_id` container.
/// - Clicking the button triggers the scroll action, applying offsets, delays, and the `behavior`.
/// - Optional callbacks (`on_begin`, `on_end`) allow integration with custom logic during scroll events.
///
/// # Notes
/// - Ensure `scroll_id` and `show_id` refer to valid element IDs in your DOM.
/// - Customize the `style` and `icon` properties to fit your application's design.
/// - The `update_hash` property is useful for SEO and navigation but can be disabled if not needed.
#[component]
pub fn Scroll(
    /// Custom inline styles for the scroll button.
    ///
    /// Accepts a `&'static str` to define CSS properties for the button.
    /// Defaults to a built-in `SCROLL_TO_TOP_STYLE`.
    #[prop(default = SCROLL_TO_TOP_STYLE)]
    style: &'static str,

    /// Custom CSS classes for the scroll button.
    ///
    /// Accepts a `&'static str` for adding additional styles via class selectors.
    /// Defaults to an empty string.
    #[prop(default = "")]
    class: &'static str,

    /// Custom icon for the scroll button.
    ///
    /// This can be a string, such as `"↑"`, or any HTML/SVG representation of the icon.
    /// Defaults to `"↑"`.
    /// TODO: Change type to View<>
    #[prop(default = "↑")]
    icon: &'static str,

    /// Behavior of the scroll action.
    ///
    /// Determines how scrolling occurs. Can be `Behavior::Smooth` or `Behavior::Instant`.
    /// Defaults to `Behavior::Smooth`.
    #[prop(default = Behavior::Smooth)]
    behavior: Behavior,

    /// Vertical scroll target position in pixels.
    ///
    /// Specifies the `top` position to scroll to. Defaults to `0.0`.
    #[prop(default = 0.0)]
    top: f64,

    /// Horizontal scroll target position in pixels.
    ///
    /// Specifies the `left` position to scroll to (useful for horizontal scrolling).
    /// Defaults to `0.0`.
    #[prop(default = 0.0)]
    left: f64,

    /// Additional offset in pixels for the scroll target.
    ///
    /// Useful for adjusting the scroll position to account for fixed headers or other elements.
    /// Defaults to `0.0`.
    #[prop(default = 0.0)]
    offset: f64,

    /// Delay before initiating the scroll action, in milliseconds.
    ///
    /// Introduces a delay before scrolling begins. Defaults to `0`.
    #[prop(default = 0)]
    delay: u64,

    /// Enable or disable automatic visibility based on scroll position.
    ///
    /// When `true`, the button will appear or hide automatically depending on the user's scroll position.
    /// Defaults to `true`.
    #[prop(default = true)]
    auto_hide: bool,

    /// Scroll threshold in pixels for button visibility.
    ///
    /// Defines how far down the page a user needs to scroll before the button appears.
    /// Defaults to `20.0`.
    #[prop(default = 20.0)]
    threshold: f64,

    /// Callback triggered when scrolling begins.
    ///
    /// Use this callback to handle actions like logging, animations, or UI updates when the scroll starts.
    #[prop(default = Callback::from(move || {}))]
    on_begin: Callback<()>,

    /// Callback triggered when scrolling ends.
    ///
    /// Use this callback to handle actions like resetting states or displaying notifications when the scroll completes.
    #[prop(default = Callback::from(move || {}))]
    on_end: Callback<()>,

    /// Update the URL hash during scrolling.
    ///
    /// When `true`, the browser's URL hash will reflect the scroll target. Defaults to `true`.
    #[prop(default = true)]
    update_hash: bool,

    /// Target container ID for displaying the scroll button.
    ///
    /// Specifies the ID of a container where the button's visibility is based on the container's scroll position.
    /// Defaults to an empty string.
    #[prop(default = "")]
    show_id: &'static str,

    /// Target container ID for scrolling actions.
    ///
    /// Specifies the ID of the container to which scrolling actions should apply.
    /// Defaults to an empty string.
    #[prop(default = "")]
    scroll_id: &'static str,
) -> impl IntoView {
    let (visible, set_visible) = signal(false);
    let (behavior, _set_behavior) = signal(behavior);

    let scroll_handler = move || {
        if let Some(container) = window().document().unwrap().get_element_by_id(show_id) {
            let scroll_position = window().scroll_y().unwrap_or(0.0);
            let container_position = container.get_bounding_client_rect().top();
            set_visible.set(scroll_position > container_position);
        } else {
            let scroll_position = window().scroll_y().unwrap_or(0.0);
            set_visible.set(scroll_position > threshold);
        }
    };

    if auto_hide {
        window_event_listener(scroll, move |_ev| scroll_handler());
    }

    let on_click = {
        move |_: MouseEvent| {
            if delay > 0 {
                let delay = Duration::from_millis(delay);
                set_timeout(
                    move || {
                        on_begin.run(());
                        perform_scroll(
                            top,
                            left,
                            offset,
                            behavior.get(),
                            update_hash,
                            scroll_id.to_string(),
                        );
                        on_end.run(());
                    },
                    delay,
                );
            } else {
                on_begin.run(());
                perform_scroll(
                    top,
                    left,
                    offset,
                    behavior.get(),
                    update_hash,
                    scroll_id.to_string(),
                );
                on_end.run(());
            }
        }
    };

    view! {
        {move ||
            if visible.get() {
                Some(view! {
                    <div
                        class={class}
                        style={style}
                        on:click={on_click}
                    >
                        {icon}
                    </div>
                })
            } else {
                None
            }
        }
    }
}

fn perform_scroll(
    top: f64,
    left: f64,
    offset: f64,
    behavior: Behavior,
    update_hash: bool,
    scroll_id: String,
) {
    let options = ScrollToOptions::new();
    options.set_left(left);
    match behavior {
        Behavior::Auto => options.set_behavior(ScrollBehavior::Auto),
        Behavior::Instant => options.set_behavior(ScrollBehavior::Instant),
        Behavior::Smooth => options.set_behavior(ScrollBehavior::Smooth),
    }

    if let Some(container) = window().document().unwrap().get_element_by_id(&scroll_id) {
        let container_position = container.get_bounding_client_rect().top();
        options.set_top(container_position + offset);
    } else {
        options.set_top(top + offset);
    }

    window().scroll_with_scroll_to_options(&options);

    if update_hash {
        let hash = format!("#{}", scroll_id);
        window()
            .history()
            .unwrap()
            .push_state_with_url(&JsValue::NULL, "", Some(&hash))
            .unwrap();
    }
}
