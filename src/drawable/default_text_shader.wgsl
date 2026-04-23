
// Vertex shader

struct TransformationUniform {
    matrix: mat4x4<f32>
};
@group(0) @binding(0)
var<uniform> transform: TransformationUniform;

struct CameraUniform {
    view_proj: mat4x4<f32>
};
@group(1) @binding(0)
var<uniform> camera: CameraUniform;

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>
};


@vertex
fn vs_main(
    @builtin(vertex_index) in_vertex_index: u32
) -> VertexOutput {

    let pos_x_arr : array<f32, 6> = array<f32, 6>(0, 1, 1, 0, 1, 0);
    let pos_y_arr : array<f32, 6> = array<f32, 6>(0, 0, 1, 0, 1, 1);

    let tex_x_arr : array<f32, 6> = array<f32, 6>(0, 1, 1, 0, 1, 0);
    let tex_y_arr : array<f32, 6> = array<f32, 6>(1, 1, 0, 1, 0, 0);

    let x = pos_x_arr[in_vertex_index];
    let y = pos_y_arr[in_vertex_index];

    var tex_cords = vec2<f32>(tex_x_arr[in_vertex_index], tex_y_arr[in_vertex_index]);

    let pos_x = (x - 0.5) * 50.0;
    let pos_y = (y - 0.5) * 50.0;

    var out: VertexOutput;
    out.clip_position = camera.view_proj * transform.matrix * vec4<f32>(pos_x, pos_y, 0.0, 1.0);
    out.tex_coords = tex_cords;
    return out;
}


// Fragment shader

@group(2) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(2) @binding(1)
var s_diffuse: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {

    return textureSample(t_diffuse, s_diffuse, in.tex_coords);
}
