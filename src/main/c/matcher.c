#include <fnmatch.h>
#include "matcher.h"
#include <string.h>
#include <stdio.h>

// the matcher from standard linux glibc (libc.so.6)
extern int glibc_match_pattern(char *line, char *pattern) {
	int result = NOT_FOUND;
	int exit_code = fnmatch(pattern, line, 0);
	if (exit_code == 0) {
		result = FOUND;
	}
	return result;
}

// the matcher from standard linux glibc (libc.so.6)
extern int glibc_matcher(char *line, char *patterns[], int length) {
	int result = NOT_FOUND;
	for (int i = 0; i < length; i++) {
		char *pattern = patterns[i];
		int exit_code = fnmatch(pattern, line, 0);
		if (exit_code == 0) {
			result = FOUND;
			break;
		}
	}
	return result;
}

// fast version with asterisk wildcard only (*)
// str     - string to match with pattern
// pattern - pattern to match
extern int match_pattern(const char *str, const char *pattern) {
	if (*pattern == '\0') {
		return *str == '\0';
	}
	if (*pattern == '*') {
		return match_pattern(str, pattern + 1)
				|| (*str && match_pattern(str + 1, pattern));
	}
	if (*str && (*pattern == *str)) {
		return match_pattern(str + 1, pattern + 1);
	}
	return 0;
}

// fast matcher with single string to find patterns in
// line - string for search patterns 
// patterns - string array with patterns 
// len - patterns array size
extern int simple_matcher(char *line, char *patterns[], int length) {
	int result = NOT_FOUND;
	for (int i = 0; i < length; i++) {
		char *pattern = patterns[i];
		int exit_code = match_one(line, pattern);
		if (exit_code == 1) {
			result = FOUND;
			break;
		}
	}
	return result;
}

extern int match_one(char *line, char *pattern) {
	int wildcard = 0;
	char *placeholder = NULL;
	char *linePlaceholder = NULL;

	while (*line) {
		if (*pattern == *line) {
			line++;
			pattern++;
		} else if (*pattern == '*') {
			pattern++;
			wildcard = 1;
			placeholder = pattern;
			linePlaceholder = line;
		} else if (wildcard) {
			if (linePlaceholder) {
				linePlaceholder++;
				line = linePlaceholder;
				pattern = placeholder;
			} else {
				return NOT_FOUND;
			}
		} else {
			return NOT_FOUND;
		}
	}

	while (*pattern == '*') {
		pattern++;
	}

	return *pattern == '\0';
}
