# Project Aurora

A highly-customizable desktop GUI audio visualizer designed for musicians and content creators to generate unique, audio-reactive music videos.

![placeholder](https://i.imgur.com/gY8g4i2.png) <!-- Placeholder image -->

---

## Core Features (Current)

*   **Real-time Audio Visualization:** Renders classic and modern MilkDrop-style presets using the projectM engine.
*   **Audio-Reactive:** Visualizations react dynamically to the frequency and beat of the currently playing audio.
*   **Wide Audio Format Support:** Plays `.mp3`, `.wav`, and `.flac` audio files via a simple menu.
*   **Full Keyboard Control:**
    *   `N` key: Switch to the next random preset.
    *   `P` key: Switch to the previously viewed preset (up to 20 presets of history).
*   **Debug & Testing Mode:** A command-line flag (`-d` or `--default-preset`) allows for starting with a consistent, non-random preset for predictable testing.

---

## Prerequisites (Arch Linux)

The application relies on a few key system packages.

```bash
sudo pacman -S --needed git make gcc cmake qt6-base projectm
```

---

## Build & Run Instructions

A `Makefile` is provided to simplify the build process.

1.  **Clone the repository:**
    ```bash
    git clone <your-repo-url>
    cd <your-repo-directory>
    ```

2.  **Build the application:**
    This will configure the project (if needed) and compile the source code.
    ```bash
    make
    ```

3.  **Run the application:**

    *   **Standard Mode (Random Presets):**
        ```bash
        ./build/AuroraVisualizer
        ```

    *   **Debug Mode (Fixed Preset):**
        ```bash
        ./build/AuroraVisualizer --default-preset
        # or
        ./build/AuroraVisualizer -d
        ```

4.  **Other Commands:**
    *   `make test`: Builds and runs the test suite.
    *   `make clean`: Removes all build artifacts.
    *   `make help`: Shows all available commands.

---

## Future Development Roadmap

This project aims to become a complete solution for automated music video production. The following features are planned for future development phases.

### Phase 2: Text & Animation Engine
*   **Layer-Based Compositor:** Implement a rendering pipeline where the projectM visualizer is one layer, and text elements are rendered into a separate, transparent overlay layer.
*   **High-Quality Text Rendering:** Integrate the FreeType library to render custom fonts for artist names, track titles, and other information.
*   **Dynamic Text Animation:**
    *   Create a state machine to manage text animations over the duration of a song.
    *   **Intro:** Text starts centered and fully opaque.
    *   **Main Sequence:** Text fades to partial transparency and begins moving/bouncing around the screen, reacting to the music.
    *   **Outro:** Text returns to the center and fades back to full opacity before the song ends.
*   **Static Overlays:** Add an option to display a static, small-text URL (e.g., for a social media channel) in a corner of the screen.

### Phase 3: Automation & Recording
*   **Video Recording:** Integrate `FFmpeg` to record the final composited output (visualizer + text overlays) and the source audio into a high-quality video file (`.mp4`).
*   **Configuration & Scripting:**
    *   Expand command-line options to control all aspects of a video render (input audio, output video, text content, fonts, colors, etc.).
    *   Add support for a configuration file (`config.json` or similar) to manage persistent settings.
*   **Batch Processing:** Enable a fully automated workflow where the application can be scripted to process an entire directory of songs, generating a unique video for each one based on a template.

### Phase 4: Advanced Features
*   **Karaoke-Style Lyrics:** Integrate a speech-to-text engine (like a local Whisper model) to pre-process songs and generate timed lyric data. The text engine will then display these lyrics in sync with the music.
*       **suno.com Integration:** Pull mp3's, maybe flac if possible, using already developed projects code on github which also dumps the raw url to parse for the  lyrics from each songs  https://suno.com/four-part-sondid-string source for each song to aide in the speech recognition, especially for songs with wild or unaudible voacls. Ideally would allow to rename songs that havnt got a proper name yet.
*   **Custom Shader Support:** Allow users to write their own GLSL shaders for text effects and post-processing, enabling highly complex and unique animations.
*   **Community Integration:** Build a system for users to easily share and download visualizer presets, text animation templates, and color schemes.
```