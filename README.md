hecker
======

Pretend you are a hacker or feel like one from Hollywood.

_hecker_ is a command-line tool that makes you seem like a professional hacker - atleast to those
who do not know much about hacking. When you execute the script in the terminal it will read
pressed keys but write characters from a C file. You may as well change that file with some
options.


Installation
------------

Install _hecker_ with [Cargo](https://github.com/rust-lang/cargo):

```text
cargo install hecker
```


Usage
-----

```text
Pretend you are a hacker or feel like one from Hollywood

Usage: hecker [OPTIONS] [source]

Arguments:
  [source]  [default: https://raw.githubusercontent.com/torvalds/linux/master/kernel/events/internal.h]

Options:
      --type <type>  The type of the source. [default: url] [possible values: file, text, url]
  -c, --clear        Clears the screen before running.
  -h, --help         Print help
  -V, --version      Print version
```


ToDo
----

* [ ] Code Highlighting.


Links
-----

* [📦 crates.io](https://crates.io/crates/hecker)
* [📄 GitHub](https://github.com/phoenixr-codes/hecker/)
