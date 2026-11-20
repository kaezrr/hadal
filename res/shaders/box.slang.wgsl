struct _MatrixStorage_float4x4_ColMajorstd140_0
{
    @align(16) data_0 : array<vec4<f32>, i32(4)>,
};

struct ModelData_std140_0
{
    @align(16) transform_0 : _MatrixStorage_float4x4_ColMajorstd140_0,
};

@binding(0) @group(0) var<uniform> model_data_0 : ModelData_std140_0;
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
    var vout_0 : VertexOutput_0;
    vout_0.position_0 = (((mat4x4<f32>(model_data_0.transform_0.data_0[i32(0)][i32(0)], model_data_0.transform_0.data_0[i32(1)][i32(0)], model_data_0.transform_0.data_0[i32(2)][i32(0)], model_data_0.transform_0.data_0[i32(3)][i32(0)], model_data_0.transform_0.data_0[i32(0)][i32(1)], model_data_0.transform_0.data_0[i32(1)][i32(1)], model_data_0.transform_0.data_0[i32(2)][i32(1)], model_data_0.transform_0.data_0[i32(3)][i32(1)], model_data_0.transform_0.data_0[i32(0)][i32(2)], model_data_0.transform_0.data_0[i32(1)][i32(2)], model_data_0.transform_0.data_0[i32(2)][i32(2)], model_data_0.transform_0.data_0[i32(3)][i32(2)], model_data_0.transform_0.data_0[i32(0)][i32(3)], model_data_0.transform_0.data_0[i32(1)][i32(3)], model_data_0.transform_0.data_0[i32(2)][i32(3)], model_data_0.transform_0.data_0[i32(3)][i32(3)])) * (vec4<f32>(_S1.position_1, 1.0f))));
    vout_0.color_0 = _S1.color_1;
    return vout_0;
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
    var fout_0 : FragmentOutput_0;
    fout_0.color_2 = vec4<f32>(_S2.color_3, 1.0f);
    return fout_0;
}

