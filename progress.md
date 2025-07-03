# Project Aurora - Progress Log & LLM Reference

## Overview

This document tracks the development progress of Project Aurora and serves as a quick reference for LLMs assisting with the project. Its main goal is to provide an "API overview" style summary of decisions, architecture, and key components.

## Project Goal

To create a highly-customizable desktop GUI audio visualizer, primarily using Rust for its frontend. The visualizer will leverage the projectM engine for core visualization capabilities and aims to provide a comprehensive solution for generating audio-reactive music videos.

## Key High-Level Requirements (from User Request):

*   **Frontend Technology:** Rust-based GUI.
*   **Core Visualizer:** Integrate with projectM (similar to [projectM-visualizer/frontend-sdl-cpp](https://github.com/projectM-visualizer/frontend-sdl-cpp)).
*   **Functionality:**
    *   Music file playback and visualization.
    *   Video recording of visualizations.
    *   Automation capabilities accessible via both command-line interface (CLI) and the graphical user interface (GUI).
    *   Extensive GUI-based configuration for all user-tweakable parameters.
    *   Persistent storage of user configurations.
*   **Target Audience:** Musicians and content creators.

## Initial Rust Frontend Considerations

This section outlines the initial thoughts on the Rust frontend's structure and dependencies. These are subject to change as development and research progress.

*   **Proposed Directory Structure (within a new `frontend_rust` directory):**
    ```
    frontend_rust/
    ├── Cargo.toml
    ├── src/
    │   ├── main.rs         // Application entry point
    │   ├── lib.rs          // Library module (if creating a library alongside binary)
    │   ├── audio/          // Modules for audio playback and processing
    │   │   └── mod.rs
    │   ├── config/         // Modules for loading/saving configurations
    │   │   └── mod.rs
    │   ├── gui/            // Modules for the graphical user interface
    │   │   ├── mod.rs
    │   │   └── widgets/    // Custom GUI widgets
    │   ├── projectm_ffi/   // Modules for libprojectM FFI bindings
    │   │   └── mod.rs
    │   ├── recording/      // Modules for video recording logic
    │   │   └── mod.rs
    │   └── cli.rs          // Module for command-line interface parsing
    ├── assets/             // Static assets like icons, default configs (if any)
    └── build.rs            // Optional build script (e.g., for FFI linking)
    ```

*   **Key Rust Crates Under Consideration:**
    *   **GUI Framework: `egui` (with SDL2 for windowing/OpenGL context)**
        *   **Decision:** `egui` has been selected.
        *   **Rationale:**
            *   Leverages `sdl2` crate for windowing, input, and OpenGL context, aligning with the `frontend-sdl2-rust` example for `projectm` integration. This provides a clear path for rendering projectM visuals.
            *   `egui` is well-suited for rapid development of custom UIs and can be easily overlaid or integrated with an OpenGL application.
            *   Crates like `egui_sdl2_gl` (or `egui_sdl2_event` + `egui-glow`) can facilitate this integration.
            *   Good balance of ease-of-use, flexibility for extensive configuration options, and performance.
        *   Other options considered: `iced`, `druid`, `tauri`.
    *   **Windowing/OpenGL Context:**
        *   `sdl2`: Will be used for creating the window, handling events, and setting up the OpenGL context required by projectM and `egui-glow`.
    *   **Audio Playback & Processing:**
        *   `rodio`: Simple audio playback.
        *   `cpal`: Low-level audio I/O, good for capturing system audio or specific inputs.
        *   `kira`: Higher-level game audio library, might be useful for advanced effects or sequencing if needed.
    *   **projectM Integration:**
        *   **`projectm` crate (version 3.1.2 on crates.io):** This is the chosen crate.
            *   Provides safe Rust bindings for `libprojectM`.
            *   Maintained by `projectM-visualizer` organization.
            *   Relies on `projectm-sys` for underlying FFI.
            *   Example usage available at `https://github.com/projectM-visualizer/frontend-sdl2-rust`.
            *   Documentation coverage on `docs.rs` is low (~11%), so the example project will be a key reference.
            *   License: LGPL (GitHub shows 2.1, docs.rs shows 3.0-or-later).
        *   ~~`libloading`: For dynamically loading `libprojectM`.~~ (Not needed if using `projectm` crate)
        *   ~~`bindgen`: For generating Rust FFI bindings from C/C++ headers of `libprojectM`.~~ (Not needed if using `projectm` crate)
    *   **Video Recording:**
        *   `ffmpeg-next`: Comprehensive Rust bindings for FFmpeg. Allows for encoding video and audio.
        *   Alternatively, direct `std::process::Command` calls to the `ffmpeg` CLI tool if finer control or simpler integration is initially preferred.
    *   **Configuration Management:**
        *   `serde`: For serializing and deserializing data structures (e.g., configurations).
        *   `serde_json` / `serde_toml`: For specific file formats (JSON or TOML are good candidates for config files).
        *   `directories-rs`: For finding standard configuration/data directories on different OSs.
    *   **CLI Argument Parsing:**
        *   `clap`: Powerful and popular library for command-line argument parsing.
    *   **Async Runtime (if needed for GUI or I/O):**
        *   `tokio` or `async-std`, depending on the GUI framework's requirements or other I/O needs. `iced` for example has its own async model.
    *   **Logging:**
        *   `log` and a logger implementation like `env_logger` or `tracing`.

## Development Log

*(This section will be updated as development progresses)*

*   **Initial Setup:**
    *   Created `progress.md` to track project status and decisions.
    *   Updated `README.md` to reflect the new Rust-based frontend plan and incorporate detailed user requirements into the project roadmap.
*   **projectM Integration Strategy:**
    *   Investigated `projectm-rs` (crate name `projectm` on crates.io, version 3.1.2).
    *   Decision: Adopt the `projectm` crate. This simplifies development by providing safe Rust bindings to `libprojectM`, eliminating the need for manual FFI binding generation (`bindgen`) for this core component.
    *   Updated `README.md` Phase 1 tasks to reflect using the `projectm` crate.
*   **Asset Path Handling for ProjectM:**
    *   Implemented logic in `gui::run_application` to determine paths for ProjectM presets and textures.
    *   Strategy:
        1.  Check environment variables: `PROJECTM_PRESET_PATH` and `PROJECTM_TEXTURE_PATH`.
        2.  If not found, check common Linux default locations: `/usr/share/projectm/...` and `/usr/local/share/projectm/...`.
        3.  If still not found, the application currently errors out with a message to the user. (Future improvement: prompt user or use a config file).
*   **Simulated Build and Test (Post Initial Rendering/Audio/Preset Implementation):**
    *   **Anticipated Build Issues:**
        *   Potential API mismatches with the `projectm` crate (v3.1.2), especially regarding:
            *   `Core::new()` signature and error type.
            *   `Core::render_frame()` - this is highly speculative. The `frontend-sdl2-rust` example uses a `projectm::Renderer` object and its `render_frame_to_texture(&mut texture_id)` method. The current code might need significant changes to align with the correct rendering approach in `projectm-rs`.
            *   PCM data submission method (`pcm_add_float`) and its exact signature.
            *   Preset switching methods (`select_next`, `select_prev`, `select_random`) and their error types.
        *   `egui-glow` event handling: Direct use of `egui_glow.on_event(&sdl_event)` might be insufficient; an adapter like `egui-sdl2-event` could be necessary for proper event translation.
        *   OpenGL state and shader correctness: Standard potential for bugs in GL calls or shader logic.
    *   **Anticipated Runtime Issues:**
        *   **ProjectM Not Rendering / Black Screen:** Most likely issue due to the speculative `render_frame()` call. The method for ProjectM to render to the designated texture (`projectm_texture_id`) needs verification against `projectm-rs` examples/docs.
        *   Incorrect asset paths leading to ProjectM initialization failure.
        *   Egui UI not rendering or interacting correctly if event translation is flawed.
    *   **Key Next Steps (for actual debugging):**
        1.  **Verify `projectm-rs` Rendering API:** Consult `projectm-rs` examples (especially `frontend-sdl2-rust`) and documentation to correctly use its rendering capabilities (e.g., `projectm::Renderer`, FBOs, texture output methods). This is the highest priority.
        2.  Confirm ProjectM asset paths are correctly found and loaded.
        3.  Iteratively debug OpenGL rendering pipeline.
        4.  Refine Egui event handling if needed.
