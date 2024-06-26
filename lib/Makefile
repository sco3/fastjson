CC = clang
CFLAGS = -Wall -Wextra -fPIC -O3 -m64
LDFLAGS = -shared
TARGET_LIB = libmatcher.so
TARGET_TEST = matchertest

OBJ_LIB = matcher.o
OBJ_TEST = matchertest.o

all: $(TARGET_LIB) $(TARGET_TEST)

$(TARGET_LIB): $(OBJ_LIB)
	$(CC) $(LDFLAGS) -o $@ $^

matcher.o: matcher.c matcher.h
	$(CC) $(CFLAGS) -c $< -o $@

matchertest.o: matchertest.c matcher.h
	$(CC) $(CFLAGS) -c $< -o $@

$(TARGET_TEST): $(OBJ_TEST) $(TARGET_LIB)
	$(CC) -o $@ $^ -L. -lmatcher

test: $(TARGET_TEST)
	LD_LIBRARY_PATH=. ./$(TARGET_TEST)

clean:
	rm -f *.o $(TARGET_LIB) $(TARGET_TEST)

.PHONY: all clean
