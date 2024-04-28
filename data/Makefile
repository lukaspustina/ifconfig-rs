all:

regexes.yaml:
	curl -s https://raw.githubusercontent.com/ua-parser/uap-core/master/regexes.yaml -o $@

build: GeoLite2-ASN.mmdb GeoLite2-City.mmdb regexes.yaml
	docker build . --tag ifconfig-rs-data:latest

push:
	docker tag ifconfig-rs-data:latest ghcr.io/lukaspustina/ifconfig-rs-data:latest
	docker push ghcr.io/lukaspustina/ifconfig-rs-data:latest

clean: clean_regexes
	rm *.mmdb

clean_regexes:
	rm regexes.yaml