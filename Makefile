
run:
	rustc run src/start.rs

main : all

all:
	rust build
