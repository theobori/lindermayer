use crate::rule::RulesWrap;

pub trait Rules {
    type Source;
    type Destination;
    type Table;

    fn set_rule<T: Into<RulesWrap>>(
        &mut self,
        src: Self::Source,
        dest: T
    ) -> &mut Self;
    fn get_rule(&self, src: Self::Source) -> Option<&Vec<Self::Destination>>;
    fn exists(&self, src: Self::Source) -> bool {
        let dest = self.get_rule(src);
        
        match dest {
            Some(_) => true,
            None => false
        }
    }
    fn get_rules(&mut self) -> Self::Table;
}
