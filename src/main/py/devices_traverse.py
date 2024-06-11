#!/usr/bin/env -S python

import json 
import os
import time

def traverse_json(data, path=""):
    if isinstance(data, dict):
        for key, value in data.items():
            new_path = f"{path}/{key}" if path else key
            traverse_json(value, new_path)
    elif isinstance(data, list):
        for index, value in enumerate(data):
            new_path = f"{path}[{index}]"
            traverse_json(value, new_path)
    else:
        print(f"{path}: {data}")


start = round(time.time() * 1000)

dictionary = None
with open(os.path.expanduser('~/.local/devices.json'), 'r') as openfile:
    dictionary = json.load(openfile)

#dictionary["returnCode"]=1
traverse_json(dictionary)

with open(os.path.expanduser("~/.local/devices.json.py.out"), "w") as outfile:
    json.dump(dictionary, outfile, separators=(',', ':'))

end = round(time.time() * 1000)

print (f"Took: {end-start}")        
        
# 

