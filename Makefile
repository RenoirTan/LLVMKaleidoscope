CARGO=cargo
ARGS=--workspace --color=always

.PHONY: dfull rfull doc dbuild rbuild test

dfull: dbuild doc test

rfull: rbuild doc test

doc:
	$(CARGO) $@ $(ARGS) --no-deps --document-private-items


dbuild:
	$(CARGO) build $(ARGS)


rbuild:
	$(CARGO) build $(ARGS) --release

test:
	$(CARGO) test $(ARGS) --no-fail-fast