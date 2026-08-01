use std::fmt;

// 1. TOKEN STRUCT
#[derive(Clone, PartialEq, Eq)]
pub struct Token<T> {
    pub token_type: T,
}

impl<T> Token<T> {
    pub fn new(token_type: T) -> Self {
        Self { token_type }
    }
}

// 2. STR & REPR MIXIN EQUIVALENT
// Implementing Display (__str__) & routing Debug (__repr__) directly to it!
impl<T: fmt::Display> fmt::Display for Token<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<Token \"{}\">", self.token_type)
    }
}

impl<T: fmt::Display> fmt::Debug for Token<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

// 3. EVALUATE STRING
pub fn evaluate_string(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    let mut chars = input.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '\\' {
            match chars.next() {
                Some('n') => result.push('\n'),
                Some('t') => result.push('\t'),
                Some('r') => result.push('\r'),
                Some('\\') => result.push('\\'),
                Some('"') => result.push('"'),
                Some('\'') => result.push('\''),
                Some(escaped) => {
                    result.push('\\');
                    result.push(escaped);
                }
                None => result.push('\\'),
            }
        } else {
            result.push(ch);
        }
    }
    result
}
