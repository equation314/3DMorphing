ratio ?= 0.5
edge ?=
sphere ?=
o1 ?= models/cube.obj
o2 ?= models/egg.obj
output=output

args :=

ifneq ($(edge), )
args += -e
endif

ifneq ($(sphere), )
args += -p
endif

all:
	cargo build --release

run:
	cargo run --release $(o1) $(o2) -r $(ratio) -o $(output).obj -s $(a) $(args)

fmt:
	cargo fmt

clean:
	cargo clean
