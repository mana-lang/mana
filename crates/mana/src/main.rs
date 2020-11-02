use mana::{start_cli, Ast};
use std::path::Path;

/// Print type of input value
fn _print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

/// Proof-of-concept home-made extensible Rust AST parser
fn _demo() {
    let path = Path::new("./crates/mana/src/lib.rs");
    let ast = Ast::from(path);
    for elem in ast.elements {
        println!("{:?} - {:?}", elem.__type, elem.position);
    }
}

fn _demo_random() {
    let a: f64 = mana::rand::f64();
    println!("{:?}", a);
    let b: f64 = mana::rand::f64();
    println!("{:?}", b);
    let c: f64 = mana::rand::f64();
    println!("{:?}", c);
}

fn main() {
    //_demo();
    //_demo_random();
    start_cli()
}
