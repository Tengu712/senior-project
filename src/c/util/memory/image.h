#pragma once

#include <vulkan/vulkan.h>

typedef struct Image_t {
    VkExtent3D extent;
    VkImage image;
    VkDeviceMemory devMemory;
    VkMemoryRequirements memReqs;
} *Image;

void deleteImage(const VkDevice device, Image image);

Image createImage(
    const VkDevice device,
    const VkPhysicalDeviceMemoryProperties *physDevMemProps,
    VkImageUsageFlags usage,
    VkMemoryPropertyFlags memPropFlags,
    VkFormat format,
    const VkExtent3D *extent
);
