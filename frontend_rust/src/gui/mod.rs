use std::time::Instant;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use projectm; // Assuming projectm crate is correctly added

// Constants for window size
const INITIAL_WINDOW_WIDTH: u32 = 1280;
const INITIAL_WINDOW_HEIGHT: u32 = 720;

pub fn run_application() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    // Setup OpenGL attributes
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(3, 3); // projectM might need a specific version, 3.3 is common
    gl_attr.set_double_buffer(true);
    gl_attr.set_depth_size(24);

    let window = video_subsystem
        .window("Project Aurora", INITIAL_WINDOW_WIDTH, INITIAL_WINDOW_HEIGHT)
        .opengl()
        .resizable()
        .build()
        .map_err(|e| e.to_string())?;

    let _gl_context = window.gl_create_context()?; // Keep context alive
    let gl = unsafe {
        glow::Context::from_loader_function(|s| video_subsystem.gl_get_proc_address(s) as *const _)
    };

    let mut egui_glow = egui_glow::EguiGlow::new(&window, &gl);

    // Initialize ProjectM
    // TODO: Get actual paths for presets and textures
    let mut projectm_core = projectm::Core::new("/usr/share/projectm/presets".to_string(),
                                                "/usr/share/projectm/textures".to_string(),
                                                INITIAL_WINDOW_WIDTH,
                                                INITIAL_WINDOW_HEIGHT)
        .map_err(|e| format!("Failed to initialize projectM: {:?}", e))?;

    // TODO: Configure projectM further if needed (e.g., select first preset)

    let mut event_pump = sdl_context.event_pump()?;
    let mut last_frame_time = Instant::now();

    'running: loop {
        let now = Instant::now();
        let delta_time = now.duration_since(last_frame_time);
        last_frame_time = now;

        egui_glow.begin_frame(&window); // Start Egui frame

        for event in event_pump.poll_iter() {
            egui_glow.on_event(&event); // Pass event to Egui
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::Window { win_event: sdl2::event::WindowEvent::Resized(w, h), .. } => {
                    // Handle window resize for projectM and viewport
                    projectm_core.set_screen_size(w as u32, h as u32);
                    unsafe { gl.viewport(0, 0, w, h); }
                }
                _ => {}
            }
        }

        // Egui UI construction
        egui::CentralPanel::default().show(egui_glow.ctx(), |ui| {
            ui.label("Project Aurora Visualizer");
            ui.separator();
            if ui.button("Next Preset").clicked() {
                // TODO: Implement preset switching in projectM
                println!("Next preset button clicked (TODO)");
            }
            ui.label(format!("FPS: {:.1}", 1.0 / delta_time.as_secs_f32()));
        });

        // TODO: Feed audio data to projectM
        // projectm_core.pcm().add_float_data(&vec_of_audio_data);

        // Render projectM
        // This is a simplified render call. Actual integration might need more setup,
        // especially around textures or framebuffer objects if projectM renders to texture.
        // For now, assuming projectM renders directly to the current OpenGL context.
        unsafe {
            gl.clear_color(0.0, 0.0, 0.0, 1.0);
            gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
        }

        // projectm_core.render_frame(); // This method might not exist directly or might need parameters

        // The projectm-rs example (frontend-sdl2-rust) uses a more involved rendering setup.
        // For now, let's assume projectM handles its rendering internally after setup.
        // If projectM renders to a texture, we'd get that texture and draw it here.
        // The `render_frame` method in the C API often takes a callback or works with an internal texture.
        // The `projectm-rs` crate's `Core::render_frame_to_texture` seems more appropriate.
        // Let's try to use that, though it requires a texture target.
        // For a direct render (if possible and how the old C API sometimes worked):
        // projectm_core.render_frame(); // This is a guess, API needs checking

        // Placeholder: just clear screen, egui will draw over it
        // Actual projectM rendering will replace/augment this.

        egui_glow.paint(&window, &gl); // Render Egui

        window.gl_swap_window();
        ::std::thread::sleep(::std::time::Duration::from_millis(10)); // Small sleep
    }

    egui_glow.destroy(&gl);
    Ok(())
}
