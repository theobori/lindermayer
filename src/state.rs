/// Location at screen
pub struct Pos {
    pub x: f64,
    pub y: f64
}

impl Default for Pos {
    fn default() -> Self {
        Self {
            x: 0.,
            y: 0.
        }
    }
}

#[derive(Debug)]
pub struct Size {
    pub w: f64,
    pub h: f64
}

impl Default for Size {
    fn default() -> Self {
        Self {
            w: 0.,
            h: 0.
        }
    }
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

impl Default for ScreenPosition {
    fn default() -> Self {
        ScreenPosition::Center
    }
}

pub enum SizeType {
    Custom(f64, f64),
    Auto
}
