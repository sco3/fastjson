extern int matcher(char *line, char *patterns[], int n);

extern int matcher_ext( //
		char *line, //
		char *patterns[], int len, //
		char *out, int outlen //
		);

enum {
	NOT_FOUND = 0, FOUND = 1, NOT_ENOUGH_SPACE = 2
};
