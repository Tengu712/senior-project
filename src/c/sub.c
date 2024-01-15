#include "c.h"

#include "util/error.h"

#include <stdio.h>

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
