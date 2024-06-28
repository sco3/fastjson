use super::*;
use criterion::{criterion_group, criterion_main, Criterion};

const NUM: i32 = 10_000_000;

fn bench(c: &mut Criterion) {
    for _i in 1..NUM {
        test_simple();
    }
}

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

criterion_group!(benches, bench,);
criterion_main!(benches);
