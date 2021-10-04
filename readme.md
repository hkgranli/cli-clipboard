# cpd - CLI Clipboard Storage 

Simple program which stores your current clipboard content into `~/.clipboardfile` for later extraction. Probably only works on linux btw idk

ALSO GUYS DONT STORE stuff over multiple lines

```
$ cpd list 
1 Condi tempest
2 Dragonhunter
3 QFB

$ cpd get 2
Copying entry for Dragonhunter to clipboard
wrote [&DQEuFSo6GzkmDwAAihIAAEgBAAD+AAAAtRIAAAAAAAAAAAAAAAAAAAAAAAA=] to clipboard
```

## Installation

```
$ cargo build --release 
$ sudo mv target/release/cpd /usr/bin
```

## Help

```
$ cpd help
```
## closing thoughts

well first of all i want to thank the reigning norwegian champion of snow volleyball : matato for support and ideas during a tough development period. check out matato games, or his new rust-based company `must industries` - it really is a must to check it out lmao 