# Визначення змінних
CARGO := cargo
TARGET_DIR := target
RELEASE_DIR := $(TARGET_DIR)/release
DEBUG_DIR := $(TARGET_DIR)/debug
BINARY_NAME := gig

.PHONY: all build install init uninit uninstall clean

all: build

build:
	@echo "Build the application"
	$(CARGO) build --release

install: build init
	@echo "Installing the application"

init:
	@echo "Initializing the application in the root directory"
	sudo cp $(RELEASE_DIR)/$(BINARY_NAME) /usr/local/bin

uninit:
	@echo "Deinitialize the application from the root directory"
	sudo rm -f /usr/local/bin/$(BINARY_NAME)

uninstall: uninit
	@echo "Delete application"
	$(MAKE) clean

clean:
	@echo "Clean up the build directory"
	$(CARGO) clean

debug:
	@echo "Building an application in debug mode"
	$(CARGO) build

run: build
	@echo "Launching the application"
	./$(RELEASE_DIR)/$(BINARY_NAME)

test:
	@echo "Running tests"
	$(CARGO) test