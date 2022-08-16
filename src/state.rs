/// Location at screen
pub struct Pos {
    pub x: f64,
    pub y: f64
}

/// State of the "turtle" (cursor) in the render
pub struct State {
    pub position: Pos,
    pub angle: f64
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

impl From<(Side, f64)> for Angle {
    fn from(angle: (Side, f64)) -> Self {
        Self {
            side: angle.0,
            value: angle.1
        }
    }
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

pub enum SizeType {
    Custom(f64, f64),
    Auto
}
