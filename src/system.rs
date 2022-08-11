use std::collections::HashMap;

use crate::models::{rules_model::Rules, trace_model::Trace, actions_model::Actions};

#[derive(Debug, Clone)]
pub struct LSystemState {
    pub current: String,
}

pub struct LData {
    pub vars: Vec<char>,
    pub consts: Vec<char>,
    pub angle: usize
}

pub struct LSystem {
    data: LData,
    states: Vec<LSystemState>,
    state_index: usize,
    rules: HashMap<char, String>,
}

impl LSystem {
    pub fn new() -> Self {
        Self {
            data: LData {
                vars: Vec::new(),
                consts: Vec::new(),
                angle: 0
            },
            states: Vec::new(),
            state_index: 0,
            rules: HashMap::new(),
        }
    }

    pub fn state(&self) -> Option<LSystemState> {
        let len = self.states.len();
        
        if len == 0 {
            return None;
        }
        
        Some(self.states[len - 1].clone())
    }
    
    pub fn set_angle(&mut self, angle: usize) -> &Self {
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

    fn add_var(&mut self, var: char) {
        if self.is_const(var) {
            panic!("{} is also a constant", var)
        }

        self.data.vars.push(var);
    }

    fn add_const(&mut self, c: char) {
        if self.is_var(c) {
            panic!("{} is also a variable", c)
        }

        self.data.consts.push(c);
    }
}

impl Actions for LSystem {

}

impl Trace for LSystem {
    type State = LSystemState;
    type States = Vec<Self::State>;

    fn add_trace(&mut self, trace: Self::State) {
        todo!()
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
}

impl Rules for LSystem {
    type Source = char;
    type Destination = String;

    fn set_rule(&mut self, src: Self::Source, dest: &str) {
        if self.is_var(src) == false {
            self.add_var(src);
        }

        self.rules.insert(src, String::from(dest));
    }

    fn get_dest(&self, src: Self::Source) -> Option<&Self::Destination> {
        self.rules.get(&src)
    }
}
