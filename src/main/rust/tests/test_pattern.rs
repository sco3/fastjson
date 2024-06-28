#[cfg(test)]
mod tests {
    use super::*;
    const NUM: i32 = 10_000_000;

    fn bench(c: &mut Criterion) {
        for _i in 1..NUM {
            test_simple();
        }
    }

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

