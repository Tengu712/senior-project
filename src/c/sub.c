#define _CRT_SECURE_NO_WARNINGS

#include "sub.h"

#define STB_IMAGE_IMPLEMENTATION
#define STB_IMAGE_WRITE_IMPLEMENTATION
#include "../../external/stb_image.h"
#include "../../external/stb_image_write.h"
#include "util/error.h"

#include <stdio.h>
#include <stdlib.h>

typedef struct TempObjsSaveRenderingResult_t {
    Buffer buffer;
    int mapped;
    uint8_t *pixels;
} TempObjsSaveRenderingResult;

void deleteTempObjsSaveRenderingResult(VulkanApp app, TempObjsSaveRenderingResult *temp) {
    if (temp == NULL) {
        return;
    }
    if (app->device != NULL) vkDeviceWaitIdle(app->device);
    if (temp->pixels != NULL) free((void *)temp->pixels);
    if (temp->mapped) vkUnmapMemory(app->device, temp->buffer->devMemory);
    if (temp->buffer != NULL) deleteBuffer(app->device, temp->buffer);
}

int saveRenderingResult(VulkanApp app) {
#define CHECK_VK(p, m) ERROR_IF((p) != VK_SUCCESS, "save_rendering_result()", (m), deleteTempObjsSaveRenderingResult(app, &temp), 0)
#define CHECK(p, m)    ERROR_IF(!(p),              "save_rendering_result()", (m), deleteTempObjsSaveRenderingResult(app, &temp), 0)
#define COMMAND_BUFFERS_COUNT 1

    TempObjsSaveRenderingResult temp = {
        NULL,
        0,
        NULL
    };

    {
        temp.buffer = createBuffer(
            app->device,
            &app->physDevMemProps,
            VK_BUFFER_USAGE_TRANSFER_DST_BIT,
            VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT,
            app->renderTargetImage->memReqs.size
        );
        CHECK(temp.buffer != NULL, "failed to create a temporary buffer.");
    }

    void *mappedData;
    {
        CHECK_VK(
            vkMapMemory(app->device, temp.buffer->devMemory, 0, temp.buffer->memReqs.size, 0, &mappedData),
            "failed to map a temporary buffer."
        );
        temp.mapped = 1;
    }

    VkCommandBuffer cmdBuffer;
    {
        cmdBuffer = allocateAndStartCommandBuffer(app);
        CHECK(cmdBuffer != NULL, "failed to allocate or start a command buffer.");
    }

    {
#define REGIONS_COUNT 1
        const VkBufferImageCopy regions[REGIONS_COUNT] = {
            {
                0,
                0,
                0,
                { VK_IMAGE_ASPECT_COLOR_BIT, 0, 0, 1 },
                { 0, 0, 0 },
                app->renderTargetImage->extent,
            }
        };
        vkCmdCopyImageToBuffer(
            cmdBuffer,
            app->renderTargetImage->image,
            VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL,
            temp.buffer->buffer,
            REGIONS_COUNT,
            regions
        );
#undef REGIONS_COUNT
    }

    CHECK(endAndSubmitCommandBuffer(app, cmdBuffer), "failed to end or submit a command buffer.");

    vkDeviceWaitIdle(app->device);

    {
        const uint32_t wh = app->renderTargetImage->extent.width * app->renderTargetImage->extent.height;
        temp.pixels = (uint8_t *)malloc(sizeof(uint8_t) * wh * 4);
        for (uint32_t i = 0; i < wh; ++i) {
            temp.pixels[i * 4 + 0] = ((uint8_t *)mappedData)[i * 4 + 2];
            temp.pixels[i * 4 + 1] = ((uint8_t *)mappedData)[i * 4 + 1];
            temp.pixels[i * 4 + 2] = ((uint8_t *)mappedData)[i * 4 + 0];
            temp.pixels[i * 4 + 3] = ((uint8_t *)mappedData)[i * 4 + 3];
        }
    }

    CHECK(
        stbi_write_png(
            "rendering-result.png",
            app->renderTargetImage->extent.width,
            app->renderTargetImage->extent.height,
            sizeof(uint8_t) * 4,
            (const void *)temp.pixels,
            0
        ),
        "failed to save the rendering result."
    );

    deleteTempObjsSaveRenderingResult(app, &temp);
    return 1;

#undef COMMAND_BUFFERS_COUNT
#undef CHECK
#undef CHECK_VK
}
