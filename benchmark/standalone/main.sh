#!/bin/bash

run_ls(){
	echo
	echo ---- ls ----
	cd $yearroot
	time ls | wc --lines
}

run_native(){
	echo
	echo ---- native ----
	time ./target/release/standalone
}

run_native_readdir(){
	echo
	echo ---- native readdir ----
	time ./count-readdir/target/release/count-readdir
}

run_wasmtime(){
	echo
	echo ---- wasmtime ----
	time wasmtime \
	  run \
	    --env ENV_YEAR_ROOT=/guest.d \
		--mapdir /guest.d::$yearroot \
		./target/wasm32-wasi/release/standalone.wasm
}

run_wasmer(){
	echo
	echo ---- wasmer ----
	time wasmer \
	  run \
	    --env ENV_YEAR_ROOT=/guest.d \
		--mapdir /guest.d::$yearroot \
		./target/wasm32-wasi/release/standalone.wasm
}

run_wasmtime_readdir(){
	echo
	echo ---- wasmtime readddir ----
	time wasmtime \
	  run \
	    --env ENV_YEAR_DIR=/guest.d \
		--mapdir /guest.d::$yearroot \
		./count-readdir/target/wasm32-wasi/release/count-readdir.wasm
}

run_wasmer_readdir(){
	echo
	echo ---- wasmer readddir ----
	time wasmer \
	  run \
	    --env ENV_YEAR_DIR=/guest.d \
		--mapdir /guest.d::$yearroot \
		./count-readdir/target/wasm32-wasi/release/count-readdir.wasm
}

main(){
	cd $wd
	run_ls || exit 1

	cd $wd
	run_native || exit 1

	cd $wd
	run_native_readdir || exit 1

	cd $wd
	run_wasmtime || exit 1

	cd $wd
	run_wasmtime_readdir || exit 1

	cd $wd
	run_wasmer || exit 1

	cd $wd
	run_wasmer_readdir || exit 1
}

yearroot=./testdata/root.d/data.d/2022
wd=$PWD

main
