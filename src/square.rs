use crate::state::{Size, Pos};

#[derive(Debug)]
pub struct Square {
    /// Top left position
    pub top_left: Pos,
    /// Square area size
    pub bottom_right: Pos,
}

impl Square {
    /// Square width
    pub fn width(&self) -> f64 {
        self.bottom_right.x - self.top_left.x
    }

    /// Square height
    pub fn height(&self) -> f64 {
        self.top_left.y - self.bottom_right.y
    }

    /// Square size
    pub fn size(&self) -> Size {
        Size {
            w: self.width(),
            h: self.height()
        }
    }

    /// Change square position, the top left corner
    pub fn set_position(&mut self, point: Pos) {
        // Storing size
        let size = self.size();

        // Changing top left corner
        self.top_left = point;

        // Adapting bottom right corner
        self.bottom_right.x = self.top_left.x + size.w;
        self.bottom_right.y = self.top_left.y + size.h;
    }

    /// Update the size if `point` is out of the square area
    pub fn update_max_area(&mut self, point: Pos) {
        // Checking top left position
        if point.x < self.top_left.x {
            self.top_left.x = point.x;
        }
        if point.y > self.top_left.y {
            self.top_left.y = point.y;
        }

        // Checking bottom right position
        if point.x > self.bottom_right.x {
            self.bottom_right.x = point.x;
        }
        if point.y < self.bottom_right.y {
            self.bottom_right.y = point.y;
        }
    }
}
