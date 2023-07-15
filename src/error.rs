pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Float index out of bounds,")]
    FloatIndexOutOfBounds,
    #[error("Int index out of bounds,")]
    IntIndexOutOfBounds,
    #[error("Bool index out of bounds,")]
    BoolIndexOutOfBounds,
    #[error("The encoding's variables haven't been exhasted")]
    Incomplete,
    #[error("Ser message: {0}")]
    SerMessage(String),
    #[error("De message: {0}")]
    DeMessage(String),
}

impl serde::ser::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        Self::SerMessage(msg.to_string())
    }
}

impl serde::de::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        Self::DeMessage(msg.to_string())
    }
}
