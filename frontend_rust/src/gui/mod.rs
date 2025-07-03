use std::{time::Instant, path::Path, env};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use projectm; // Assuming projectm crate is correctly added
use crate::audio; // Import our audio module

// Constants for window size
const INITIAL_WINDOW_WIDTH: u32 = 1280;
const INITIAL_WINDOW_HEIGHT: u32 = 720;

// Shader and quad rendering code removed as ProjectM is now assumed to render directly to backbuffer.

fn translate_sdl_mouse_button(button: &sdl2::mouse::MouseButton) -> Option<egui::PointerButton> {
    match button {
        sdl2::mouse::MouseButton::Left => Some(egui::PointerButton::Primary),
        sdl2::mouse::MouseButton::Right => Some(egui::PointerButton::Secondary),
        sdl2::mouse::MouseButton::Middle => Some(egui::PointerButton::Middle),
        // sdl2::mouse::MouseButton::X1 => Some(egui::PointerButton::Extra1), // egui doesn't have X1/X2 directly
        // sdl2::mouse::MouseButton::X2 => Some(egui::PointerButton::Extra2),
        _ => None,
    }
}

pub fn resolve_asset_paths() -> Result<(String, String), String> {
    // 1. Environment Variables
    let preset_env_var = "PROJECTM_PRESET_PATH";
    let texture_env_var = "PROJECTM_TEXTURE_PATH";

    if let (Ok(p_path_str), Ok(t_path_str)) = (env::var(preset_env_var), env::var(texture_env_var)) {
        if Path::new(&p_path_str).is_dir() && Path::new(&t_path_str).is_dir() {
            println!("Using asset paths from environment variables: {} and {}", p_path_str, t_path_str);
            return Ok((p_path_str, t_path_str));
        } else {
            // Only return error if env vars are set but invalid. Otherwise, proceed to common paths.
            if env::var(preset_env_var).is_ok() || env::var(texture_env_var).is_ok() {
                 println!(
                    "Warning: {} or {} environment variables set, but one or both paths are invalid.",
                    preset_env_var, texture_env_var
                );
                return Err(format!(
                    "Invalid asset paths from environment variables: {} or {} are invalid.",
                    preset_env_var, texture_env_var
                ));
            }
        }
    }

    // 2. Common Linux Paths
    let common_paths_to_check = [
        ("/usr/share/projectm", "presets", "textures"),
        ("/usr/local/share/projectm", "presets", "textures"),
        // Add more common paths for other OSs here if needed later
    ];

    for (base, preset_subdir, texture_subdir) in common_paths_to_check.iter() {
        let p_path = Path::new(base).join(preset_subdir);
        let t_path = Path::new(base).join(texture_subdir);
        if p_path.is_dir() && t_path.is_dir() {
            println!("Found common asset paths: {:?} and {:?}", p_path, t_path);
            return Ok((
                p_path.to_string_lossy().into_owned(),
                t_path.to_string_lossy().into_owned(),
            ));
        }
    }

    // 3. Fallback: Error out
    eprintln!("Could not find ProjectM preset/texture paths automatically.");
    eprintln!("Please set {} and {} environment variables,", preset_env_var, texture_env_var);
    eprintln!("or ensure projectM is installed in a standard location (e.g., /usr/share/projectm).");
    Err("ProjectM asset paths not found.".to_string())
}


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

    // No longer creating a separate texture or shader for ProjectM,
    // as it's assumed to render directly to the backbuffer.

    let mut egui_glow = egui_glow::EguiGlow::new(&window, &gl);

    let (preset_path_str, texture_path_str) = resolve_asset_paths()?;

    // Initialize ProjectM
    let mut projectm_core = projectm::Core::new(preset_path_str,
                                                texture_path_str,
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

        let mut raw_input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(window.size().0 as f32, window.size().1 as f32) / egui_glow.pixels_per_point(),
            )),
            // TODO: Fill other fields like time, modifiers, etc.
            ..Default::default()
        };

        for event in event_pump.poll_iter() {
            // Manually translate SDL events to egui events and add to raw_input.events
            // This is a simplified translation. A more complete one would handle more event types.
            match &event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                Event::MouseMotion { x, y, .. } => {
                    raw_input.events.push(egui::Event::PointerMoved(egui::pos2(*x as f32, *y as f32)));
                }
                Event::MouseButtonDown { mouse_btn, x, y, .. } => {
                    if let Some(button) = translate_sdl_mouse_button(mouse_btn) {
                        raw_input.events.push(egui::Event::PointerButton {
                            pos: egui::pos2(*x as f32, *y as f32),
                            button,
                            pressed: true,
                            modifiers: egui::Modifiers::default(), // TODO: map SDL modifiers
                        });
                    }
                }
                Event::MouseButtonUp { mouse_btn, x, y, .. } => {
                    if let Some(button) = translate_sdl_mouse_button(mouse_btn) {
                        raw_input.events.push(egui::Event::PointerButton {
                            pos: egui::pos2(*x as f32, *y as f32),
                            button,
                            pressed: false,
                            modifiers: egui::Modifiers::default(), // TODO: map SDL modifiers
                        });
                    }
                }
                // TODO: Add Keyboard input (Key, Text), Scroll, etc.
                // Event::KeyDown { keycode, keymod, .. } => { ... }
                // Event::TextInput { text, .. } => { raw_input.events.push(egui::Event::Text(text.clone())); }
                // Event::MouseWheel { x, y, .. } => { ... }

                _ => {} // Other SDL events not handled for egui yet
            }

            // Handle non-egui SDL events (like window resize)
            match event {
                Event::Window { win_event: sdl2::event::WindowEvent::Resized(w, h), .. } => {
                    projectm_core.set_screen_size(w as u32, h as u32);
                    unsafe {
                        gl.viewport(0, 0, w, h);
                    }
                    // Update egui screen rect for next frame
                    raw_input.screen_rect = Some(egui::Rect::from_min_size(
                        egui::Pos2::ZERO,
                        egui::vec2(w as f32, h as f32) / egui_glow.pixels_per_point(),
                    ));
                }
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::Window { win_event: sdl2::event::WindowEvent::Resized(w, h), .. } => {
                    // Handle window resize for projectM and viewport
                    projectm_core.set_screen_size(w as u32, h as u32);
                    unsafe {
                        gl.viewport(0, 0, w, h);
                        // No longer need to update our own texture for ProjectM
                    }
                }
                _ => {}
            }
        }

        // Feed placeholder audio data to ProjectM
        let audio_data = audio::get_placeholder_audio_buffer();
        projectm_core.pcm_add_float(&audio_data);

        // Clear the main framebuffer (optional, projectM might overwrite fully)
        unsafe {
            gl.clear_color(0.1, 0.1, 0.1, 1.0); // Slightly different clear color for distinction
            gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
        }

        // Render projectM directly to the backbuffer
        projectm_core.render_frame();

        // Run Egui frame
        let egui_output = egui_glow.egui_ctx.run(raw_input, |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.label("Project Aurora Visualizer");
                ui.separator();
                if ui.button("Next Preset").clicked() {
                    if let Err(e) = projectm_core.select_next() {
                        eprintln!("Failed to select next preset: {:?}", e);
                    } else {
                        println!("Switched to next preset.");
                    }
                }
                if ui.button("Previous Preset").clicked() { // Added for completeness
                    if let Err(e) = projectm_core.select_prev() {
                        eprintln!("Failed to select previous preset: {:?}", e);
                    } else {
                        println!("Switched to previous preset.");
                    }
                }
                if ui.button("Random Preset").clicked() { // Added for completeness
                    if let Err(e) = projectm_core.select_random() {
                        eprintln!("Failed to select random preset: {:?}", e);
                    } else {
                        println!("Switched to random preset.");
                    }
                }
                ui.label(format!("FPS: {:.1}", 1.0 / delta_time.as_secs_f32()));
            });
        });

        // TODO: Handle egui_output.platform_output (e.g., clipboard, text cursor shape)

        let paint_jobs = egui_glow.egui_ctx.tessellate(egui_output.shapes, egui_output.pixels_per_point);

        unsafe {
            egui_glow.paint_glow(&window, &gl, paint_jobs);
        }


        window.gl_swap_window();
        ::std::thread::sleep(::std::time::Duration::from_millis(10)); // Small sleep
    }

    // Cleanup
    // No longer deleting shader_program, vao, vbo, projectm_texture_id as they are removed.
    egui_glow.destroy(&gl);
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    // Helper to set up mock directories for testing Path::is_dir()
    // This is still somewhat integration-testy as it touches filesystem.
    // For pure unit tests of logic, one might need to inject a "filesystem" trait.
    fn setup_mock_dirs(base_path: &Path, preset_subdir: &str, texture_subdir: &str) -> (String, String) {
        let preset_dir = base_path.join(preset_subdir);
        let texture_dir = base_path.join(texture_subdir);
        fs::create_dir_all(&preset_dir).unwrap();
        fs::create_dir_all(&texture_dir).unwrap();
        (preset_dir.to_string_lossy().into_owned(), texture_dir.to_string_lossy().into_owned())
    }

    #[test]
    fn test_resolve_asset_paths_env_vars_valid() {
        let temp_dir = tempdir().unwrap();
        let (mock_preset_path, mock_texture_path) = setup_mock_dirs(temp_dir.path(), "presets_env", "textures_env");

        env::set_var("PROJECTM_PRESET_PATH", &mock_preset_path);
        env::set_var("PROJECTM_TEXTURE_PATH", &mock_texture_path);

        let result = resolve_asset_paths();
        assert!(result.is_ok());
        let (presets, textures) = result.unwrap();
        assert_eq!(presets, mock_preset_path);
        assert_eq!(textures, mock_texture_path);

        env::remove_var("PROJECTM_PRESET_PATH");
        env::remove_var("PROJECTM_TEXTURE_PATH");
        // temp_dir and its contents are automatically cleaned up
    }

    #[test]
    fn test_resolve_asset_paths_env_vars_invalid_preset() {
        let temp_dir = tempdir().unwrap();
        let (_mock_preset_path_real, mock_texture_path) = setup_mock_dirs(temp_dir.path(), "presets_env_inv", "textures_env_inv");
        let invalid_preset_path = temp_dir.path().join("non_existent_presets");

        env::set_var("PROJECTM_PRESET_PATH", invalid_preset_path.to_string_lossy().as_ref());
        env::set_var("PROJECTM_TEXTURE_PATH", &mock_texture_path);

        let result = resolve_asset_paths();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Invalid asset paths from environment variables: PROJECTM_PRESET_PATH or PROJECTM_TEXTURE_PATH are invalid.");

        env::remove_var("PROJECTM_PRESET_PATH");
        env::remove_var("PROJECTM_TEXTURE_PATH");
    }

    #[test]
    fn test_resolve_asset_paths_env_vars_invalid_texture() {
        let temp_dir = tempdir().unwrap();
        let (mock_preset_path, _mock_texture_path_real) = setup_mock_dirs(temp_dir.path(), "presets_env_inv2", "textures_env_inv2");
        let invalid_texture_path = temp_dir.path().join("non_existent_textures");

        env::set_var("PROJECTM_PRESET_PATH", &mock_preset_path);
        env::set_var("PROJECTM_TEXTURE_PATH", invalid_texture_path.to_string_lossy().as_ref());

        let result = resolve_asset_paths();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Invalid asset paths from environment variables: PROJECTM_PRESET_PATH or PROJECTM_TEXTURE_PATH are invalid.");

        env::remove_var("PROJECTM_PRESET_PATH");
        env::remove_var("PROJECTM_TEXTURE_PATH");
    }

    #[test]
    fn test_resolve_asset_paths_no_env_fallback_to_error() {
        // Ensure env vars are not set
        env::remove_var("PROJECTM_PRESET_PATH");
        env::remove_var("PROJECTM_TEXTURE_PATH");

        // This test assumes common paths like /usr/share/projectm do NOT exist
        // or are not relevant in the test environment. This is a limitation.
        // For a CI environment, these paths would typically not exist.
        let result = resolve_asset_paths();
        // If common paths *do* exist on the system running the test, this will fail.
        // To make it robust, you'd mock `Path::is_dir` or run in a very controlled env.
        // For now, we expect it to error if common paths aren't found.
        if !Path::new("/usr/share/projectm/presets").is_dir() && !Path::new("/usr/local/share/projectm/presets").is_dir() {
            assert!(result.is_err());
            assert_eq!(result.unwrap_err(), "ProjectM asset paths not found.");
        } else {
            // If common paths ARE found, then it should be Ok.
            // This part makes the test dependent on the environment.
            assert!(result.is_ok(), "Test assumes no common paths, but found some: {:?}", result.ok());
        }
    }

    // Note: Testing the "common paths" logic directly is hard in unit tests
    // without mocking `Path::is_dir` or actually creating those directories
    // in system locations, which is not feasible for typical unit tests.
    // The `test_resolve_asset_paths_no_env_fallback_to_error` indirectly covers
    // the case where common paths are not found.
    // A true integration test would be needed to confirm common path discovery on target systems.
}
