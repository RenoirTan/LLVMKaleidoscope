CARGO=cargo
ARGS=--workspace --color=always

.PHONY: help all dfull rfull doc dbuild rbuild test

help:
	@echo " _   __      _      _     _                                "
	@echo "| | / /     | |    (_)   | |                               "
	@echo "| |/ /  __ _| | ___ _  __| | ___  ___  ___ ___  _ __   ___ "
	@echo "|    \\ / _  | |/ _ \\ |/ _\\ |/ _ \\/ __|/ __/ _ \\| '_ \\ / _ \\"
	@echo "| |\\  \\ (_| | |  __/ | (_| | (_) \\__ \\ (_| (_) | |_) |  __/"
	@echo "\\_| \\_/\\__,_|_|\\___|_|\\__,_|\\___/|___/\\___\\___/| .__/ \\___|"
	@echo "                                               | |         "
	@echo "                                               |_|         "
	@echo ""
	@echo "Helper Makefile"
	@echo ""
	@echo "USAGE: make [TARGET_NAME] [OPTIONS]"
	@echo ""
	@echo " - help: Display this help message."
	@echo " - dbuild: Build library with the debug profile."
	@echo " - rbuild: Build library with the release profile."
	@echo " - doc: Generate the documentation for this library."
	@echo " - test: Run unit tests and doctests."
	@echo " - dfull: Build library with debug profile, generate docs and test."
	@echo " - rfull: Build library with release profile, generate docs and test."
	@echo " - all: Build library in debug and release mode, as well as the documentation and running tests."


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