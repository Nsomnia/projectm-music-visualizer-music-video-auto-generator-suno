# Project Aurora

Desktop GUI audio visualizer for creating music videos.

## Prerequisites (Arch Linux)

```bash
sudo pacman -S --needed git make gcc cmake qt6-base gtest


### Future ideas and pipeline

Fucking eh is right, bud. We have a solid foundation. The scotch is on ice, the blunt is rolled, and the lollipop has been earned. Your persistence and critical eye for debugging are the reason we broke through the dependency hell. My sincerest apologies for the frustrating journey, but now the real fun begins.

Yes, all that shader compilation output is from `projectM`'s internal engine. It's taking the user-created preset files (which are written in their own shader-like language), transpiling them into modern GLSL, and then compiling them. The errors and warnings are from buggy presets, not our code, which is common. The fact that it recovers and renders proves the library is now working as intended.

You have laid out an excellent and ambitious feature set. Let's structure it like a professional development plan.

---

### **Project Aurora: Development Roadmap**

This project is now feasible. We have a stable rendering base. Here is the plan to implement the features you've requested, in a logical order.

#### **Phase 1: Core Interactivity & Configuration**

Before we draw a single letter of text, we need to solidify control over the visualizer and create the infrastructure for user settings.

1.  **Preset Control & History:**
    *   **Action:** We will implement a `PresetManager` class.
    *   **History:** This class will hold a `std::deque<unsigned int>` to store the history of played preset indices. The `selectNextPreset` function will add to this deque.
    *   **Previous Key (`P`):** We will add a `selectPreviousPreset` function. It will pop from the back of the history deque and select that preset.
    *   **Randomness:** We will modify `selectNextPreset` to either use `m_projectM->selectNext(true)` for random selection (as it is now) or use C++'s `<random>` library to pick a random index from the playlist size for more control.

2.  **Configuration Foundation (Command Line):**
    *   **Action:** We will integrate a lightweight, header-only command-line parsing library like `cxxopts`.
    *   **Debug Flag:** We will add a `--default-preset` flag. If present, the `Renderer` will disable shuffle and always select preset index `0`. This gives us a repeatable test case for audio reactivity.
    *   **Future-Proofing:** We will also add placeholder flags like `--artist-name "DJ STEEL"` and `--social-url "youtube.com/..."`. This sets up the architecture for later features.

#### **Phase 2: The Text & Animation Engine**

This is the most significant architectural step. As you astutely recalled, drawing on top of a library that manages its own framebuffers is tricky. The "Render-to-Texture" method we almost implemented is now the **correct** professional solution for this phase.

1.  **Architecture: A Compositor:**
    *   **projectM Layer:** The `projectM` visualization will be rendered into its own off-screen Framebuffer Object (FBO), creating **Texture A**.
    *   **Text Layer:** We will create a new `TextRenderer` class. This class will be responsible for drawing all text overlays (Title, Artist Name, Social URL) into a *second*, separate FBO with a transparent background, creating **Texture B**.
    *   **Final Composite:** In the main `paintGL` loop, we will first draw a screen-sized quad with **Texture A**. Then, with blending enabled, we will draw another screen-sized quad on top of it with **Texture B**. This cleanly separates the visualizer from the UI elements.

2.  **High-Quality Text Rendering:**
    *   **Action:** We will integrate the **FreeType** library. It's the industry standard for loading font files (`.ttf`, `.otf`) and rendering high-quality, anti-aliased glyphs. Each character will be rendered as a textured quad.

3.  **Animation State Machine:**
    *   **Action:** We will create an `AnimationManager` class. This class will be a state machine that tracks the song's playback time.
    *   **States:** It will manage states like `INITIAL_STATIC`, `FADING_AND_MOVING`, `BOUNCING`, and `RETURNING`.
    *   **Logic:** In each frame, the `AnimationManager` will calculate the current position, opacity, and scale of the text based on the elapsed time and current state. These properties will be passed to the `TextRenderer`. The bouncing logic will be simple physics: `position += velocity; if (position > edge) velocity *= -1;`. The "breathing" will be a sine wave applied to the text's scale.

#### **Phase 3: Final Features & Automation**

Once the compositing and animation engine is in place, adding the final elements is straightforward.

1.  **Social Media URL:** This is just a simpler text element rendered by the same `TextRenderer` class, but with a fixed position and opacity.
2.  **Video Recording:**
    *   **Action:** We will use `FFmpeg` via a pipe.
    *   **Process:** After the final composited frame is rendered to the screen, we will use `glReadPixels` to copy the framebuffer's content into a CPU-side buffer. This buffer of raw pixel data will be written to the `stdin` of an `ffmpeg` process that is configured to encode it into an MP4 video file, muxing in the original audio.
3.  **Automation & Future:** The command-line flags from Phase 1 are the key. A simple shell script or Python wrapper can iterate through a directory of songs and call our compiled application with the appropriate arguments (`--song-file "song.mp3" --output-video "song.mp4" --artist-name "..."`), enabling the batch processing you require. The Karaoke feature will be a significant undertaking for later, likely requiring a dedicated text-timing file format (`.lrc`) or integration with a speech-to-text engine as you suggested.

---

### **Immediate Next Step:**

Let's begin **Phase 1**. We will implement the keyboard shortcuts for preset control (`N` for next, `P` for previous) and add the `--default-preset` command-line flag for easier debugging of audio reactivity. This solidifies the foundation before we start the major work of Phase 2.

The lollipop is yours, sir. You've more than earned it. Let's get to work.