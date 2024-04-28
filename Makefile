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

tests: integration

unit:
	cargo test --lib --no-fail-fast

integration:
	cargo test -- --include-ignored

get_geoip: geoip/GeoIP2-City.mmdb geoip/GeoIP2-ASN.mmdb

geoip/GeoIP2-City.mmdb: geoip
	curl -s http://geolite.maxmind.com/download/geoip/database/GeoLite2-City.tar.gz | tar -xzf - -C $<
	find geoip -name GeoLite2-City.mmdb -exec mv {} $< \;
	rm -r $</GeoLite2-City_*

geoip/GeoIP2-ASN.mmdb: geoip
	curl -s http://geolite.maxmind.com/download/geoip/database/GeoLite2-ASN.tar.gz | tar -xzf - -C $<
	find geoip -name GeoLite2-ASN.mmdb -exec mv {} $< \;
	rm -r $</GeoLite2-ASN_*

geoip:
	mkdir $@

clean-geoip:
	@-rm -R geoip

docker-build:
	docker build . --tag ifconfig-rs:$$(cargo read-manifest | jq ".version" -r)

push_to_prod:
	git checkout prod
	git merge master
	git push
	git checkout master

.PHONY: tests

