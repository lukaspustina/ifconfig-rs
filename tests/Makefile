all: integration acceptance

integration: Dockerfile.tests
	docker build -f $< ..

acceptance:
	$(MAKE) -C e2e $@

.PHONY: e2e
