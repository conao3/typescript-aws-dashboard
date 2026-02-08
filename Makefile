.PHONY: help
help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'

.PHONY: setup
setup: ## setup project
	nix develop -c pnpm install -C frontend

.PHONY: build
build: ## build project
	nix develop -c cargo build --manifest-path backend/Cargo.toml
	nix develop -c pnpm build -C frontend

.PHONY: build-release
build-release: ## build project in release mode
	nix develop -c cargo build --release --manifest-path backend/Cargo.toml
	nix develop -c pnpm build -C frontend

.PHONY: test
test: ## run tests
	nix develop -c cargo test --manifest-path backend/Cargo.toml
	nix develop -c pnpm test -C frontend

.PHONY: fmt
fmt: ## format code
	nix fmt

.PHONY: lint
lint: ## lint code
	nix develop -c cargo clippy --manifest-path backend/Cargo.toml
	nix develop -c pnpm lint -C frontend

.PHONY: check
check: ## check code
	nix flake check

.PHONY: clean
clean: ## clean build artifacts
	nix develop -c cargo clean --manifest-path backend/Cargo.toml
	rm -rf frontend/node_modules
	rm -rf frontend/.output

.PHONY: dev
dev: ## start frontend development server
	nix develop -c pnpm dev -C frontend

.PHONY: dev-backend
dev-backend: ## start backend server
	nix develop -c cargo run --manifest-path backend/Cargo.toml
