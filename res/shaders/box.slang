struct VertexInput {
    float3 position;
    float3 color;
}

struct VertexOutput {
    float4 position: SV_Position;
    float3 color;
}

struct FragmentOutput {
    float4 color: SV_Target;
}

struct ModelData {
    column_major float4x4 transform;
}

[vk::binding(0, 0)]
ConstantBuffer<ModelData> model_data;

[shader("vertex")]
VertexOutput vs_main(VertexInput vin) {
    VertexOutput vout;
    float4 position = float4(vin.position, 1.0);

    vout.position = mul(model_data.transform, position);
    vout.color = vin.color;

    return vout;
}

[shader("fragment")]
FragmentOutput fs_main(VertexOutput vout) {
    FragmentOutput fout;
    fout.color = float4(vout.color, 1.0);

    return fout;
}
