CARGO=cargo
ARGS=--workspace --color=always

.PHONY: help clean all ascii_art dfull rfull doc dbuild rbuild test fmt

help: ascii_art
	@echo "Helper Makefile"
	@echo ""
	@echo "USAGE: make [TARGET_NAME] [OPTIONS]"
	@echo ""
	@echo " - help: Display this help message."
	@echo " - clean: Remove the target directory along with any other cache folders."
	@echo " - dbuild: Build library with the debug profile."
	@echo " - rbuild: Build library with the release profile."
	@echo " - doc: Generate the documentation for this library."
	@echo " - test: Run unit tests and doctests."
	@echo " - fmt: Format code to follow the configuration in /rustfmt.toml"
	@echo " - dfull: Build library with debug profile, generate docs and test."
	@echo " - rfull: Build library with release profile, generate docs and test."
	@echo " - all: Build library in debug and release mode, as well as the documentation and running tests."


ascii_art:
	@echo " _   __      _      _     _                                "
	@echo "| | / /     | |    (_)   | |                               "
	@echo "| |/ /  __ _| | ___ _  __| | ___  ___  ___ ___  _ __   ___ "
	@echo "|    \\ / _  | |/ _ \\ |/ _\\ |/ _ \\/ __|/ __/ _ \\| '_ \\ / _ \\"
	@echo "| |\\  \\ (_| | |  __/ | (_| | (_) \\__ \\ (_| (_) | |_) |  __/"
	@echo "\\_| \\_/\\__,_|_|\\___|_|\\__,_|\\___/|___/\\___\\___/| .__/ \\___|"
	@echo "                                               | |         "
	@echo "                                               |_|         "
	@echo ""


all: dbuild rbuild test doc


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

fmt:
	$(CARGO) +nightly fmt

clean:
	$(CARGO) clean