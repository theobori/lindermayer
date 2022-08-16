use std::collections::HashMap;

use crate::{
    models::{
        rules_model::Rules,
        action_model::Action,
        render_model::Render
    },
    action::Do,
    state::ScreenPosition,
    renders::render::RenderType
};

#[derive(Debug, Clone)]
pub struct LState {
    /// Current line value
    pub value: String,
}

impl Default for LState {
    fn default() -> Self {
        Self {
            value: String::from(""),
         }
    }
}

pub struct LData {
    /// Containing variables
    pub vars: Vec<char>,
    /// Containing constants
    pub consts: Vec<char>,
}

impl LData {
    pub fn new() -> Self {
        Self {
            vars: Vec::new(),
            consts: Vec::new(),
        }
    }
}

pub struct Lindenmayer {
    /// Constant data
    data: LData,
    /// Current state
    current_state: LState,
    /// Rules table
    rules: HashMap<char, String>,
    /// Actions for vars / consts
    actions: HashMap<char, Do>,
    /// Graphics cursor
    cursor: Box<dyn Render>,
}

impl Lindenmayer {
    pub fn new(render: RenderType) -> Self {
        Self {
            data: LData::new(),
            current_state: LState::default(),
            rules: HashMap::new(),
            actions: HashMap::new(),
            cursor: render.get_render()
        }
    }

    pub fn state(&self) -> LState {
        self.current_state.clone()
    }
    
    fn is_var(&self, var: char) -> bool {
        self.data.vars.contains(&var)
    }

    fn is_const(&self, c: char) -> bool {
        self.data.consts.contains(&c)
    }

    pub fn vars(&mut self, vars: &str) -> &mut Self {
        for var in vars.chars() {
            if self.is_const(var) {
                panic!("{} is also a constant", var)
            }

            self.data.vars.push(var);
        }

        self
    }

    pub fn consts(&mut self, consts: &str) -> &mut Self{
        for c in consts.chars() {
            if self.is_var(c) {
                panic!("{} is also a variable", c)
            }

            self.data.consts.push(c);
        }

        self
    }

    pub fn axiom(&mut self, value: &str) -> &mut Self {
        self.current_state.value = String::from(value);
        
        self
    }

    fn overwrite_state_value(&mut self) {
        // Overwrite state value with the rules table
        let mut next_value = String::from("");

        // Building new string
        for c in self.current_state.value.chars() {
            let part = match self.get_rule(c) {
                Some(value) => value.clone(),
                None => c.to_string()
            };

            next_value.push_str(&part);
        }

        // New current state
        self.current_state = LState {
            value: next_value,
        };
    }

    pub fn draw(&mut self) -> &mut Self {
        let state = self.current_state.clone();
        let chars = state.value.chars();

        // Iterate over the chars
        for c in chars {
            // Get linked action and check if its linked
            let action = self.get_action(c);

            // Execute action
            match action {
                Some(value) => self.call(value),
                None => continue
            }
        }

        self
    }

    fn step(&mut self) {
        // Overwriting
        self.overwrite_state_value();
    }

    pub fn iterate(&mut self, n: usize) -> &mut Self {
        for _ in 0..n {
            self.step();
        }
        
        self
    }

    pub fn set_render(&mut self, cursor: RenderType) -> &mut Self {
        self.cursor = cursor.get_render();

        self
    }

    pub fn save_svg(&mut self, filename: &str) -> &mut Self {
        self.cursor.save_svg(filename);

        self
    }

    pub fn set_start_pos(&mut self, pos: ScreenPosition) -> &mut Self {
        self.cursor.set_pos(pos);
        
        self
    }

    pub fn set_background(&mut self, r: f64, g: f64, b: f64) -> &mut Self {
        self.cursor.set_bg(r, g, b);
        
        self
    }

    pub fn reset(&mut self) -> &mut Self {
        // Reset Screen
        self.cursor.reset();
        
        // Reset LState
        self.current_state = LState::default();

        self
    }
}

impl Action for Lindenmayer {
    type Name = char;

    type Do = Do;

    fn action(&mut self, name: Self::Name, action: Self::Do) -> &mut Self {
        self.actions.insert(name, action);

        self
    }

    fn get_action(&mut self, name: Self::Name) -> Option<Self::Do> {
        if let Some(v) = self.actions.get(&name) {
            return Some(v.clone());
        }

        None
    }

    fn call(&mut self, action: Self::Do) {
        match action {
            Do::Forward(length) => self.cursor.step_forward(length),
            Do::Backward(length) => self.cursor.step_backward(length),
            Do::Left(angle) => self.cursor.turn_left(angle),
            Do::Right(angle) => self.cursor.turn_right(angle),
            Do::PenUp => self.cursor.pen_up(),
            Do::PenDown => self.cursor.pen_down(),
            Do::TurnRandom => self.cursor.turn_random(),
            Do::ColorRandom => self.cursor.color_random(),
            Do::Save => self.cursor.save_state(),
            Do::Restore => self.cursor.restore_state(),
            Do::LineSize(size) => self.cursor.set_pen_size(size),
            Do::SaveAndTurn(angle) => self.cursor.save_state_and_turn(angle),
            Do::RestoreAndTurn(angle) => self.cursor.restore_state_and_turn(angle),
            Do::PenColor(r, g, b) => self.cursor.set_pen_color(r, g, b),
        }
    }
}

impl Rules for Lindenmayer {
    type Source = char;
    type Destination = String;
    type Table = HashMap<char, String>;

    fn rule(&mut self, src: Self::Source, dest: &str) -> &mut Self {
        if self.is_var(src) == false {
            self.vars(&src.to_string());
        }

        self.rules.insert(src, String::from(dest));
    
        self
    }

    fn get_rule(&self, src: Self::Source) -> Option<&Self::Destination> {
        self.rules.get(&src)
    }

    fn get_rules(&mut self) -> Self::Table {
        self.rules.clone()
    }
}
