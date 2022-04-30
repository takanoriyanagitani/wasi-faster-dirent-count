import json
import sys
import os
import functools

map_row = lambda time_typ, row, runtime: row[runtime][time_typ]

new_mapper = lambda time_typ: functools.partial(map_row, time_typ)

mapper = new_mapper(os.getenv("ENV_TYPE") or "real")

row2mapd = lambda row: {
	key: mapper(row, key) for key in [
	  "ls",
	  "native",
	  "native_readdir",
	  "wasmtime",
	  "wasmtime_readdir",
	  "wasmer",
	  "wasmer_readdir",
	]
}

functools.reduce(lambda state,f:f(state), [
  lambda r: map(json.loads, r),
  lambda rows: map(row2mapd, rows),
  lambda rows: map(json.dumps, rows),
  lambda jsons: map(print, jsons),
  lambda prints: sum(1  for _ in prints),
], sys.stdin)
