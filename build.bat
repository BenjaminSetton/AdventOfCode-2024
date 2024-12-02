
@echo off

set DAY_NUMBER=1

rustc -o "./out/aoc.exe" "./src/day%DAY_NUMBER%/main.rs"
