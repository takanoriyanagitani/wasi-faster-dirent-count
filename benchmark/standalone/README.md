# Performance comparison

| time | ls | native | wasmtime | wasmer | native/readdir | wasmtime/readdir | wasmer/readdir |
|:----:|:--:|:------:|:--------:|:------:|:--------------:|:----------------:|:--------------:|
| real | 4  | 4      | 24       | 28     | 5              | 83               | 148            |
| user | 3  | 0      | 12       | 12     | 1              | 26               |  68            |
| sys  | 3  | 3      | 17       | 16     | 4              | 62               |  80            |
