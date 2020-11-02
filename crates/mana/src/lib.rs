#![allow(dead_code, unused_imports)]

mod cli;
mod common;
mod lexer;

pub use cli::{start_cli, App};
pub use common::*;
pub use lexer::{Ast, AstElement, Token};
