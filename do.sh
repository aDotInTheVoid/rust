#!/bin/bash
./x.py b src/librustdoc library/std                
#rustdoc +stage1 --edition 2018 --output-format json src/test/rustdoc-json/nested.rs -o . 2> log_raw
#python3 tree.py
cd ../regex
cargo +stage1 rustdoc -v -- --output-format json 2>&1 | rg "START|END" > ../rust/tree
mv target/doc/regex.json ../rust/nested.json
cd ../rust
./tree.py > slog
cd ../regex
cargo +stage1 rustdoc -- --output-format html 2>&1 | rg "START|END" > ../rust/tree
cd ../rust
./tree.py > slog_html

#TODO: Autogen slog and slog_raw here, and then rebase to master
