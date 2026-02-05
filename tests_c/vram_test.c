// Define the type manually since we don't have stdint.h
typedef unsigned int uint32_t;

volatile uint32_t* VRAM = (volatile uint32_t*)0x11000000;

void _start() {
    int width = 320;
    int height = 240;

    while(1) {
        for (int y = 0; y < height; y++) {
            for (int x = 0; x < width; x++) {
                // RGBA format for Raylib (Little-Endian: 0xAABBGGRR)
                unsigned char r = (x * 255) / width;
                unsigned char g = (y * 255) / height;
                unsigned char b = 128;
                unsigned char a = 255;

                VRAM[y * width + x] = (a << 24) | (b << 16) | (g << 8) | r;
            }
        }
    }
}
