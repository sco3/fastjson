CC = gcc 
CFLAGS = -Wall -Wextra -fPIC -O3 -m64
LDFLAGS = -shared
TARGET_LIB = libmatcher.so
TARGET_TEST = matchertest
TARGET_TEST_STATIC = matchertest-static

OBJ_LIB = matcher.o
OBJ_TEST = matchertest.o

all: $(TARGET_LIB) $(TARGET_TEST) $(TARGET_TEST_STATIC)

$(TARGET_LIB): $(OBJ_LIB)
	$(CC) $(LDFLAGS) -o $@ $^

matcher.o: matcher.c matcher.h
	$(CC) $(CFLAGS) -c $< -o $@

matchertest.o: matchertest.c matcher.h
	$(CC) $(CFLAGS) -c $< -o $@

$(TARGET_TEST): $(OBJ_TEST) $(TARGET_LIB)
	$(CC) -o $@ $^ -L. -lmatcher

$(TARGET_TEST_STATIC): $(OBJ_TEST) $(TARGET_LIB)
	$(CC) -o $@ -O3 matcher.c matchertest.c 

test: $(TARGET_TEST)
	LD_LIBRARY_PATH=. ./$(TARGET_TEST) ; ./$(TARGET_TEST_STATIC)

clean:
	rm -f *.o $(TARGET_LIB) $(TARGET_TEST) $(TARGET_TEST_STATIC)

.PHONY: all clean
