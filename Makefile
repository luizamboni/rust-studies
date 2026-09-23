
regular:
	# como existem mais de um main é preciso especificar
	cd hello-rust && cargo run --bin hello-rust

second_binary:
	cd hello-rust && cargo run --bin main_2


array-examples:
	cd hello-rust && cargo run --bin array_examples

struct-examples:
	cd hello-rust && cargo run --bin struct_examples