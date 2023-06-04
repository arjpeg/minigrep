## Minigrep - A simple grep clone

Minigrep is a simple grep clone written in Rust. It is a command line tool that searches for a given string in a given file.

Unlike grep, minigrep accepts directoires as well as files. If a directory is given, minigrep will search all files in the directory and its subdirectories.

It does not currently support regular expressions.

### Usage

```bash
mgrep query filename
```
