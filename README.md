# `si-scale`

[![crate](https://img.shields.io/crates/v/si-scale.svg)](https://crates.io/crates/si-scale)
[![documentation](https://docs.rs/si-scale/badge.svg)](https://docs.rs/si-scale)
[![minimum rustc 1.8](https://img.shields.io/badge/rustc-1.50+-red.svg)](https://rust-lang.github.io/rfcs/2495-min-rust-version.html)
[![build status](https://github.com/u0xy/si-scale/workflows/main/badge.svg)](https://github.com/u0xy/si-scale/actions)

<!-- cargo-sync-readme start -->

Format value with units according to SI ([système international d’unités](https://en.wikipedia.org/wiki/International_System_of_Units)).

_Version requirement: rustc 1.50+_

```toml
[dependencies]
si-scale = "0.1"
```


## Getting started

This crate parses and formats numbers using the
[SI Scales](https://en.wikipedia.org/wiki/International_System_of_Units):
from 1 y (yocto, i.e. 1e-24) to 1 Y (Yotta, i.e. 1e24). It is essentially
agnostic of units per-se; you can totally keep representing units with
strings or [uom](https://crates.io/crates/uom), or something else.


### Pre-defined helper functions

You can use one of the predefined helper functions to format numbers:

```rust
use si_scale::helpers::{seconds, seconds3};

let actual = format!("{}", seconds(1.3e-5));
let expected = "13 µs";
assert_eq!(actual, expected);

let actual = format!("{}", seconds3(1.3e-5));
let expected = "13.000 µs";
assert_eq!(actual, expected);
```

Currently the helper functions are:

| helper fn    | mantissa  | prefix constraint | base  | groupings | example                |
| ---          | --        | ---               | ---   | ---       | ---                    |
| `seconds()`  | `"{}"`    | `UnitAndBelow`    | B1000 | none      | `1.234567 µs`, `16 ms` |
| `seconds3()` | `"{:.3}"` | `UnitAndBelow`    | B1000 | none      | `1.235 µs`, `9876 s`   |
| ---          | --        | ---               | ---   | ---       | ---                    |
| `bytes()`    | `"{}"`    | `UnitAndAbove`    | B1000 | `_`       | `1.234_567 kB`         |
| `bytes_()`   | `"{}"`    | `UnitOnly`        | B1000 | `_`       | `1_234_567 B`          |
| `bytes1()`   | `"{:.1}"` | `UnitAndAbove`    | B1000 | none      | `2.3 TB`               |
| ---          | --        | ---               | ---   | ---       | ---                    |
| `bibytes()`  | `"{}"`    | `UnitAndAbove`    | B1024 | `_`       | `1.234_567 MiB`        |
| `bibytes1()` | `"{:.1}"` | `UnitAndAbove`    | B1024 | none      | `1.2 GiB`              |


## Custom helper functions

To define your own format function, use the
[`scale_fn!()`](`crate::scale_fn!\(\)`) macro. All pre-defined helper
functions from this crate are defined using this macro.

For instance, let's define a formatting function for bits per sec which
prints the mantissa with 2 decimals, and also uses base 1024 (where 1 ki =
1024). Note that although we define the function in a separate module,
this is not a requirement.

```rust
mod unit_fmt {
    use si_scale::scale_fn;
    use si_scale::prelude::Value;

    // defines the `bits_per_sec()` function
    scale_fn!(bits_per_sec,
              base: B1024,
              constraint: UnitAndAbove,
              mantissa_fmt: "{:.2}",
              groupings: '_',
              unit: "bit/s");
}

use unit_fmt::bits_per_sec;

fn main() {
    let x = 2.1 * 1024 as f32;
    let actual = format!("throughput: {:>15}", bits_per_sec(x));
    let expected = "throughput:    2.10 kibit/s";
    assert_eq!(actual, expected);

    let x = 2;
    let actual = format!("throughput: {}", bits_per_sec(x));
    let expected = "throughput: 2.00 bit/s";
    assert_eq!(actual, expected);
}

```


## SI Scales

With base = 1000, 1k = 1000, 1M = 1\_000\_000, 1m = 0.001, 1µ = 0.000\_001,
etc.

| min (incl.) | max (excl.)      | magnitude | prefix          |
| ---         | ---              | ---       | ----            |
| ..          | ..               | -24       | `Prefix::Yocto` |
| ..          | ..               | -21       | `Prefix::Zepto` |
| ..          | ..               | -18       | `Prefix::Atto`  |
| ..          | ..               | -15       | `Prefix::Femto` |
| ..          | ..               | -12       | `Prefix::Pico`  |
| ..          | ..               | -9        | `Prefix::Nano`  |
| 0.000\_001  | 0.001            | -6        | `Prefix::Micro` |
| 0.001       | 1                | -3        | `Prefix::Milli` |
| 1           | 1_000            | 0         | `Prefix::Unit`  |
| 1000        | 1\_000\_000      | 3         | `Prefix::Kilo`  |
| 1\_000\_000 | 1\_000\_000\_000 | 6         | `Prefix::Mega`  |
| ..          | ..               | 9         | `Prefix::Giga`  |
| ..          | ..               | 12        | `Prefix::Tera`  |
| ..          | ..               | 15        | `Prefix::Peta`  |
| ..          | ..               | 18        | `Prefix::Exa`   |
| ..          | ..               | 21        | `Prefix::Zetta` |
| ..          | ..               | 24        | `Prefix::Yotta` |


The base is usually 1000, but can also be 1024 (bibytes).

With base = 1024, 1ki = 1024, 1Mi = 1024 * 1024, etc.

## Overview

The central representation is the [`Value`](`crate::value::Value`) type,
which holds

- the mantissa,
- the SI unit prefix (such as "kilo", "Mega", etc),
- and the base which represents the cases where "1 k" means 1000 (most
common) and the cases where "1 k" means 1024 (for kiB, MiB, etc).

This crate provides 2 APIs: a low-level API, and a high-level API for
convenience.

For the low-level API, the typical use case is

- first parse a number into a [`Value`](`crate::value::Value`). For doing
this, you have to specify the base, and maybe some constraint on the SI
scales. See [`Value::new()`](`crate::value::Value::new\(\)`) and
[`Value::new_with()`](`crate::value::Value::new_with\(\)`)

- then display the `Value` either by yourself formatting the mantissa
  and prefix (implements the `fmt::Display` trait), or using the provided
  Formatter.

For the high-level API, the typical use cases are

1. parse and display a number using the provided functions such as
   `bibytes()`, `bytes()` or `seconds()`, they will choose for each number
   the most appropriate SI scale.

2. In case you want the same control granularity as the low-level API
   (e.g. constraining the scale in some way, using some base, specific
   mantissa formatting), then you can build a custom function using the
   provided macro `scale_fn!()`. The existing functions such as
   `bibytes()`, `bytes()`, `seconds()` are all built using this same
   macro.


### The high-level API

The `seconds3()` function parses a number into a `Value` and displays it
using 3 decimals and the appropriate scale for seconds (`UnitAndBelow`),
so that non-sensical scales such as kilo-seconds may not appear. The
`seconds()` function does the same but formats the mantissa with the
default `"{}"`, so no decimals are printed for integer mantissa.

```rust
use si_scale::helpers::{seconds, seconds3};

let actual = format!("result is {:>15}", seconds(1234.5678));
let expected = "result is     1234.5678 s";
assert_eq!(actual, expected);

let actual = format!("result is {:>10}", seconds3(12.3e-7));
let expected = "result is   1.230 µs";
assert_eq!(actual, expected);
```

The `bytes()` function parses a number into a `Value` *using base 1000*
and displays it using 1 decimal and the appropriate scale for bytes
(`UnitAndAbove`), so that non-sensical scales such as milli-bytes may not
appear.

```rust
use si_scale::helpers::{bytes, bytes1};

let actual = format!("result is {}", bytes1(12_345_678));
let expected = "result is 12.3 MB";
assert_eq!(actual, expected);

let actual = format!("result is {:>10}", bytes(16));
let expected = "result is       16 B";
assert_eq!(actual, expected);

let actual = format!("result is {}", bytes(0.12));
let expected = "result is 0.12 B";
assert_eq!(actual, expected);
```

The `bibytes1()` function parses a number into a `Value` *using base 1024*
and displays it using 1 decimal and the appropriate scale for bytes
(`UnitAndAbove`), so that non-sensical scales such as milli-bytes may not
appear.

```rust
use si_scale::helpers::{bibytes, bibytes1};

let actual = format!("result is {}", bibytes1(12_345_678));
let expected = "result is 11.8 MiB";
assert_eq!(actual, expected);

<!-- cargo-sync-readme end -->
