#pragma once

#include "util/memory/image.h"
#include "util/memory/buffer.h"

#include <stdint.h>
#include <vulkan/vulkan.h>

#define WIDTH 1920
#define HEIGHT 1080

typedef struct Vertex_t {
    float inPos[3];
    float inUV[2];
} Vertex;

typedef struct Uniform_t {
    float uniCameraPosition[4];
    float uniLightPosition[4];
    float uniLightAmbient[4];
    float uniLightDiffuse[4];
    float uniLightSpecular[4];
    float uniModelAmbient[4];
    float uniModelDiffuse[4];
    float uniModelSpecular[4];
    float uniModelShininess;
} Uniform;

typedef struct PushConstant_t {
    float param[4];
} PushConstant;

typedef struct VulkanApp_t {
    // core
    VkInstance instance;
    VkPhysicalDevice physDevice;
    VkPhysicalDeviceMemoryProperties physDevMemProps;
    VkDevice device;
    VkQueue queue;
    VkCommandPool cmdPool;
    VkQueryPool queryPool;
    // rendering
    Image renderTargetImage;
    VkImageView renderTargetImageView;
    VkRenderPass renderPass;
    VkFramebuffer framebuffer;
    // shader binding
    VkDescriptorPool descPool;
    VkDescriptorSetLayout descSetLayout;
    VkDescriptorSet descSet;
    Buffer uniformBuffer;
    // model
    uint32_t indicesCount;
    Buffer vtxBuffer;
    Buffer idxBuffer;
    // pipeline
    VkShaderModule vertShader;
    VkShaderModule fragShader;
    VkPipelineLayout pipelineLayout;
    VkPipeline pipeline;
} *VulkanApp;

void deleteVulkanApp(VulkanApp app);

VulkanApp createVulkanApp(void);

VkCommandBuffer allocateAndStartCommandBuffer(VulkanApp app);

int endAndSubmitCommandBuffer(VulkanApp app, VkCommandBuffer cmdBuffer);

int render(VulkanApp app, uint64_t *time);
