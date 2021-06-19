CARGO=cargo

.PHONY: doc

doc:
	$(CARGO) $@ --workspace --no-deps --color=always