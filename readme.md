STM32 + CubeMX 環境で rust を使った組み込みプログラミング環境を検討している。

組み込みでは`no_std`環境を使うことになるが、便利に使える、または、頻繁に必要になるルーチンがある。それらの中で、 STM32 や CubeMX に依存しないものを独立させて、`nostd_tool`とした。

* 名前のとおり`no_std`環境で使う。
* STM32やCubeMXに依存しない。
* ユニットテスト済。

## `#![no_std]`環境で使うライブラリのテスト

* `src/lib.rs`。　　
    + 通常であれば`#![no_std]`と書くべき位置に`#![cfg_attr(not(feature = "std"), no_std)]`と書く。意味は「`std`環境でない場合は`no_std`とする」。
    + ライブラリの結合テストを書く場所に次のように書く。
        - cfg=testのときに有効になる。
        - std環境のときに有効になる。
        - std 環境のときは`core`の代わりに`std`を使う。
        - tests モジュールを定義し、その中で結合テストを実施する。
* `.cargo/config`。
* クロス環境なので`rustup override set nightly`してNightlyコンパイラを有効にしておく。
* ビルドのときは、利用先のクレート(通常binだろう)で`xargo build --target=thumbv7m-none-eabi`。最新の`libcore`をダウンロードして、ビルドしてくれる。
* テストのときは`cargo test`。

```rust 
#[cfg(test)]
#[cfg(feature = "std")]
use std as core;
mod tests {
    #[test]
    fn it_works() {
    }
}
```



## doctest

* `src/lib.rs`
* `src/lock.rs`
```rust
//! # Examples
//!
//! ```
//! use nostd_tool::lock;
//! let mut l = lock::Lock::Unlocked;
//! assert_eq!(l,lock::Lock::Unlocked);
```
    + モジュールの中で doctest を書くときは`//!`を使う。
    + 例を示すときは`# Examples`セクションを使う。
    + `use クレート名::モジュール名`で自分自身のモジュールの使用を宣言する。ちょうど、そのlib crate を外部から使うときの書式と同じ。
    + あとは普通に。
