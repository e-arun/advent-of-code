# AoC 2016 - Typescript

Advent of code 2016 problems solved using [Typescript](https://www.typescriptlang.org/).
[Bun](https://bun.sh) is used as the runtime instead of the more popular
[Node.js](https://nodejs.org/).


## Installation and setup

- Install [Bun](https://bun.sh/docs/installation)
- Install the projects dependencies by running `bun install` in this folder.

## Running

Each script in `src/` folder corresponds to a single AoC problem. Part 1 and
part 2 of each problem are implemented in separate files. All scripts take
input from stdin and print the answer to stdout.

```sh
bun src/day_01_p1.ts < input.txt
```

You can get the inputs from [AoC 2016](https://adventofcode.com/2016).
