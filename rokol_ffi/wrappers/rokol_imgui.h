//! File for generating Rust FFI

#define SOKOL_TRACE_HOOKS

// blacklisted includes
#include "sokol_app.h"
#include "sokol_gfx.h"

#define CIMGUI_DEFINE_ENUMS_AND_STRUCTS
#include "cimgui.h"

// simgui*
#include "sokol_imgui.h"
// sg_imgui*
#include "sokol_gfx_imgui.h"
