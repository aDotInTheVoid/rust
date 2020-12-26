#!/bin/bash
./x.py b src/librustdoc library/std                
rustdoc +stage1 --edition 2018 --output-format json src/test/rustdoc-json/nested.rs -o . 2> log_raw
python3 tree.py