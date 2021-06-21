CARGO=cargo
ARGS=--workspace --color=always

.PHONY: doc

doc:
	$(CARGO) $@ $(ARGS) --no-deps


dbuild:
	$(CARGO) build $(ARGS)


rbuild:
	$(CARGO) build $(ARGS) --release