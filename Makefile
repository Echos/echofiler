# Makefile for echofiler

# Variables
PREFIX ?= $(HOME)/.local
BINDIR = $(PREFIX)/bin
CONFIGDIR = $(HOME)/.config/echofiler

# Build target (release by default)
BUILD_TYPE ?= release
CARGO_FLAGS = $(if $(filter release,$(BUILD_TYPE)),--release,)
TARGET_DIR = target/$(BUILD_TYPE)
BINARY = $(TARGET_DIR)/echofiler

# Features
FEATURES ?= archive,preview,plugin
FEATURES_FLAG = $(if $(FEATURES),--features "$(FEATURES)",)

.PHONY: all build install uninstall clean test help

all: build

# Build the project
build:
	@echo "Building echofiler ($(BUILD_TYPE))..."
	cargo build $(CARGO_FLAGS) $(FEATURES_FLAG)
	@echo "Build complete: $(BINARY)"

# Install binary and config files
install: build
	@echo "Installing echofiler..."
	@mkdir -p $(BINDIR)
	@mkdir -p $(CONFIGDIR)
	install -m 755 $(BINARY) $(BINDIR)/echofiler
	@if [ ! -f $(CONFIGDIR)/echofiler.toml ]; then \
		install -m 644 config/default/echofiler.toml $(CONFIGDIR)/echofiler.toml; \
		echo "Installed config: $(CONFIGDIR)/echofiler.toml"; \
	else \
		echo "Config already exists: $(CONFIGDIR)/echofiler.toml (skipped)"; \
	fi
	@if [ ! -f $(CONFIGDIR)/keymap.toml ]; then \
		install -m 644 config/default/keymap.toml $(CONFIGDIR)/keymap.toml; \
		echo "Installed keymap: $(CONFIGDIR)/keymap.toml"; \
	else \
		echo "Keymap already exists: $(CONFIGDIR)/keymap.toml (skipped)"; \
	fi
	@if [ ! -f $(CONFIGDIR)/theme.toml ]; then \
		install -m 644 config/default/theme.toml $(CONFIGDIR)/theme.toml; \
		echo "Installed theme: $(CONFIGDIR)/theme.toml"; \
	else \
		echo "Theme already exists: $(CONFIGDIR)/theme.toml (skipped)"; \
	fi
	@if [ ! -f $(CONFIGDIR)/opener.toml ]; then \
		install -m 644 config/default/opener.toml $(CONFIGDIR)/opener.toml; \
		echo "Installed opener: $(CONFIGDIR)/opener.toml"; \
	else \
		echo "Opener already exists: $(CONFIGDIR)/opener.toml (skipped)"; \
	fi
	@mkdir -p $(CONFIGDIR)/plugins
	@echo "Installation complete!"
	@echo "Binary: $(BINDIR)/echofiler"
	@echo "Config: $(CONFIGDIR)/"
	@echo ""
	@echo "Make sure $(BINDIR) is in your PATH."
	@echo "Add this line to your ~/.bashrc or ~/.zshrc if needed:"
	@echo "  export PATH=\"$(BINDIR):\$$PATH\""

# Uninstall binary and config files
uninstall:
	@echo "Uninstalling echofiler..."
	rm -f $(BINDIR)/echofiler
	@echo "Binary removed: $(BINDIR)/echofiler"
	@echo ""
	@echo "Config files are NOT removed to preserve your settings."
	@echo "To remove config manually, run:"
	@echo "  rm -rf $(CONFIGDIR)"

# Clean build artifacts
clean:
	@echo "Cleaning build artifacts..."
	cargo clean
	@echo "Clean complete."

# Run tests
test:
	@echo "Running tests..."
	cargo test --lib $(FEATURES_FLAG)

# Run with all features
run:
	@echo "Running echofiler..."
	cargo run $(CARGO_FLAGS) $(FEATURES_FLAG)

# Development build (debug mode)
dev:
	@$(MAKE) build BUILD_TYPE=debug

# Install from debug build
install-dev:
	@$(MAKE) install BUILD_TYPE=debug

# Help message
help:
	@echo "echofiler Makefile"
	@echo ""
	@echo "Usage:"
	@echo "  make [target] [OPTIONS]"
	@echo ""
	@echo "Targets:"
	@echo "  build        Build the project (default: release)"
	@echo "  install      Install binary and config files"
	@echo "  uninstall    Uninstall binary (keeps config)"
	@echo "  clean        Remove build artifacts"
	@echo "  test         Run tests"
	@echo "  run          Run echofiler"
	@echo "  dev          Build in debug mode"
	@echo "  install-dev  Install debug build"
	@echo "  help         Show this help message"
	@echo ""
	@echo "Options:"
	@echo "  PREFIX       Installation prefix (default: ~/.local)"
	@echo "               Example: make install PREFIX=/usr/local"
	@echo "  BUILD_TYPE   Build type: release or debug (default: release)"
	@echo "               Example: make build BUILD_TYPE=debug"
	@echo "  FEATURES     Cargo features to enable (default: archive,preview,plugin)"
	@echo "               Example: make build FEATURES=archive,preview"
	@echo ""
	@echo "Examples:"
	@echo "  make                              # Build release binary"
	@echo "  make install                      # Install to ~/.local/bin"
	@echo "  make install PREFIX=/usr/local    # Install to /usr/local/bin"
	@echo "  make dev                          # Build debug binary"
	@echo "  make test                         # Run tests"
