#include "raylib.h"
#include "interface.h"

#define SCREEN_WIDTH 320
#define SCREEN_HEIGHT 240

static Texture2D screen_texture;
static Image screen_image;

void gui_init(int width, int height) {
  InitWindow(SCREEN_WIDTH, SCREEN_HEIGHT, "Iron-clad RISC-V");
  SetTargetFPS(60);

  Image initial_image = GenImageColor(width, height, BLACK);
  screen_texture = LoadTextureFromImage(initial_image);
  UnloadImage(initial_image);
}

void gui_update(const uint32_t* vram){
  UpdateTexture(screen_texture, vram);

  BeginDrawing();
  ClearBackground(BLACK);

  DrawTexture(screen_texture, 0, 0, WHITE);

  EndDrawing();
}

int gui_should_close(){ return WindowShouldClose(); }

void gui_terminate(){
  UnloadTexture(screen_texture);
  CloseWindow();
}
