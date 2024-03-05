# WaTra

和暦 <-> 西暦　コマンドライン相互変換ツール

## Usage

### コマンドとして  

- `watra 平成15年`や`watra R2`と入力すると西暦を表示します
- `watra 2018`や`watra 1956年`と入力すると和暦を表示します
- `format`オプションで言語の漢字表記とイニシャル表記を切り替えられます  
  - 漢字表記のときは元年、イニシャル表記のときは1年と表記する

### 対話モードで

`watra [option]`で対話モードで実行できます

- 対話モード中は直接入力したものを変換します
- 表記切り替えは`i`でイニシャル表記、`j`で漢字表記に切り替えられます
- `q`で対話モードを終了できます

## Installation

### For Rustacean

1. このリポジトリをクローン
2. クローンファイルを解凍
3. 解凍したディレクトリで`cargo install --path .`を実行

### Other User (準備中)

リリースにあるインストーラをダウンロードして実行

## 対応年号

明治元年(1868)から令和まで対応しています。

### License

Copyright (c) 2024 PARADISO

`WaTra` is made available under the terms of the MIT license.

See the [License](./LICENSE) files for license details.
