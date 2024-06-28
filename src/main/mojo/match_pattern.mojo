
from testing import assert_true

fn match_pattern(str: String, pattern: String) -> Int32:
    if len(pattern) == 0:
        return 1 if len(str) == 0 else 0

    if pattern[0] == '*':
        return (match_pattern(str, pattern[1:]) == 1) or \
               (len(str) > 0 and match_pattern(str[1:], pattern) == 1)

    if len(str) > 0 and pattern[0] == str[0]:
        return match_pattern(str[1:], pattern[1:])

    return 0

# Test cases
fn main() raises:
	var n:Int = 10_000
	for i in range(n):
		assert_true(match_pattern("example", "ex*ple") == 1)
		assert_true(match_pattern("example", "ex*plez") == 0)
		assert_true(match_pattern("example", "ex*") == 1)
		assert_true(match_pattern("example", "*le") == 1)
		assert_true(match_pattern("example", "*xam*le") == 1)
		assert_true(match_pattern("example", "*xam*") == 1)
		assert_true(match_pattern("example", "*xam*z") == 0)
		assert_true(match_pattern("", "*") == 1)
		assert_true(match_pattern("example", "") == 0)
    
	print (n)


