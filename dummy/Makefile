all: dummy dummy.asm dummy.sym

dummy: dummy.c
	riscv64-unknown-elf-gcc -Wall -O -fno-omit-frame-pointer -ggdb -MD -mcmodel=medany -ffreestanding -fno-common -nostdlib -mno-relax -fno-pie -no-pie -fno-stack-protector -march=rv64g -nostdinc -c dummy.c -o dummy.o
	riscv64-unknown-elf-ld -o dummy -T linker.ld dummy.o

dummy.asm: dummy
	rust-objdump -S dummy > dummy.asm

dummy.sym: dummy
	rust-objdump -t dummy | sed '1,/SYMBOL TABLE/d; s/ .* / /; /^$$/d' | sort > dummy.sym

clean:
	rm -rf dummy dummy.asm dummy.sym