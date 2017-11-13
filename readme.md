STM32 + CubeMX 環境で rust を使った組み込みプログラミング環境を検討している。

組み込みでは`no_std`環境を使うことになるが、便利に使える、または、頻繁に必要になるルーチンがある。それらの中で、 STM32 や CubeMX に依存しないものを独立させて、`nostd_tool`とした。

* 名前のとおり`no_std`環境で使う。
* STM32やCubeMXに依存しない。
* ユニットテスト済。

## `#![no_std]`環境で使うライブラリのテスト

* `src/lib.rs`。
    + 通常どおり`#![no_std]`で書き始める。
    + ライブラリの結合テストを書く場所に次のように書く。
        - cfg=testのときに有効になる。
        - std環境のときに有効になる。
        - std 環境のときは`core`の代わりに`std`を使う。
        - tests モジュールを定義し、その中で結合テストを実施する。
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

* `src/queue.rs`。
    + だいたいイデオムどおり。
    + `#[cfg(test)]`はテストのときに有効になる。
    + `mod tests`でテスト用のモジュール名前空間を定義する。
    + `use super::*;`で上位(モジュール自身)の識別子をインポートする。
    + `#[test]`でテスト時に実行する関数を指定する。
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let lock = lock::Lock::Unlocked;
        assert_eq!(lock, lock::Lock::Unlocked);
    }
}
```

* `.cargo/config`。
* クロス環境なので`rustup override set nightly`してNightlyコンパイラを有効にしておく。
* ビルドのときは、利用先のクレート(通常binだろう)で`xargo build --target=thumbv7m-none-eabi`。最新の`libcore`をダウンロードして、ビルドしてくれる。
* テストのときは`cargo test`。


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


## テストの実行

`cargo test`でセルフ環境でテストコードが(stdとともに)コンパイルされてテストが実行される。


## ドキュメントの生成

`cargo doc`で`target/doc/`の下にドキュメントが生成される。

## 使う時

* Cargo.toml に依存関係と入手元を書く。
```
[dependencies]
nostd_tool = {path = "../nostd_tool"}
```
* 使う側の最上位のクレート(`src/main.rs`または`src/lib.rs`)で、外部クレートを取り込む。この時点で `nostd_tool::lock`などの名前で、クレートが提供するモジュールが使えるようになっている。
```rust
extern crate nostd_tool;
```
* 通常は使いやすくするために、短縮名を付ける。こうすると`lock`やその下の`lock::Lock`などが使えるようになる。
```rust
use nostd_tool::lock;
```
これは次の構文糖衣と解釈するとわかりやすい。
```rust
use nostd_tool::lock as lock;
```

* トップレベルクレートではなくサブレベルクレートで取り込むときは、次のように、トップレベルから見たモジュールパスを記述する。
```rust
extern crate nostd_tool;
use self::nostd_tool::lock;
```


## 条件コンパイルのフラグ設定

外部ライブラリを使うとき、外部ライブラリ側で、`std`環境と`no_std`環境で条件コンパイルされるようになっていることがある。
例えば`lazy_static`では、次のようになっている。

```
#[cfg(not(feature="nightly"))]
#[doc(hidden)]
pub mod lazy;

#[cfg(all(feature="nightly", not(feature="spin_no_std")))]
#[path="nightly_lazy.rs"]
#[doc(hidden)]
pub mod lazy;

#[cfg(all(feature="nightly", feature="spin_no_std"))]
#[path="core_lazy.rs"]
#[doc(hidden)]
pub mod lazy;
```

この場合、`core_lazy.rs`を読み込ませる場合は、`feature="spin_no_std"`が有効にならなければならない。ドキュメントが見つけられずに悩んだが、`Cargo.toml`の`[dependencies]`セクションでライブラリを読み込むときに、次のように設定すれば良い。
```
[dependencies]
lazy_static = {version = "*", features = ["spin_no_std"] }
```
公式ドキュメントには、`[features]`セクションに書けと記されていたが、どうもうまく行かなかった。

## 継承

rust では「クラス」を作るためには struct などに impl で実装を追加する。通常のオブジェクト指向言語では実装の再利用で特殊化が使えるときには「継承」を使うことが多いが、rustでは継承は無いので「has-a」関係を用いて実装する。


