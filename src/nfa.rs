use std::collections::HashSet;
use crate::RegexAST;

pub struct NFAMachine {
    digraph: Vec<Vec<(char, usize)>>,
}



impl NFAMachine {
    fn new(ast: Box<RegexAST>) -> Self {
        todo!()
    }
    fn transition(state: HashSet<usize>, transition: char) -> HashSet<usize> {
        todo!()
    }
}