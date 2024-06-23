

import os
from sys import exit
import ctypes
from ctypes import c_char_p, c_int, POINTER, CDLL, cast


def __init__():
    print ("init")


lib_so = 'libmatcher.so'
so_file = os.path.dirname (__file__) + '/' + lib_so
so = os.path.exists(so_file)
if not so:
    so_file = (
        os.path.dirname (__file__) 
        +'/../../../build/libs/matcher/shared/'
        +lib_so
        )
    
so = os.path.exists(so_file)
if not so:
    print (f"Not found {so_file}!")
    exit(1) 

clib = CDLL(so_file)
clib.matcher.argtypes = [c_char_p, POINTER(c_char_p), c_int]
clib.matcher.restypes = c_int

clib.matcher_ext.argtypes = [c_char_p, POINTER(c_char_p), c_int, c_char_p, c_int]
clib.matcher_ext.restypes = c_int

patterns = [b"a*", b"b*", b"c"]
patterns_type = c_char_p * len(patterns)
c_patterns = patterns_type (*patterns)

print (isinstance(c_patterns, list))
print (isinstance(["a"], list))

result = clib.matcher(b"asdf", c_patterns, len(c_patterns))
print (f"result = {result}")

out = bytes(10)
outlen = len(out)

result = clib.matcher_ext(b"asdf", c_patterns, len(c_patterns), out, outlen)
outstr = out.decode()
print (f"result = {result} matching pattern: {outstr}")

result = clib.matcher_ext(b"bsdf", c_patterns, len(c_patterns), out, outlen)
outstr = out.decode()
print (f"result = {result} matching pattern: {outstr}")

result = clib.matcher_ext(b"csdf", c_patterns, len(c_patterns), out, outlen)
outstr = out.decode()
print (f"result = {result} matching pattern: {outstr}")

clib.triad_matcher.argtypes = [
    c_char_p, c_char_p, c_char_p,
    POINTER(POINTER(c_char_p)),
    c_int
]
clib.triad_matcher.restype = c_int

triad = [b"a*", b"b*", b"c"]
triad_type = c_char_p * len(triad)
c_triad = patterns_type (*triad)
p_triad = cast (c_triad, POINTER(POINTER(c_char_p)))

clib.triad_matcher (b'1', b'2', b'3', p_triad, 1)
