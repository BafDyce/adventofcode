#!/usr/bin/env bash

print_usage() {
    echo >&2 "Usage: $0 <day>"
    echo >&2 "day must be a number, without leading zeros!"
}

if [ "$#" -lt "1" ]; then
    print_usage
    exit 1
fi

# check whether input is valid (if it's a number)
day=$(echo -n "$1" | rg "^[1-9]([0-9]+)?\$")
check=$?

if [ $check -eq 0 ]; then
    day_normal=$(printf "%d" $day)
    day_leading_0=$(printf "%02d" $day)
    #echo normal $day_normal
    #echo zeros $day_leading_0
else
    print_usage
    exit 1
fi

# Create directory for input files (+ some default empty input files)
# But only if the dir doesnt exist yet
inputdir="../_inputs/day${day_normal}"
test -d "${inputdir}" && {
    echo "$inputdir already exists, skipping input dir creation"
} || {
    mkdir -p "$inputdir"
    touch $inputdir/{example,puzzle}1.input
}

# Create cargo project dir (but only if it doesnt exist yet)
projectdir="day${day_leading_0}"
test -d "${projectdir}" && {
    echo "$projectdir already exists, skipping project dir creation"
} || {
    cp -r day00 $projectdir

    # configure files
    cd $projectdir
    sed -i "s/aoc18-00/aoc18-${day_leading_0}/" Cargo.toml
    sed -i "s/const DAY: u32 = 0;/const DAY: u32 = ${day_normal};/" src/main.rs
}
