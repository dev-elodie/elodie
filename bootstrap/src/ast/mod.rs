use crate::ast::ast::SourceFile;
use crate::ast::lex::lex;
use crate::ast::parse::parse;

mod lex;
mod parse;
mod ast;
mod token;

#[derive(Debug)]
pub enum Error {
    Lexer(lex::Error),
    Parser(parse::Error),
    Rewriter(ast::Error),
}

impl From<lex::Error> for Error {
    fn from(value: lex::Error) -> Self {
        Self::Lexer(value)
    }
}

impl From<parse::Error> for Error {
    fn from(value: parse::Error) -> Self {
        Self::Parser(value)
    }
}

impl From<ast::Error> for Error {
    fn from(value: ast::Error) -> Self {
        Self::Rewriter(value)
    }
}

pub(crate) type Result<T, E = Error> = core::result::Result<T, E>;


pub fn parse_str(str: &str) -> Result<SourceFile> {
    let tokens = lex(str)?;
    let root = parse(tokens)?;
    Ok(ast::from(root)?)
}
