use crate::{
    models::render_model::Render,
    state::ScreenPosition
};

pub struct TurtleHeadless {

}

impl Render for TurtleHeadless {
    fn step_forward(&mut self, distance: f64) {
        todo!()
    }

    fn step_backward(&mut self, distance: f64) {
        todo!()
    }

    fn turn_left(&mut self, angle: f64) {
        todo!()
    }

    fn turn_right(&mut self, angle: f64) {
        todo!()
    }

    fn turn_random(&mut self) {
        todo!()
    }

    fn pen_up(&mut self) {
        todo!()
    }

    fn pen_down(&mut self) {
        todo!()
    }

    fn color_random(&mut self) {
        todo!()
    }

    fn save_state(&mut self) {
        todo!()
    }

    fn restore_state(&mut self) {
        todo!()
    }

    fn set_pen_size(&mut self, size: f64) {
        todo!()
    }

    fn save_svg(&mut self, filename: &str) {
        todo!()
    }

    fn set_pos(&mut self, pos: ScreenPosition) {
        todo!()
    }
}
