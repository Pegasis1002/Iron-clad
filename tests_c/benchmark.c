// tests_c/benchmark.c
typedef unsigned int uint32_t;
typedef unsigned long long uint64_t;

// Define the UART address
#define UART_ADDR 0x10000000

// Helper to print a single character to the console
void uart_putc(char c) {
    volatile char* uart = (volatile char*)UART_ADDR;
    *uart = c;
}

// Function to print a 32-bit integer in Hexadecimal format (0x1234ABCD)
void print_hex(unsigned int val) {
    const char hex_chars[] = "0123456789ABCDEF";
    
    // Print the prefix
    uart_putc('0');
    uart_putc('x');

    // Loop through each of the 8 nibbles (4-bit chunks) in the 32-bit int
    // We start from the most significant nibble (leftmost)
    for (int i = 7; i >= 0; i--) {
        // Shift right to bring the nibble to the bottom 4 bits, then mask with 0xF
        int nibble = (val >> (i * 4)) & 0xF;
        uart_putc(hex_chars[nibble]);
    }
    
    // Print a newline at the end
    uart_putc('\n');
}

// Inline assembly to read the 64-bit cycle counter
uint64_t read_cycles() {
    uint32_t lo, hi, hi2;
    do {
        asm volatile("csrr %0, mcycleh" : "=r"(hi));
        asm volatile("csrr %0, mcycle"  : "=r"(lo));
        asm volatile("csrr %0, mcycleh" : "=r"(hi2));
    } while (hi != hi2); // Handle overflow between reads
    return ((uint64_t)hi << 32) | lo;
}

void _start() {
    uint64_t start = read_cycles();
    
    // Perform 1,000,000 math operations
    volatile int dummy = 0;
    for (int i = 0; i < 1000000; i++) {
        dummy += (i * 3) / 2; 
    }

    uint64_t end = read_cycles();
    uint32_t diff = (uint32_t)(end - start);

    // Print the result using your UART hex printer
    print_hex(diff); 

    asm volatile("li a7, 93; ecall");
}
