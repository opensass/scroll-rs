# 🌱 Leptos Scroll-RS Usage

Adding Scroll-RS to your Leptos project is simple:

1. Make sure your project is set up with **Leptos**. Refer to their [Getting Started Guide](https://book.leptos.dev/getting_started/index.html) for setup instructions.

1. Add `scroll-rs` to your dependencies:

   ```sh
   cargo add scroll-rs --features=lep
   ```

1. Import `Scroll` into your component and start enhancing your app's scroll functionality.

## 🛠️ Usage

Below is an example of how to integrate `scroll-rs` into your Leptos app:

```rust
use leptos::*;
use scroll_rs::leptos::Scroll;
use scroll_rs::Behavior;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div class="min-h-screen bg-gray-900 text-white p-8">
            <h1 class="text-4xl font-bold text-center mb-8">"Scroll-RS Demo"</h1>

            // Scrollable content
            <div id="top" class="h-96 bg-gray-700 p-8 text-center">
                <h2 class="text-3xl font-bold">"Top of the Page"</h2>
                <p>"Scroll down to see buttons in action!"</p>
            </div>

            <div id="bottom" class="h-96 bg-gray-800 p-8 text-center">
                <h2 class="text-3xl font-bold">"Bottom of the Page"</h2>
                <p>"You reached the bottom!"</p>
            </div>

            // Scroll components
            <Scroll
                style="position: fixed; bottom: 2rem; right: 2rem; background: #10B981; padding: 1rem; border-radius: 50%;"
                icon="↑"
                scroll_id="top"
            />
            <Scroll
                style="position: fixed; bottom: 2rem; left: 2rem; background: #F59E0B; padding: 1rem; border-radius: 50%;"
                icon="↓"
                scroll_id="bottom"
            />
        </div>
    }
}
```

## 🔧 Props

| Property      | Type           | Description                                                                   | Default         |
| ------------- | -------------- | ----------------------------------------------------------------------------- | --------------- |
| `style`       | `&'static str` | Inline CSS styles for the scroll button.                                      | Default styling |
| `class`       | `&'static str` | Custom CSS classes for styling the button.                                    | None            |
| `icon`        | `&'static str` | Custom icon (HTML/SVG) for the scroll button (TODO: Change Type to `View<>`). | Default SVG     |
| `behavior`    | `Behavior`     | Scrolling behavior: `Smooth`, `Instant`.                                      | `Smooth`        |
| `top`         | `f64`          | Target top position for scrolling.                                            | `0.0`           |
| `left`        | `f64`          | Target left position for scrolling (horizontal scrolling).                    | `0.0`           |
| `offset`      | `f64`          | Offset to apply when scrolling to the target position.                        | `0.0`           |
| `delay`       | `u32`          | Delay (in ms) before initiating the scroll.                                   | `0`             |
| `auto_hide`   | `bool`         | Whether to hide the button automatically based on scroll position.            | `true`          |
| `threshold`   | `f64`          | Scroll threshold to determine button visibility.                              | `20.0` px       |
| `update_hash` | `bool`         | Whether to update the URL hash during scrolling.                              | `true`          |
| `show_id`     | `&'static str` | ID of the target element for the scroll button visibility logic.              | None            |
| `scroll_id`   | `&'static str` | ID of the target container to scroll to.                                      | None            |
