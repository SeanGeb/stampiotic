# stampiotic

A flyweight tool for checking stampfile freshness.

## Installation

Install from source with

```bash
$ cargo install stampiotic
```

You can build optimised binaries using Nightly's build-std feature, saving
around 200&nbsp;KiB.

```bash
$ cargo +nightly build --release
    -Z build-std=std,panic_abort
    -Z build-std-features=panic_immediate_abort
    --target x86_64-unknown-linux-gnu
```

## Why?

Spinning up a shell (or worse, a Python or Perl script) takes substantially
longer than invoking a small binary, so why bother? Some containers might not
even include a shell by default. `stampiotic` has exactly one dependency: libc.

`stampiotic` is a particularly useful tool for implementing a watchdog timer in
Kubernetes. Legacy services can just touch a stampfile periodically, which can
then be checked using `stampiotic` as a periodic Kubernetes health check.

## Usage

```bash
$ stampiotic max-age stampfile [...]
```

e.g.

```bash
$ stampiotic 60s /run/foo/foo-server.stamp 60s
$ stampiotic '30 min' make.stamp
$ stampiotic 1d long-process-1.stamp long-process-2.stamp
File "long-process-2.stamp" is too old (12345s > 86400s)
$ echo $?
1
```

## Max-age parsing

Durations are parsed by [`humantime`](https://crates.io/crates/humantime).

| Unit         | Supported suffixes           | Definition                    |
|--------------|------------------------------|-------------------------------|
| nanoseconds  | `nsec`/`ns`                  | 10<sup>−9</sup>&nbsp;s        |
| microseconds | `usec`/`us`                  | 10<sup>−6</sup>&nbsp;s        |
| milliseconds | `msec`/`ms`                  | 10<sup>−3</sup>&nbsp;s        |
| seconds      | `seconds`/`second`/`sec`/`s` | 1&nbsp;s                      |
| minutes      | `minutes`/`minute`/`min`/`m` | 60&nbsp;s                     |
| hours        | `hours`/`hour`/`hr`/`h`      | 3600&nbsp;s = 60&nbsp;minutes |
| days         | `days`/`day`/`d`             | 86,400&nbsp;s = 24&nbsp;hours |
| weeks        | `weeks`/`week`/`w`           | 604,800&nbsp;s = 7&nbsp;days  |
| months       | `months`/`month`/`M`         | 30.44 days = 2630016&nbsp;s   |
| years        | `years`/`year`/`y`           | 365.25 days = 31557600&nbsp;s |

