# wasi-faster-dirent-count
count known files; will NOT use readdir/getdents64 (0201.txt, 0202.txt, 0203.txt, ... , 0229.txt -> 28)

## Performance Comparison(Example)

- wasmtime: 3x times faster(83 ms -> 24 ms)
- wasmer: 5x times faster(148 ms -> 28 ms)
