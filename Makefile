

all: debug-lingnu

run:
	@cargo run

debug-lingnu:
	@cargo build --target=x86_64-unknown-linux-gnu

release:
	@cargo build --release --target=x86_64-unknown-linux-gnu
	@cargo build --release --target=x86_64-pc-windows-gnu	
