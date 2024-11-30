use core::fmt;
use std::{
    error::Error,
    num::{IntErrorKind, ParseIntError},
};



/// Enum to store the various types of errors that can cause parsing an integer to fail.
///
/// # Example
///
/// ```
/// # fn main() {
/// if let Err(e) = i32::from_str_radix("a12", 10) {
///     println!("Failed conversion to i32: {:?}", e.kind());
/// }
/// # }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum AIntErrorKind {
    /// Value being parsed is empty.
    ///
    /// This variant will be constructed when parsing an empty string.
    Empty,
    /// Contains an invalid digit in its context.
    ///
    /// Among other causes, this variant will be constructed when parsing a string that
    /// contains a non-ASCII char.
    ///
    /// This variant is also constructed when a `+` or `-` is misplaced within a string
    /// either on its own or in the middle of a number.
    InvalidDigit,
    /// Integer is too large to store in target integer type.
    PosOverflow,
    /// Integer is too small to store in target integer type.
    NegOverflow,
    /// Value was Zero
    ///
    /// This variant will be emitted when the parsing string has a value of zero, which
    /// would be illegal for non-zero types.
    Zero,

    #[default]
    Unknown,
}

impl AIntErrorKind {
    pub const fn from_native(kind: &IntErrorKind) -> Self {
        use core::num::IntErrorKind::*;
        match kind {
            &Empty => AIntErrorKind::Empty,
            &InvalidDigit => AIntErrorKind::InvalidDigit,
            &PosOverflow => AIntErrorKind::PosOverflow,
            &NegOverflow => AIntErrorKind::NegOverflow,
            &Zero => AIntErrorKind::Zero,
            _ => AIntErrorKind::Unknown,
        }
    }
}

impl fmt::Display for AIntErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use AIntErrorKind::*;
        match self {
            Empty => "cannot parse integer from empty string",
            InvalidDigit => "invalid digit found in string",
            PosOverflow => "number too large to fit in target type",
            NegOverflow => "number too small to fit in target type",
            Zero => "number would be zero for non-zero type",
            Unknown => "unknown error occured",
        }
        .fmt(f)
    }
}

/// An error which can be returned when parsing an integer.
///
/// This error is used as the error type for the `from_str_radix()` functions
/// on the primitive integer types, such as [`i8::from_str_radix`].
///
/// # Potential causes
///
/// Among other causes, `ParseReNumError` can be thrown because of leading or trailing whitespace
/// in the string e.g., when it is obtained from the standard input.
/// Using the [`str::trim()`] method ensures that no whitespace remains before parsing.
///
/// # Example
///
/// ```
/// if let Err(e) = i32::from_str_radix("a12", 10) {
///     println!("Failed conversion to i32: {e}");
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ParseAIntError {
    pub(crate) kind: AIntErrorKind,
}


impl ParseAIntError {
    /// Outputs the detailed cause of parsing an integer failing.
    pub const fn kind(&self) -> &AIntErrorKind {
        &self.kind
    }

    pub const fn from_native(err: ParseIntError) -> Self {
        Self {
            kind: AIntErrorKind::from_native(err.kind()),
        }
    }
}

impl fmt::Display for ParseAIntError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.kind.fmt(f)
    }
}

impl Error for ParseAIntError {}

impl Into<ParseIntError> for ParseAIntError {
    fn into(self) -> ParseIntError {
        // we can't construct a ParseIntError. But we can trigger one :)
        use AIntErrorKind::*;
        use core::num::NonZeroU8;
        match self.kind {
            Empty => "".parse::<u8>().unwrap_err(),
            InvalidDigit => "q".parse::<u8>().unwrap_err(),
            PosOverflow => "512".parse::<u8>().unwrap_err(),
            NegOverflow => "-1".parse::<u8>().unwrap_err(),
            Zero => "1".parse::<NonZeroU8>().unwrap_err(),
            // ParseIntError is non-exhastive. Let's just default to empty
            Unknown => "".parse::<u8>().unwrap_err(),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct TryNewError {
    pub(crate) kind: AIntErrorKind,
}

impl TryNewError {
    /// Outputs the detailed cause of parsing an integer failing.
    pub const fn kind(&self) -> &AIntErrorKind {
        &self.kind
    }
}
impl fmt::Display for TryNewError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.kind.fmt(f)
    }
}

impl Error for TryNewError {}
