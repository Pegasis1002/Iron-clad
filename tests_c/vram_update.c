typedef unsigned int uint32_t;

volatile uint32_t* VRAM = (volatile uint32_t*)0x11000000;

// Helper to draw a rectangle
void draw_rect(int x, int y, int w, int h, uint32_t color, int screen_width) {
    for (int i = y; i < y + h; i++) {
        for (int j = x; j < x + w; j++) {
            VRAM[i * screen_width + j] = color;
        }
    }
}

void _start() {
    int width = 320;
    int height = 240;

    int rect_x = 0;
    int rect_y = 100;
    int rect_w = 50;
    int rect_h = 50;
    int speed = 1;

    while(1) {
        // 1. Erase the old position (draw black or background color)
        draw_rect(rect_x, rect_y, rect_w, rect_h, 0x00000000, width);

        // 2. Update the X position
        rect_x += speed;

        // 3. Keep it within screen bounds (bounce)
        if (rect_x + rect_w >= width || rect_x <= 0) {
            speed = -speed;
        }

        // 4. Draw the new position (e.g., White color)
        draw_rect(rect_x, rect_y, rect_w, rect_h, 0xFFFFFFFF, width);

        // Optional: Add a small delay loop here if the movement is too fast
        for(volatile int d = 0; d < 50000; d++); 
    }
}
