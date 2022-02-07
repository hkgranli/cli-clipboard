# cpd - CLI Clipboard Storage 

Simple program which stores your current clipboard content into `~/.clipboardfile` for later extraction. Probably only works on linux btw idk

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
$ sudo mv target/release/cpd /usr/local/bin
```

## Help

```
$ cpd help
```
## closing thoughts

shout out to the community for bughunting