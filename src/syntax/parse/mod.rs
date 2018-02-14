//! Parser utilities

use lalrpop_util::ParseError as LalrpopError;
use source::pos::{BytePos, RawIndex, Span};
use source::reporting::Diagnostic;
use std::fmt;
use std::str::FromStr;

use syntax::concrete;
use syntax::parse::lexer::Lexer;

pub use syntax::parse::lexer::{LexerError, Token};

mod lexer;

mod grammar {
    include!(concat!(env!("OUT_DIR"), "/syntax/parse/grammar.rs"));
}

#[derive(Fail, Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    #[fail(display = "{}", _0)]
    Lexer(#[cause] LexerError),
    #[fail(display = "An identifier was expected when parsing a pi type at byte range {}.", span)]
    IdentifierExpectedInPiType { span: Span },
    #[fail(display = "An integer literal {} was too large for the target type at byte range {}.",
           value, span)]
    IntegerLiteralOverflow { span: Span, value: u64 },
    #[fail(display = "Unknown repl command `:{}` found at byte range {}.", command, span)]
    UnknownReplCommand { span: Span, command: String },
    #[fail(display = "Unexpected EOF at byte pos {}, expected one of: {}.", end, expected)]
    UnexpectedEof {
        end: BytePos,
        expected: ExpectedTokens,
    },
    #[fail(display = "Unexpected token {}, found at byte range {}, expected one of: {}.", token,
           span, expected)]
    UnexpectedToken {
        span: Span,
        token: Token<String>,
        expected: ExpectedTokens,
    },
    #[fail(display = "Extra token {} found at byte range {}", token, span)]
    ExtraToken { span: Span, token: Token<String> },
}

impl ParseError {
    /// Return the span of source code that this error originated from
    pub fn span(&self) -> Span {
        match *self {
            ParseError::Lexer(ref err) => err.span(),
            ParseError::IdentifierExpectedInPiType { span }
            | ParseError::IntegerLiteralOverflow { span, .. }
            | ParseError::UnknownReplCommand { span, .. }
            | ParseError::UnexpectedToken { span, .. }
            | ParseError::ExtraToken { span, .. } => span,
            ParseError::UnexpectedEof { end, .. } => Span::new(end, end),
        }
    }

    fn from_lalrpop<T>(src: &str, err: LalrpopError<BytePos, T, ParseError>) -> ParseError
    where
        T: Into<Token<String>>,
    {
        match err {
            LalrpopError::User { error } => error,
            LalrpopError::InvalidToken { .. } => unreachable!(),
            LalrpopError::UnrecognizedToken {
                token: None,
                expected,
            } => ParseError::UnexpectedEof {
                end: BytePos(src.len() as RawIndex),
                expected: ExpectedTokens(expected),
            },
            LalrpopError::UnrecognizedToken {
                token: Some((lo, token, hi)),
                expected,
            } => ParseError::UnexpectedToken {
                span: Span::new(lo, hi),
                token: token.into(),
                expected: ExpectedTokens(expected),
            },
            LalrpopError::ExtraToken {
                token: (lo, token, hi),
            } => ParseError::ExtraToken {
                span: Span::new(lo, hi),
                token: token.into(),
            },
        }
    }

    /// Convert the error into a diagnostic message
    pub fn to_diagnostic(&self) -> Diagnostic {
        use source::reporting::Severity;

        let message = match *self {
            ParseError::Lexer(LexerError::UnexpectedCharacter { found, .. }) => {
                format!("unexpected character {:?}", found)
            },
            ParseError::IdentifierExpectedInPiType { .. } => {
                format!("identifier expected when parsing pi type")
            },
            ParseError::IntegerLiteralOverflow { .. } => format!("integer literal overflow"),
            ParseError::UnknownReplCommand { ref command, .. } => {
                format!("unknown repl command {}", command)
            },
            ParseError::UnexpectedToken {
                ref token,
                ref expected,
                ..
            } => format!("unexpected token {}, expected one of {}", token, expected),
            ParseError::UnexpectedEof { ref expected, .. } => {
                format!("unexpected EOF, expected one of {}", expected)
            },
            ParseError::ExtraToken { ref token, .. } => format!("extra token {}", token),
        };

        Diagnostic::spanned(self.span(), Severity::Error, message)
    }
}

