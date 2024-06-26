extern int matcher(char *line, char *patterns[], int n);

extern int matcher_ext( //
		char *line, //
		char *patterns[], int len, //
		char *out, int outlen //
		);

extern int triad_matcher( //
		const char *one, const char *two, const char *three,  //
		const char ***patterns, //
		int len //
		);

enum {
	NOT_FOUND = 0, FOUND = 1, NOT_ENOUGH_SPACE = 2
};

extern int matcher_two(char *line, char *pattern);
