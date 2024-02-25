use std::{fmt, str::Split};

use crate::controller::Command;

/// `ParseResult` represents an Ok result returned by parser when parsing was successful
///
/// # Example
/// ```rust
/// use sgdb::parser::{self, ParseResult};
/// let s = "begin";
/// assert_eq!(parser::parse_to_command(s), ParseResult::Command::Begin);
/// let s = "h";
/// assert_eq!(parser::parse_to_command(s), ParseResult::Help);
/// let s = "q";
/// assert_eq!(parser::parse_to_command(s), ParseResult::Quit);
/// ```
#[derive(Debug, PartialEq, Eq)]
pub enum ParseResult {
    /// The user wants the view to print the help
    Help,
    /// The user wants the view to quit
    Quit,
    /// The user wants to execute a controller [`Command`]
    Command(Command),
}

/// `ParseError` represents an error returned by parser if parsing was unsuccessful
///
/// # Example
/// ```rust
/// use sgdb::parser::{self, ParseError};
/// let s = "x";
/// assert_eq!(parser::parse_to_command(s), ParseError::Default);
/// let s = "re";
/// assert_eq!(parser::parse_to_command(s), ParseError::NoStudent);
/// let s = "re 1";
/// assert_eq!(parser::parse_to_command(s), ParseError::NoInstrument);
/// ```
#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    /// Command was not of any recognised type or incorrect input
    Default,
    /// No instrument was supplied to command which requires it
    NoInstrument,
    /// No student was supplied to command which requires it
    NoStudent,
}

impl From<Command> for ParseResult {
    fn from(value: Command) -> Self {
        Self::Command(value)
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Default => write!(f, "Command not understood! Invalid command."),
            Self::NoInstrument => {
                write!(f, "Command not understood! Missing instrument in command!")
            }
            Self::NoStudent => write!(f, "Command not understood! Missing student in command!"),
        }
    }
}

/// Parses a &str to a [`ParseResult`] or [`ParseError`]
///
/// # Parameters
/// - `s` the string reference to parse
///
/// # Returns
/// - [`ParseResult`] if the string was parsable
/// - [`ParseError`] if the string was not parsable
///
/// # Example
/// ```rust
/// use sgdb::parser::{self, ParseResult};
/// use sgdb::controller::Command;
/// let s = "begin";
/// assert_eq!(parser::parse_to_command(s), ParseResult::Command(Command::Begin));
/// ```
pub fn parse_to_command(s: &str) -> Result<ParseResult, ParseError> {
    let mut words = s.trim().split(' ');

    words.next().map_or_else(
        || Err(ParseError::Default),
        |w| match w.chars().next().unwrap_or_default() {
            'b' => Ok(Command::Begin.into()),
            'c' => Ok(Command::Commit.into()),
            'h' => Ok(ParseResult::Help),
            'l' => Ok(parse_list(words)),
            'q' => Ok(ParseResult::Quit),
            't' => parse_terminate(words),
            'r' => match w.chars().nth(1).unwrap_or_default() {
                'e' => parse_rent(words),
                'o' => Ok(Command::Rollback.into()),
                _ => Err(ParseError::Default),
            },
            _ => Err(ParseError::Default),
        },
    )
}

fn parse_list(mut words: Split<'_, char>) -> ParseResult {
    let instrument_type = words.next().unwrap_or_default();
    if instrument_type.is_empty() {
        Command::List(None).into()
    } else {
        Command::List(Some(String::from(instrument_type))).into()
    }
}

fn parse_rent(mut words: Split<'_, char>) -> Result<ParseResult, ParseError> {
    let user = words.next().ok_or(ParseError::NoStudent)?;
    let instrument = words.next().ok_or(ParseError::NoInstrument)?;

    Ok(Command::Rent(user.into(), instrument.into()).into())
}

fn parse_terminate(mut words: Split<'_, char>) -> Result<ParseResult, ParseError> {
    let user = words.next().ok_or(ParseError::NoStudent)?;
    let instrument = words.next().ok_or(ParseError::NoInstrument)?;

    Ok(Command::TryTerminate(user.into(), instrument.into()).into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::controller::Command;

    #[test]
    fn test_corr_parse_to_command() {
        let corr = vec![
            ParseResult::Command(Command::Begin),
            ParseResult::Command(Command::Begin),
            ParseResult::Command(Command::Commit),
            ParseResult::Command(Command::Commit),
            ParseResult::Help,
            ParseResult::Help,
            ParseResult::Command(Command::List(None)),
            ParseResult::Command(Command::List(None)),
            ParseResult::Command(Command::List(Some(String::from("gui")))),
            ParseResult::Command(Command::List(Some(String::from("gui")))),
            ParseResult::Quit,
            ParseResult::Quit,
            ParseResult::Command(Command::Rent("1".into(), "2".into())),
            ParseResult::Command(Command::Rent("1".into(), "2".into())),
            ParseResult::Command(Command::Rollback),
            ParseResult::Command(Command::Rollback),
            ParseResult::Command(Command::TryTerminate("1".into(), "2".into())),
            ParseResult::Command(Command::TryTerminate("1".into(), "2".into())),
        ];

        let data = vec![
            "b",
            "begin",
            "c",
            "commit",
            "h",
            "help",
            "l",
            "list",
            "l gui",
            "list gui",
            "q",
            "quit",
            "re 1 2",
            "rent 1 2",
            "ro",
            "rollback",
            "t 1 2",
            "terminate 1 2",
        ];

        for i in 0..data.len() {
            assert_eq!(parse_to_command(data[i]).unwrap(), corr[i]);
        }
    }

    #[test]
    fn test_fail_parse_to_command() {
        let corr = vec![
            ParseError::Default,
            ParseError::NoStudent,
            ParseError::NoStudent,
            ParseError::NoInstrument,
            ParseError::NoInstrument,
        ];

        let data = vec!["x", "re", "t", "re 1", "t 1"];

        for i in 0..data.len() {
            assert_eq!(parse_to_command(data[i]).unwrap_err(), corr[i]);
        }
    }
}
