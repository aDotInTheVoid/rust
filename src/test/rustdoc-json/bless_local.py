#!/usr/bin/env python3

# python3 bless_local.py nested.json nested.expected
# Where nested.json is raw rustdoc output

# Your'll need to add $TEST_BASE_DIR manualy for now
import json
import sys


def keep(k, v) -> bool:
    # Ensure it's a local (not std and frends), and also not an impl.
    return k.split(":")[0] == "0" and v["kind"] != "impl"


with open(sys.argv[1]) as f:
    raw = json.load(f)

raw["index"] = {k: v for (k, v) in raw["index"].items() if keep(k, v)}

# Beautifull magestic glorious hack
idxstr = repr(raw["index"])
raw["paths"] = {k: v for (k, v) in raw["paths"].items() if k in idxstr}

crates_needed = {i["crate_id"] for i in raw["paths"].values()}
raw["external_crates"] = {k: v for (k, v) in raw["external_crates"].items() if k in crates_needed}


with open(sys.argv[2], 'w') as f:
    json.dump(raw, f, indent=2, sort_keys=True)
