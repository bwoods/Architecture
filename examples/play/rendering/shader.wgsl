struct VertexInput {
	@location(0) xy: u32,
	@location(1) s: u32, // sRGBA
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
	out.rgba = to_linear(unpack4x8unorm(in.s));

	return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
	return in.rgba;
}

// sRGB → linear color space (https://entropymine.com/imageworsener/srgbformula/)
fn to_linear(s: vec4<f32>) -> vec4<f32>
{
	let chi = vec3<f32>(0.0404482362771082);
	let phi = vec3<f32>(12.92);

	let gamma = vec3<f32>(2.4);
	let alpha = vec3<f32>(0.055);

	let exp = pow((s.rgb + alpha) / (1 + alpha), gamma);
	let lin = s.rgb / phi;

	return vec4(select(exp, lin, s.rgb <= chi), s.a);
}