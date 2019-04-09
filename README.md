# issue検証用リポジトリ

## issue

現在の [Cannyls](https://github.com/frugalos/cannyls) （正確には [v0.9.3](<https://github.com/frugalos/cannyls/tree/87599402837317bb03efd842db2c07b97488edce>) 以前のCannyls) は、異常終了時に、[ジャーナル領域](<https://github.com/frugalos/cannyls/wiki/Journal-Region>)の末尾を表す特別なレコード [EndOfRecords ](<https://github.com/frugalos/cannyls/wiki/Storage-Format#3-2-1-end_of_records%E3%83%AC%E3%82%B3%E3%83%BC%E3%83%89>)の書き込みに失敗し、書き込みに失敗したlusfファイルはそれ以降 `StorageCorrupted` エラーとなり使うことができない。

## 本リポジトリの目的▷issueの再現

```bash
$ cargo build --release
$ ./loop.sh
...
+ prog=./target/release/miss_eor
+ rm -f test.lusf
++ seq 1 5000
+ for i in '`seq 1 5000`'
+ echo 1
1
+ ./target/release/miss_eor
unH = 0, H = 0, T = 0
+ (( i % 10 ))
+ for i in '`seq 1 5000`'
+ echo 2
2
...
...
...
+ ./target/release/miss_eor
unH = 5759360, H = 5759360, T = 377533
+ (( i % 10 ))
+ for i in '`seq 1 5000`'
+ echo 8
8
+ ./target/release/miss_eor
Error: Error(TrackableError { kind: StorageCorrupted, cause: Some(Cause(StringError("assertion failed: `!self.is_second_lap`"))), history: History([Location { module_path: "cannyls::storage::journal::ring_buffer", file: "/home/yuezato/.cargo/git/checkouts/cannyls-6d66531919646a3c/8759940/src/storage/journal/ring_buffer.rs", line: 327, message: "" }, Location { module_path: "cannyls::storage::journal::region", file: "/home/yuezato/.cargo/git/checkouts/cannyls-6d66531919646a3c/8759940/src/storage/journal/region.rs", line: 334, message: "" }, Location { module_path: "cannyls::storage::journal::region", file: "/home/yuezato/.cargo/git/checkouts/cannyls-6d66531919646a3c/8759940/src/storage/journal/region.rs", line: 96, message: "" }, Location { module_path: "cannyls::storage::builder", file: "/home/yuezato/.cargo/git/checkouts/cannyls-6d66531919646a3c/8759940/src/storage/builder.rs", line: 189, message: "" }, Location { module_path: "cannyls::storage", file: "/home/yuezato/.cargo/git/checkouts/cannyls-6d66531919646a3c/8759940/src/storage/mod.rs", line: 114, message: "" }, Location { module_path: "miss_eor", file: "src/main.rs", line: 23, message: "" }]) })
```

### 再現できた環境

* Mac OSX 10.13.6 on SSD,
* Ubuntu 16.04 (Kernel version 4.4.0) on SSD, HDD
* Ubuntu 18.10 (Kernel version 4.18.0) on SSD, HDD



### 行っていること

主な処理は次の部分:

```rust
for i in 0..10000 {
    let len: usize = rng.gen_range(0, 1024);
    let lump_id = LumpId::new(rng.gen_range(0, i+1));
    let embed_data = track!(LumpData::new_embedded(vec![42; len]))?;
    track!(storage.put(&lump_id, &embed_data))?;
}
// simulate crash
std::mem::forget(storage);
```

小規模の埋め込みputを、lumpidを乱択生成しながら行っているのみである。
ただし、crashを模倣するために、`storage`構造体を正常にdropさせる代わりに`forget`している。