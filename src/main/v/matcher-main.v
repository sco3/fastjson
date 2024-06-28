

import mt

fn main() {
	result := mt.match_pattern_i('a\0'.bytes(), '*\0'.bytes())
	println(int(result))
}
