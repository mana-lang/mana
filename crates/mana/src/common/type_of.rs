/// Print type of value
pub fn type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
