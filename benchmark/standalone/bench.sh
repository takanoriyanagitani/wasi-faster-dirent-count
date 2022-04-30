#!/bin/bash

bench(){
  ./main.sh 2>&1 |
    sed --silent \
      --expression /real/p \
  	  --expression /user/p \
  	  --expression /sys/p 
}

bench10(){
  rm --force bench.txt
  
  bench > /dev/null
  
  bench >> bench.txt
  bench >> bench.txt
  bench >> bench.txt
  bench >> bench.txt
  bench >> bench.txt
  bench >> bench.txt
  bench >> bench.txt
  bench >> bench.txt
  bench >> bench.txt
  bench >> bench.txt
}

bench10

cat bench.txt |
  sed \
    --expression 's,0m,,' \
	--expression 's,real.,,' \
	--expression 's,user.,,' \
	--expression 's,sys.,,' \
	--expression 's,s$,,' |
  python3 to_jsons.py |
  cat > bench.jsonl

printf '%s\n' real user sys | while read rus; do
  cat bench.jsonl |
    ENV_TYPE=$rus python3 map_jsons.py |
	cat > bench.$rus.jsonl
done

printf '%s\n' real user sys | while read rus; do
  cat bench.$rus.jsonl |
    python3 jsons2csv.py |
	cat > bench.$rus.csv
done
