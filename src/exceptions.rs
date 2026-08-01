use std::error::Error;
use std::fmt;

// --- DUMMY STRUCTS (Replace with your actual AST/Expression types) ---
#[derive(Debug, Clone)]
pub struct Expression {
    pub name: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Node;

impl Node {
    pub fn prettily(&self, _error: &Node) -> String {
        "Tree view...".to_string()
    }
}

// ==========================================
// 1. INPUT TEXT ENUM (Handles str vs tokens)
// ==========================================
#[derive(Debug, Clone)]
pub enum InputText {
    String(String),
    Tokens(Vec<String>),
}

impl InputText {
    pub fn line(&self, pos: usize) -> Option<usize> {
        match self {
            InputText::Tokens(_) => None,
            InputText::String(text) => {
                let slice = &text[..pos.min(text.len())];
                Some(slice.matches('\n').count() + 1)
            }
        }
    }

    pub fn column(&self, pos: usize) -> usize {
        match self {
            InputText::Tokens(_) => pos + 1,
            InputText::String(text) => {
                let slice = &text[..pos.min(text.len())];
                match slice.rfind('\n') {
                    Some(last_newline) => pos - last_newline,
                    None => pos + 1,
                }
            }
        }
    }

    pub fn window(&self, pos: usize) -> &str {
        match self {
            InputText::String(text) => {
                let start = pos.min(text.len());
                let end = (pos + 20).min(text.len());
                &text[start..end]
            }
            InputText::Tokens(_) => "",
        }
    }
}

// ==========================================
// 2. PARSIMONIOUS ERROR ENUM
// ==========================================
#[derive(Debug)]
pub enum ParsimoniousError {
    Parse {
        text: InputText,
        pos: usize,
        expr: Option<Expression>,
    },
    LeftRecursion {
        text: InputText,
        pos: usize,
        expr: Option<Expression>,
    },
    IncompleteParse {
        text: InputText,
        pos: usize,
        expr: Option<Expression>,
    },
    Visitation {
        exc_class_name: String,
        message: String,
        node: Node,
    },
    BadGrammar(String),
    UndefinedLabel(String),
}

// ==========================================
// 3. DISPLAY TRAIT (__str__ & __repr__)
// ==========================================
impl fmt::Display for ParsimoniousError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParsimoniousError::Parse { text, pos, expr } => {
                let rule_name = get_rule_name(expr);
                let line_str = text.line(*pos).map_or("N/A".to_string(), |l| l.to_string());
                write!(
                    f,
                    "Rule {} didn't match at '{}' (line {}, column {}).",
                    rule_name,
                    text.window(*pos),
                    line_str,
                    text.column(*pos)
                )
            }

            ParsimoniousError::LeftRecursion { text, pos, expr } => {
                let rule_name = get_rule_name(expr);
                let line_str = text.line(*pos).map_or("N/A".to_string(), |l| l.to_string());
                write!(
                    f,
                    "Left recursion in rule '{}' at '{}' (line {}, column {}).\n\n\
                    Parsimonious is a packrat parser, so it can't handle left recursion.\n\
                    See https://en.wikipedia.org/wiki/Parsing_expression_grammar#Indirect_left_recursion\n\
                    for how to rewrite your grammar into a rule that does not use left-recursion.",
                    rule_name,
                    text.window(*pos),
                    line_str,
                    text.column(*pos)
                )
            }

            ParsimoniousError::IncompleteParse { text, pos, expr } => {
                let name = expr.as_ref().and_then(|e| e.name.as_deref()).unwrap_or("unknown");
                let line_str = text.line(*pos).map_or("N/A".to_string(), |l| l.to_string());
                write!(
                    f,
                    "Rule '{}' matched in its entirety, but it didn't consume all the text. \
                    The non-matching portion of the text begins with '{}' (line {}, column {}).",
                    name,
                    text.window(*pos),
                    line_str,
                    text.column(*pos)
                )
            }

            ParsimoniousError::Visitation { exc_class_name, message, node } => {
                write!(
                    f,
                    "{}: {}\n\nParse tree:\n{}",
                    exc_class_name,
                    message,
                    node.prettily(node)
                )
            }

            ParsimoniousError::BadGrammar(msg) => write!(f, "{}", msg),

            ParsimoniousError::UndefinedLabel(label) => {
                write!(f, "The label \"{}\" was never defined.", label)
            }
        }
    }
}

impl Error for ParsimoniousError {}

// Helper function to extract expression string
fn get_rule_name(expr: &Option<Expression>) -> String {
    match expr {
        Some(e) => e.name.clone().map_or_else(|| format!("{:?}", e), |n| format!("'{}'", n)),
        None => "None".to_string(),
    }
}
