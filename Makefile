all: check unit integration

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


heroku:

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

clean-geoip: geoip
	@-rm -R $<

.PHONY: tests

