#pragma once

#include <stdint.h>
#include <vulkan/vulkan.h>

VkDeviceMemory allocateDeviceMemory(
    const VkDevice device,
    const VkPhysicalDeviceMemoryProperties *physDevMemProps,
    uint32_t type,
    VkMemoryPropertyFlags memPropFlags,
    VkDeviceSize size
);

int uploadToDeviceMemory(const VkDevice device, const VkDeviceMemory devMemory, const void *source, uint32_t size);
