CC:= x86_64-solo5-none-static-cc
LD:= x86_64-solo5-none-static-ld
HVT:= solo5-hvt
ELFTOOL := solo5-elftool
LIBNAME := http_client
LIB_KERNEL := target/x86_64-unknown-none/debug/lib$(LIBNAME).a
KERNEL_PATH := kernel.hvt

kernel: manifest.o $(LIB_KERNEL) lib.o
	$(LD) -z solo5-abi=hvt -o $(KERNEL_PATH) $(LIB_KERNEL) manifest.o ~/solo5/lib.o

$(LIB_KERNEL): src/**.rs src/*.rs Cargo.toml Cargo.lock
	cargo +nightly build -Zbuild-std --target x86_64-unknown-none

manifest.c: manifest.json
	$(ELFTOOL) gen-manifest manifest.json manifest.c

manifest.o: manifest.c
	$(CC) -z solo5-api=hvt -c -o manifest.o manifest.c

.PHONY: lib.o
lib.o:
	$(CC) -c ~/solo5/bindings/lib.c

.PHONY: build # yet another alias for 'make kernel'
build:
	$(MAKE) kernel

TAP_DEV := tap0

.PHONY: init
init: 
	sudo ip tuntap add dev $(TAP_DEV) mode tap
	sudo ip link set $(TAP_DEV) up
	sudo ip addr add 10.0.0.1/24 dev $(TAP_DEV)
	sudo ip route add dev $(TAP_DEV) 10.0.0.1

.PHONY: deinit
deinit:
	sudo ip addr del 10.0.0.1/24 dev $(TAP_DEV)
	sudo ip tuntap del dev $(TAP_DEV) mode tap

.PHONY: run
run: 
	$(HVT) --mem=1024 --net:net0=$(TAP_DEV) -- $(KERNEL_PATH)

.PHONY: dev
dev: kernel run

.PHONY: clean
clean:
	cargo clean
	- rm manifest.c manifest.o $(KERNEL_PATH) lib.o
	
