use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub enum Error {
    ProblemNotReady,
    ProblemAlreadySolved,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Self::ProblemNotReady => "Problem is not ready to be solved, add residual blocks",
            Self::ProblemAlreadySolved => "Problem is already solved, drop and create new",
        };
        write!(f, "{}", msg)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}
