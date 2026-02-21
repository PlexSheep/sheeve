# sheeve

![Project badge](https://img.shields.io/badge/language-Rust-blue.svg)
![Crates.io License](https://img.shields.io/crates/l/sheeve)
![GitHub Release](https://img.shields.io/github/v/release/PlexSheep/sheeve)
![GitHub language count](https://img.shields.io/github/languages/count/PlexSheep/sheeve)
[![Rust CI](https://github.com/PlexSheep/sheeve/actions/workflows/cargo.yaml/badge.svg)](https://github.com/PlexSheep/hedu/actions/workflows/cargo.yaml)

- [GitHub](https://github.com/PlexSheep/sheeve)
- [crates.io](https://crates.io/crates/sheeve)

Keeps tab of things that increment.

In my case, it manages names for eve online ships.

---

```
$ sheeve -h
Usage: target/debug/sheeve [options]
Store: /home/plex/.local/share/sheeve.msgpack
sheeve v0.1.0

Options:
    -o NAME             set output file name
    -c, --custom CUSTOM_NAME
                        use a custom non-default name
    -h, --help          print this help menu
    -l, --list          list all names and their counters
    -d, --default NEW_DEFAULT
                        set a new default name that is not 'sheep'
    -r, --remove BAD_NAME
                        remove a name from the store
    -g, --get NAME      get next counter for a name without incrementing it
    -s, --set NAME      set counter for a custom name
$ sheeve
sheep0
$ sheeve
sheep1
$ sheeve
sheep2
$ sheeve -l
Name                                     | Counter
=====================================================
sheep(d)                                 | 2
=====================================================
1 names                                  | 2 total
$ sheeve -c ape
ape0
$ sheeve -l
Name                                     | Counter
=====================================================
ape                                      | 1
sheep(d)                                 | 2
=====================================================
1 names                                  | 2 total
$ sheeve -c ape -s 40
$ sheeve -c sheep -s 40
$ sheeve -l
Name                                     | Counter
=====================================================
ape(d)                                   | 40
sheep                                    | 40
=====================================================
2 names                                  | 80 total
$ sheeve -g afs
afs is at 0
$ sheeve -r ape
ape (40) was removed
$ sheeve -l
Name                                     | Counter
=====================================================
sheep                                    | 40
=====================================================
1 names                                  | 40 total
```
