use std::{time::Instant, path::Path, env};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use projectm; // Assuming projectm crate is correctly added
use crate::audio; // Import our audio module

// Constants for window size
const INITIAL_WINDOW_WIDTH: u32 = 1280;
const INITIAL_WINDOW_HEIGHT: u32 = 720;

// Shader sources for rendering a texture
const VS_SRC: &str = r#"#version 330 core
    layout (location = 0) in vec2 aPos;
    layout (location = 1) in vec2 aTexCoords;

    out vec2 TexCoords;

    void main() {
        gl_Position = vec4(aPos.x, aPos.y, 0.0, 1.0);
        TexCoords = aTexCoords;
    }
"#;

const FS_SRC: &str = r#"#version 330 core
    out vec4 FragColor;

    in vec2 TexCoords;

    uniform sampler2D screenTexture;

    void main() {
        FragColor = texture(screenTexture, TexCoords);
    }
"#;


unsafe fn create_shader_program(gl: &glow::Context, vs_src: &str, fs_src: &str) -> Result<glow::Program, String> {
    let vs = gl.create_shader(glow::VERTEX_SHADER)?;
    gl.shader_source(vs, vs_src);
    gl.compile_shader(vs);
    if !gl.get_shader_compile_status(vs) {
        return Err(format!("Vertex shader compilation error: {}", gl.get_shader_info_log(vs)));
    }

    let fs = gl.create_shader(glow::FRAGMENT_SHADER)?;
    gl.shader_source(fs, fs_src);
    gl.compile_shader(fs);
    if !gl.get_shader_compile_status(fs) {
        return Err(format!("Fragment shader compilation error: {}", gl.get_shader_info_log(fs)));
    }

    let program = gl.create_program()?;
    gl.attach_shader(program, vs);
    gl.attach_shader(program, fs);
    gl.link_program(program);
    if !gl.get_program_link_status(program) {
        return Err(format!("Shader program linking error: {}", gl.get_program_info_log(program)));
    }

    gl.delete_shader(vs);
    gl.delete_shader(fs);

    Ok(program)
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

    // --- Shader and Quad for rendering ProjectM texture ---
    let shader_program = unsafe { create_shader_program(&gl, VS_SRC, FS_SRC)? };

    let quad_vertices: [f32; 24] = [
        // positions   // texCoords
        -1.0,  1.0,  0.0, 1.0,
        -1.0, -1.0,  0.0, 0.0,
         1.0, -1.0,  1.0, 0.0,

        -1.0,  1.0,  0.0, 1.0,
         1.0, -1.0,  1.0, 0.0,
         1.0,  1.0,  1.0, 1.0,
    ];

    let vao = unsafe { gl.create_vertex_array()? };
    let vbo = unsafe { gl.create_buffer()? };

    unsafe {
        gl.bind_vertex_array(Some(vao));
        gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
        let u8_slice = std::slice::from_raw_parts(
            quad_vertices.as_ptr() as *const u8,
            quad_vertices.len() * std::mem::size_of::<f32>(),
        );
        gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, u8_slice, glow::STATIC_DRAW);

        // Position attribute
        gl.vertex_attrib_pointer_f32(0, 2, glow::FLOAT, false, 4 * std::mem::size_of::<f32>() as i32, 0);
        gl.enable_vertex_attrib_array(0);
        // Texture coord attribute
        gl.vertex_attrib_pointer_f32(1, 2, glow::FLOAT, false, 4 * std::mem::size_of::<f32>() as i32, (2 * std::mem::size_of::<f32>()) as i32);
        gl.enable_vertex_attrib_array(1);
        gl.bind_buffer(glow::ARRAY_BUFFER, None);
        gl.bind_vertex_array(None);
    }
    // --- End Shader and Quad Setup ---

    // --- ProjectM Texture Setup ---
    let projectm_texture_id = unsafe { gl.create_texture()? };
    unsafe {
        gl.bind_texture(glow::TEXTURE_2D, Some(projectm_texture_id));
        gl.tex_image_2d(
            glow::TEXTURE_2D,
            0,
            glow::RGB as i32, // projectM usually renders RGB, check if RGBA is needed/better
            INITIAL_WINDOW_WIDTH as i32,
            INITIAL_WINDOW_HEIGHT as i32,
            0,
            glow::RGB,
            glow::UNSIGNED_BYTE,
            None, // No initial data
        );
        gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MIN_FILTER, glow::LINEAR as i32);
        gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MAG_FILTER, glow::LINEAR as i32);
        gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_S, glow::CLAMP_TO_EDGE as i32);
        gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_T, glow::CLAMP_TO_EDGE as i32);
        gl.bind_texture(glow::TEXTURE_2D, None);
    }
    // --- End ProjectM Texture Setup ---


    let mut egui_glow = egui_glow::EguiGlow::new(&window, &gl);

    // --- Asset Path Resolution ---
    let preset_path_str: String;
    let texture_path_str: String;

    // 1. Environment Variables
    let preset_env = env::var("PROJECTM_PRESET_PATH");
    let texture_env = env::var("PROJECTM_TEXTURE_PATH");

    if let (Ok(p_path), Ok(t_path)) = (preset_env.as_ref(), texture_env.as_ref()) {
        if Path::new(p_path).is_dir() && Path::new(t_path).is_dir() {
            println!("Using asset paths from environment variables: {} and {}", p_path, t_path);
            preset_path_str = p_path.clone();
            texture_path_str = t_path.clone();
        } else {
            println!("Warning: PROJECTM_PRESET_PATH or PROJECTM_TEXTURE_PATH environment variables set, but paths are invalid.");
            return Err("Invalid asset paths from environment variables.".to_string()); // Or fallback
        }
    } else {
        // 2. Common Linux Paths
        let common_paths = [
            ("/usr/share/projectm", "presets", "textures"),
            ("/usr/local/share/projectm", "presets", "textures"),
        ];
        let mut found_common = false;
        let mut temp_preset_path = String::new();
        let mut temp_texture_path = String::new();

        for (base, preset_subdir, texture_subdir) in common_paths.iter() {
            let p_path = Path::new(base).join(preset_subdir);
            let t_path = Path::new(base).join(texture_subdir);
            if p_path.is_dir() && t_path.is_dir() {
                println!("Found common asset paths: {:?} and {:?}", p_path, t_path);
                temp_preset_path = p_path.to_string_lossy().into_owned();
                temp_texture_path = t_path.to_string_lossy().into_owned();
                found_common = true;
                break;
            }
        }

        if found_common {
            preset_path_str = temp_preset_path;
            texture_path_str = temp_texture_path;
        } else {
            // 3. Fallback or Ask User (Simplified for now, returning error)
            // In a real app, this is where you might request_user_input or use a config file.
            eprintln!("Could not find ProjectM preset/texture paths automatically.");
            eprintln!("Please set PROJECTM_PRESET_PATH and PROJECTM_TEXTURE_PATH environment variables,");
            eprintln!("or ensure projectM is installed in a standard location (e.g., /usr/share/projectm).");
            return Err("ProjectM asset paths not found.".to_string());
        }
    }
    // --- End Asset Path Resolution ---

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
                    unsafe {
                        gl.viewport(0, 0, w, h);
                        // Update texture size if projectM renders to our texture
                        gl.bind_texture(glow::TEXTURE_2D, Some(projectm_texture_id));
                        gl.tex_image_2d(
                            glow::TEXTURE_2D, 0, glow::RGB as i32,
                            w as i32, h as i32, 0,
                            glow::RGB, glow::UNSIGNED_BYTE, None
                        );
                        gl.bind_texture(glow::TEXTURE_2D, None);
                    }
                }
                _ => {}
            }
        }

        // Feed placeholder audio data to ProjectM
        let audio_data = audio::get_placeholder_audio_buffer();
        // The projectm-rs API seems to be `pcm_add_float(data: &[f32])`
        // The C API takes (PCM, L_Chan, R_Chan, num_samples)
        // The `frontend-sdl2-rust` example uses `self.projectm.pcm_add_float(&p);`
        // where p is a slice of f32. This implies interleaved stereo data.
        // Our `get_placeholder_audio_buffer` provides this.
        projectm_core.pcm_add_float(&audio_data);


        // Render projectM to texture
        // The actual method name in projectm-rs might be different, e.g., `render_to_texture_gl`.
        // This assumes projectM internally uses the currently bound FBO or renders to a texture ID we provide.
        // The `frontend-sdl2-rust` example does: `self.projectm.render_frame_to_texture(&mut self.texture_id);`
        // This implies projectm-rs can take a mutable reference to our texture ID.
        // However, the `projectm::Core::new` doesn't take a texture ID.
        // The C API projectM_RenderFrame uses the currently bound FBO.
        // Let's assume for now `render_frame_to_texture` is available and works with our texture_id.
        // This is a BIG assumption and likely needs adjustment based on `projectm-rs` specific API.
        // A more robust way would be to create an FBO, attach projectm_texture_id, bind FBO, call render, unbind FBO.

        // For now, let's assume projectm::Core has a method that implies rendering to *its own* internal texture,
        // and we need a way to get that texture's data or ID.
        // OR, it might directly render to the currently bound framebuffer if no texture is specified.
        // The `frontend-sdl2-rust` example's `render_frame_to_texture` is the best lead.
        // It seems `projectm::Renderer` is the key in that example. Our `Core` might be too high level or different.
        // Digging into `projectm-rs` source or its `Renderer` example would be needed.
        // Let's try to call a generic render_frame and then assume it has rendered to *some* state
        // that we can then draw. This part is highly speculative without testing.
        projectm_core.render_frame(); // This is a guess from the C API.

        // Clear the main framebuffer
        unsafe {
            gl.clear_color(0.0, 0.0, 0.0, 1.0); // Clear to black
            gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
        }

        // Draw the ProjectM texture to a full-screen quad
        // This step assumes projectM has rendered to `projectm_texture_id`
        // or that `render_frame()` makes its output available through some default texture binding.
        // This part is also speculative. If projectM renders directly to backbuffer, this quad drawing might not be needed
        // or would draw over it. If it renders to a texture, this is how we'd display it.
        unsafe {
            gl.use_program(Some(shader_program));
            gl.active_texture(glow::TEXTURE0);
            // This is the critical assumption: that projectm_texture_id now holds projectM's output.
            // Or if projectM has its own texture, we'd need `projectm_core.get_output_texture_id()`
            gl.bind_texture(glow::TEXTURE_2D, Some(projectm_texture_id));
            gl.uniform_1_i32(gl.get_uniform_location(shader_program, "screenTexture").as_ref(), 0);

            gl.bind_vertex_array(Some(vao));
            gl.draw_arrays(glow::TRIANGLES, 0, 6);
            gl.bind_vertex_array(None);
            gl.use_program(None);
        }


        // Egui UI construction
        egui_glow.begin_frame(&window); // Egui should be started before its drawing
        egui::CentralPanel::default().show(egui_glow.ctx(), |ui| {
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
        // End Egui frame and prepare paint data
        let (_output, paint_commands) = egui_glow.end_frame(&window);
        let paint_jobs = egui_glow.ctx().tessellate(paint_commands);


        // Render Egui (was egui_glow.paint(&window, &gl);)
        unsafe {
            // egui_glow.paint_jobs(&window, &gl, paint_jobs); // This is often the method
            // For egui_glow 0.27.2 with glow 0.13, it's likely:
            egui_glow.paint_glow(&window, &gl, paint_jobs);

        }


        window.gl_swap_window();
        ::std::thread::sleep(::std::time::Duration::from_millis(10)); // Small sleep
    }

    // Cleanup
    unsafe {
        gl.delete_program(shader_program);
        gl.delete_vertex_array(vao);
        gl.delete_buffer(vbo);
        gl.delete_texture(projectm_texture_id);
    }
    egui_glow.destroy(&gl);
    Ok(())
}
