struct VertexInput {
    @location(0) xy: u32,
    @location(1) rgba: u32,
};

struct VertexOutput {
    @builtin(position) xyzw: vec4<f32>,
    @location(0) rgba: vec4<f32>,
};

@vertex
fn vs_main(
    in: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;

    out.xyzw = vec4<f32>(unpack2x16snorm(in.xy), 1.0, 1.0);
    out.rgba = unpack4x8unorm(in.rgba);

    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.rgba;
}
