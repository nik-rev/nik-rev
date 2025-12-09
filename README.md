Hi! My favorite programming language is:

```rust
fn you_guessed_it() -> impl Debug {
    ..=..=.. ..    .. .. .. ..    .. .. .. ..    .. .. .. ..
    ..=.. ..=..    .. .. .. ..    .. .. .. ..    .. ..=.. ..
    ..=.. ..=..    ..=.. ..=..    .. ..=..=..    ..=..=..=..
    ..=..=.. ..    ..=.. ..=..    ..=.. .. ..    .. ..=.. ..
    ..=.. ..=..    ..=.. ..=..    .. ..=.. ..    .. ..=.. ..
    ..=.. ..=..    ..=.. ..=..    .. .. ..=..    .. ..=.. ..
    ..=.. ..=..    .. ..=..=..    ..=..=.. ..    .. ..=..=..
}
```

here are some of my stuffs:

## `derive_aliases` [![github](https://img.shields.io/github/stars/nik-rev/derive-aliases)](https://github.com/nik-rev/derive-aliases)

Custom `#[derive]` aliases. Write this:

```rust
#[derive(Debug, ..Ord, ..Copy)]
//              ^^^^^^^^^^^^^
//              aliases
struct User;
```

which expands to this:

```rust
[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
//             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
//             expanded
struct User;
```

## `docstr` [![github](https://img.shields.io/github/stars/nik-rev/docstr)](https://github.com/nik-rev/docstr)

Ergonomic multi-line string literals, even composes with any macro like `format!`.

```rust
use docstr::docstr;

let hello_world_in_c: &'static str = docstr!(
    /// #include <stdio.h>
    ///
    /// int main(int argc, char **argv) {
    ///     printf("hello world\n");
    ///     return 0;
    /// }
);

assert_eq!(hello_world_in_c, r#"#include <stdio.h>

int main(int argc, char **argv) {
    printf("hello world\n");
    return 0;
}"#)
```

## `culit` [![github](https://img.shields.io/github/stars/nik-rev/culit)](https://github.com/nik-rev/culit)

Lets you define custom literals. Like literals for `Duration`:

```rust
#[culit]
fn main() {
    assert_eq!(
        100d + 11h + 8m + 7s,
        Duration::from_secs(100 * 60 * 60 * 24)
        + Duration::from_secs(11 * 60 * 60)
        + Duration::from_secs(8 * 60)
        + Duration::from_secs(7)
    );
}
```

Fully custom, can be whatever you want. Like `10nzusize` which produces `NonZeroUsize` and compile errors if it is `0`.

## `simply_colored` [![github](https://img.shields.io/github/stars/nik-rev/simply_colored)](https://github.com/nik-rev/simply_colored)

Simplest crate in existence for terminal styles.

```rust
use simply_colored::*;

println!("{BLUE}{BOLD}Simply colored!")
```

## countryfetch [![github](https://img.shields.io/github/stars/nik-rev/countryfetch)](https://github.com/nik-rev/countryfetch)

Like Neofetch but for your country
