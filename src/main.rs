use core::sync::atomic::AtomicBool;
use minwebgl as gl;
use serde::Deserialize;
use std::sync::{atomic::Ordering, Mutex, OnceLock};
use wasm_bindgen::{
    prelude::wasm_bindgen,
};
use web_sys::{WebGl2RenderingContext as GL};

#[derive(Clone, Copy, Deserialize)]
struct ResolutionUniform {
    width: f32,
    height: f32,
    pixel_aspect_ratio: f32,
}

#[derive(Clone, Copy, Deserialize, Default)]
struct Uniforms {
    resolution: Option<ResolutionUniform>,
    time: Option<f32>,
    frame: Option<f32>,
}

#[derive(Clone, Copy, Deserialize, Default)]
struct PlayerState {
    uniforms: Option<Uniforms>,
}

static FRAGMENT_SHADER_STORAGE: OnceLock<Mutex<String>> = OnceLock::new();
static RELOAD_FRAGMENT_SHADER: AtomicBool = AtomicBool::new(false);

#[wasm_bindgen]
pub fn set_fragment_shader(new_shader_code: &str) {
    if let Some(mutex) = FRAGMENT_SHADER_STORAGE.get() {
        if let Ok(mut shader) = mutex.lock() {
            *shader = prepare_shader(new_shader_code);
        } else {
            return;
        }
    } else if FRAGMENT_SHADER_STORAGE
        .set(Mutex::new(prepare_shader(new_shader_code)))
        .is_err()
    {
        return;
    }

    RELOAD_FRAGMENT_SHADER.store(true, Ordering::Relaxed);
}

#[inline]
fn prepare_shader(shadertoy_code: &str) -> String {
    format!("#version 300 es 
precision mediump float;

uniform vec3 u_resolution; // image/buffer	The viewport resolution (z is pixel aspect ratio, usually 1.0)
uniform float	u_time; // image/sound/buffer	Current time in seconds
uniform int	u_frame; // image/buffer	Current frame
{shadertoy_code}
in vec2 vUv;
out vec4 frag_color;

void main() {{
    render_image(frag_color, vUv * u_resolution.xy);
}}")
}

fn get_shader() -> Option<String> {
    Some(FRAGMENT_SHADER_STORAGE.get()?.lock().ok()?.to_owned())
}

fn run() -> Result<(), gl::WebglError> {
    gl::browser::setup(minwebgl::browser::Config::default());
    let canvas = gl::canvas::retrieve_or_make()?;
    let gl = gl::context::from_canvas(&canvas)?;


    // Vertex and fragment shader source code
    let vertex_shader_src = include_str!("../shaders/shader.vert");
    let default_frag_shader_src = include_str!("../shaders/shader.frag");
    let frag_shader = get_shader().unwrap_or(prepare_shader(default_frag_shader_src));
    let program =
        gl::ProgramFromSources::new(vertex_shader_src, &frag_shader).compile_and_link(&gl)?;
    gl.use_program(Some(&program));
    RELOAD_FRAGMENT_SHADER.store(false, Ordering::Relaxed);

    let mut frame = 0f32;
    let player_state = PlayerState::default();

    let resolution_loc = gl.get_uniform_location(&program, "u_resolution");
    let time_loc = gl.get_uniform_location(&program, "u_time");
    let frame_loc = gl.get_uniform_location(&program, "u_frame");

    // Define the update and draw logic
    let update_and_draw = move |mut t: f64| {
        t /= 1000f64;

        // u_resolution
        if let Some(Uniforms {
            resolution: Some(resolution),
            ..
        }) = player_state.uniforms
        {
            gl.uniform3f(
                resolution_loc.as_ref(),
                resolution.width,
                resolution.height,
                resolution.pixel_aspect_ratio,
            );
        } else {
            gl.uniform3f(
                resolution_loc.as_ref(),
                gl.drawing_buffer_width() as f32,
                gl.drawing_buffer_height() as f32,
                if let Some(window) = web_sys::window() {
                    window.device_pixel_ratio() as f32
                } else {
                    1.0
                },
            );
        };

        // This code is designed to seamlessly continue playback after `Resume`
        let time = t;
        
        // u_time
        gl.uniform1f(
            time_loc.as_ref(),
            if let Some(Uniforms {
                time: Some(fixed_time),
                ..
            }) = player_state.uniforms
            {
                fixed_time
            } else {
                time as f32
            },
        );

        // u_frame
        gl.uniform1f(
            frame_loc.as_ref(),
            if let Some(Uniforms {
                frame: Some(fixed_frame),
                ..
            }) = player_state.uniforms
            {
                fixed_frame
            } else {
                frame
            },
        );
        frame += 1f32;

        // Draw points
        gl.draw_arrays(GL::TRIANGLE_STRIP, 0, 4);
        true
    };

    // Run the render loop
    gl::exec_loop::run(update_and_draw);
    Ok(())
}

fn main() {
    run().unwrap();
}
