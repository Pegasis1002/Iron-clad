#include "raylib.h"
#include <stdint.h>

#define SCREEN_WIDTH 320
#define SCREEN_HEIGHT 240

void gui_init(int width, int height) {
  InitWindow(SCREEN_WIDTH, SCREEN_HEIGHT, "Iron-clad RISC-V");
  SetTargetFPS(60);
}

void gui_update(const uint32_t* vram){
  BeginDrawing();
  ClearBackground(BLACK);

  for (int y = 0; y < SCREEN_HEIGHT; y++) {
        for (int x = 0; x < SCREEN_WIDTH; x++) {
            uint32_t pixel = vram[y * SCREEN_WIDTH + x];
            
            // Convert 0xRRGGBB to Raylib Color
            Color color = {
                (pixel >> 16) & 0xFF, // R
                (pixel >> 8) & 0xFF,  // G
                pixel & 0xFF,         // B
                255                   // Alpha
            };
            DrawPixel(x, y, color);
        }
    }


  EndDrawing();
}

int gui_should_close(){ return WindowShouldClose(); }

void gui_terminate(){ CloseWindow(); }
