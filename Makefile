
RUN = @cargo run --release

help:
	@cat help.txt

run:
	$(RUN) $(ARGS)


# Set SILENT to 's' if --quiet/-s set, otherwise ''.
# SILENT := $(findstring s,$(word 1, $(MAKEFLAGS)))
# test:
# ifeq ($(SILENT),s)
# 	@bash test.sh
# else
# 	@bash old_test.sh 2> /dev/null
# endif

.PHONY: help run test