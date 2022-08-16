use crate::state::Angle;

#[derive(Clone)]
pub enum Do {
    Forward(f64),
    Backward(f64),
    Left(f64),
    Right(f64),
    PenUp,
    PenDown,
    TurnRandom,
    ColorRandom,
    Save,
    Restore,
    SaveAndTurn(Angle),
    RestoreAndTurn(Angle),
    IncreaseSize(f64),
    DecreaseSize(f64),
    LineSize(f64)
}
