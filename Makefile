# Makefile for Aurora Visualizer

# --- Configuration ---
.DEFAULT_GOAL := all
CXX_COMPILER ?= g++
CMAKE_BUILD_DIR := build
CMAKE_BUILD_TYPE ?= Debug
EXECUTABLE_NAME := AuroraVisualizer
SCRIPTS_DIR := scripts

# --- Phony Targets for commands ---
.PHONY: all build configure run test clean help pre-build-hook

# --- Main Targets ---

## Builds the application and tests (default)
all: build

## Configure the project using CMake
configure:
	@echo "--- Configuring project (Build Type: ${CMAKE_BUILD_TYPE}) ---"
	@cmake -S . -B ${CMAKE_BUILD_DIR} -DCMAKE_CXX_COMPILER=${CXX_COMPILER} -DCMAKE_BUILD_TYPE=${CMAKE_BUILD_TYPE}

## Build the project if not already built
build: pre-build-hook ${CMAKE_BUILD_DIR}/CMakeCache.txt
	@echo "--- Building project ---"
	@if cmake --build ${CMAKE_BUILD_DIR}; then \
		${SCRIPTS_DIR}/backup.sh post-success; \
	fi

# If build dir doesn't exist, run configure first, then build
${CMAKE_BUILD_DIR}/CMakeCache.txt:
	@$(MAKE) configure

## Build and run the main application
run: build
	@echo "--- Running application ---"
	@${CMAKE_BUILD_DIR}/${EXECUTABLE_NAME}

## Build and run tests
test: build
	@echo "--- Running tests ---"
	@cd ${CMAKE_BUILD_DIR} && ctest --output-on-failure

## Remove all build artifacts
clean:
	@echo "--- Cleaning build directory ---"
	@rm -rf ${CMAKE_BUILD_DIR}

## Display this help message
help:
	@echo "Usage: make [target]"
	@echo ""
	@echo "Targets:"
	@awk '/^## /{printf "  %-12s %s\n", substr($$1, 3), substr($$0, index($$0, $$2))}' $(MAKEFILE_LIST)

# --- Hooks ---
pre-build-hook:
	@chmod +x ${SCRIPTS_DIR}/backup.sh
	@${SCRIPTS_DIR}/backup.sh pre-build