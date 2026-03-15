#ifndef IRONCLAD_GUI_H
#define IRONCLAD_GUI_H

#include <cstdint>
#include <stdint.h>

void gui_init(int width, int height);
void gui_update(const uint32_t* vram);
int gui_should_close();
void terminate();
uint32_t gui_get_key();

#endif // !IRONCLAD_GUI_H
