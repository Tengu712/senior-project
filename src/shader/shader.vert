#version 450

layout(location=0) in vec3 inPos;
layout(location=1) in vec2 inUV;

layout(location=0) out vec4 bridgePosition;
layout(location=1) out vec4 bridgeNormal;
layout(location=2) out vec4 bridgeUV;

void main() {
    gl_Position = vec4(inPos, 1.0);
    bridgePosition = vec4(inPos, 1.0);
    bridgeNormal = vec4(0.0, 0.0, 1.0, 1.0);
    bridgeUV = vec4(inUV.x, inUV.y, 0.0, 0.0);
}
