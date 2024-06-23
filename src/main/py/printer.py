
import os
from sys import exit
import ctypes
from ctypes import c_char_p, c_int, POINTER, CDLL


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
clib.printer.argtypes = [POINTER(c_char_p), c_int]
clib.printer.restypes = c_int

patterns = ["a*", "b*"]
bufs = [ p.encode() for p in patterns ]
patterns_type = c_char_p * len(patterns)
c_patterns = patterns_type (*bufs)

clib.printer (c_patterns, len(patterns))
