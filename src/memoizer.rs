use std::collections::HashMap;

pub struct Memoizer<T, E> {
    function: fn(T) -> E,
    values: HashMap<T, E>,
}
impl<T: std::cmp::Eq + std::hash::Hash + std::clone::Clone, E: std::clone::Clone> Memoizer<T, E> {
    pub fn new(function: fn(T) -> E) -> Memoizer<T, E> {
        Memoizer {
            function,
            values: HashMap::new(),
        }
    }
    pub fn run(&mut self, argument: T) -> E {
        match self.values.get(&argument) {
            Some(value) => value.to_owned(),
            None => {
                let new_value = (self.function)(argument.to_owned());
                self.values.insert(argument, new_value.to_owned());
                new_value
            }
        }
    }
}
