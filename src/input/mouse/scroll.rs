use sdl3::mouse::MouseWheelDirection;

/// # Scroll
///
/// Provides information about a mouse wheel scrolling event in the window.
#[derive(Debug, Clone)]
pub struct Scroll {
    /// Distance horizontally scrolled
    pub distance_x: f32,
    /// Distance vertically scrolled
    pub distance_y: f32,
    /// The direction of the scroll
    pub direction: MouseWheelDirection,
}
