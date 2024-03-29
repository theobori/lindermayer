use std::collections::HashMap;
use weighted_rand::builder::*;

use crate::{
    models::{
        rules_model::Rules,
        action_model::Action,
        render_model::Render
    },
    action::Do,
    state::ScreenPosition,
    renders::renderer::Renderer, rule::{Rule, RulesWrap}
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

impl Default for LData {
    fn default() -> Self {
        Self {
            vars: Vec::new(),
            consts: Vec::new(),
        }
    }
}

impl LData {
    pub fn new() -> Self {
        Self::default()
    }
}

pub struct Lindenmayer {
    /// Constant data
    data: LData,
    /// Current state
    current_state: LState,
    /// Rules table
    rules: HashMap<char, Vec<Rule>>,
    /// Actions for vars / consts
    actions: HashMap<char, Do>,
    /// Graphics cursor
    cursor: Box<dyn Render>,
}

impl Lindenmayer {
    pub fn new(render: Renderer) -> Self {
        Self {
            data: LData::new(),
            current_state: LState::default(),
            rules: HashMap::new(),
            actions: HashMap::new(),
            cursor: render.get_render_obj()
        }
    }

    /// Return a clone of the current system state
    pub fn state(&self) -> LState {
        self.current_state.clone()
    }
    
    fn is_var(&self, var: char) -> bool {
        self.data.vars.contains(&var)
    }

    fn is_const(&self, c: char) -> bool {
        self.data.consts.contains(&c)
    }

    /// Add a variable to the system
    pub fn set_vars(&mut self, vars: &str) -> &mut Self {
        for var in vars.chars() {
            if self.is_const(var) {
                panic!("{} is also a constant", var)
            }

            self.data.vars.push(var);
        }

        self
    }

    /// Add a constant to the system
    pub fn set_consts(&mut self, consts: &str) -> &mut Self{
        for c in consts.chars() {
            if self.is_var(c) {
                panic!("{} is also a variable", c)
            }

            self.data.consts.push(c);
        }

        self
    }

    /// Set the beginning value for the system
    pub fn set_axiom(&mut self, value: &str) -> &mut Self {
        self.current_state.value = String::from(value);
        
        self
    }

    fn overwrite_state_value(&mut self) {
        // Overwrite state value with the rules table
        let mut next_value = String::from("");

        // Building new string
        for c in self.current_state.value.chars() {
            let part = match self.get_rule(c) {
                Some(value) => {
                    let weights: Vec<u32> = value
                        .iter()
                        .map(| rule | rule.weight() as u32)
                        .collect();
                    
                    let values: Vec<&str> = value
                        .iter()
                        .map(| rule | &*rule.value)
                        .collect();

                    let builder = WalkerTableBuilder::new(&weights);
                    let wa_table = builder.build();
                    
                    values[wa_table.next()].to_string()
                },
                None => c.to_string()
            };

            next_value.push_str(&part);
        }

        // New current state
        self.current_state = LState {
            value: next_value,
        };
    }

    /// Draw / compose the graphic figure
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

    /// Executes `n` step(s)
    pub fn iterate(&mut self, n: usize) -> &mut Self {
        for _ in 0..n {
            self.step();
        }
        
        self
    }

    /// Set the renderer
    pub fn set_render(&mut self, cursor: Renderer) -> &mut Self {
        self.cursor = cursor.get_render_obj();

        self
    }

    /// Save the drawing as SVG
    pub fn save_svg(&mut self, filename: &str) -> &mut Self {
        self.cursor.save_svg(filename);

        self
    }

    /// Set the graphic figure position on the drawing
    pub fn set_figure_pos(&mut self, pos: ScreenPosition) -> &mut Self {
        self.cursor.set_figure_pos(pos);
        
        self
    }

    /// Set the drawing background color
    pub fn set_background(&mut self, r: f64, g: f64, b: f64) -> &mut Self {
        self.cursor.set_bg(r, g, b);
        
        self
    }

    /// Resets the system
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

    fn set_action(&mut self, name: Self::Name, action: Self::Do) -> &mut Self {
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
    type Destination = Rule;
    type Table = HashMap<char, Vec<Rule>>;

    fn set_rule<T: Into<RulesWrap>>(
        &mut self,
        src: Self::Source,
        dest: T
    ) -> &mut Self {
        if self.is_var(src) == false {
            self.set_vars(&src.to_string());
        }

        let rules_wrap: RulesWrap = dest.into();

        self.rules.insert(src, rules_wrap.into());
    
        self
    }

    fn get_rule(
        &self,
        src: Self::Source
    ) -> Option<&Vec<Self::Destination>> {
        self.rules.get(&src)
    }

    fn get_rules(&mut self) -> Self::Table {
        self.rules.clone()
    }
}
