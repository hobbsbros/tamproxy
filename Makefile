# TAMProxy Makefile

TEENSY = TEENSY41

VERSION = 0.1.0

HEXFILE = tamproxy-$(VERSION).hex

# Silence make
.SILENT:

# Alias for `hex`
all: hex

# Compile `tamproxy.hex`
hex:
	cargo objcopy -- -O ihex $(HEXFILE)
	size $(HEXFILE)

upload: hex
	echo "Press the RESET button on your Teensy to upload."
	teensy_loader_cli/teensy_loader_cli --mcu=$(TEENSY) -w $(HEXFILE)

clean:
	cargo clean
	rm *.hex