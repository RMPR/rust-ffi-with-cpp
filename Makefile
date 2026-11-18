# Run everything from this directory: snippets use paths relative to it.
PRESENTERM = presenterm --config-file config.yaml

.PHONY: present notes notes-window next-notes dev warm clean

# `make present` also opens the speaker notes in a second terminal window.
# Use `make present NOTES=0` to skip that window.
present: warm next-notes notes-window   ## Give the talk, with speaker notes alongside
	$(PRESENTERM) --present --publish-speaker-notes slides.md

notes:                   ## Speaker notes only; follows the presenting window
	$(PRESENTERM) --listen-speaker-notes slides.md

next-notes:              ## Refresh the "Next: ..." line in every speaker note
	@tools/next-notes.py slides.md

notes-window:
	@if [ "$(NOTES)" = "0" ]; then \
		true; \
	elif command -v ghostty >/dev/null 2>&1; then \
		(ghostty --title="Speaker notes" -e $(MAKE) --no-print-directory notes \
			>/dev/null 2>&1 &); \
	else \
		echo "ghostty not found: run 'make notes' in a second terminal."; \
	fi

dev: warm                ## Edit mode: re-runs every snippet on each reload
	$(PRESENTERM) --validate-snippets --validate-overflows slides.md

warm:                    ## Pre-build the demos so live runs are instant
	cd demos && cargo build --quiet

clean:
	cd demos && cargo clean && rm -rf 07-cbindgen/include && rm -f */build/*
