volatile char* UART = (volatile char*)0x10000000;

void _start() {
    char* msg = "Hello Iron-clad! I'm alive!\n";
    for (int i = 0; msg[i] != '\0'; i++) {
        *UART = msg[i]; 
    }
    
    // Graceful exit using your new ECALL (93)
    asm volatile (
        "li a7, 93\n"
        "ecall"
    );
}
