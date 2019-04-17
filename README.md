# containers-from-scratch-rs
A port of https://github.com/lizrice/containers-from-scratch using Rust instead of Go

# Setup
You must have a Linux Filesystem folder such as [Alpine's mini root system](https://alpinelinux.org/downloads/) and reference it instead of `/home/myuser/ubuntufs`

# Running
You must run the build binary as root

```sh
$ cargo build
$ sudo ./target/debug/rocker run /bin/ls
```
