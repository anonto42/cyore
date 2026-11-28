// The tokenization rule shared by the dataset and (later) the model and the agent:
//   1. split on whitespace
//   2. strip trailing `?`, `!` and `,` from every token
//   3. strip a trailing `.` from the last token (a sentence-ending full stop)
//   4. drop tokens that end up empty
// File names keep their dots (`notes.txt`) because only the *last* token loses a final `.`.

const TRAILING: [char; 3] = ['?', '!', ','];

pub fn tokenize(text: &str) -> Vec<String> {
    let words: Vec<&str> = text.split_whitespace().collect();
    let last = words.len().saturating_sub(1);
    words
        .iter()
        .enumerate()
        .map(|(i, word)| {
            let mut token = word.trim_end_matches(TRAILING);
            if i == last {
                // "please." or "Downloads?." — strip the full stop and anything before it.
                token = token.trim_end_matches(['.', '?', '!', ',']);
            }
            token.to_string()
        })
        .filter(|token| !token.is_empty())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn toks(text: &str) -> Vec<String> {
        tokenize(text)
    }

    #[test]
    fn test_question_mark_stripped() {
        assert_eq!(
            toks("can you list the files in Downloads?"),
            ["can", "you", "list", "the", "files", "in", "Downloads"]
        );
    }

    #[test]
    fn test_file_extension_kept() {
        assert_eq!(
            toks("show me notes.txt please."),
            ["show", "me", "notes.txt", "please"]
        );
    }

    #[test]
    fn test_comma_stripped_mid_sentence() {
        assert_eq!(
            toks("hey, what is my ip"),
            ["hey", "what", "is", "my", "ip"]
        );
    }

    #[test]
    fn test_empty_and_spaces() {
        assert!(toks("").is_empty());
        assert!(toks("   ").is_empty());
        assert_eq!(toks("  ls    -la  "), ["ls", "-la"]);
    }

    #[test]
    fn test_lone_punctuation_dropped() {
        assert_eq!(toks("disk space ?"), ["disk", "space"]);
    }

    #[test]
    fn test_case_preserved() {
        // The model may lowercase for its vocabulary, but slot values must keep their case.
        assert_eq!(toks("open README.md"), ["open", "README.md"]);
    }
}
