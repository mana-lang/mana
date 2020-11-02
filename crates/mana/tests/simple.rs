use mana::{Ast, Token};

#[test]
fn simple_test() {
    let ast = Ast::from("let a = 1;".to_string());
    assert_eq!(ast.elements[0].__type, Token::KwLet);
    assert_eq!(ast.elements[1].__type, Token::Identifier);
    assert_eq!(ast.elements[2].__type, Token::Equal);
    assert_eq!(ast.elements[3].__type, Token::Number);
    assert_eq!(ast.elements[4].__type, Token::SemiColon);
}

#[test]
fn double_quoted_strings() {
    let ast = Ast::from(r#"let a = "Here is a \"double-quoted string\"";"#.to_string());
    assert_eq!(ast.elements[0].__type, Token::KwLet);
    assert_eq!(ast.elements[1].__type, Token::Identifier);
    assert_eq!(ast.elements[2].__type, Token::Equal);
    assert_eq!(ast.elements[3].__type, Token::QuotedString);
    assert_eq!(ast.elements[4].__type, Token::SemiColon);
}
