all:
	@grep '^[a-z]\+:' Makefile | tr -d ':'

build:
	docker build . --tag ifconfig-rs-data:latest

push:
	docker tag ifconfig-rs-data:latest ghcr.io/lukaspustina/ifconfig-rs-data:latest
	docker push ghcr.io/lukaspustina/ifconfig-rs-data:latest

