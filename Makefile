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
	rustup run nightly cargo fmt -- --write-mode overwrite

clippy:
	rustup run nightly cargo clippy

tests: integration ignored

unit:
	cargo test --lib --no-fail-fast

integration:
	cargo test

ignored:
	cargo test -- --ignored

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

.PHONY: tests

