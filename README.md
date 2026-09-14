# RISC-V Simulator

A personal CLI tool created to avoid the GUI simulator.

The motivation was an **Introduction to the Computer Architecture** course at my university, where the only reliable way to test RISC-V assembly was using a provided GUI tool, so I created a terminal alternative.

## Features

Aside from code execution (`run`, `step <n>`):

- Error highlighting

```
Error: expected number of 12 bits
   ╭─[ fail.S:1:16 ]
   │
 1 │ addi t4, t2, 0x1984
   │                ──┬─
   │                  ╰─── expected number of 12 bits
───╯
```

- Hexdump

```
0x00000200:     93 8e f3 ff  ....
0x00001000:     72 69 73 63  risc
```

- Objdump

```
0x00000200:     93 8e f3 ff     addi x29, x7, -1
```

- Register dump and memory inspection

## Quickstart

Download [Rust](https://rust-lang.org/tools/install/) (and [git](https://git-scm.com/install/) if you don't have it).

Install (ensure both `git` and `cargo` are on the `PATH`):

```
git clone https://github.com/wivest/riscv-simulator.git
cd riscv-simulator
cargo install --path .
```

Uninstall:

```
cargo uninstall rvsim
```

## Usage

### CLI

Loading and executing a file is as simple as:

```
rvsim <path>
```

And then:

```
> run
```

To list all available commands type `help` after loading a file.

### Examples

The files under [tests/asm/](tests/asm/) folder can be used as examples to learn assembly. For instance run `rvsim tests/asm/replace.S` to inspect the code.

### What is supported

This simulator was tested against course assignments to pass them, and they were the main driving point while developing. The tool should be a subset of [QtRvSim](https://github.com/cvut/qtrvsim). As I built it from a university course, it only covers RV32I (base 32-bit integer ISA) with most multiplication extension instructions. All directives work similar, _but_ `.org` sets writer position only inside _current_ section (i.e. sections are not ignored, which might be of use in the future). Empirically the starting value of the program counter (entry point) is fixed to a `0x200` address. More about specification of QtRvSim can be read [here](https://github.com/cvut/qtrvsim#integrated-assembler).
