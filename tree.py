#!/usr/bin/env python3
import json

TABW = 1

with open("./nested.json") as f:
    jinp = json.load(f)

with open("tree", "r") as myfile:
    data = myfile.readlines()

indent = 0

items = []

ids = {}


def path(idx):
    # breakpoint()
    if name := jinp["index"].get(idx):
        name = name["name"]
    else:
        return idx
    path = jinp["paths"].get(idx)
    path = "::".join(path["path"]) if path is not None else name
    return path if path else idx


for i in data:
    i = i.replace("\n", "")
    if i == "END":
        indent -= TABW
    else:
        # breakpoint()
        items.append([i, indent])
        if ids.get(i) == None:
            ids[i] = 1
        else:
            ids[i] += 1

        indent += TABW

for [item, indent] in items:
    # breakpoint()
    true_id = item[10:-2]
    if ty := jinp["index"].get(true_id):
        ty = ty["kind"]
    else:
        ty = "???"

    pre = ">" * indent + (" " if indent else "")
    pre += path(item[10:-2])
    print(pre, " " * (25 - len(pre)), ty, "\t", ids[item])
