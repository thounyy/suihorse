use std::fmt;

#[derive(Debug)]
pub struct ActionError {
    pub kind: ActionErrorKind,
}

impl fmt::Display for ActionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.kind)
    }
}

impl std::error::Error for ActionError {}

#[derive(PartialEq, Clone, Debug)]
pub enum ActionErrorKind {
    NotFound,
}

impl fmt::Display for ActionErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ActionErrorKind::NotFound => f.write_str("NotFound"),
        }
    }
}