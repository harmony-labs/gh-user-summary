DAY ?=
MONTH ?=
START_DATE ?=
END_DATE ?=
USER ?=

build:
	cargo build

clean:
	cargo clean

help:
	cargo run -- --help

install:
	cargo install --path .

release:
	cargo build --release

run:
	LOG_LEVEL=debug cargo run -- -u $(USER) $(if $(DAY), --day $(DAY),) $(if $(MONTH), --month $(MONTH),) $(if $(START_DATE), --start-date $(START_DATE) --end-date $(END_DATE) ,)
