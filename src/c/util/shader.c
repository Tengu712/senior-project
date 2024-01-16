#define _CRT_SECURE_NO_WARNINGS

#include "shader.h"

#include "error.h"

#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

const char *readBinaryFile(const char *path, long int *size) {
#define CHECK(p, m, d) ERROR_IF(!(p), "readBinaryFile()", (m), d, NULL)

    FILE *file;
    {
        file = fopen(path, "rb");
        CHECK(file != NULL, "failed to open a file", {});
    }

    fpos_t pos = 0;
    {
        CHECK(fseek(file, 0L, SEEK_END) == 0, "failed to seek end.", fclose(file));
        CHECK(fgetpos(file, &pos) == 0, "failed to get the file size.", fclose(file));
        CHECK(fseek(file, 0L, SEEK_SET) == 0, "failed to seek set.", fclose(file));
    }

    const char *binary;
    {
        binary = (const char *)malloc(sizeof(const char) * pos);
        CHECK(
            fread((void *)binary, sizeof(const char), pos, file) == (size_t)pos,
            "failed to read a file.",
            { free((void *)binary); fclose(file); }
        );
    }

    fclose(file);

    if (size != NULL) {
        *size = (long int)pos;
    }
    
    return binary;

#undef CHECK
}

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
