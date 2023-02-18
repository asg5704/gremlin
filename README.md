# 👹 Gremlin

A lightweight, grep-like searcher written in Rust that uses a gremlin to finds patterns.

## Motivation

- As a means of learning the Rust programming language

## Example

// TODO

## Installation

To install `gremlin`, you can download the binaries [here](https://github.com/asg5704/gremlin/releases). Remember to put the path to the binary in your `PATH`.

## Usage

```
$ gremlin -h
gremlin 0.1.0
Alexander Garcia <https://alexandergarcia.me>
A grep-like pattern searcher

USAGE:
    gremlin <pattern> [input]

ARGS:
    <pattern>    Pattern to search for
    <input>      File to search [optional]

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information
```

### _Search within a file_

```bash
gremlin <pattern> <input-file>
gremlin Banana test.txt
```

### _Search using pipe_

```bash
... | gremlin <pattern>
cat test.txt | gremlin Banana
```
