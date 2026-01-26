use macroquad::prelude::*;

/// Fragment shader for parallax background
const PARALLAX_FRAGMENT: &str = r#"#version 100
precision mediump float;

varying vec2 uv;

uniform vec2 u_resolution;
uniform vec2 u_camera_pos;
uniform vec2 u_level_size;
uniform float u_time;

// Gradient colors (underwater theme)
const vec3 COLOR_TOP = vec3(0.05, 0.15, 0.3);
const vec3 COLOR_BOTTOM = vec3(0.1, 0.35, 0.45);

// Simple hash for procedural patterns
float hash(vec2 p) {
    return fract(sin(dot(p, vec2(127.1, 311.7))) * 43758.5453);
}

// Smooth noise
float noise(vec2 p) {
    vec2 i = floor(p);
    vec2 f = fract(p);
    f = f * f * (3.0 - 2.0 * f);

    float a = hash(i);
    float b = hash(i + vec2(1.0, 0.0));
    float c = hash(i + vec2(0.0, 1.0));
    float d = hash(i + vec2(1.0, 1.0));

    return mix(mix(a, b, f.x), mix(c, d, f.x), f.y);
}

// Draw a single parallax layer
float draw_layer(vec2 uv_coord, float depth, float scale) {
    vec2 offset = u_camera_pos * depth / max(u_level_size, vec2(1.0));
    vec2 layer_uv = uv_coord * scale - offset;

    float pattern = 0.0;

    // Add some floating particles
    for (int i = 0; i < 3; i++) {
        vec2 particle_uv = layer_uv + vec2(float(i) * 0.3, u_time * 0.02 * (1.0 + float(i) * 0.5));
        float particle = noise(particle_uv * 8.0);
        particle = smoothstep(0.7, 0.75, particle);
        pattern += particle * 0.3;
    }

    return pattern * (1.0 - depth);
}

void main() {
    vec2 uv_coord = uv;

    // Base gradient
    vec3 color = mix(COLOR_TOP, COLOR_BOTTOM, uv_coord.y);

    // Parallax layers at different depths
    float depths[4];
    depths[0] = 0.1;
    depths[1] = 0.25;
    depths[2] = 0.5;
    depths[3] = 0.8;

    for (int i = 0; i < 4; i++) {
        float depth = depths[i];
        float scale = 2.0 + float(i) * 1.5;
        float layer = draw_layer(uv_coord, depth, scale);

        vec3 layer_color = mix(
            vec3(0.2, 0.4, 0.5),
            vec3(0.1, 0.3, 0.4),
            depth
        );

        color = mix(color, layer_color, layer * 0.5);
    }

    // Subtle vignette
    float vignette = 1.0 - length((uv_coord - 0.5) * 1.2);
    color *= 0.8 + 0.2 * vignette;

    // Caustics effect near top
    float caustics = noise(uv_coord * 10.0 + u_time * 0.5) * noise(uv_coord * 15.0 - u_time * 0.3);
    caustics *= smoothstep(0.5, 0.0, uv_coord.y) * 0.1;
    color += vec3(caustics);

    gl_FragColor = vec4(color, 1.0);
}
"#;

const PARALLAX_VERTEX: &str = r#"#version 100
precision mediump float;

attribute vec3 position;
attribute vec2 texcoord;

varying vec2 uv;

uniform mat4 Model;
uniform mat4 Projection;

void main() {
    gl_Position = Projection * Model * vec4(position, 1.0);
    uv = texcoord;
}
"#;

pub struct ParallaxBackground {
    material: Material,
}

impl ParallaxBackground {
    pub fn new() -> Result<Self, String> {
        let material = load_material(
            ShaderSource::Glsl {
                vertex: PARALLAX_VERTEX,
                fragment: PARALLAX_FRAGMENT,
            },
            MaterialParams {
                uniforms: vec![
                    UniformDesc::new("u_resolution", UniformType::Float2),
                    UniformDesc::new("u_camera_pos", UniformType::Float2),
                    UniformDesc::new("u_level_size", UniformType::Float2),
                    UniformDesc::new("u_time", UniformType::Float1),
                ],
                ..Default::default()
            },
        )
        .map_err(|e| format!("Failed to load shader: {:?}", e))?;

        Ok(Self { material })
    }

    pub fn draw(&self, camera_pos: Vec2, level_size: Vec2, time: f32) {
        self.material
            .set_uniform("u_resolution", (screen_width(), screen_height()));
        self.material
            .set_uniform("u_camera_pos", (camera_pos.x, camera_pos.y));
        self.material
            .set_uniform("u_level_size", (level_size.x, level_size.y));
        self.material.set_uniform("u_time", time);

        gl_use_material(&self.material);
        draw_rectangle(0.0, 0.0, screen_width(), screen_height(), WHITE);
        gl_use_default_material();
    }
}
