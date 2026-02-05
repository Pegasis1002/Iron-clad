#include "raylib.h"

void gui_init(int width, int height) {
  InitWindow(width, height, "Iron-clad RISC-V");
  SetTargetFPS(60);
}

void gui_update(const uint32_t* vram){
  BeginDrawing();
  ClearBackground(BLACK);

  DrawText("Iron-clad!!!", width/2, height/2, 20, RAYWHITE);

  EndDrawing();
}

int gui_should_close(){ return WindowShouldClose(); }

void terminate(){ CloseWindow(); }
