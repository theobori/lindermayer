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
    LineSize(f64),
    PenColor(f64, f64, f64)
}
