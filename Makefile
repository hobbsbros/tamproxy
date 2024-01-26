# Makefile

TEENSY = TEENSY41

VERSION = 0.1.0

PROJECTNAME = $(notdir $(CURDIR))

HEXFILE = $(PROJECTNAME)-$(VERSION).hex

# Silence make output
.SILENT:

# Alias for `make hex`
all: hex

# Compile HEX file
hex:
	cargo objcopy -- -O ihex $(HEXFILE)
	size $(HEXFILE)

upload: hex
	echo "Press the RESET button on your Teensy to upload."
	teensy_loader_cli/teensy_loader_cli --mcu=$(TEENSY) -w $(HEXFILE)

clean:
	cargo clean
	rm *.hex