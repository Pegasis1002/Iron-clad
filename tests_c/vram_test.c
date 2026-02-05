volatile unsigned int* VRAM = (volatile unsigned int*)0x11000000;

void _start(){
  int width = 320;
  int height = 240;

  for (int y = 0; y < height; y++) {
    for (int x = 0; x < width; x++) {
      unsigned char r = (x * 255) / width;
      unsigned char g = (y * 255) / height;
      unsigned char b = 128;

      VRAM[y * width + x] = (r << 16) | ( g << 8) | b;
    }
  }

  while (1) {
    asm volatile("nop");
  }
}
