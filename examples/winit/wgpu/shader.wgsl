struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) rgba: u32,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) rgba: vec4<f32>,
};

@vertex
fn vs_main(
    in: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.rgba = unpack4x8unorm(in.rgba);
    out.clip_position = vec4<f32>(in.position, 1.0, 1.0);
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.rgba;
}
