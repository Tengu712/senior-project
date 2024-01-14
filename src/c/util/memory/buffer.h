#pragma once

#include <vulkan/vulkan.h>

typedef struct Buffer_t {
    VkBuffer buffer;
    VkDeviceMemory devMemory;
    VkMemoryRequirements memReqs;
} *Buffer;

void deleteBuffer(const VkDevice device, Buffer buffer);

Buffer createBuffer(
    const VkDevice device,
    const VkPhysicalDeviceMemoryProperties *physDevMemProps,
    VkBufferUsageFlags usage,
    VkMemoryPropertyFlags memPropFlags,
    VkDeviceSize size
);
