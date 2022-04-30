import json
import csv
import os
import sys
import functools

rows_writer = lambda writer, rows: writer.writerows(rows)
new_csv_writer = lambda f, keys: csv.DictWriter(f, keys)
keys2csv_writer = lambda f: functools.partial(new_csv_writer, f)
keys2stdout = keys2csv_writer(sys.stdout)

writer = keys2stdout([
  "ls",
  "native",
  "wasmtime",
  "wasmer",
  "native_readdir",
  "wasmtime_readdir",
  "wasmer_readdir",
])
rows2writer = lambda writer: functools.partial(rows_writer, writer)
rows_writer = rows2writer(writer)

functools.reduce(lambda state,f:f(state), [
  lambda lines: map(json.loads, lines),
  lambda rows: rows_writer(rows),
], sys.stdin)
