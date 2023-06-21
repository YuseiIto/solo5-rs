CC:= x86_64-solo5-none-static-cc
LD:= x86_64-solo5-none-static-ld
HVT:= solo5-hvt
LIBNAME := do_nothing
LIB_KERNEL := target/x86_64-unknown-none/debug/lib$(LIBNAME).a
KERNEL_PATH := kernel.hvt

$(LIB_KERNEL): src/**.rs src/*.rs Cargo.toml Cargo.lock build.rs
	cargo +nightly build --target x86_64-unknown-none

manifest.c: manifest.json
	solo5-elftool gen-manifest manifest.json manifest.c

manifest.o: manifest.c
	$(CC) -z solo5-api=hvt -c -o manifest.o manifest.c
kernel: $(LIB_KERNEL) manifest.o lib.o
	$(LD) -z solo5-abi=hvt -o $(KERNEL_PATH) $(LIB_KERNEL) manifest.o ~/solo5/lib.o

.PHONY: lib.o
lib.o:
	gcc -c ~/solo5/bindings/lib.c

.PHONY: build # yet another alias for 'make kernel'
build:
	$(MAKE) kernel

.PHONY: run
run: ${BLOCK}
	$(HVT) --mem=1024 -- $(KERNEL_PATH)

.PHONY: dev
dev: kernel run
