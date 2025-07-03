# Project Aurora (Rust Frontend Branch)

A highly-customizable desktop GUI audio visualizer, built with Rust, designed for musicians and content creators to generate unique, audio-reactive music videos. This branch focuses on developing a new frontend in Rust, aiming to provide a robust and modern user experience.

![placeholder](https://i.imgur.com/gY8g4i2.png) <!-- Placeholder image -->

---

## Core Vision

Project Aurora aims to be a premier tool for creating captivating music visualizations. The Rust frontend will offer:

*   **Real-time Audio Visualization:** Leveraging the power of the projectM engine for stunning MilkDrop-style visuals.
*   **Intuitive GUI:** A user-friendly interface built in Rust for controlling all aspects of the visualization.
*   **Music Playback:** Support for various audio formats.
*   **Video Recording:** Built-in capabilities to record the visualizations as video files.
*   **Deep Customization:** Extensive options to tweak visual parameters, effects, and more, all configurable through the GUI.
*   **Persistent Settings:** User configurations will be saved and loaded, allowing for personalized experiences.
*   **Automation:** Control via both command-line arguments for scripting and directly through the GUI for interactive sessions.

---

## Current C++ Version Features (Reference)

The existing C++ version (on `master` branch) includes:
*   Real-time audio visualization with projectM.
*   Dynamic audio reactivity.
*   Support for `.mp3`, `.wav`, `.flac`.
*   Keyboard controls for preset switching.
*   A debug mode for consistent preset testing.

---

## Prerequisites (Rust Frontend - Anticipated)

The Rust frontend will have its own set of dependencies. These will be detailed as development progresses. Core system dependencies for `projectM` itself will likely still be required.

For `projectM` (example for Arch Linux):
```bash
sudo pacman -S --needed projectm
```

Rust Development Environment:
```bash
# Instructions for installing Rust (e.g., via rustup) will be added here.
# curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

---

## Build & Run Instructions (Rust Frontend - To Be Developed)

Instructions for building and running the Rust frontend will be provided here once the initial framework is in place. This will likely involve standard Rust tooling (`cargo build`, `cargo run`).

---

## Project Roadmap: Rust Frontend Development

This roadmap outlines the planned development phases for the new Rust-based frontend.

### Phase 1: Core ProjectM Integration & Basic GUI (Rust)

*   **Goal:** Establish a foundational Rust application that can host and render projectM visualizations.
*   **Tasks:**
    *   Set up Rust project structure.
    *   Choose a GUI framework (e.g., Iced, Egui, Druid, Tauri) and implement a basic window.
    *   Develop FFI (Foreign Function Interface) bindings to `libprojectM`.
    *   Integrate projectM rendering into the Rust GUI window.
    *   Implement basic audio input selection and playback (e.g., using `rodio` or `cpal`).
    *   Ensure visualizations react to live audio.
    *   Basic preset switching (next/previous random).

### Phase 2: Enhanced GUI, Configuration & Control

*   **Goal:** Build out the GUI to allow comprehensive control over projectM and visualization parameters.
*   **Tasks:**
    *   Design and implement GUI elements for:
        *   Audio source selection (file, input device).
        *   Preset selection and management (browsing, searching, favoriting).
        *   Configuration of projectM parameters (e.g., beat sensitivity, texture size).
        *   Access to all tweakable visual settings that projectM exposes.
    *   Implement a system for saving and loading user configurations persistently (e.g., using `serde` with TOML or JSON files).
    *   Refine keyboard and mouse controls for interaction.

### Phase 3: Recording & Automation

*   **Goal:** Enable users to record their visualizations and automate the creation process.
*   **Tasks:**
    *   Integrate video recording functionality (e.g., using `ffmpeg-next` or direct FFmpeg CLI calls) to capture the visual output and audio.
        *   Allow selection of output format, resolution, and quality.
    *   Develop a command-line interface (CLI) using a crate like `clap` for:
        *   Loading specific audio files.
        *   Selecting specific presets or cycling modes.
        *   Starting/stopping recording.
        *   Applying saved configurations.
        *   Output file specification.
    *   Ensure CLI options map to GUI configurable parameters, allowing for scripted automation of video generation.

### Phase 4: Text & Animation Engine (Port or Re-implement in Rust)

*   **Goal:** Incorporate dynamic text overlays and animations, similar to the original C++ roadmap, but implemented or integrated within the Rust ecosystem.
*   **Tasks:**
    *   **Layer-Based Compositor:** Design a rendering pipeline in Rust where projectM is one layer, and text/graphics are composited on top.
    *   **High-Quality Text Rendering:** Integrate a Rust text rendering solution (e.g., `rusttype`, `glyphon`, or a GUI framework's built-in capabilities).
    *   **Dynamic Text Animation:**
        *   State machine for text animations (intro, main, outro).
        *   Text movement and effects, potentially audio-reactive.
    *   **Static Overlays:** Option for static text (e.g., URLs).

### Phase 5: Advanced Features & Polish

*   **Goal:** Add cutting-edge features and refine the user experience.
*   **Tasks:**
    *   **Karaoke-Style Lyrics:**
        *   Investigate Rust libraries or bindings for speech-to-text (e.g., local Whisper model via FFI or Rust-native alternatives if available).
        *   Integrate with suno.com lyric sources as described in the original C++ roadmap, adapting tools for Rust.
        *   Timed lyric display.
    *   **Custom Shader Support (Rust context):** Explore how users could provide custom shaders (e.g., WGSL if using `wgpu` directly or via a GUI framework, or GLSL if OpenGL context is managed).
    *   **Community Integration:** System for sharing/downloading presets, animation templates, color schemes.
    *   **Performance Optimization:** Profile and optimize the Rust application for smooth performance.
    *   **Cross-Platform Compatibility:** Test and ensure functionality across major operating systems (Linux, Windows, macOS).
    *   **Comprehensive Documentation:** User guides and developer documentation.

---

This README will be updated as the project progresses on this `rust-frontend` branch.
For the original C++ project, please refer to the `master` branch.
```