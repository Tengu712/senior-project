#include "image.h"

#include "../error.h"
#include "memory.h"

#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

void deleteImage(const VkDevice device, Image image) {
    if (image == NULL) {
        return;
    }
    vkDeviceWaitIdle(device);
    if (image->devMemory != NULL) vkFreeMemory(device, image->devMemory, NULL);
    if (image->image != NULL) vkDestroyImage(device, image->image, NULL);
    free((void *)image);
}

Image createImage(
    const VkDevice device,
    const VkPhysicalDeviceMemoryProperties *physDevMemProps,
    VkImageUsageFlags usage,
    VkMemoryPropertyFlags memPropFlags,
    VkFormat format,
    const VkExtent3D *extent
) {
#define CHECK_VK(p, m) ERROR_IF((p) != VK_SUCCESS, "createImage()", (m), deleteImage(device, image), NULL)
#define CHECK(p, m)    ERROR_IF(!(p),              "createImage()", (m), deleteImage(device, image), NULL)

    const Image image = (Image)malloc(sizeof(struct Image_t));
    memset(image, 0, sizeof(struct Image_t));

    image->extent = *extent;

    {
        const VkImageCreateInfo ci = {
            VK_STRUCTURE_TYPE_IMAGE_CREATE_INFO,
            NULL,
            0,
            VK_IMAGE_TYPE_2D,
            format,
            image->extent,
            1,
            1,
            VK_SAMPLE_COUNT_1_BIT,
            VK_IMAGE_TILING_OPTIMAL,
            usage,
            VK_SHARING_MODE_EXCLUSIVE,
            0,
            NULL,
            VK_IMAGE_LAYOUT_UNDEFINED,
        };
        CHECK_VK(vkCreateImage(device, &ci, NULL, &image->image), "failed to create an image.");
    }

    vkGetImageMemoryRequirements(device, image->image, &image->memReqs);

    {
        image->devMemory = allocateDeviceMemory(
            device,
            physDevMemProps,
            image->memReqs.memoryTypeBits,
            memPropFlags,
            image->memReqs.size
        );
        CHECK(image->devMemory != NULL, "failed to allocate device memory for an image.");
    }

    CHECK_VK(vkBindImageMemory(device, image->image, image->devMemory, 0), "failed to bind a device memory and an image.");

    return image;

#undef CHECK
#undef CHECK_VK
}
