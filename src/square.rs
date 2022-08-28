use turtle::Point;

use crate::state::Size;

#[derive(Debug)]
pub struct Square {
    /// Top left position
    pub pos: Point,
    /// Square area size
    pub size: Size,
}

impl Square {
    /// Change square position, the top left
    pub fn set_position(&mut self, point: Point) {
        self.pos = point;
    }

    /// Update the size if `point` is out of the square area
    pub fn update_max_area(&mut self, point: Point) {
        // Updating top left position
        if point.x < self.pos.x {
            self.pos.x = point.x;
        }
        if point.y > self.pos.y {
            self.pos.y = point.y;
        }

        // Storing points differences
        let x_diff = point.x - self.pos.x;
        let y_diff = self.pos.y - point.y;

        // Updating the square size (w, h)
        if self.size.w < x_diff {
            self.size.w = x_diff;
        }
        if self.size.h < y_diff {
            self.size.h = y_diff;
        }
    }
}
