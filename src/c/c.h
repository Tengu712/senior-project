#pragma once

#include "util/memory/image.h"

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
