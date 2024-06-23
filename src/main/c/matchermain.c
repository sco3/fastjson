#include <fnmatch.h>
#include <stdio.h>
#include "matcher.h"

int main() {
	printf("%s\n", "hi");
	char *p[] = { "a*", "b*" };
	size_t size = sizeof(p) / sizeof(p[0]);
	int result = matcher("asdf", p, size);
	printf("result = %d\n", result);

	char *p1[] = { "a*", "b*", "c*" };
	char *p2[] = { "d*", "*e", "f*" };
	char *p3[] = { "x*", "y*", "z*" };

	char **all[] = { p1, p2, p3 };

	int triad_result = triad_matcher( //
			"door bell", "asdf.enable", "front light", //
			all, 3 //
			);
	printf("result = %d\n", triad_result);

	triad_result = triad_matcher( //
			"door bell", "y", "front light", //
			all, 3 //
			);
	printf("result = %d\n", triad_result);

}
