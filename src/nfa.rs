use crate::RegexAST;
use std::collections::HashSet;

pub enum NFATransition {
    Chr(char, usize),
    Any(usize),
    Eps(usize),
}

pub struct NFAMachine {
    digraph: Vec<Vec<NFATransition>>,
}

impl NFAMachine {
    // a new nfa constructed from regex ast
    pub fn new(ast: Box<RegexAST>) -> Self {
        let mut this = Self { digraph: vec![vec![], vec![]] };
        this.add(ast, 0, 1);
        return this;
    }
    // move along a transition
    pub fn mov(&self, state: HashSet<usize>, c: char) -> HashSet<usize> {
        use NFATransition::*;
        let mut new_state = HashSet::new();
        for s in state {
            for t in &self.digraph[s] {
                match *t {
                    Chr(_c, _s) => {
                        if _c == c { new_state.insert(_s); }
                    },
                    Any(_s) => {
                        new_state.insert(_s);
                    },
                    Eps(_s) => {
                        new_state.insert(_s);
                        new_state.insert(s);
                    }
                }
            }
        }
        return new_state;
    }
    // plot itself into a graphviz dot string
    // you can use graphviz dot command to visualize this nfa
    pub fn dot(&self) -> String {
        use NFATransition as T;
        let mut result = String::new() + "digraph nfa {\n";
        result += "\t0 [shape=doublecircle];\n";
        result += "\t1 [shape=doublecircle];\n";
        for (i, v) in self.digraph.iter().enumerate() {
            if i != 0 && i != 1 {
                result += &format!("\t{i} [shape=circle];\n");
            }
            for t in v {
                let (attr, j) = match t {
                    &T::Chr(c, j) => {
                        let e = match c { c if c.is_whitespace() => "\\", _ => "" };
                        let s = format!("[shape=circle, label=\"{e}{}\"]", c.escape_default());
                        (s, j)
                    },
                    &T::Any(j) => (format!("[shape=circle, label=\"<α>\"]"), j),
                    &T::Eps(j) => (format!("[shape=circle, label=\"<ε>\"]"), j),
                };
                result += &format!("\t{i} -> {j} {attr};\n");
            }
        }
        return result + "}";
    }
    // add an ast to current NFA Machine
    fn add(&mut self, ast: Box<RegexAST>, start: usize, end: usize) {
        use {RegexAST as AST, NFATransition as T};
        match *ast {
            AST::Chr(x) => {
                self.digraph[start].push(T::Chr(x, end))
            }
            AST::Any => {
                self.digraph[start].push(T::Any(end))   
            }
            AST::Join(x, y) => {
                self.add(x, start, end);
                self.add(y, start, end);
            }
            AST::Cat(x, y) => {
                self.digraph.push(Vec::new());
                let (start_x, end_x) = (start, self.digraph.len()-1);
                let (start_y, end_y) = (self.digraph.len()-1, end);
                self.add(x, start_x, end_x);
                self.add(y, start_y, end_y);
            }
            AST::Star(x) => {
                self.add(x, start, start);
                if start != end {
                    self.digraph[start].push(T::Eps(end));
                }
            }
            AST::Empty => {
                if start != end {
                    self.digraph[start].push(T::Eps(end))
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn plot() {
        let nfa = NFAMachine::new(RegexAST::new("//|(.*)\n"));
        std::fs::write("plot.dot", nfa.dot()).unwrap();
    }
}