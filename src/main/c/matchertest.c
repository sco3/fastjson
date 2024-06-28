#include <fnmatch.h>
#include <assert.h>
#include <stdio.h>
#include "matcher.h"
#include <time.h>

#define N (10*1000*1000)

void test_match_one(char *line, char *pattern) {
	clock_t begin = clock();
	for (int i = 0; i < N; i++) {
		assert(match_one(line, pattern));
	}
	clock_t end = clock();
	double time_spent = (double) (end - begin) / CLOCKS_PER_SEC;
	printf("Time match_one: %d ms\n", (int) (1000 * time_spent));
}
void test_match_pattern(char *line, char *pattern) {
	clock_t begin = clock();
	for (int i = 0; i < N; i++) {
		assert(match_pattern(line, pattern));
	}
	clock_t end = clock();
	double time_spent = (double) (end - begin) / CLOCKS_PER_SEC;
	printf("Time match_pattern: %d ms\n", (int) (1000 * time_spent));
}

void test_simple() {
	clock_t begin = clock();
	for (int i = 0; i < N; i++) {
		assert(match_pattern("", ""));
		assert(!match_pattern("hello", ""));
		assert(!match_pattern("", "a"));
		assert(match_pattern("", "*"));
		assert(match_pattern("hello", "hello"));
		assert(!match_pattern("hello", "world"));
		assert(match_pattern("hello", "h*o"));
		assert(match_pattern("hello", "*o"));
		assert(match_pattern("hello", "h*"));
		assert(match_pattern("hello", "*"));
		assert(match_pattern("hello", "h*l*o"));
		assert(match_pattern("hello", "h*e*o"));
		assert(match_pattern("hello", "*e*"));
		assert(match_pattern("hello", "h*l*l*o"));
		assert(match_pattern("hello", "*ello"));
		assert(match_pattern("hello", "*hello"));
		assert(match_pattern("hello", "hell*"));
		assert(match_pattern("hello", "he*"));
		assert(!match_pattern("hello", "hallo"));
		assert(!match_pattern("hello", "helxo"));
		assert(match_pattern("abracadabra", "a*a*a"));
		assert(match_pattern("mississippi", "m*i*i*i"));
		assert(match_pattern("hello world", "h*o w*d"));
	}

	clock_t end = clock();
	double time_spent = (double) (end - begin) / CLOCKS_PER_SEC;
	printf("Time match_pattern: %d ms\n", (int) (1000 * time_spent));

}

int main() {
	// matcher from libc

	char *p[] = { "a*", "b*" };
	int result = glibc_matcher("asdf", p, 2);
	assert(result == 1);

	char *p1[] = { "*.txt" };
	assert(glibc_matcher("file.txt", p1, 1) == 1);

	char *p2[] = { "*.txt" };
	assert(glibc_matcher("document.docx", p2, 1) == 0);

	char *p3[] = { "image.*" };
	assert(glibc_matcher("image.jpg", p3, 1) == 1);

	char *p4[] = { "" };
	assert(glibc_matcher("any_file.txt", p4, 1) == 0);
	char *p5[] = { "file\\[1\\].txt" };
	assert(glibc_matcher("file[1].txt", p5, 1) == 1);
	assert(glibc_matcher("File.TXT", (char*[] ) { "*.txt" }, 1) == 0);

	assert(match_one("aa", "*aa*") == 1);

	test_match_one("foo bar foo", "fo* bar *foo");
	test_match_pattern("foo bar foo", "fo* bar *foo");
	test_simple();
}
