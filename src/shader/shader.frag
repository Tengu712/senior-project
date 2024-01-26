#version 450

#define M_PI 3.14159265358f

layout(binding = 0) uniform Uniform {
    vec4 uniCameraPosition;
    vec4 uniLightPosition;
    vec4 uniLightAmbient;
    vec4 uniLightDiffuse;
    vec4 uniLightSpecular;
    vec4 uniModelAmbient;
    vec4 uniModelDiffuse;
    vec4 uniModelSpecular;
    float uniModelShininess;
};
// layout(binding=1) uniform sampler2D diffuseMap;

layout(push_constant) uniform PushConstant {
    vec4 param;
} constant;

layout(location=0) in vec4 bridgePosition;
layout(location=1) in vec4 bridgeNormal;
layout(location=2) in vec4 bridgeUV;

layout(location=0) out vec4 outColor;

float gaussian(float ave, float variance, float x) {
    return 1.0f / sqrt(2.0f * M_PI * variance) * exp(-pow(x - ave, 2.0f) / (2.0f * variance));
}

void main() {
    // phong reflection
    if (constant.param.x < 1.0) {
        vec4 position = bridgePosition;
        vec4 normal = bridgeNormal;
        vec3 vecNormal = vec3(normal.xyz);
        vec3 vecToLight = normalize(uniLightPosition.xyz - position.xyz);
        vec3 vecToCamera = normalize(uniCameraPosition.xyz - position.xyz);
        vec3 vecReflect = normalize(-vecToLight + 2.0 * (dot(vecToLight, vecNormal) * vecNormal));
        float cosDiffuseAngle = dot(vecNormal, vecToLight);
        float cosDiffuseAngleClamp = clamp(cosDiffuseAngle, 0.0, 1.0);
        float cosReflectAngle = dot(vecToCamera, vecReflect);
        float cosReflectAngleClamp = clamp(cosReflectAngle, 0.0, 1.0);
        vec3 color =
        uniModelAmbient.xyz * uniLightAmbient.xyz
        + uniModelDiffuse.xyz * cosDiffuseAngleClamp * uniLightDiffuse.xyz
        + uniModelSpecular.xyz * pow(cosReflectAngleClamp, uniModelShininess) * uniLightSpecular.xyz;
        outColor = /* texture(diffuseMap, bridgeUV.xy) * */ vec4(color, 1.0);
    }
    // gaussian blur
    else if (constant.param.x < 2.0) {
        vec4 color = vec4(0.0);
        for (int i = -5; i <= 5; ++i) {
            for (int j = -5; j <= 5; ++j) {
                float u = bridgeUV.x + float(i);
                float v = bridgeUV.y + float(j);
                if (u < 0.0f || u > 1.0f || v < 0.0f || v > 1.0f) {
                    color += /* texture(diffuseMap, vec2(u, v)) * */ gaussian(0.0f, 1.0f, i) * gaussian(0.0f, 1.0f, j);
                }
            }
        }
        outColor = color;
    }
}
