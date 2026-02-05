#ifndef IRONCLAD_GUI_H
#define IRONCLAD_GUI_H

#include <stdint.h>

void gui_init(int width, int height);
void gui_update(const uint32_t* vram);
int gui_should_close();
void terminate();

#endif // !IRONCLAD_GUI_H
