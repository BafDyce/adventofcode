#!/bin/bash

# Usage: script.sh <year> <day>
# env var AOC_SID must be set!!
# TODO: some input sanitization and so on

SCRIPTNAME=`basename $0`

print_usage() {
    echo "Usage: $SCRIPTNAME <year> <day>"
    echo "environment variable \"AOC_SID\" must be set!"
}

source AOC_SID.sh
if [ "$#" -lt "2" ] || [ -z "$AOC_SID" ]; then
  print_usage
  exit 1
fi

YEAR=$1
DAY=$2

mkdir -p tmp
curl -H "Cookie: session=$AOC_SID" "https://adventofcode.com/${YEAR}/day/${DAY}" > tmp/aoc-desc.html 2> /dev/null
pandoc -f html -t gfm tmp/aoc-desc.html > tmp/aoc-desc.md


# remove header
lineStart=$(grep -n -- "## \\\\--- Day $DAY:" tmp/aoc-desc.md | cut -d: -f1)
tail "+$lineStart" < tmp/aoc-desc.md > tmp/aoc-desc-tmp.md

# remove footer
trailerStart=$(grep -n -- "Your puzzle answer was" tmp/aoc-desc-tmp.md | cut -d: -f1 | tail +2)
if [ "$trailerStart" != "" ]; then
    head -n "$trailerStart" < tmp/aoc-desc-tmp.md > tmp/aoc-desc-tmp2.md
else
    mv tmp/aoc-desc-tmp.md tmp/aoc-desc-tmp2.md
fi

# remove solutions
sed -i '/Your puzzle answer was/d' tmp/aoc-desc-tmp2.md

# save
cp tmp/aoc-desc-tmp2.md "${YEAR}/_tasks/day$(printf %02d $DAY).md"

# clean up
rm -r tmp
