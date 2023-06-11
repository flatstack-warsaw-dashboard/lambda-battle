SOURCE_DIR := lib/common/compute/lambda
DEST_DIR := packages
SUBDIRS := $(wildcard $(SOURCE_DIR)/*)
ZIP_TARGETS := $(patsubst $(SOURCE_DIR)/%,$(DEST_DIR)/%.zip,$(SUBDIRS))

CDK_DEPLOY_COMMAND := cdk deploy
AWS_PROFILE_FILE := .aws_profile

all: package
package: $(DEST_DIR) $(ZIP_TARGETS)

$(DEST_DIR):
	@mkdir -p $(DEST_DIR)

$(DEST_DIR)/%.zip: $(SOURCE_DIR)/%
	@echo "Packaging $<..."
	@cd $< && zip -r $(abspath $@) .

clean:
	@echo "Cleaning up..."
	@rm -rf $(DEST_DIR)/*.zip

deploy: clean package
	@if [ -f "$(AWS_PROFILE_FILE)" ]; then \
		aws_profile=$$(cat $(AWS_PROFILE_FILE)); \
	else \
		read -p "Please enter your AWS profile name to proceed: " aws_profile; \
		echo $$aws_profile > $(AWS_PROFILE_FILE); \
	fi; \
	aws-vault clear; \
	aws-vault exec --no-session $$aws_profile -- $(CDK_DEPLOY_COMMAND); \

.PHONY: all package clean
