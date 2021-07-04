# option-extra

## About

Additional utilities for the standard `Option` type that feel missing from `std`.

## Usage

New methods live in the `OptionExt` trait:

```rust
use option_extra::OptionExt;

assert_eq!(Some(1).zip_lazy(|| Some("abcd")), Some((1, "abcd")));
```