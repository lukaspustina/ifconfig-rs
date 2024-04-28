all: check unit integration

doctoc:
	doctoc --maxlevel 2 Readme.md

check:
	cargo $@

build:
	cargo $@

clean:
	cargo $@

fmt:
	cargo $@

clippy:
	cargo $@

tests: unit integration acceptance

unit:
	cargo test --lib --no-fail-fast
	cargo test

integration:
	$(MAKE) -C tests $@

acceptance:
	$(MAKE) -C tests $@

docker-build:
	docker build . --tag ifconfig-rs:$$(cargo read-manifest | jq ".version" -r)

push_to_prod:
	git checkout prod
	git merge master
	git push
	git checkout master

.PHONY: tests

