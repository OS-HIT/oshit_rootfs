typedef int int32_t;
typedef unsigned int uint32_t;
typedef long long int int64_t;
typedef unsigned long long int uint64_t;

const uint64_t READ = 63;
const uint64_t WRITE = 64;

static inline uint64_t syscall(uint64_t which, uint64_t arg0, uint64_t arg1, uint64_t arg2) {
    // cannot assign register directly, use register variable instead. Not a reliable solution, consider switching to pure asm.
	register uint64_t a0 asm ("a0") = (uint64_t)(arg0);
	register uint64_t a1 asm ("a1") = (uint64_t)(arg1);
	register uint64_t a2 asm ("a2") = (uint64_t)(arg2);
	register uint64_t a7 asm ("a7") = (uint64_t)(which);
	asm volatile ("ecall"
		: "+r" (a0)
		: "r" (a1), "r" (a2), "r" (a7)
		: "memory");
	return a0;
}

static inline void putc(char in) {
	syscall(WRITE, 1, &in, 1);
}

void puts(char* str) {
	char* iter = str;
	while(*iter) {
		putc(*iter);
		iter++;
	}
}

static inline void put_hex(uint64_t num) {
	char buffer[100] = {0};
	char* iter = buffer + 98;
	while(num) {
		char c = 0;
		if (num % 16 >= 10) {
			c = num % 16 - 10 + 'a';
		} else {
			c = num % 16 + '0';
		}
		*(iter--) = c;
		num /= 16;
	}
	puts(iter + 1);
}

uint64_t _start(uint64_t argc, uint64_t argv, uint64_t envp) {
	void* p = 0;
	puts("Hello world, sp=");
	put_hex((uint64_t)&p);
	puts("\nargv=");
	put_hex((uint64_t)argv);
	puts("\nenvp=");
	put_hex((uint64_t)envp);
	
	while(1);
}