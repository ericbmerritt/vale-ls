.PHONY: release build test all

all: build

release:
	git tag $(tag)
	git push origin $(tag)

build:
	cargo build --release

test:
	cargo test

format-nix:
	rg --files -g '*.nix' -g '!.*' | xargs alejandra

lint-nix: ## Lint Nix files.
	rg --files -g '*.nix' -g '!.*' | xargs -L 1 statix check --
