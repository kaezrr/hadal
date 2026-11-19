struct VertexOutput_0
{
    @builtin(position) position_0 : vec4<f32>,
    @location(0) color_0 : vec3<f32>,
};

struct vertexInput_0
{
    @location(0) position_1 : vec3<f32>,
    @location(1) color_1 : vec3<f32>,
};

@vertex
fn vs_main( _S1 : vertexInput_0) -> VertexOutput_0
{
    var out_0 : VertexOutput_0;
    out_0.position_0 = vec4<f32>(_S1.position_1, 1.0f);
    out_0.color_0 = _S1.color_1;
    return out_0;
}

struct FragmentOutput_0
{
    @location(0) color_2 : vec4<f32>,
};

struct pixelInput_0
{
    @location(0) color_3 : vec3<f32>,
};

@fragment
fn fs_main( _S2 : pixelInput_0, @builtin(position) position_2 : vec4<f32>) -> FragmentOutput_0
{
    var out_1 : FragmentOutput_0;
    out_1.color_2 = vec4<f32>(_S2.color_3, 1.0f);
    return out_1;
}

