# OJMNReader
A O2Jam OJM/OJN file reader of sorts written in Rust.

Reads stuff from OJN and prints them to stdout, and then loads sounds from OJM
playing them them one by one with SFML.

Requires whatever [this crate](https://github.com/jeremyletang/rust-sfml)
requires, which is probably having SFML and CSFML installed in your system.

## Usage

To read `o2ma495.ojn` and `o2ma495.ojm`:

```sh
$ OJMNReader o2ma495
```
