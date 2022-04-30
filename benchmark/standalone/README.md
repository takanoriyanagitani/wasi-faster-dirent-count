# Performance comparison

## native vs native/readdir

- native: 4 ms
- native/readdir: 5 ms

## wasmtime vs wasmtime/readdir

- wasmtime: 24 ms(3x times faster)
- wasmtime/readdir: 83 ms

## wasmer vs wasmer/readdir

- wasmer: 28 ms(5x times faster)
- wasmer/readdir: 148 ms

| time | ls | native | wasmtime | wasmer | native/readdir | wasmtime/readdir | wasmer/readdir |
|:----:|:--:|:------:|:--------:|:------:|:--------------:|:----------------:|:--------------:|
| real | 4  | 4      | 24       | 28     | 5              | 83               | 148            |
| user | 3  | 0      | 12       | 12     | 1              | 26               |  68            |
| sys  | 3  | 3      | 17       | 16     | 4              | 62               |  80            |
