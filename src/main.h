#pragma once

#include <cstring>
#include <fcntl.h>
#include <iostream>
#include <chrono>
#include <thread>
#include <unistd.h>
#include <libportal/portal.h> // Used to take screenshot on Wayland
#include <png.h>
#include <X11/Xlib.h>
#include <X11/Xutil.h>
#include <X11/extensions/XTest.h> // Used to simulate mouse click when using X11

#define VERSION "1.2"
#define INTERVAL 4000   // Time in ms to wait between searches

using namespace std;


#define die(str, args...) do { \
    perror(str); \
    exit(EXIT_FAILURE); \
} while(0)



// process_image.cpp
extern bool process_image(XImage *img, png_structp *png, png_infop *info, int *match_x, int *match_y);

// wayland_display.cpp
extern void wl_take_screenshot(void (*screenshot_callback)(XImage *img, png_structp *png, png_infop *info));

// wayland_mouse.cpp
extern void wl_mouse_cleanup(int signo);
extern void wl_get_mouse();
extern void wl_set_mouse_pos(int x, int y);
extern void wl_mouse_click(int depressed);

// x11_display.cpp
extern Display *display;
extern Window   root;

extern void x11_take_screenshot(int width, int height, void (*screenshot_callback)(XImage *img, png_structp *png, png_infop *info));
extern void x11_get_display(int *width, int *height);

// x11_mouse.cpp
extern void x11_set_mouse_pos(int x, int y);
extern void x11_mouse_click(int depressed);
