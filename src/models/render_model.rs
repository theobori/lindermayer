use crate::state::{
    ScreenPosition,
    Angle
};

pub trait Render {
    fn step_forward(&mut self, distance: f64);
    fn step_backward(&mut self, distance: f64);
    fn turn_left(&mut self, angle: f64);
    fn turn_right(&mut self, angle: f64);
    fn turn_random(&mut self);
    fn pen_up(&mut self);
    fn pen_down(&mut self);
    fn color_random(&mut self);
    fn save_state(&mut self);
    fn restore_state(&mut self);
    fn save_state_and_turn(&mut self, angle: Angle);
    fn restore_state_and_turn(&mut self, angle: Angle);
    fn set_pen_size(&mut self, size: f64);
    fn save_svg(&mut self, filename: &str);
    fn set_pos(&mut self, pos: ScreenPosition);
}
