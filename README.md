# adventofcode
My personal implementations for [Advent of Code (AoC)](http://adventofcode.com/)

Currently I'm only doing Rust implementations, however other languages will
(hopefully) come too eventually.

# Directory structure
Beginning from 2016 (maybe, I'll adapt 2015 too some day) I will use the
following directory structure:
```
adventofcode/
├── 20xx
│   ├── _inputs
│   │   └── dayXX
│   │       ├── test.input
│   │       └── test.solution
│   ├── _tasks
│   │   └── dayXX.md
│   └── language
│       └── dayXX
│           ├── part1.rs
│           └── part2.rs
├── LICENSE
└── README.md
```

* Each `_inputs/dayXX/`-directory can contain any number of possible input sets.
    * Each set can have an arbitrary name (use of whitespace-characters is
      **highly discouraged**)
    * Each `.input` file **should** have a coresponding `.solution` file
    * Format of the content of the `.input`-files **MUST** be in the same format
    as AoC provides it.
    * Format of the content of the `.solution`-files **MUST** be in the same
    format as AoC expects it.
    * It is **recommended** that implementations provide a possibility to chose
    a specific input set by its name
* Depending on the language, the actual (language-internal) structure *may* be
slightly different and it *may* contain additional files.
* Where possible (e.g. for object oriented languages), I'll try to keep
**a single main source file** for the whole year/language combination,
implementing each day as a class/object with a consistent interface.

# Commit messages
All commit messages **should** be of the following format (when applicable):

`year language day(/part)| Msg`

where:
* `year` is the four digit number of the year the task is from
    * lists, ranges, and combinations of those, such as `2015, 2017`,
    `2015-2017`, `2016+`, `2015`, `2017-2019`, and `all`* are allowed
* `language` is the language this commit is for
    * can also be `all`* or a comma-separated list, such as `c,rust`
* `day` is the two-digit specifier of the day the task is from
    * same rules as for the year apply
* `part` is optional if this commit only affects a specific part
    * as of now, this can only be `1` or `2`
    * this **must** be ommitted if it affects all parts
* the separator `|` directly follows the `day` or optional `part` and is
followed by exactly one space before the message
* `Msg` **should** start with a capital letter
* The complete title of the message **must not** be longer than 72 characters

\* `all` means `all elements of this set which are in the repo at this moment`.
In this case, the body of the commit message should exactly specify all affected
elements by its identifier.
