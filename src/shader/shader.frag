#version 450

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

layout(location=0) in vec4 bridgePosition;
layout(location=1) in vec4 bridgeNormal;

layout(location=0) out vec4 outColor;

void main() {
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
    outColor = vec4(color, 1.0);
}
