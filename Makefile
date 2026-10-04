# Makefile — standard commands. `make gate` = the doctrine enforcer; `make check` = Rust.
SHELL := /usr/bin/env bash
LOCAL_RUN := python3 -I -B "$(CURDIR)/scripts/local_environment.py" --

.PHONY: help gate check fmt clippy test wasm push-due book hooks bootstrap update-scaffold probes toolchain

help:
	@echo "make toolchain       - install the selected Rust channel into local stores, without self-update"
	@echo "make gate            - run the doctrine enforcer (scripts/check_doctrines.sh)"
	@echo "make check           - cargo fmt --check + clippy (deny warnings) + test"
	@echo "make fmt             - cargo fmt --all"
	@echo "make clippy          - cargo clippy --all-targets -- -D warnings"
	@echo "make test            - cargo test --all"
	@echo "make book            - build the mdBook (requires mdbook)"
	@echo "make hooks           - install the git hooks (core.hooksPath=.githooks)"
	@echo "make bootstrap       - first-time project bootstrap"
	@echo "make update-scaffold - pull the latest bedrock spine (set URL=<bedrock-repo>)"
	@echo "make wasm            - the wasm-viewer smoketest: build sc-units + sc-core + sc-measure for wasm32-unknown-unknown"
	@echo "make push-due        - is an exceptional push owed? (CI/doctrine paths changed since origin)"
	@echo "make probes          - run every probe suite under docs/tasks/artifacts/ (scratch on this volume)"

gate:
	$(LOCAL_RUN) bash scripts/check_doctrines.sh

toolchain:
	$(LOCAL_RUN) rustup toolchain install --profile minimal --component rustfmt --component clippy --target wasm32-unknown-unknown --no-self-update

check:
	$(LOCAL_RUN) cargo fmt --all -- --check
	$(LOCAL_RUN) cargo clippy --all-targets --all-features -- -D warnings
	$(LOCAL_RUN) cargo test --all

fmt:
	$(LOCAL_RUN) cargo fmt --all

clippy:
	$(LOCAL_RUN) cargo clippy --all-targets --all-features -- -D warnings

test:
	$(LOCAL_RUN) cargo test --all

# ROADMAP.md §7.3: the `wasm-viewer` runtime profile is a documented capability subset, and the G0 CI
# clause is a real cross-compilation, not a host `cargo check`. Run this before claiming a crate is
# browser-capable.
wasm:
	$(LOCAL_RUN) cargo build --target wasm32-unknown-unknown -p sc-units -p sc-core -p sc-measure
	@echo "wasm-viewer smoketest: sc-units + sc-core + sc-measure build for wasm32-unknown-unknown"

# Is an exceptional push owed? CI and doctrine changes are unverified until a runner executes them,
# so they push immediately regardless of the 400-commit cadence (COMMIT.md -> Push cadence).
# Reports and never fails the build: exit 1 means "a push is due", not "an error occurred".
push-due:
	-scripts/check_push_due.sh

book:
	$(LOCAL_RUN) mdbook build docs/book

hooks:
	git config core.hooksPath .githooks
	@echo "git hooks activated (core.hooksPath=.githooks)"

bootstrap:
	scripts/bootstrap.sh

update-scaffold:
	scripts/update_scaffold.sh $(URL)

# Scratch stays on the REPOSITORY volume. The inherited probe suites call `mktemp -d`, which
# otherwise resolves to the system temp directory — another volume on this machine — so their
# throwaway repositories and logs leave the project (defect D16). The path is derived from
# $(CURDIR) at run time and never hardcoded, so the repository stays relocatable.
probes:
	@$(LOCAL_RUN) bash -c 'rc=0; found=0; \
	for p in $$(find docs/tasks/artifacts -type f -name "run_*probe*.sh" | sort); do \
	  found=$$((found+1)); echo "== $$p"; \
	  bash "$$p" || rc=1; \
	done; \
	if [ "$$found" -eq 0 ]; then echo "make probes: no probe suites found under docs/tasks/artifacts/"; exit 1; fi; \
	if [ "$$rc" -ne 0 ]; then echo "make probes: FAILED (at least one suite reported failures)"; exit 1; fi; \
	echo "make probes: $$found suite(s) green (scratch under $$TMPDIR)"'
