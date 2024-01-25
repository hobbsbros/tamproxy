# TAMProxy Makefile

TEENSY = TEENSY41

# Silence make
.SILENT:

# Alias for `hex`
all: hex

# Compile `tamproxy.hex`
hex:
	cargo objcopy --release -- -O ihex tamproxy.hex

upload: hex
	echo "Press the RESET button on your Teensy to upload."
	teensy_loader_cli/teensy_loader_cli --mcu=$(TEENSY) -w tamproxy.hex

clean:
	cargo clean
	rm *.hex