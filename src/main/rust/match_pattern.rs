use glob::Pattern;

fn fn_match_pattern(text: &str, pattern: &str) -> bool {
    let pattern = Pattern::new(pattern).unwrap();
    pattern.matches(text)
}

fn match_pattern(str: &str, pattern: &str) -> bool {
    if pattern.is_empty() {
        return str.is_empty();
    }

    let str_bytes = str.as_bytes();
    let pattern_bytes = pattern.as_bytes();

    if pattern_bytes[0] == b'*' {
        return match_pattern(str, &pattern[1..])
            || (!str.is_empty() && match_pattern(&str[1..], pattern));
    }

    if !str.is_empty() && (pattern_bytes[0] == str_bytes[0]) {
        return match_pattern(&str[1..], &pattern[1..]);
    }

    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        assert!(match_pattern("", ""));
        assert!(!match_pattern("hello", ""));
        assert!(!match_pattern("", "a"));
        assert!(match_pattern("", "*"));
        assert!(match_pattern("hello", "hello"));
        assert!(!match_pattern("hello", "world"));
        assert!(match_pattern("hello", "h*o"));
        assert!(match_pattern("hello", "*o"));
        assert!(match_pattern("hello", "h*"));
        assert!(match_pattern("hello", "*"));
        assert!(match_pattern("hello", "h*l*o"));
        assert!(match_pattern("hello", "h*e*o"));
        assert!(match_pattern("hello", "*e*"));
        assert!(match_pattern("hello", "h*l*l*o"));
        assert!(match_pattern("hello", "*ello"));
        assert!(match_pattern("hello", "*hello"));
        assert!(match_pattern("hello", "hell*"));
        assert!(match_pattern("hello", "he*"));
        assert!(!match_pattern("hello", "hallo"));
        assert!(!match_pattern("hello", "helxo"));
        assert!(match_pattern("abracadabra", "a*a*a"));
        assert!(match_pattern("mississippi", "m*i*i*i"));
        assert!(match_pattern("hello world", "h*o w*d"));
    }

    #[test]
    fn test_fn() {
        assert!(fn_match_pattern("", ""));
        assert!(!fn_match_pattern("hello", ""));
        assert!(!fn_match_pattern("", "a"));
        assert!(fn_match_pattern("", "*"));
        assert!(fn_match_pattern("hello", "hello"));
        assert!(!fn_match_pattern("hello", "world"));
        assert!(fn_match_pattern("hello", "h*o"));
        assert!(fn_match_pattern("hello", "*o"));
        assert!(fn_match_pattern("hello", "h*"));
        assert!(fn_match_pattern("hello", "*"));
        assert!(fn_match_pattern("hello", "h*l*o"));
        assert!(fn_match_pattern("hello", "h*e*o"));
        assert!(fn_match_pattern("hello", "*e*"));
        assert!(fn_match_pattern("hello", "h*l*l*o"));
        assert!(fn_match_pattern("hello", "*ello"));
        assert!(fn_match_pattern("hello", "*hello"));
        assert!(fn_match_pattern("hello", "hell*"));
        assert!(fn_match_pattern("hello", "he*"));
        assert!(!fn_match_pattern("hello", "hallo"));
        assert!(!fn_match_pattern("hello", "helxo"));
        assert!(fn_match_pattern("abracadabra", "a*a*a"));
        assert!(fn_match_pattern("mississippi", "m*i*i*i"));
        assert!(fn_match_pattern("hello world", "h*o w*d"));
    }
}

fn main() {
    let str = "hello";
    let pattern = "h*o";
    println!(
        "Does '{}' match '{}'? {}",
        str,
        pattern,
        match_pattern(str, pattern)
    );
    println!(
        "Does '{}' match '{}'? {}",
        str,
        pattern,
        fn_match_pattern(str, pattern)
    );
}

