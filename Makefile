# Run everything from this directory: snippets use paths relative to it.
PRESENTERM = presenterm --config-file config.yaml

.PHONY: present dev warm clean

present: warm            ## Give the talk
	$(PRESENTERM) --present slides.md

dev: warm                ## Edit mode: re-runs every snippet on each reload
	$(PRESENTERM) --validate-snippets --validate-overflows slides.md

warm:                    ## Pre-build the demos so live runs are instant
	cd demos && cargo build --quiet

clean:
	cd demos && cargo clean && rm -rf 07-cbindgen/include && rm -f */build/*
