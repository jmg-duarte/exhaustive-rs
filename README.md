# Exhaustive

> exhaustive /ɪɡˈzɔːstɪv,ɛɡˈzɔːstɪv/ - **including or considering all elements or aspects; fully comprehensive.**

This crate was born from issue [#23](https://github.com/dtolnay/proc-macro-workshop/issues/23) in `dtolnay/proc_macro_workshop`.

> I'd like to see a basic implementation of a macro for Python-style list and map comprehensions, and decide whether it would be a better teaching example for function-like proc macros than the current `seq!` project.
>
> Something like:
> ```rust
> let squares_map = c![n => n*n for n in 0..100 if n % 5 != 0];
> ```

## Previous Work

There are some crates which implement this functionality or really similar,
however, most of the crates are implemented using `macro_rules!`.

**Captain Obvious**: Which is *not* a procedural macro and thus not adequate as a learning example of such.

The existing crates are:

- [`cute`](https://crates.io/crates/cute) - *Macro for Python-esque list comprehensions in Rust.*
- [`mapcomp`](https://crates.io/crates/mapcomp) - *Python-like list comprehension via macros for some standard containers.*
- [`iter-comprehensions`](https://crates.io/crates/iter-comprehensions) - *A library for iterator comprehensions.*
- [`comprehension`](https://crates.io/crates/comprehension) - *Iterator comprehension in Rust.*

The last one, `comprehension`, is the only one implemented using function-like procedural macros,
however the syntax is based on Haskell, not Python.