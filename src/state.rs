/// Location at screen
pub struct Pos {
    pub x: f64,
    pub y: f64
}

/// State of the "turtle" (cursor) in the render
pub struct State {
    pub position: Pos,
    pub angle: Angle
}

#[derive(Clone)]
pub enum Side {
    Left,
    Right
}

#[derive(Clone)]
pub struct Angle {
    pub side: Side,
    pub value: f64
}

impl Default for Angle {
    fn default() -> Self {
        Self {
            side: Side::Left, 
            value: 0.0
        }
    }
}

pub enum ScreenPosition {
    Coord(f64, f64),
    Center,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight
}
