# The verb contract of ifconfig-rs (pdt-adlc ADR 0008).
#
# Migrated from a Makefile on 2026-08-18, by reduction: eight of 27 targets are
# gone — help (`just --list` builds the listing), all (frontend + build, which
# `build` already says), the two aggregates ci and pre-push (the contract names
# their contents `check` and `adlc-verify`), the aliases unit and test-frontend,
# and release, which pushes and publishes a GitHub release; a publishing verb
# does not belong beside the gate's verbs.
#
# The eighth removal is the point. `check` was `cargo check` — a compile — and
# the ADLC contract resolver preferred a target of that name, so every
# attestation this repository produced proved that the tests COMPILE
# (pdt-adlc backlog I14, found in seven netray repositories at once).
#
# Two things the gate deliberately does NOT run, both measured on 2026-08-18:
#
#   * the integration test binaries. They construct AppState, which loads
#     data/GeoLite2-City.mmdb at startup and panics without it — the file is not
#     in the repository and `just update-data` downloads it. Five binaries, all
#     red on a clean checkout. `cargo test --lib` is 201 tests and offline, so
#     that is the gate; `test` keeps the rest.
#   * the frontend. `npm ci` needs NODE_AUTH_TOKEN for GitHub Packages, which
#     repo-contract requirement 4 rules out of a gate.
#
# What the gate DOES depend on is frontend/dist: the Rust build embeds it with
# RustEmbed and does not compile without it, and it is gitignored. That is a
# host-owned input, so it is checked and named rather than silently assumed —
# see check-frontend-dist.

app          := "ifconfig-rs"
cargo        := "cargo"
npm          := "npm"
frontend_dir := "frontend"

default: adlc-verify

# --- the contract ------------------------------------------------------------

# What the ADLC gate runs: fmt-check, clippy, the 201 library tests. No network.
adlc-verify: check-frontend-dist lint test-lib

# Everything: lint, the whole Rust suite, the frontend and its tests.
check: lint test frontend

# The whole suite — Rust (library + integration) and frontend.
test: test-rust frontend-test

# clippy + fmt-check.
lint: clippy fmt-check

# --- preconditions -----------------------------------------------------------

# The Rust build embeds frontend/dist (RustEmbed) and it is gitignored.
check-frontend-dist:
    #!/usr/bin/env bash
    if [ ! -d "{{frontend_dir}}/dist" ]; then
        echo "{{frontend_dir}}/dist is missing — the Rust build embeds it (RustEmbed)." >&2
        echo "  run 'just frontend' once; it needs NODE_AUTH_TOKEN for GitHub Packages." >&2
        exit 1
    fi

# --- rust --------------------------------------------------------------------

# Library tests only — offline, no GeoIP database needed.
test-lib:
    {{cargo}} test --lib --no-fail-fast

# Library plus integration tests. The latter need data/GeoLite2-City.mmdb.
test-rust: test-lib
    {{cargo}} test --no-fail-fast

clippy:
    {{cargo}} clippy -- -D warnings

fmt-check:
    {{cargo}} fmt -- --check

fmt:
    {{cargo}} fmt

# --- build -------------------------------------------------------------------

# Release binary (builds the frontend first — it is embedded).
build: frontend
    {{cargo}} build --release

# Build and run the release binary.
run: build
    ./target/release/{{app}}

# Local dev server on :8080.
dev:
    {{cargo}} run -- ifconfig.dev.toml

clean:
    {{cargo}} clean
    rm -rf {{frontend_dir}}/dist {{frontend_dir}}/node_modules

# --- frontend (network: GitHub Packages needs NODE_AUTH_TOKEN) ---------------

frontend-install:
    cd {{frontend_dir}} && {{npm}} ci

# npm ci + vite build.
frontend: frontend-install
    cd {{frontend_dir}} && {{npm}} run build

# Vite dev server with API proxy.
frontend-dev:
    cd {{frontend_dir}} && {{npm}} run dev

# vitest.
frontend-test: frontend-install
    cd {{frontend_dir}} && npx vitest run --passWithNoTests

# --- docker ------------------------------------------------------------------

docker:
    docker build . --tag {{app}}:latest

docker-run:
    docker run --rm -p 8080:8080 {{app}}:latest

# --- project-specific --------------------------------------------------------

# Docker-based integration tests.
integration:
    make -C tests integration

# Playwright E2E tests.
acceptance:
    make -C tests acceptance

# Criterion benchmarks.
bench:
    {{cargo}} bench

# Refresh all enrichment data files (see data/README.md).
update-data:
    make -C data get_all

# Project statistics.
stats:
    #!/usr/bin/env bash
    version=$(grep -m1 '^version' Cargo.toml | sed 's/.*"\(.*\)"/\1/')
    sha=$(git rev-parse --short HEAD 2>/dev/null || echo unknown)
    echo "─── {{app}} v${version} (${sha}) ───"
    echo ""
    echo "Backend (Rust):"
    find src -name '*.rs' | xargs wc -l | tail -1 | awk '{printf "  Lines of code:  %s (across ", $1}'
    find src -name '*.rs' | wc -l | awk '{printf "%s files)\n", $1}'
    find src -name '*.rs' -exec grep -cE '#\[test\]|#\[tokio::test\]' {} + 2>/dev/null | awk -F: '{s+=$2} END {printf "  Unit tests:     %d\n", s}'
    find tests -name '*.rs' -exec grep -cE '#\[test\]|#\[tokio::test\]' {} + 2>/dev/null | awk -F: '{s+=$2} END {printf "  Integration:    %d\n", s}'
    echo ""
    echo "Git:"
    echo "  Commits:       $(git rev-list --count HEAD 2>/dev/null || echo '?')"
    echo "  Branch:        $(git branch --show-current 2>/dev/null || echo '?')"
