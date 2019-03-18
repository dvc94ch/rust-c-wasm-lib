LIB_PATH=../../target/debug
LD_FLAGS=-L$(LIB_PATH) -Wl,--gc-sections,-rpath,$(LIB_PATH) -lpthread -ldl

run: build-dynamic build-static
	./fib-dynamic
	./fib-static

build-dynamic:
	gcc main.c -I.. $(LD_FLAGS) -l:libfib_c.so -o fib-dynamic

build-static:
	gcc main.c -I.. $(LD_FLAGS) -l:libfib_c.a -o fib-static
