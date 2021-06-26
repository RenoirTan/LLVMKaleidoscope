CARGO=cargo
ARGS=--workspace --color=always

.PHONY: doc

doc:
	$(CARGO) $@ $(ARGS) --no-deps --document-private-items


dbuild:
	$(CARGO) build $(ARGS)


rbuild:
	$(CARGO) build $(ARGS) --release

test:
	$(CARGO) test $(ARGS) --no-fail-fast