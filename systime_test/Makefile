NAME = $(notdir $(shell pwd))
BIN = ./target/riscv64imac-unknown-none-elf/release/$(NAME)

all: $(NAME)

$(NAME): $(BIN)
	cp $(BIN) ./$(NAME)

$(BIN):
	cargo build --bin $(NAME) --release

clean:
	cargo clean
	rm -rf $(NAME)