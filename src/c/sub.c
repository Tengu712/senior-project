#define _CRT_SECURE_NO_WARNINGS

#include "c.h"

#define STB_IMAGE_IMPLEMENTATION
#define STB_IMAGE_WRITE_IMPLEMENTATION
#include "../../external/stb_image.h"
#include "../../external/stb_image_write.h"
#include "util/error.h"

#include <stdio.h>
#include <stdlib.h>

void releasePipeline(VulkanApp app) {
    if (app == NULL) {
        return;
    }
    if (app->device) vkDeviceWaitIdle(app->device);
    if (app->pipeline != NULL) vkDestroyPipeline(app->device, app->pipeline, NULL);
    if (app->fragShader != NULL) vkDestroyShaderModule(app->device, app->fragShader, NULL);
    if (app->pipelineLayout != NULL) vkDestroyPipelineLayout(app->device, app->pipelineLayout, NULL);
    if (app->descSetLayout != NULL) vkDestroyDescriptorSetLayout(app->device, app->descSetLayout, NULL);
}

VkCommandBuffer allocateAndStartCommandBuffer(VulkanApp app) {
#define CHECK_VK(p, m) ERROR_IF((p) != VK_SUCCESS, "createAndStartCommandBuffer()", (m), {}, NULL)

    VkCommandBuffer cmdBuffer;
    {
        const VkCommandBufferAllocateInfo ai = {
            VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO,
            NULL,
            app->cmdPool,
            VK_COMMAND_BUFFER_LEVEL_PRIMARY,
            1,
        };
        CHECK_VK(vkAllocateCommandBuffers(app->device, &ai, &cmdBuffer), "failed to allocate a command buffer.");
    }

    {
        const VkCommandBufferBeginInfo bi = {
            VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO,
            NULL,
            VK_COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT,
            NULL,
        };
        CHECK_VK(vkBeginCommandBuffer(cmdBuffer, &bi), "failed to begin a command buffer.");
    }

    return cmdBuffer;

#undef CHECK_VK
}

int endAndSubmitCommandBuffer(VulkanApp app, VkCommandBuffer cmdBuffer) {
#define CHECK_VK(p, m) ERROR_IF((p) != VK_SUCCESS, "submitCommandBuffer()", (m), {}, 0)

    vkEndCommandBuffer(cmdBuffer);

    {
#define SUBMITS_COUNT 1
#define WAIT_SEMAPHORES_COUNT 0
#define COMMAND_BUFFERS_COUNT 1
#define SIGNAL_SEMAPHORES_COUNT 0
        const VkCommandBuffer cmdBuffers[COMMAND_BUFFERS_COUNT] = { cmdBuffer };
        const VkSubmitInfo sis[SUBMITS_COUNT] = {
            {
                VK_STRUCTURE_TYPE_SUBMIT_INFO,
                NULL,
                WAIT_SEMAPHORES_COUNT,
                NULL,
                NULL,
                COMMAND_BUFFERS_COUNT,
                cmdBuffers,
                SIGNAL_SEMAPHORES_COUNT,
                NULL,
            },
        };
        CHECK_VK(vkQueueSubmit(app->queue, SUBMITS_COUNT, sis, NULL), "failed to submit a command buffer.");
#undef SIGNAL_SEMAPHORES_COUNT
#undef COMMAND_BUFFERS_COUNT
#undef WAIT_SEMAPHORES_COUNT
#undef SUBMITS_COUNT
    }

    return 1;

#undef CHECK_VK
}

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
    
#undef CHECK
#undef CHECK_VK
}
