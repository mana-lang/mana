use std::path::Path;
use mana::{Ast, start_cli};

/// Print type of input value
fn _print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

/// Proof-of-concept home-made extensible Rust AST parser
fn demo() {
    let path = Path::new("./crates/mana/src/lib.rs");
    let ast = Ast::from(path);
    for elem in ast.elements {
        println!("{:?} - {:?}", elem.__type, elem.position);
    }
}

fn main() {
    demo();
    start_cli()
}