# Define the name of the executable
EXECUTABLE = pass-words

# Define the installation directory
INSTALL_DIR = $(HOME)/.local/bin

# Default target: build the executable
all: $(EXECUTABLE)

# Build the executable
$(EXECUTABLE):
	cargo build --release

# Install the executable locally for the current user
install: $(EXECUTABLE)
	@mkdir -p $(INSTALL_DIR)
	@cp target/release/$(EXECUTABLE) $(INSTALL_DIR)
	@echo "Installed $(EXECUTABLE) to $(INSTALL_DIR)"

# Uninstall the application
uninstall:
	@rm -f $(INSTALL_DIR)/$(EXECUTABLE)
	@echo "Uninstalled $(EXECUTABLE) from $(INSTALL_DIR)"

# Clean the build artifacts
clean:
	cargo clean

.PHONY: all install uninstall clean
