//! File for compiling `sokol_gfx.h`

// Sokol render flag is selected and defined by `build.rs`
// #define SOKOL_<RENDERER>

#define SOKOL_IMPL
#define SOKOL_IMGUI_IMPL
#define SOKOL_IMGUI_GFX_IMPL

#define SOKOL_NO_ENTRY
#define SOKOL_NO_DEPRECATED
#define SOKOL_TRACE_HOOKS

// NOTE: `sokol_app.h` automatically links to OpenGL (c.f. ln 1446~)
#ifdef __APPLE__
    #define GL_SILENCE_DEPRECATION
    #include <OpenGL/gl3.h>
#else
    #include <GL/gl3.h>
#endif

// search from include path (-I flag)
#include "sokol_gfx.h"

