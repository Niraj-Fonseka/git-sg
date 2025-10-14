# Makefile for building and running the gitsg project

# Variables
CARGO = cargo

# Default target
all: build

# Build the project
build:
	$(CARGO) build --release

# Run the project
run:
	$(CARGO) run

# Clean the build artifacts
clean:
	$(CARGO) clean