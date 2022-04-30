#!/bin/bash

main(){
	mkdir --parents ./testdata/root.d/data.d/2022 || exit 1
    cd ./testdata/root.d/data.d/2022
	seq 1 12 | xargs printf '%02d\n' | while read mon; do
	  seq 1 31 | xargs printf $mon-'%02d\n' | xargs touch || exit 1
	done
	rm --force --verbose ./02-3[01]
	rm --force --verbose ./0[469]-31
	rm --force --verbose ./11-31
}

main
