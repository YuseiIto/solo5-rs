LIBNAME := http_server
TAP_DEV := tap1
TAP_IP := 10.0.1.1
CIDER_LEN:= 24
HVT_ARGS := --mem=1024 --net:net0=$(TAP_DEV)

include ../Common.mk
.PHONY: init
init: 
	sudo ip tuntap add dev $(TAP_DEV) mode tap
	sudo ip link set $(TAP_DEV) up
	sudo ip addr add $(TAP_IP)/$(CIDER_LEN) dev $(TAP_DEV)
	sudo ip route add dev $(TAP_DEV) $(TAP_IP)

.PHONY: deinit
deinit:
	sudo ip addr del $(TAP_IP)/$(CIDER_LEN) dev $(TAP_DEV)
	sudo ip tuntap del dev $(TAP_DEV) mode tap

