#pragma once

#include <vulkan/vulkan.h>

VkShaderModule createShaderModuleFromFile(const VkDevice device, const char *path);
