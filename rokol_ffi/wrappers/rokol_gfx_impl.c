//! File for compiling `sokol_gfx.h`

// Sokol render flag is selected and defined by `build.rs`
// #define SOKOL_<RENDERER>

#define SOKOL_IMPL
#define SOKOL_IMGUI_IMPL
#define SOKOL_IMGUI_GFX_IMPL

#define SOKOL_NO_ENTRY
#define SOKOL_NO_DEPRECATED
#define SOKOL_TRACE_HOOKS

// search from include path (-I flag)
#include "sokol_gfx.h"
