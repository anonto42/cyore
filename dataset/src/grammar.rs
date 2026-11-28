// A tiny phrasing grammar. One line describes many sentences:
//
//   (a|b|c)   choose exactly one option
//   [x]       optional: with or without x
//   {slot}    a slot placeholder, kept as-is in the output
//
// Groups nest: `list [the] files [in (this|that) folder]`.
// `(show|list) [me] the files` expands to 2 × 2 = 4 sentences.
//
// Parsing is recursive descent: a *sequence* is a list of nodes; a node is text, a slot,
// a choice between sequences, or an optional sequence.

#[derive(Debug, Clone, PartialEq)]
enum Node {
    Text(String),
    Slot(String),
    Choice(Vec<Vec<Node>>),
    Optional(Vec<Node>),
}

struct Parser<'a> {
    chars: Vec<char>,
    pos: usize,
    line: &'a str,
}

impl Parser<'_> {
    fn error(&self, message: &str) -> String {
        format!(
            "grammar error at column {} in \"{}\": {message}",
            self.pos + 1,
            self.line
        )
    }

    /// Parse nodes until one of `stops` (not consumed) or the end of the line.
    fn sequence(&mut self, stops: &[char]) -> Result<Vec<Node>, String> {
        let mut nodes = Vec::new();
        let mut text = String::new();
        while let Some(&c) = self.chars.get(self.pos) {
            if stops.contains(&c) {
                break;
            }
            match c {
                '(' | '[' | '{' => {
                    if !text.is_empty() {
                        nodes.push(Node::Text(std::mem::take(&mut text)));
                    }
                    self.pos += 1;
                    nodes.push(match c {
                        '(' => self.choice()?,
                        '[' => self.optional()?,
                        _ => self.slot()?,
                    });
                }
                ')' | ']' | '}' | '|' => return Err(self.error(&format!("unexpected '{c}'"))),
                _ => {
                    text.push(c);
                    self.pos += 1;
                }
            }
        }
        if !text.is_empty() {
            nodes.push(Node::Text(text));
        }
        Ok(nodes)
    }

    /// After '(': options separated by '|' up to ')'.
    fn choice(&mut self) -> Result<Node, String> {
        let mut options = Vec::new();
        loop {
            let option = self.sequence(&['|', ')'])?;
            if is_blank(&option) {
                return Err(self.error("empty option in (...); use [...] for optional text"));
            }
            options.push(option);
            match self.chars.get(self.pos) {
                Some('|') => self.pos += 1,
                Some(')') => {
                    self.pos += 1;
                    return Ok(Node::Choice(options));
                }
                _ => return Err(self.error("missing ')'")),
            }
        }
    }

    /// After '[': a sequence up to ']'.
    fn optional(&mut self) -> Result<Node, String> {
        let inner = self.sequence(&[']'])?;
        if self.chars.get(self.pos) != Some(&']') {
            return Err(self.error("missing ']'"));
        }
        self.pos += 1;
        if is_blank(&inner) {
            return Err(self.error("empty [...]"));
        }
        Ok(Node::Optional(inner))
    }

    /// After '{': a slot name up to '}'.
    fn slot(&mut self) -> Result<Node, String> {
        let start = self.pos;
        while let Some(&c) = self.chars.get(self.pos) {
            if c == '}' {
                let name: String = self.chars[start..self.pos].iter().collect();
                self.pos += 1;
                if name.is_empty() || !name.chars().all(|c| c.is_ascii_lowercase() || c == '_') {
                    return Err(self.error(&format!("bad slot name '{name}'")));
                }
                return Ok(Node::Slot(name));
            }
            self.pos += 1;
        }
        Err(self.error("missing '}'"))
    }
}

fn is_blank(nodes: &[Node]) -> bool {
    nodes
        .iter()
        .all(|n| matches!(n, Node::Text(t) if t.trim().is_empty()))
}

/// Every string a sequence can produce (raw, before whitespace clean-up).
fn expand_sequence(nodes: &[Node]) -> Vec<String> {
    let mut results = vec![String::new()];
    for node in nodes {
        let parts: Vec<String> = match node {
            Node::Text(t) => vec![t.clone()],
            Node::Slot(name) => vec![format!("{{{name}}}")],
            Node::Choice(options) => options.iter().flat_map(|o| expand_sequence(o)).collect(),
            Node::Optional(inner) => {
                let mut v = vec![String::new()];
                v.extend(expand_sequence(inner));
                v
            }
        };
        // Cartesian product: every result so far, followed by every part.
        results = results
            .iter()
            .flat_map(|prefix| parts.iter().map(move |p| format!("{prefix}{p}")))
            .collect();
    }
    results
}

/// Parse a grammar line and return every sentence it describes, with whitespace
/// normalised (single spaces, no leading/trailing space) and duplicates removed,
/// in a fixed order.
pub fn expand(line: &str) -> Result<Vec<String>, String> {
    let mut parser = Parser {
        chars: line.chars().collect(),
        pos: 0,
        line,
    };
    let nodes = parser.sequence(&[])?;
    let mut out: Vec<String> = Vec::new();
    for raw in expand_sequence(&nodes) {
        let clean = raw.split_whitespace().collect::<Vec<_>>().join(" ");
        if !clean.is_empty() && !out.contains(&clean) {
            out.push(clean);
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_design_example_has_24_phrasings() {
        let all = expand("(show|list|display) [me] [all] the files [in {dir}]").unwrap();
        assert_eq!(all.len(), 24);
        assert!(all.contains(&"show the files".to_string()));
        assert!(all.contains(&"display me all the files in {dir}".to_string()));
    }

    #[test]
    fn test_nested_groups() {
        let all = expand("list files [in (this|that) folder]").unwrap();
        assert_eq!(
            all,
            [
                "list files",
                "list files in this folder",
                "list files in that folder"
            ]
        );
    }

    #[test]
    fn test_attached_suffix() {
        // No space between ')' and '[' joins the words.
        let all = expand("the (computer|machine)['s] name").unwrap();
        assert!(all.contains(&"the machine's name".to_string()));
        assert!(all.contains(&"the computer name".to_string()));
    }

    #[test]
    fn test_plain_text() {
        assert_eq!(expand("pwd").unwrap(), ["pwd"]);
    }

    #[test]
    fn test_unbalanced_brackets() {
        assert!(
            expand("(show|list files")
                .unwrap_err()
                .contains("missing ')'")
        );
        assert!(expand("[me files").unwrap_err().contains("missing ']'"));
        assert!(expand("show me)").unwrap_err().contains("unexpected ')'"));
        assert!(expand("in {dir").unwrap_err().contains("missing '}'"));
    }

    #[test]
    fn test_empty_option_rejected() {
        assert!(expand("(show|)").unwrap_err().contains("empty option"));
        assert!(expand("show [ ]").unwrap_err().contains("empty"));
    }

    #[test]
    fn test_bad_slot_name() {
        assert!(expand("in {Dir}").unwrap_err().contains("bad slot name"));
    }
}
