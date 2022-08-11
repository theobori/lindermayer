use std::collections::HashMap;

use crate::{
    models::{
        rules_model::Rules,
        trace_model::Trace,
        action_model::Action,
        cursor_model::Cursor
    },
    action::Do, cursor::turtle::Turtle
};

#[derive(Debug, Clone)]
pub struct LState {
    /// Current line value
    pub value: String,
    /// Cursor location (x)
    pub x: i32,
    /// Cursor location (y)
    pub y: i32,
    /// Cursor orientation
    pub angle: usize
}

impl Default for LState {
    fn default() -> Self {
        Self {
            value: String::from(""),
            x: 0,
            y: 0,
            angle: 180
         }
    }
}

pub struct LData {
    /// Containing variables
    pub vars: Vec<char>,
    /// Containing constants
    pub consts: Vec<char>,
    /// Width of the graphic
    pub w: usize,
    /// Height of the graphic
    pub h: usize
}

impl LData {
    pub fn new(w: usize, h: usize) -> Self {
        Self {
            vars: Vec::new(),
            consts: Vec::new(),
            w,
            h
        }
    }
}

pub struct Lindenmayer {
    /// Constant data
    data: LData,
    /// Trace of the states over iterations
    trace_states: Vec<LState>,
    /// Index to navigate throught states (for Trace)
    state_index: usize,
    /// Current state
    current_state: LState,
    /// Rules table
    rules: HashMap<char, String>,
    /// Actions for vars / consts
    actions: HashMap<char, Do>,
    /// Graphics cursor
    cursor: Box<dyn Cursor>,
}

impl Lindenmayer {
    pub fn new(w: usize, h: usize) -> Self {
        Self {
            data: LData::new(w, h),
            trace_states: Vec::new(),
            current_state: LState::default(),
            state_index: 0,
            rules: HashMap::new(),
            actions: HashMap::new(),
            cursor: Box::new(Turtle::new())
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

    pub fn angle(&mut self, angle: usize) -> &mut Self {
        self.current_state.angle = angle;
        
        self
    }

    pub fn location(&mut self, x: i32, y: i32) -> &mut Self {
        self.current_state.x = x;
        self.current_state.y = y;
        
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
            x: self.current_state.x,
            y: self.current_state.y,
            angle: self.current_state.angle
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
        // Saving the current state in the traces
        self.add_trace(self.current_state.clone());

        // Overwriting
        self.overwrite_state_value();
    }

    pub fn iterate(&mut self, n: usize) -> &mut Self {
        for _ in 0..n {
            self.step();
        }
        
        self
    }

    pub fn set_render(&mut self, cursor: Box<dyn Cursor>) -> &mut Self {
        self.cursor = cursor;

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
            Do::TurnRandom => todo!(),
            Do::ColorRandom => todo!(),
            Do::Save => self.cursor.save_state(),
            Do::Restore => self.cursor.restore_state(),
            Do::IncreaseSize(size) => todo!(),
            Do::DecreaseSize(size) => todo!(),
            Do::LineSize(size) => self.cursor.set_size(size),
        }
    }
}

impl Trace for Lindenmayer {
    type State = LState;
    type States = Vec<Self::State>;

    fn add_trace(&mut self, trace: Self::State) {
        self.trace_states.push(trace);
    }

    fn get_step(&self, n: usize) -> Option<Self::State> {
        if n >= self.trace_states.len() {
            return None;
        }

        Some(self.trace_states[n].clone())
    }

    fn next_trace(&mut self) -> Option<Self::State> {
        let index = self.state_index + 1;

        if self.is_empty() || index >= self.trace_states.len() {
            return None;
        }

        self.state_index = index;

        Some(self.trace_states[index].clone())
    }

    fn is_empty(&self) -> bool {
        self.trace_states.len() == 0
    }

    fn previous_trace(&mut self) -> Option<Self::State> {
        if self.is_empty() || self.state_index <= 0 {
            return None;
        }

        self.state_index -= 1;

        Some(self.trace_states[self.state_index].clone())
    }

    fn all_traces(&self) -> Self::States {
        self.trace_states.clone()
    }

    fn set_step(&mut self, n: usize) -> Option<Self::State> {
        if n >= self.trace_states.len() {
            return None;
        }

        Some(self.trace_states[n].clone())
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
