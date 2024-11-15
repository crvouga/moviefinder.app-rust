pub fn is_valid(selector: &str) -> bool {
    if selector.is_empty() {
        return false;
    }

    let mut last_char = None;
    let mut inside_attribute = false;

    for ch in selector.chars() {
        if !ch.is_alphanumeric()
            && ch != '.'
            && ch != '#'
            && ch != '-'
            && ch != '_'
            && ch != ' '
            && ch != '>'
            && ch != '+'
            && ch != '*'
            && ch != ':'
            && ch != '['
            && ch != ']'
            && ch != '='
            && ch != '"'
            && ch != '\''
            && ch != '('
            && ch != ')'
        {
            return false;
        }

        if let Some(last) = last_char {
            if (last == '.' && ch == '.') || (last == '#' && ch == '#') {
                return false;
            }
        }

        if ch == '[' {
            if inside_attribute {
                return false;
            }
            inside_attribute = true;
        } else if ch == ']' {
            if !inside_attribute {
                return false;
            }
            inside_attribute = false;
        }

        last_char = Some(ch);
    }

    if inside_attribute {
        return false;
    }

    if let Some(last) = last_char {
        if last == '.' || last == '#' || last == ' ' || last == '>' || last == '+' {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::is_valid;

    #[test]
    fn test_valid_selectors() {
        assert!(is_valid("div"));
        assert!(is_valid(".classname"));
        assert!(is_valid("#id"));
        assert!(is_valid("div.classname#id"));
        assert!(is_valid("div > p"));
        assert!(is_valid("ul > li + li"));
        assert!(is_valid("div:nth-child(2)"));
        assert!(is_valid("[type='text']"));
        assert!(is_valid("input[type=\"checkbox\"]"));
    }

    #[test]
    fn test_invalid_selectors() {
        assert!(!is_valid(""));
        assert!(!is_valid("div..classname"));
        assert!(!is_valid("#id##another"));
        assert!(!is_valid("div#id."));
        assert!(!is_valid("div>"));
        assert!(!is_valid("div#id["));
        // todo
        // assert!(!is_valid("div[class='value]"));
        // todo
        // assert!(!is_valid("div # id"));
        assert!(!is_valid("div#id@keyframes"));
    }

    #[test]
    fn test_edge_cases() {
        assert!(is_valid("a"));
        assert!(is_valid("*"));
        assert!(is_valid(".a"));
        assert!(is_valid("#1"));
        assert!(is_valid("[data-test='value']"));
        assert!(!is_valid(".."));
        assert!(!is_valid("###"));
    }
}
