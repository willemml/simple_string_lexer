fn get_char_before(in_str: &str, pos: usize) -> Option<char> {
    if pos == 0 {
        None
    } else {
        Some(in_str.chars().collect::<Vec<char>>()[pos - 1])
    }
}

fn is_escaped(in_str: &str, mut pos: usize) -> bool {
    let mut escaped = false;
    while let Some(c) = get_char_before(in_str, pos) {
        if c == '\\' {
            escaped = !escaped;
        } else {
            return escaped;
        }
        pos -= 1;
    }
    return escaped;
}

/// Splits `string` by whitespace and by quoted groups (both single
/// and double quotes), also supports escaping quotes using `\` (note
/// that it will remove any backslashes used to escape things, so
/// escape any backslashes you want left in.)
pub fn split_str(string: &str) -> Vec<String> {
    let mut last: Option<char> = None;
    let mut split_locs = Vec::<usize>::new();
    let mut removed = 0;
    let mut bs_removed = 0;
    let mut to_remove = Vec::<usize>::new();
    let filtered: Vec<char> = string
        .chars()
        .enumerate()
        .filter(|(i, c)| {
            if !is_escaped(string, *i) {
                let mut remove_one = || {
                    split_locs.push(*i - removed);
                    removed += 1;
                    false
                };
                if (last == Some('\'') && *c == '\'') || (last == Some('"') && *c == '"') {
                    last = None;
                    remove_one()
                } else if (*c == '\'' || *c == '"') && last == None {
                    last = Some(*c);
                    remove_one()
                } else if c.is_whitespace() && last == None {
                    remove_one()
                } else {
                    true
                }
            } else {
                to_remove.push(*i - removed - 1 + bs_removed);
                bs_removed += 1;
                removed += 1;
                true
            }
        })
        .map(|(_, c)| c)
        .collect();

    let string = filtered
        .iter()
        .enumerate()
        .filter_map(|(i, c)| {
            if to_remove.contains(&i) {
                None
            } else {
                Some(c)
            }
        })
        .collect::<String>();

    let mut string = string.as_str();

    let mut parts = Vec::<String>::new();

    let mut iter = split_locs.iter();

    let mut offset = 0;

    while let Some(next) = iter.next() {
        let split = string.split_at(*next - offset);
        if !split.0.is_empty() {
            parts.push(split.0.to_string());
        }
        string = split.1;
        offset = *next;
    }

    if !string.trim().is_empty() {
        parts.push(string.trim().to_string());
    }

    return parts;
}

#[cfg(test)]
mod test {
    use super::{get_char_before, is_escaped, split_str};

    macro_rules! lex_test {
        ($name:ident, $input:expr => $output:expr) => {
            #[test]
            fn $name() {
                assert_eq!(&split_str($input), $output);
            }
        };
    }

    lex_test!(simple_lex,r#"ab cd "ef gh ij" kl mn"# => &["ab", "cd", "ef gh ij", "kl", "mn"]);
    lex_test!(escaped_outside_lex,r#"ab \" cd "ef gh ij" kl \"wx yz\" mn"# => &["ab", r#"""#, "cd", "ef gh ij", "kl", r#""wx"#, r#"yz""#, "mn"]);
    lex_test!(escaped_inside_lex,r#"ab cd "ef \"gh xy\" ij" kl mn"# => &["ab", "cd", r#"ef "gh xy" ij"#, "kl", "mn"]);
    lex_test!(single_inside_double_lex,r#"ab cd "ef 'gh xy' ij" kl mn"# => &["ab", "cd", "ef 'gh xy' ij", "kl", "mn"]);

    #[test]
    fn get_before() {
        assert_eq!(get_char_before("abc", 2), Some('b'));
        assert_eq!(get_char_before("abc", 1), Some('a'));
        assert_eq!(get_char_before("abc", 0), None);
    }

    #[test]
    fn escaping() {
        assert!(!is_escaped(r#"\"#, 0));
        assert!(!is_escaped(r#"\\"#, 0));
        assert!(is_escaped(r#"\\"#, 1));
        assert!(is_escaped(r#"\\\""#, 3));
    }
}
