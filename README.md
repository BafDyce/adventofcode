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
