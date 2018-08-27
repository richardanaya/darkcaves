# Project level variables
PROJECT_NAME         = darkcaves
PROJECT_DESCRIPTION  =

# Tools
GIT                  = git

# Vendoring
ifneq ("$(wildcard .vendor)","")
include .vendor/make/prelude.mk
include .vendor/make/help.mk
include .vendor/make/app_rust.mk
endif

.PHONY : all check deploy clean

##all    - Build everything
all: app_rust__build

##clean  - Clean up project
clean: app_rust__clean

##vendor - Vendor makefiles
vendor:
	@echo Vendoring Makefiles
	@rm -rf .vendor
	@$(GIT) clone https://github.com/richardanaya/makefiles.git .vendor/make
