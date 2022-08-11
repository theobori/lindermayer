pub trait Cursor {
    fn step_forward(&mut self, multiplier: usize);
    fn step_backward(&mut self, multiplier: usize);
    fn turn_left(&mut self);
    fn turn_right(&mut self);
    fn turn_random(&mut self);
    fn pen_up(&mut self);
    fn pen_down(&mut self);
    fn color_random(&mut self);
    fn save_state(&mut self);
    fn restore_state(&mut self);
    fn set_size(&mut self, size: usize);
}
