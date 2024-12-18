#[derive(Clone, PartialEq)]
pub enum Behavior {
    Auto,
    Instant,
    Smooth,
}

/// Default CSS style for the scroll-to-top button.
pub const SCROLL_TO_TOP_STYLE: &str =
    "position: fixed; bottom: 1rem; right: 1rem; background-color: #3b82f6; color: #ffffff; padding: 0.75rem; border-radius: 50%; cursor: pointer; transition: background-color 300ms ease-in-out;";
