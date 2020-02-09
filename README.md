# tasinput2 - Input plugin for sending inputs to mupen64plus

This is essentially trying to do what tasinput did for mupen64-rr, and allow for TAS inputs to be sent to mupen64plus.

## Build/Install

First, download the QT libraries. Then:

```sh
$ git clone https://github.com/not-a-seagull/tasinput2.git && cd tasinput2
$ cargo build --release
$ mupen64plus --input target/release/libtasinput2.so
```

First class support (hopefully) coming soon.
