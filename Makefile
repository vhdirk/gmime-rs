GIR = gir/target/bin/gir
CONFIGS = $(wildcard conf/gir-*.toml)
GIR_SRC = gir/Cargo.toml gir/Cargo.lock gir/build.rs $(shell find gir/src -name '*.rs')
GIR_FILES = gir-files/GMime-3.0.gir

LIBS = $(CONFIGS:conf/gir-%.toml=%-sys/src/lib.rs)

libs : $(LIBS)

%-sys/src/lib.rs : conf/gir-%.toml $(GIR) $(GIR_FILES)
	$(GIR) -c $< -o $(abspath $*-sys) -d gir-files

# Run `gir` generating the bindings
gir : src/auto/mod.rs

src/auto/mod.rs : Gir.toml $(GIR) $(GIR_FILES)
	$(GIR) -c Gir.toml
#
# $(GIR) : $(GIR_SRC)
# 	cd gir && cargo build --release


$(GIR) : $(GIR_SRC)
	rm -f gir/target/bin/gir
	cargo install --path gir --root gir/target
	rm -f gir/target/.crates.toml

$(GIR_SRC) $(GIR_FILES) :
	git submodule update --init
