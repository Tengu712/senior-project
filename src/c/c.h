#pragma once

#include "util/memory/image.h"
#include "util/memory/buffer.h"

#include <stdint.h>
#include <vulkan/vulkan.h>

typedef struct VulkanApp_t {
    // core
    VkInstance instance;
    VkPhysicalDevice physDevice;
    VkPhysicalDeviceMemoryProperties physDevMemProps;
    VkDevice device;
    VkQueue queue;
    VkCommandPool cmdPool;
    // rendering
    Image renderTargetImage;
    VkImageView renderTargetImageView;
    VkRenderPass renderPass;
    VkFramebuffer framebuffer;
    VkDescriptorPool descPool;
    // model
    uint32_t indicesCount;
    Buffer vtxBuffer;
    Buffer idxBuffer;
    // pipeline
    VkDescriptorSetLayout descSetLayout;
    VkShaderModule vertShader;
    VkShaderModule fragShader;
    VkPipelineLayout pipelineLayout;
    VkPipeline pipeline;
} *VulkanApp;

void releasePipeline(VulkanApp app);

VkCommandBuffer allocateAndStartCommandBuffer(VulkanApp app);

int endAndSubmitCommandBuffer(VulkanApp app, VkCommandBuffer cmdBuffer);

typedef struct TempObjsSaveRenderingResult_t {
    Buffer buffer;
    int mapped;
    uint8_t *pixels;
} TempObjsSaveRenderingResult;

void deleteTempObjsSaveRenderingResult(const VulkanApp app, TempObjsSaveRenderingResult *temp);

int saveRenderingResult(VulkanApp app);
