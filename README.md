# maybe-uninit

Quite often, uses of `std::mem::uninitialized()` end up in unsound code.
Therefore, the `MaybeUninit` union has been added to `std::mem` and `std::mem::uninitialized()` is being deprecated.
However, `MaybeUninit` has been added quite recently.
Sometimes you might want to support older versions of Rust as well.
Here is where `maybe-uninit` comes in: it supports rustc versions starting from 1.20.0.
