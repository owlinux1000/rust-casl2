# CASL2 assembler written in Rust

Rustの勉強用に書いたCASL2アセンブラ

## 使い方

```
$ rust-casl2 example/sample.casl2
[*] Create object file `example/sample`
$ cat example/sample
0003
1010
0005
1020
0006
1421
000a
00ff
```

ここで生成したファイルは，[rust-comet2](https://git.alicemacs.com/chihiro/rust-comet2) のコマンドラインツールで読み込むと実行することができます :thums_up:

## 補足

* IN, OUTのマクロ非対応
* **Rust初心者なのでRustっぽい書き方を教えてください**

