

import os
from sys import exit
import ctypes
from ctypes import c_char_p, c_int, POINTER, CDLL, cast, addressof


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

clib.triad_matcher.argtypes = [
    c_char_p, c_char_p, c_char_p,
    POINTER(POINTER(c_char_p)),
    c_int
]
clib.triad_matcher.restype = c_int

patterns = [
    ["a1", "b1", "c1"],
    ["a*", "*", "*r*"],
    ["a*", "*sw*", "*t"]
]
bufs = [[token.encode('utf-8') for token in triad] for triad in patterns]
triad_type = c_char_p * 3
c_triad_arr = (POINTER(c_char_p) * len(bufs))()
for i, triad in enumerate(bufs):
    c_triad = triad_type (*triad)    
    c_triad_arr[i] = c_triad 

result = clib.triad_matcher (
    b'asdf', b'svitch', b'front',
    c_triad_arr, len(c_triad_arr)
)

print (result)
