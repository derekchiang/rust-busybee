# Rust parameters
SRC ?= src
BUILD ?= build
RUSTC ?= rustc
RUSTC_FLAGS ?= -O -g
LIBBUSYBEE_SRC ?= $(SRC)/busybee/lib.rs
TEST_SRC ?= $(SRC)/busybee/test.rs


all: lib test

$(BUILD):
	mkdir -p $(BUILD)

lib: $(BUILD)
	$(RUSTC) $(RUSTC_FLAGS) --out-dir $(BUILD) $(LIBBUSYBEE_SRC)

test: lib
	$(RUSTC) $(RUSTC_FLAGS) --test -L $(BUILD) -L /usr/local/lib --out-dir $(BUILD) $(TEST_SRC)

clean:
	rm -rf $(BUILD)

.PHONY: all test clean
