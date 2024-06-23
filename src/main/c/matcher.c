#include <fnmatch.h>
#include "matcher.h"
#include <string.h>

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

extern int matcher(char *line, char *patterns[], int n) {
	return matcher_ext(line, patterns, n, NULL, 0);

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

