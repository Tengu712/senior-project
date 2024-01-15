#include "shader.h"

#include "error.h"
#include "file.h"

#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

VkShaderModule createShaderModuleFromFile(const VkDevice device, const char *path) {
#define CHECK_VK(p, m, d) ERROR_IF((p) != VK_SUCCESS, "createShaderModuleFromFile()", (m), d, NULL)
#define CHECK(p, m, d)    ERROR_IF(!(p),              "createShaderModuleFromFile()", (m), d, NULL)

    const char *binary;
    long int binarySize = 0;
    {
        binary = readBinaryFile(path, &binarySize);
        CHECK(binary != NULL, "failed to read a shader file.", {});
    }

    VkShaderModule shader;
    {
        const VkShaderModuleCreateInfo ci = {
            VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO,
            NULL,
            0,
            binarySize,
            (const uint32_t*)binary,
        };
        CHECK_VK(vkCreateShaderModule(device, &ci, NULL, &shader), "failed to create a shader module.", free((void *)binary));
    }

    free((void *)binary);

    return shader;

#undef CHECK
#undef CHECK_VK
}
