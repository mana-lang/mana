use super::token;
use logos::{Logos, Span};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

/// Abstract syntax tree element
pub struct AstElement {
    /// Token type
    pub __type: token::Token,
    /// Type alias to `Range<usize>`. Token position in the file.
    pub position: Option<Span>,
    /// Child elements. Relevant for functions, modules, tuples, macros, etc.
    pub children: Vec<AstElement>,
    /// Parent element reference
    pub parent: Option<&'static AstElement>,
}

impl AstElement {
    pub fn new() -> AstElement {
        AstElement {
            __type: token::Token::Error,
            position: None,
            children: Vec::<AstElement>::new(),
            parent: None,
        }
    }
}

fn read(path: &'static Path) -> Result<String, std::io::Error> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

/// Abstract syntax tree per file
pub struct Ast {
    /// File path
    pub path: Option<&'static Path>,
    /// Abstract syntax tree elements
    pub elements: Vec<AstElement>,
}

impl Ast {
    /// Instanciate a new `Ast` item
    pub fn new() -> Ast {
        Ast {
            path: None,
            elements: Vec::<AstElement>::new(),
        }
    }
    /// Internal method to feed the `Ast` item from `String` source code
    /// 
    /// Here is what it does:
    /// 
    /// [1] - Use the lexer based on Token enum
    /// 
    /// [2] - Collect all found tokens in a vector
    /// 
    /// [2] - Feed the AST (self) with element type and position (Range)
    fn feed_from_source(&mut self, source: String) {
        let lexer = super::Token::lexer(&source);
        let v: Vec<_> = lexer.spanned().collect();
        for elem in v {
            let mut ast_element = AstElement::new();
            ast_element.__type = elem.0;
            ast_element.position = Some(elem.1);
            self.elements.push(ast_element);
        }
    }
}

impl From<String> for Ast {
    /// Create a new `Ast` item from `String` source code
    fn from(source: String) -> Self {
        let mut ast = Ast::new();
        ast.feed_from_source(source);
        ast
    }
}

impl From<&'static Path> for Ast {
    /// Create a new `Ast` item from a `std::path::Path` value
    fn from(path: &'static Path) -> Self {
        let content = read(path);
        let mut ast = Ast::new();
        ast.path = Some(path);
        match content {
            Ok(st) => ast.feed_from_source(st),
            Err(error) => panic!("Problem opening the file: {:?}", error),
        }
        ast
    }
}

#[cfg(test)]
mod tests {
    use super::AstElement;

    #[test]
    fn it_works() {
        let el = AstElement::new();
        assert_eq!(el.position.is_none(), true);
    }
}
