
<h1 align="center">maflow</h1>

<p align="center">
    <a href="https://github.com/dekirisu/maflow" style="position:relative">
        <img src="https://img.shields.io/badge/github-dekirisu/maflow-ee6677">
    </a>
    <a href="https://crates.io/crates/maflow" style="position:relative">
        <img src="https://img.shields.io/crates/v/maflow">
    </a>
</p>

Simple **Ma**cros, changing the **flow** of development:
- **`kill!( flow )`** panics
- **`exit!( flow )`** returns `::default()`
- **`next!( flow )`** continues
- **`hold!( flow )`** breaks

**Flow** can be:
1. Option `..!{ inner = option }` same as `let Some(inner) = option else {..}`
1. Result `..!{ inner = result }` same as `let Ok(inner) = result else {..}`
1. bool `..!{ if bool }` same as `if bool {..}`

## Example
```toml
[dependencies]
maflow = "0.1"
```
```rust
use maflow::*;
fn main(){
    // Options and Results
    let some = Some(true);
    kill!{is_true = some} // same as .unwrap()
    exit!{is_true = some} // returns default() if None => ()
    for i in 0..9 {
        next!{is_true = some} // continues if None
        hold!{is_true = some} // breaks if None
        if is_true {}
    }
    // Conditional
    let is_true = true;
    kill!{if is_true} // panics if ..
    exit!{if is_true} // returns default() if ..
    for i in 0..9 {
        next!{if is_true} // continues if ..
        hold!{if is_true} // breaks if ..
    }
}
```
## Also..
..implements the trait `YayNay` for `Result`, `Option` and `bool` with those methods:
- `.yay()` returns `true` if `Ok(..)` `Some(..)` or `true`
- `.nay()` returns `true` for the negative counter part

---
### License
<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>
<br>
<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>