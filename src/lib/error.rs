use std::{error::Error as StdError, fmt};

#[derive(Debug)]
pub enum Error {
    /// Some unspecified error.
    Any(Box<dyn StdError + Send + Sync + 'static>),
    TokioError {
        description: String,
    },
    UnknownError,
    UnreadableMessage,
    FileNotFound {
        path: String,
    },
    RequestError {
        description: String,
    },
    ParsingError {
        description: String,
    },
    MissingChatId,
    NoInput,
}

impl Error {
    /// Prints error to stderr and exits the program.
    pub fn exit(self) {
        eprintln!("{}", self);
        std::process::exit(0);
    }
}

impl<'a> fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::RequestError { ref description } | Error::TokioError { ref description } => {
                write!(f, "\nMessage failed to send due to:\n\t{}", description)
            }
            Error::FileNotFound { ref path } => {
                write!(f, "\nCould not find file in path:\n\t{}", path)
            }
            Error::MissingChatId => {
                write!(f, "\nChat ID not found in flags or TEPE_TELEGRAM_CHAT_ID")
            }
            Error::NoInput => write!(f, "\nNo input was given"),
            Error::ParsingError { ref description } => {
                write!(f, "\nError from parsing:\n\t{}", description)
            }
            Error::UnreadableMessage => write!(f, "\nIssue parsing message"),
            _ => write!(f, "\nTODO: add error description"),
        }
    }
}

use teloxide::RequestError;
impl From<RequestError> for Error {
    fn from(error: RequestError) -> Self {
        Error::RequestError {
            description: format!("{}", error),
        }
    }
}

use tokio::io::Error as TokioError;
impl From<TokioError> for Error {
    fn from(error: TokioError) -> Self {
        Error::TokioError {
            description: format!("{}", error),
        }
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(error: std::num::ParseIntError) -> Self {
        Error::ParsingError {
            description: error.to_string(),
        }
    }
}

/// Trait to allow for an panic without Rust errors printing.
/// This is mainly meant for Option and Result.
pub trait CliExit<T> {
    /// Prints message to stderr and exits the program.
    fn cli_expect(self, message: &str) -> T;
}

impl<T, E> CliExit<T> for Result<T, E> {
    fn cli_expect(self, message: &str) -> T {
        match self {
            Ok(t) => t,
            Err(_e) => {
                eprintln!("{}", message);
                std::process::exit(0);
            }
        }
    }
}

impl<T> CliExit<T> for Option<T> {
    fn cli_expect(self, message: &str) -> T {
        match self {
            Some(t) => t,
            None => {
                eprintln!("{}", message);
                std::process::exit(0);
            }
        }
    }
}
