.PHONY: help
help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'

.PHONY: setup
setup: ## setup project
	nix develop -c pnpm install

.PHONY: build
build: ## build project
	nix develop -c cargo build
	nix develop -c pnpm build

.PHONY: build-release
build-release: ## build project in release mode
	nix develop -c cargo build --release
	nix develop -c pnpm build

.PHONY: test
test: ## run tests
	nix develop -c cargo test
	nix develop -c pnpm test

.PHONY: fmt
fmt: ## format code
	nix fmt

.PHONY: lint
lint: ## lint code
	nix develop -c cargo clippy
	nix develop -c pnpm lint

.PHONY: check
check: ## check code
	nix flake check

.PHONY: clean
clean: ## clean build artifacts
	nix develop -c cargo clean
	rm -rf node_modules
	rm -rf dist

.PHONY: dev
dev: ## start development server
	nix develop -c pnpm dev
