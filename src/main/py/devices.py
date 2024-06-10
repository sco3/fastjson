#!/usr/bin/env -S python 

import json
import time

start = round(time.time() * 1000)

dictionary = None
with open('devices.json', 'r') as openfile:
    dictionary = json.load(openfile)

dictionary["returnCode"]=1

with open("devices.json.py.out", "w") as outfile:
    json.dump(dictionary, outfile, separators=(',', ':'))

end = round(time.time() * 1000)

print (f"Took: {end-start}")