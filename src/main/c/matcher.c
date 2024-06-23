#include <fnmatch.h>
#include "matcher.h"
#include <string.h>
#include <stdio.h>

int matchPattern(const char *str, const char *pattern) {
	if (*pattern == '\0') {
		return *str == '\0';
	}
	if (*pattern == '*') {
		return matchPattern(str, pattern + 1)
				|| (*str && matchPattern(str + 1, pattern));
	}
	if (*str && (*pattern == *str)) {
		return matchPattern(str + 1, pattern + 1);
	}
	return 0;
}

extern int simple_matcher(char *line, char *patterns[], int patslen) {
	int result = NOT_FOUND;
	for (int i = 0; i < patslen; i++) {
		char *pattern = patterns[i];
		int exit_code = matchPattern(line, pattern);
		if (exit_code == 1) {
			result = FOUND;
			break;
		}
	}
	return result;
}

extern int printer2(const char **patterns, int len) {
	for (int i = 0; i < len; i++) {
		printf("%s \n", patterns[i]);
	}
	return 0;
}

extern int printer3(const char ***patterns, int len) {
	return 0;
}

extern int triad_matcher( //
		const char *tag0, const char *tag1, const char *tag2,  //
		const char ***patterns, //
		int patslen //
		) {
	printf("tags: %s %s %s\n", tag0, tag1, tag2);
	int result = NOT_FOUND;
	for (int i = 0; i < patslen; i++) {
		const char **triad = patterns[i];
		printf("triad: %s %s %s\n", triad[0], triad[1], triad[2]);
		int triad_result = (matchPattern(tag0, triad[0])
				&& matchPattern(tag1, triad[1]) && matchPattern(tag2, triad[2]) //
		);
		if (triad_result) {
			result = FOUND;
			break;
		}
	}
	return result;
}

extern int matcher(char *line, char *patterns[], int patslen) {
	int result = NOT_FOUND;
	for (int i = 0; i < patslen; i++) {
		char *pattern = patterns[i];
		int exit_code = fnmatch(pattern, line, 0);
		if (exit_code == 0) {
			result = FOUND;
			break;
		}
	}
	return result;
}
extern int matcher_ext( //
		char *line, // string to find match
		char *patterns[], int patslen, // pattern list
		char *out, int outlen // output string for pattern
		) {

	int result = NOT_FOUND;
	for (int i = 0; i < patslen; i++) {
		char *pattern = patterns[i];
		int exit_code = fnmatch(pattern, line, 0);
		if (exit_code == 0) {
			if (out == NULL) {
				result = FOUND;
			} else {
				int len = strlen(pattern);
				if (outlen > len) {
					strcpy(out, pattern);
					result = FOUND;
				} else {
					result = NOT_ENOUGH_SPACE;
				}
			}
			break;
		}
	}
	if (out != NULL && result == NOT_FOUND && outlen > 0) {
		out[0] = 0;
	}
	return result;
}

