//! Errors that the library can return.

/// Errors that the library can return while reading/writing the binary format.
#[derive(Debug)]
pub enum Error {
    /// The file doesn't have the correct magic number. ("NYA^")
    WrongMagic,
    /// The value of a DynamicType is invalid.
    InvalidDynamicType(i32),
    /// The value of a StaticType is invalid.
    InvalidStaticType(i32),
    /// The value of an ObjectPropertyType is invalid.
    InvalidObjectPropertyType(i32),
    /// The value of an ActionType is invalid.
    InvalidActionType(i32),
    /// An error occurred while reading a LEB128 value.
    LebRead(leb128::read::Error),
    /// An I/O error occurred while reading/writing to a file.
    Io(std::io::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::WrongMagic => write!(f, "wrong magic number"),
            Self::InvalidDynamicType(value) => write!(f, "invalid dynamic type: {value}"),
            Self::InvalidStaticType(value) => write!(f, "invalid static type: {value}"),
            Self::InvalidObjectPropertyType(value) => {
                write!(f, "invalid object property type: {value}")
            }
            Self::InvalidActionType(value) => write!(f, "invalid action type: {value}"),
            Self::LebRead(err) => write!(f, "{err}"),
            Self::Io(err) => write!(f, "{err}"),
        }
    }
}

impl std::error::Error for Error {}

impl From<leb128::read::Error> for Error {
    fn from(err: leb128::read::Error) -> Self {
        Self::LebRead(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}
