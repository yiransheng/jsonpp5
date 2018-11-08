BIN := ../target/debug/jsonpp5

verify_json: good_json
	jq --argfile a good_json/1.json --argfile b good_json/1.json.formatted -n '$a == $b'
	# ls -d good_json/*.json\
	# | parallel -j 8\
	| jq --argfile a {} --argfile b {}.formatted -n '$a == $b'

good_json:
	ls -d good_json/* | parallel -j 8 --workdir ${CURDIR} $(BIN) {} -o {}.formatted

.PHONY: clean

clean:
	rm -f good_json/*.formatted
