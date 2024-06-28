

module mt

pub fn match_pattern_i(str []u8, pattern []u8) int {
	result := match_pattern(str, pattern)
	return int(result)
}

fn match_pattern(str []u8, pattern []u8) bool {
	println('str: ${rune(str[0])} pattern: ${rune(pattern[0])}')
	if pattern[0] == 0 {
		return str[0] == 0
	}
	if pattern[0] == `*` {
		return match_pattern(str, pattern[1..]) //
			|| (str[0] > 0 && match_pattern(str[1..], pattern))
	}

	if str[0] > 0 && pattern[0] == str[0] {
		return match_pattern(str[1..], pattern[1..])
	}

	return false
}

fn main() {
	result := match_pattern('a\0'.bytes(), '*\0'.bytes())
	println(int(result))
}
