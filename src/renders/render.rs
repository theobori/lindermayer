use crate::models::render_model::Render;

use super::turtle::TurtleRender;

pub enum RenderType {
    TurtleNormal(u32, u32),
    TurtleHeadless(f64, f64)
}

impl RenderType {
    pub fn get_render(self) -> Box<dyn Render> {
        let render = match self {
            RenderType::TurtleNormal(w, h) => TurtleRender::new(w, h),
            RenderType::TurtleHeadless(_, _) => todo!(),
        };

        Box::new(render)
    }
}
