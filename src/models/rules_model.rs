pub trait Rules {
    type Source;
    type Destination;

    fn set_rule(&mut self, src: Self::Source, dest: &str) -> &mut Self;
    fn get_rule(&self, src: Self::Source) -> Option<&Self::Destination>;
    fn exists(&self, src: Self::Source) -> bool {
        let dest = self.get_rule(src);
        
        match dest {
            Some(_) => true,
            None => false
        }
    }
}
