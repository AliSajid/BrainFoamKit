# Getting Started

## Installation

The software can be installed in different ways, depending on your taste:

### Cargo Install

You can install it directly from [crates.io](https://crates.io) using `cargo`:

```bash
cargo install brainfoamkit
```

### Cargo Binaries

You can also use the excellent [cargo-binstall](https://github.com/cargo-bins/cargo-binstall) tool to install the binaries directly from the repository:

```bash
cargo install cargo-binstall
cargo binstall brainfoamkit
```

### Manual Installation

You can also install the binaries manually by downloading the appropriate binaries from the [releases](https://github.com/AliSajid/BrainFoamKit/releases) page and placing them in your `$PATH`.

## Usage

### `bfkrun`

`bfkrun` is the main binary that can be used to run programs written in either `BF` or `BFK` dialects.

`bfkrun` accepts both strings and files as input.

```bash
# Use a direct input string
bfkrun --input "+[-->-[>>+>-----<<]<--<---]>-.>>>+.>>..+++[.>]<<<<.+++.------.<<-.>>>>+."

#> Hello, World!
```

```bash
# Use a file as input

# Create a file with the program
echo "+[-->-[>>+>-----<<]<--<---]>-.>>>+.>>..+++[.>]<<<<.+++.------.<<-.>>>>+." > hello_world.bf

# Run the program
bfkrun --file hello_world.bf

#> Hello, World!
```

### `bfkview`

`bfkview` is a TUI application with a (hopefully) intutive interface that lets you step through BF[K] programs.

_Detailed instructions coming soon._
