#include <fnmatch.h>
#include <stdio.h>
#include "matcher.h"

int main() {
	printf("%s\n", "hi");
	char *p[] = { "a*", "b*" };
	size_t size = sizeof(p) / sizeof(p[0]);
	int result = matcher("asdf", p, size);
	printf("result = %d\n", result);
}
