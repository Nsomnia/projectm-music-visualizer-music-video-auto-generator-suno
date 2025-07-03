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
    *   **GUI Framework:**
        *   `iced`: Data-centric, Elm-inspired, good for custom UIs.
        *   `egui`: Easy-to-use immediate mode GUI, good for rapid development and integration.
        *   `druid`: Data-oriented, native-looking widgets, strong focus on correctness.
        *   `tauri`: Allows using web technologies (HTML, CSS, JS) for UI, with Rust backend. (Might be heavier than needed if a pure Rust GUI is preferred).
        *   *Decision on GUI framework will be made after further evaluation based on projectM integration needs and desired UI complexity.*
    *   **Audio Playback & Processing:**
        *   `rodio`: Simple audio playback.
        *   `cpal`: Low-level audio I/O, good for capturing system audio or specific inputs.
        *   `kira`: Higher-level game audio library, might be useful for advanced effects or sequencing if needed.
    *   **projectM Integration:**
        *   `libloading`: For dynamically loading `libprojectM`.
        *   `bindgen`: For generating Rust FFI bindings from C/C++ headers of `libprojectM`. This would likely be part of a `build.rs` script.
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
