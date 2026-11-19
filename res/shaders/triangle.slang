struct VertexInput
{
    float3 position : POSITION;
    float3 color : COLOR0;
}

struct VertexOutput
{
    float4 position : SV_Position;
    float3 color : COLOR0;
}

struct FragmentOutput
{
    float4 color : SV_Target;
}

[shader("vertex")]
VertexOutput vs_main(VertexInput vin)
{
    VertexOutput out;
    out.position = float4(vin.position, 1.0);
    out.color = vin.color;
    return out;
}

[shader("fragment")]
FragmentOutput fs_main(VertexOutput vout)
{
    FragmentOutput out;
    out.color = float4(vout.color, 1.0);
    return out;
}
