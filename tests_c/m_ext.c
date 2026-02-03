volatile char* UART = (volatile char*)0x10000000;

void print_hex(unsigned int val) {
    char hex[] = "0123456789ABCDEF";
    for (int i = 7; i >= 0; i--) {
        *UART = hex[(val >> (i * 4)) & 0xF];
    }
    *UART = '\n';
}

void _start() {
    unsigned int a = 123;
    unsigned int b = 456;
    unsigned int res = a * b;
    
    print_hex(res);

    asm volatile ("li a7, 93; ecall");
}
