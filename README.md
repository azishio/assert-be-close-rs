# assert-be-close-rs

Provides functions to verify the equivalence of floating-point numbers at arbitrary precision.

任意の精度で浮動小数点数の等価性を検証するための関数を提供します。

## インポート

```rust
use assert_be_close_rs::{assert_be_close, assert_not_close};
```

## 使い方

Check for equivalence with arbitrary precision.

任意の精度で等価であることを確認。

```rust
assert_be_close(1.0, 1.0001, 3);
assert_be_close(1.0, 1.0001, 4); // panic
```

Check for non-equivalence with arbitrary precision.

任意の精度で等価でないことを確認。

```rust
assert_not_close(1.0, 1.0001, 4);
assert_not_close(1.0, 1.0001, 3); // panic
```

## ライセンス

Licensed under either of

+ Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
+ MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
