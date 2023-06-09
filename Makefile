SOURCE_DIR := lambdas
DEST_DIR := packages
SUBDIRS := $(wildcard $(SOURCE_DIR)/*)
ZIP_TARGETS := $(patsubst $(SOURCE_DIR)/%,$(DEST_DIR)/%.zip,$(SUBDIRS))

all: package
package: $(ZIP_TARGETS)


$(DEST_DIR):
	@mkdir -p $(DEST_DIR)

$(DEST_DIR)/%.zip: $(SOURCE_DIR)/%
	@echo "Packaging $<..."
	@cd $< && zip -r $(abspath $@) .

clean:
	@echo "Cleaning up..."
	@rm -rf $(DEST_DIR)/*.zip

.PHONY: all package clean
