//! File for compiling Sokol

// Sokol render flag is selected and defined by `build.rs`

#define SOKOL_IMPL
#define SOKOL_IMGUI_IMPL

#define SOKOL_NO_ENTRY
#define SOKOL_NO_DEPRECATED
#define SOKOL_TRACE_HOOKS

// search from include path (-I flag)
#include "sokol_app.h"
#include "sokol_gfx.h"
#include "sokol_glue.h"

// #define CIMGUI_DEFINE_ENUMS_AND_STRUCTS
// #include "cimgui.h"
// #include "sokol_imgui.h"
// #include "sokol_gfx_imgui.h"
