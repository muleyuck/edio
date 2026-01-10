[![unit-test](https://github.com/muleyuck/edio/actions/workflows/unit-test.yml/badge.svg)](https://github.com/muleyuck/edio/actions/workflows/unit-test.yml)
![Software License](https://img.shields.io/badge/license-MIT-brightgreen.svg?style=flat-square)

# 📻 edio

A command-line tool that opens stdin in your editor, lets you edit the content interactively, and outputs the result to stdout.

![demo](https://github.com/user-attachments/assets/ed4a030d-1734-44fd-b745-8a1108c501c3)

## Installation

### From source

```bash
cargo install --path .
```

## Usage

```bash
edio [OPTIONS]
```

### Options

- `-e, --extension <EXTENSION>`: Set file extension for the temporary file (default: `txt`)

### Examples

**Basic usage: Edit stdin and print to stdout**
```bash
echo "Hello, World!" | edio
```

**Edit and save to a file**
```bash
echo "Initial content" | edio > output.txt
```

**Edit stdin in specific editor(e.g. VSCode)**
```bash
export EDITOR="code --wait"
echo "Initial content" | edio > output.txt
```

**Edit with specific file extension for syntax highlighting**
```bash
cat script.py | edio -e py | python
```

## Editor Selection Logic

edio determines which editor to use in the following order:

1. `$GIT_EDITOR` environment variable
1. `$VISUAL` environment variable
1. `$EDITOR` environment variable
1. `git config core.editor`
1. `vi` (fallback)

## Requirements

- Unix-like operating system (Linux, macOS)
- Access to `/dev/tty` when output is piped

## LICENCE

[The MIT Licence](https://github.com/muleyuck/edio/blob/main/LICENSE)

