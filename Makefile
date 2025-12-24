# XF Tradesmen Makefile
# Common development tasks

.PHONY: dev watch css build test clean help

# Default target
help:
	@echo "XF Tradesmen Development Commands"
	@echo ""
	@echo "Development:"
	@echo "  make dev       - Start full development environment"
	@echo "  make watch     - Start Leptos dev server only"
	@echo "  make css       - Build CSS once"
	@echo "  make css-watch - Watch CSS files"
	@echo ""
	@echo "Testing:"
	@echo "  make test      - Run all tests"
	@echo "  make test-lib  - Run lib-core tests only"
	@echo "  make check     - Run cargo check on all crates"
	@echo ""
	@echo "Building:"
	@echo "  make build     - Build release version"
	@echo "  make clean     - Clean build artifacts"
	@echo ""
	@echo "Database:"
	@echo "  make migrate   - Run database migrations"
	@echo "  make db-test   - Test database connection"
	@echo ""
	@echo "Deployment:"
	@echo "  make deploy    - Deploy to Fly.io"
	@echo "  make logs      - View Fly.io logs"

# Development
dev:
	@echo "Starting development environment..."
	@echo "Run these commands in separate terminals:"
	@echo "  Terminal 1: make css-watch"
	@echo "  Terminal 2: make watch"

watch:
	cargo leptos watch

css:
	npm run build:css

css-watch:
	npm run watch:css

# Testing
test:
	cargo test --all

test-lib:
	cargo test -p lib-core

check:
	cargo check --all

clippy:
	cargo clippy --all -- -D warnings

fmt:
	cargo fmt --all

fmt-check:
	cargo fmt --all -- --check

# Building
build:
	npm run build:css
	cargo leptos build --release

clean:
	cargo clean
	rm -rf target/site

# Database
migrate:
	cargo run -p api -- --migrate

db-test:
	cargo run -p api -- --test-db

# Deployment (Fly.io)
deploy:
	fly deploy

logs:
	fly logs

status:
	fly status

# Docker
docker-build:
	docker build -f Dockerfile.production -t xftradesmen .

docker-run:
	docker run -p 3000:3000 xftradesmen
