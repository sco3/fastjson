extern int glibc_match_pattern(char *line, char *pattern);
extern int glibc_matcher(char *line, char *patterns[], int n);
extern int match_pattern(const char *str, const char *pattern);
extern int simple_matcher(char *line, char *patterns[], int patslen);
extern int match_two(char *line, char *pattern);
extern int match_one(char *line, char *pattern);

enum {
	NOT_FOUND = 0, FOUND = 1
};
