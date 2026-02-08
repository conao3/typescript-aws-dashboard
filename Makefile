.PHONY: help
help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'

.PHONY: setup
setup: ## setup project
	pnpm -C frontend install

.PHONY: build
build: ## build project
	cargo build --manifest-path backend/Cargo.toml
	pnpm -C frontend build

.PHONY: build-release
build-release: ## build project in release mode
	cargo build --release --manifest-path backend/Cargo.toml
	pnpm -C frontend build

.PHONY: test
test: ## run tests
	cargo test --manifest-path backend/Cargo.toml
	pnpm -C frontend test

.PHONY: fmt
fmt: ## format code
	nix fmt

.PHONY: lint
lint: ## lint code
	cargo clippy --manifest-path backend/Cargo.toml
	pnpm -C frontend lint

.PHONY: check
check: ## check code
	nix flake check

.PHONY: clean
clean: ## clean build artifacts
	cargo clean --manifest-path backend/Cargo.toml
	rm -rf frontend/node_modules
	rm -rf frontend/.output

.PHONY: dev
dev: ## start frontend development server
	pnpm -C frontend dev

.PHONY: dev-backend
dev-backend: ## start backend server
	cargo run --manifest-path backend/Cargo.toml

.PHONY: watch
watch: ## watch and restart backend server on changes
	cargo watch -x 'run --manifest-path backend/Cargo.toml'

.PHONY: db-up
db-up: ## start postgres database
	docker compose up -d

.PHONY: db-down
db-down: ## stop postgres database
	docker compose down

.PHONY: db-logs
db-logs: ## show postgres logs
	docker compose logs -f postgres

.PHONY: db-reset
db-reset: ## reset postgres database
	docker compose down -v
	docker compose up -d
