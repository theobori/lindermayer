use std::collections::HashMap;

use crate::{
    models::{
        rules_model::Rules,
        trace_model::Trace,
        action_model::Action,
        cursor_model::Cursor
    },
    action::Do
};

#[derive(Debug, Clone)]
pub struct LState {
    pub current: String,
    pub x: i32,
    pub y: i32,
    pub line_size: usize
}

pub struct LData {
    pub vars: Vec<char>,
    pub consts: Vec<char>,
    pub angle: usize,
    pub w: usize,
    pub h: usize
}

impl LData {
    pub fn new(w: usize, h: usize) -> Self {
        Self {
            vars: Vec::new(),
            consts: Vec::new(),
            angle: 0,
            w,
            h
        }
    }
}

pub struct Lindenmayer {
    data: LData,
    states: Vec<LState>,
    state_index: usize,
    rules: HashMap<char, String>,
    actions: HashMap<char, Do>,
    cursor: Option<Box<dyn Cursor>>
}

impl Lindenmayer {
    pub fn new(w: usize, h: usize) -> Self {
        Self {
            data: LData::new(w, h),
            states: Vec::new(),
            state_index: 0,
            rules: HashMap::new(),
            actions: HashMap::new(),
            cursor: None
        }
    }

    pub fn state(&self) -> Option<LState> {
        let len = self.states.len();
        
        if len == 0 {
            return None;
        }
        
        Some(self.states[len - 1].clone())
    }
    
    pub fn set_angle(&mut self, angle: usize) -> &mut Self {
        if angle > 360 {
            panic!("The angle has to be between 0° and 360°")
        }

        self.data.angle = angle;        

        self
    }

    fn is_var(&self, var: char) -> bool {
        self.data.vars.contains(&var)
    }

    fn is_const(&self, c: char) -> bool {
        self.data.consts.contains(&c)
    }

    pub fn add_vars(&mut self, vars: &str) -> &mut Self {
        for var in vars.chars() {
            if self.is_const(var) {
                panic!("{} is also a constant", var)
            }

            self.data.vars.push(var);
        }

        self
    }

    pub fn add_consts(&mut self, consts: &str) -> &mut Self{
        for c in consts.chars() {
            if self.is_var(c) {
                panic!("{} is also a variable", c)
            }

            self.data.consts.push(c);
        }

        self
    }

    pub fn set_render(&mut self, cursor: Box<dyn Cursor>) -> &mut Self {
        self.cursor = Some(cursor);

        self
    }

}

impl Action for Lindenmayer {
    type Name = char;

    type Do = Do;

    fn set_action(&mut self, name: Self::Name, action: Self::Do) {
        self.actions.insert(name, action);
    }

    fn get_action(&self, name: Self::Name) -> Option<&Self::Do> {
        self.actions.get(&name)
    }

    fn call(&self, action: Self::Do) {
        match action {
            Do::Forward => println!("Forward"),
            Do::Backward => println!("Backward"),
            Do::HalfForward => println!("HalfForward"),
            Do::HalfBackward => println!("HalfBackward"),
            Do::TurnRight => println!("TurnRight"),
            Do::TurnLeft => println!("TurnLeft"),
            Do::PenUp => println!("PenUp"),
            Do::PenDown => println!("PenDown"),
            Do::TurnRandom => println!("TurnRandom"),
            Do::ColorRandom => println!("ColorRandom"),
            Do::Save => println!("Save"),
            Do::Restore => println!("Restore"),
            Do::IncreaseSize => println!("IncreaseSize"),
            Do::DecreaseSize => println!("DecreaseSize"),
            Do::Size1 => println!("Size1"),
            Do::Size2 => println!("Size2"),
            Do::Size3 => println!("Size3"),
            Do::Size4 => println!("Size4"),
            Do::Size5 => println!("Size5"),
            Do::Size6 => println!("Size6"),
        }
    }
}

impl Trace for Lindenmayer {
    type State = LState;
    type States = Vec<Self::State>;

    fn add_trace(&mut self, trace: Self::State) {
        self.states.push(trace);
    }

    fn get_step(&self, n: usize) -> Option<Self::State> {
        if n >= self.states.len() {
            return None;
        }

        Some(self.states[n].clone())
    }

    fn next_trace(&mut self) -> Option<Self::State> {
        let index = self.state_index + 1;

        if self.is_empty() || index >= self.states.len() {
            return None;
        }

        self.state_index = index;

        Some(self.states[index].clone())
    }

    fn is_empty(&self) -> bool {
        self.states.len() == 0
    }

    fn previous_trace(&mut self) -> Option<Self::State> {
        if self.is_empty() || self.state_index <= 0 {
            return None;
        }

        self.state_index -= 1;

        Some(self.states[self.state_index].clone())
    }

    fn all_traces(&self) -> Self::States {
        self.states.clone()
    }

    fn set_step(&mut self, n: usize) -> Option<Self::State> {
        if n >= self.states.len() {
            return None;
        }

        Some(self.states[n].clone())
    }
}

impl Rules for Lindenmayer {
    type Source = char;
    type Destination = String;

    fn set_rule(&mut self, src: Self::Source, dest: &str) -> &mut Self {
        if self.is_var(src) == false {
            self.add_vars(&src.to_string());
        }

        self.rules.insert(src, String::from(dest));
    
        self
    }

    fn get_rule(&self, src: Self::Source) -> Option<&Self::Destination> {
        self.rules.get(&src)
    }
}
