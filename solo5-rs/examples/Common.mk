ifndef LIBNAME
	$(error Set LIBNAME before importing Common.mk)
endif

ifndef HVT_ARGS
	HVT_ARGS:= --mem=1024
endif

CC:= x86_64-solo5-none-static-cc
LD:= x86_64-solo5-none-static-ld
HVT:= solo5-hvt
ELFTOOL := solo5-elftool
KERNEL_PATH := kernel.hvt
CARGO := cargo +nightly
CARGO_FLAGS := -Zbuild-std --target x86_64-unknown-none
LIB_KERNEL := target/x86_64-unknown-none/debug/lib$(LIBNAME).a

kernel: manifest.o $(LIB_KERNEL) lib.o
	$(LD) -z solo5-abi=hvt -o $(KERNEL_PATH) $(LIB_KERNEL) manifest.o ~/solo5/lib.o

$(LIB_KERNEL): src/**.rs src/*.rs Cargo.toml Cargo.lock
	$(CARGO) build $(CARGO_FLAGS) 

manifest.c: manifest.json
	$(ELFTOOL) gen-manifest manifest.json manifest.c

manifest.o: manifest.c
	$(CC) -z solo5-api=hvt -c -o manifest.o manifest.c

.PHONY: lib.o
lib.o:
	$(CC) -c ~/solo5/bindings/lib.c

.PHONY: build # yet another alias for 'make kernel'
build: kernel

.PHONY: run
run: ${BLOCK}
	$(HVT) $(HVT_ARGS) -- $(KERNEL_PATH) $(RUN_ARGS)

.PHONY: dev
dev: kernel run

.PHONY: clean
clean:
	cargo clean
	- rm -f manifest.c manifest.o $(KERNEL_PATH) lib.o
	
