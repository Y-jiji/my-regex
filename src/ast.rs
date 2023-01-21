const T: bool = true;
const F: bool = false;

/// definition of regex ast
pub enum RegexAST {
    // language concatenation
    Cat(Box<RegexAST>, Box<RegexAST>),
    // a char
    Raw(char),
    // any char
    Any,
    // language join
    Join(Box<RegexAST>, Box<RegexAST>),
    // language star
    Star(Box<RegexAST>),
    // empty
    Empty,
}

// helper module, construct regex ast with binary operations
mod helper {
    use super::RegexAST;
    impl<T: Into<Self>> std::ops::BitOr<T> for RegexAST {
        type Output = Self;
        fn bitor(self, rhs: T) -> Self {
            match (self, rhs.into()) {
                (Self::Empty, Self::Empty) => Self::Empty,
                (Self::Empty, rhs) => rhs,
                (lhs, Self::Empty) => lhs,
                (lhs, rhs) => Self::Join(Box::new(lhs), Box::new(rhs))
            }
        }
    }
    impl<T: Into<Self>> std::ops::Add<T> for RegexAST {
        type Output = Self;
        fn add(self, rhs: T) -> Self {
            match (self, rhs.into()) {
                (Self::Empty, Self::Empty) => Self::Empty,
                (Self::Empty, rhs) => rhs,
                (lhs, Self::Empty) => lhs,
                (lhs, rhs) => Self::Cat(Box::new(lhs), Box::new(rhs))
            }
        }
    }
    impl From<char> for RegexAST {
        fn from(value: char) -> Self {
            Self::Raw(value)
        }
    }
    impl std::fmt::Display for RegexAST {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            use RegexAST::*;
            match self {
                Any => f.write_str(".")?,
                Cat(a, b) => write!(f, "({a})({b})")?,
                Raw(a) => write!(f, "{a}")?,
                Join(a, b) => write!(f, "({a}|{b})")?,
                Star(a) => write!(f, "({a}*)")?,
                Empty => write!(f, "ε")?,
            };
            Ok(())
        }
    }
}

impl RegexAST {
    fn escape_mode<const RVT: bool>(input: &mut std::str::Chars<'_>) -> RegexAST {
        // nothing is recursively embedded in escape mode
        // parsing behaviours are determined only by '{' and '}' 
        let mut current = RegexAST::Empty;
        let mut count = 1usize;
        while let Some(c) = input.next() {
            count = match c { '{' => count + 1, '}' => count - 1, _ => count };
            if count == 0 { break; }
            if RVT { current = current | c }
            else { current = current + c }
        }
        return current;
    }
    // <Revert Join and Cat, In open bracket, In Squre open bracket>
    fn normal_mode<const RVT: bool, const OPEN: bool, const SQBR: bool>(input: &mut std::str::Chars<'_>) -> RegexAST {
        let closing = if SQBR { ']' } else { ')' };
        let mut current = Self::Empty;
        while let Some(c) = input.next() {
            current = match c {
                '{' => {
                    let x = Self::escape_mode::<RVT>(input);
                    if RVT { current | x } else { current + x }
                }
                '(' => {
                    let x = Self::normal_mode::<RVT, T, F>(input);
                    if RVT { current | x } else { current + x }
                }
                '|' => {
                    let x = Self::normal_mode::<RVT, T, SQBR>(input);
                    if RVT { current + x } else { current | x }
                }
                '[' if RVT => {
                    let x = Self::normal_mode::<F, T, T>(input);
                    if RVT { current | x } else { current + x }
                }
                '[' if !RVT => {
                    // Self::normal_mode::<!RVT, T, T>(input);
                    let x = Self::normal_mode::<T, T, T>(input);
                    if RVT { current | x } else { current + x }
                }
                '*' => {Self::Star(Box::new(current))}
                c if c == closing && OPEN => { break }
                '.' => 
                    if RVT { current | Self::Any } else { current + Self::Any }
                c => 
                    if RVT { current | c } else { current + c }
            }
        }
        return current;
    }
    pub fn new(input: &str) -> Box<RegexAST> {
        let mut input = input.chars();
        Box::new(Self::normal_mode::<F, F, F>(&mut input))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_regex() {
        println!("{}", RegexAST::new("(ef*a)|({ncdf\\*}nx*)|p*"));
        println!("{}", RegexAST::new("[ab[ce*]d]"));
        println!("{}", RegexAST::new("[abced]"));
    }
}