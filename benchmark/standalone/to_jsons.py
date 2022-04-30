import json
import functools
import sys

reader2next = lambda r: next(r, "0").strip()
reader2num = lambda r: float(reader2next(r))
reader2rus = lambda r: dict(
  real=reader2num(r),
  user=reader2num(r),
  sys=reader2num(r),
)
filter_dict = lambda d: dict == type(d) and d.get("real") and d or None
reader2row = lambda r: filter_dict(reader2rus(r))

rows2next = lambda rows: next(rows, None)
rows2dict = lambda rows: dict(
  ls=rows2next(rows),
  native=rows2next(rows),
  native_readdir=rows2next(rows),
  wasmtime=rows2next(rows),
  wasmtime_readdir=rows2next(rows),
  wasmer=rows2next(rows),
  wasmer_readdir=rows2next(rows),
)
filter_sample = lambda d: dict == type(d) and d.get("native") and d or None
rows2sample = lambda rows: filter_sample(rows2dict(rows))

functools.reduce(lambda state,f: f(state), [
  lambda r: iter(functools.partial(reader2row, r), None),
  lambda rows: iter(functools.partial(rows2sample, rows), None),
  lambda samples: map(json.dumps, samples),
  lambda i: map(print, i),
  lambda prints: sum(1 for _ in prints),
], sys.stdin)
