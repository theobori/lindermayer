pub trait Action {
    type Name;
    type Do;

    fn set_action(&mut self, name: Self::Name, action: Self::Do);
    fn get_action(&self, name: Self::Name) -> Option<&Self::Do>;
    fn call(&self, action: Self::Do);
}
