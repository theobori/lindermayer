#[derive(Clone)]
pub enum Do {
    Forward(usize),
    Backward(usize),
    Left(usize),
    Right(usize),
    PenUp,
    PenDown,
    TurnRandom,
    ColorRandom,
    Save,
    Restore,
    IncreaseSize(usize),
    DecreaseSize(usize),
    LineSize(usize)
}
