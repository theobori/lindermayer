pub trait Action {
    type Name;
    type Do;

    fn action(&mut self, name: Self::Name, action: Self::Do) -> &mut Self;
    fn get_action(&mut self, name: Self::Name) -> Option<Self::Do>;
    fn call(&mut self, action: Self::Do);
}
