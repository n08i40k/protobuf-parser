use lalrpop_util::lalrpop_mod;

lalrpop_mod!(
    #[allow(clippy::ptr_arg)]
    #[rustfmt::skip]
    pub proto
);

pub mod ast;
pub mod lexer;
mod parser;

pub use ast::Root;
pub use parser::{parse, ParseError, ParseResult};

#[cfg(test)]
mod tests;