impl From<LexerError> for ParseError {
    fn from(src: LexerError) -> ParseError {
        ParseError::Lexer(src)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExpectedTokens(pub Vec<String>);

impl fmt::Display for ExpectedTokens {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, token) in self.0.iter().enumerate() {
            match i {
                0 => write!(f, "{}", token)?,
                i if i < self.0.len() - 1 => write!(f, ", {}", token)?,
                _ => write!(f, ", or {}", token)?,
            }
        }
        Ok(())
    }
}

impl FromStr for concrete::ReplCommand {
    type Err = ParseError;

    fn from_str(src: &str) -> Result<concrete::ReplCommand, ParseError> {
        grammar::parse_ReplCommand(Lexer::new(src).map(|x| x.map_err(ParseError::from)))
            .map_err(|err| ParseError::from_lalrpop(src, err))
    }
}

impl FromStr for concrete::Module {
    type Err = ParseError;

    fn from_str(src: &str) -> Result<concrete::Module, ParseError> {
        grammar::parse_Module(Lexer::new(src).map(|x| x.map_err(ParseError::from)))
            .map_err(|err| ParseError::from_lalrpop(src, err))
    }
}

impl FromStr for concrete::Declaration {
    type Err = ParseError;

    fn from_str(src: &str) -> Result<concrete::Declaration, ParseError> {
        grammar::parse_Declaration(Lexer::new(src).map(|x| x.map_err(ParseError::from)))
            .map_err(|err| ParseError::from_lalrpop(src, err))
    }
}

impl FromStr for concrete::Term {
    type Err = ParseError;

    fn from_str(src: &str) -> Result<concrete::Term, ParseError> {
        grammar::parse_Term(Lexer::new(src).map(|x| x.map_err(ParseError::from)))
            .map_err(|err| ParseError::from_lalrpop(src, err))
    }
}

/// This is an ugly hack that cobbles together a pi type from a binder term and
/// a body. See the comments on the `PiTerm` rule in the `grammer.lalrpop` for
/// more information.
fn reparse_pi_type_hack<L, T>(
    span: Span,
    binder: concrete::Term,
    body: concrete::Term,
) -> Result<concrete::Term, LalrpopError<L, T, ParseError>> {
    use syntax::concrete::Term;

    fn param_names<L, T>(
        term: Term,
        names: &mut Vec<(Span, String)>,
    ) -> Result<(), LalrpopError<L, T, ParseError>> {
        match term {
            Term::Var(span, name) => names.push((span, name)),
            Term::App(fn_expr, arg) => {
                param_names(*fn_expr, names)?;
                param_names(*arg, names)?;
            },
            term => {
                return Err(LalrpopError::User {
                    error: ParseError::IdentifierExpectedInPiType { span: term.span() }, // TODO: better error!
                });
            },
        }
        Ok(())
    }

    match binder {
        Term::Parens(paren_span, term) => {
            let term = *term; // HACK: see https://github.com/rust-lang/rust/issues/16223
            match term {
                Term::Ann(params, ann) => {
                    let mut names = Vec::new();
                    param_names(*params, &mut names)?;
                    Ok(Term::Pi(span.lo(), (names, ann), body.into()))
                },
                ann => {
                    let parens = Term::Parens(paren_span, ann.into()).into();
                    Ok(Term::Arrow(parens, body.into()))
                },
            }
        },
        ann => Ok(Term::Arrow(ann.into(), body.into())),
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn pi_bad_ident() {
        let parse_result = concrete::Term::from_str("((x : Type) : Type) -> Type");

        assert_eq!(
            parse_result,
            Err(ParseError::IdentifierExpectedInPiType {
                span: Span::new(BytePos(1), BytePos(11)),
            })
        );
    }
}