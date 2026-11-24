.DEFAULT_GOAL := help

# Crate whose binary `run`/`dev` launch (one workspace member per learning milestone)
CRATE ?= xor
IMAGE := cyore-$(CRATE)
CARGO := $(shell which cargo 2>/dev/null || echo $(HOME)/.cargo/bin/cargo)
OPENSPEC := $(shell which openspec 2>/dev/null || echo npx --yes @fission-ai/openspec@1.14.2)

.PHONY: help build run dev test bench lint fmt fmt-check clean check spec size docker-build docker-run

help:
	@printf '%s\n' 'Cyore commands (CRATE=$(CRATE); override with CRATE=<member>):'
	@printf '%s\n' '  make help          Show available commands'
	@printf '%s\n' '  make build         Build the optimised release binary for CRATE'
	@printf '%s\n' '  make run           Build and run the release binary for CRATE'
	@printf '%s\n' '  make dev           Run CRATE with the debug profile (fast compile)'
	@printf '%s\n' '  make test          Run all workspace tests'
	@printf '%s\n' '  make bench         Run benchmarks (cargo bench)'
	@printf '%s\n' '  make lint          Run clippy with warnings as errors'
	@printf '%s\n' '  make fmt           Format all Rust code'
	@printf '%s\n' '  make size          Show the release binary size (Stage 4 target: 5-30 MB)'
	@printf '%s\n' '  make spec          Validate OpenSpec specs and changes (strict)'
	@printf '%s\n' '  make docker-build  Build the container image for CRATE'
	@printf '%s\n' '  make docker-run    Run the container image for CRATE'
	@printf '%s\n' '  make clean         Remove build artifacts'
	@printf '%s\n' '  make check         Run fmt check, clippy, tests, and release build'

build:
	@printf '%s\n' 'Compiling $(CRATE) (release)...'
	@$(CARGO) build --release -p $(CRATE)
	@printf '%s\n' 'Build complete: ./target/release/$(CRATE)'

run: build
	@./target/release/$(CRATE)

dev:
	@$(CARGO) run -p $(CRATE)

test:
	@printf '%s\n' 'Running workspace test suite...'
	@$(CARGO) test --workspace

bench:
	@$(CARGO) bench --workspace

lint:
	@$(CARGO) clippy --workspace --all-targets -- -D warnings

fmt:
	@$(CARGO) fmt --all

fmt-check:
	@$(CARGO) fmt --all -- --check

size: build
	@ls -lh target/release/$(CRATE) | awk '{print $$5, $$9}'

spec:
	@printf '%s\n' 'Validating OpenSpec specs and changes...'
	@$(OPENSPEC) validate --all --strict --no-interactive

docker-build:
	@docker build --build-arg CRATE=$(CRATE) -t $(IMAGE):local .

docker-run: docker-build
	@docker run --rm $(IMAGE):local

clean:
	@printf '%s\n' 'Cleaning up build artifacts...'
	@$(CARGO) clean
	@printf '%s\n' 'Clean complete.'

check: fmt-check lint test build
	@printf '%s\n' 'All quality checks passed.'
