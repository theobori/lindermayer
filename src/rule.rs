/// Representing a lindenmayer rule
#[derive(Clone)]
pub struct Rule {
    /// Rule value as str
    pub value: String,
    /// Weigth for stochastic alternative
    weight: usize
}

impl Rule {
    pub fn new(value: &str, weight: usize) -> Rule {
        Self {
            value: String::from(value),
            weight
        }
    }

    /// Returns the rule weight
    pub fn weight(&self) -> usize {
        self.weight
    }
}

impl Into<Rule> for &str {
    fn into(self) -> Rule {
        Rule {
            value: String::from(self),
            weight: 100
        }
    }
}

impl Into<Rule> for char {
    fn into(self) -> Rule {
        Rule {
            value: String::from(self),
            weight: 100
        }
    }
}

impl Into <Rule> for (&str, usize) {
    fn into(self) -> Rule {
        Rule {
            value: String::from(self.0),
            weight: self.1
        }
    }
}

pub struct RulesWrap(Vec<Rule>);

impl From<&str> for RulesWrap {
    fn from(value: &str) -> Self {
        RulesWrap(vec![Rule::new(value, 1)])
    }
}

impl Into<Vec<Rule>> for RulesWrap {
    fn into(self) -> Vec<Rule> {
        self.0
    }
}

impl From<Vec<Rule>> for RulesWrap {
    fn from(value: Vec<Rule>) -> Self {
        Self(value)
    }
}
