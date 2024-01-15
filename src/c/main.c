#include "c.h"

#include "util/error.h"
#include "util/shader.h"

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define WIDTH 1920
#define HEIGHT 1080

typedef struct Camera_t {
    float proj[16];
} Camera;

typedef struct PushConstant_t {
    float scl[4];
    float trs[4];
    float uv[4];
} PushConstant;

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

void delete_vulkan_app(VulkanApp app) {
    if (app == NULL) {
        return;
    }
    if (app->device != NULL) vkDeviceWaitIdle(app->device);

    // pipeline
    releasePipeline(app);
    if (app->vertShader != NULL) vkDestroyShaderModule(app->device, app->vertShader, NULL);

    // rendering
    if (app->descPool != NULL) vkDestroyDescriptorPool(app->device, app->descPool, NULL);
    if (app->framebuffer != NULL) vkDestroyFramebuffer(app->device, app->framebuffer, NULL);
    if (app->renderPass != NULL) vkDestroyRenderPass(app->device, app->renderPass, NULL);
    if (app->renderTargetImageView != NULL) vkDestroyImageView(app->device, app->renderTargetImageView, NULL);
    if (app->renderTargetImage != NULL) deleteImage(app->device, app->renderTargetImage);

    // core
    if (app->cmdPool != NULL) vkDestroyCommandPool(app->device, app->cmdPool, NULL);
    if (app->device != NULL) vkDestroyDevice(app->device, NULL);
    if (app->instance != NULL) vkDestroyInstance(app->instance, NULL);
    free((void *)app);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

VulkanApp create_vulkan_app(void) {
#define CHECK_VK(p, m) ERROR_IF((p) != VK_SUCCESS, "create_vulkan_app()", (m), delete_vulkan_app(app), NULL)
#define CHECK(p, m)    ERROR_IF(!(p),              "create_vulkan_app()", (m), delete_vulkan_app(app), NULL)

    // create app
    const VulkanApp app = (const VulkanApp)malloc(sizeof(struct VulkanApp_t));
    memset(app, 0, sizeof(struct VulkanApp_t));

    // ===================================================================================================================================================== //
    //     core                                                                                                                                              //
    // ===================================================================================================================================================== //

    // create instance
    {
#define LAYER_NAMES_COUNT 1
#define EXT_NAMES_COUNT 0
        const char *instLayerNames[LAYER_NAMES_COUNT] = { "VK_LAYER_KHRONOS_validation" };
        const VkApplicationInfo ai = {
            VK_STRUCTURE_TYPE_APPLICATION_INFO,
            NULL,
            "VulkanApplication\0",
            0,
            "VulkanApplication\0",
            VK_MAKE_VERSION(1, 0, 0),
            VK_API_VERSION_1_2,
        };
        const VkInstanceCreateInfo ci = {
            VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
            NULL,
            0,
            &ai,
            LAYER_NAMES_COUNT,
            instLayerNames,
            EXT_NAMES_COUNT,
            NULL,
        };
        CHECK_VK(vkCreateInstance(&ci, NULL, &app->instance), "failed to create a Vulkan instance.");
#undef LAYER_NAMES_COUNT
#undef EXT_NAMES_COUNT
    }

    // get a physical device
    {
        uint32_t count = 0;
        CHECK_VK(vkEnumeratePhysicalDevices(app->instance, &count, NULL), "failed to get the number of physical devices.");
        VkPhysicalDevice *physDevices = (VkPhysicalDevice *)malloc(sizeof(VkPhysicalDevice) * count);
        CHECK_VK(vkEnumeratePhysicalDevices(app->instance, &count, physDevices), "failed to enumerate physical devices.");

        app->physDevice = physDevices[0];
        free(physDevices);

        vkGetPhysicalDeviceMemoryProperties(app->physDevice, &app->physDevMemProps);
    }

    // get a queue family index
    int32_t queueFamIndex = -1;
    {
        uint32_t count = 0;
        vkGetPhysicalDeviceQueueFamilyProperties(app->physDevice, &count, NULL);
        VkQueueFamilyProperties *props = (VkQueueFamilyProperties *)malloc(sizeof(VkQueueFamilyProperties) * count);
        vkGetPhysicalDeviceQueueFamilyProperties(app->physDevice, &count, props);

        for (int32_t i = 0; i < (int32_t)count; ++i) {
            if ((props[i].queueFlags & VK_QUEUE_GRAPHICS_BIT) > 0) {
                queueFamIndex = i;
                break;
            }
        }
        free(props);
        CHECK(queueFamIndex >= 0, "failed to get a queue family index.");
    }

    // create a logical device
    {
#define QUEUE_FAMILIES_COUNT 1
#define QUEUES_COUNT 1
#define LAYER_NAMES_COUNT 0
#define EXT_NAMES_COUNT 0
        const float queuePriors[QUEUES_COUNT] = { 1.0f };
        const VkDeviceQueueCreateInfo queueCIs[QUEUE_FAMILIES_COUNT] = {
            {
                VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO,
                NULL,
                0,
                queueFamIndex,
                QUEUES_COUNT,
                queuePriors,
            },
        };
        const VkDeviceCreateInfo ci = {
            VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO,
            NULL,
            0,
            QUEUE_FAMILIES_COUNT,
            queueCIs,
            LAYER_NAMES_COUNT,
            NULL,
            EXT_NAMES_COUNT,
            NULL,
            NULL,
        };
        CHECK_VK(vkCreateDevice(app->physDevice, &ci, NULL, &app->device), "failed to create a logical device.");
#undef LAYER_NAMES_COUNT
#undef EXT_NAMES_COUNT
#undef QUEUES_COUNT
#undef QUEUE_FAMILIES_COUNT
    }

    // get a queue
    vkGetDeviceQueue(app->device, queueFamIndex, 0, &app->queue);

    // create a command pool
    {
        const VkCommandPoolCreateInfo ci = {
            VK_STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO,
            NULL,
            VK_COMMAND_POOL_CREATE_TRANSIENT_BIT,
            queueFamIndex,
        };
        CHECK_VK(vkCreateCommandPool(app->device, &ci, NULL, &app->cmdPool), "failed to create a command pool.");
    }

    // ===================================================================================================================================================== //
    //     rendering                                                                                                                                         //
    // ===================================================================================================================================================== //

#define ATTACHMENTS_COUNT 1
#define RENDER_TARGET_PIXEL_FORMAT VK_FORMAT_B8G8R8A8_SRGB

    // create a render target image
    {
        const VkExtent3D extent = { WIDTH, HEIGHT, 1 };
        app->renderTargetImage = createImage(
            app->device,
            &app->physDevMemProps,
            VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT | VK_IMAGE_USAGE_TRANSFER_SRC_BIT,
            VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT,
            RENDER_TARGET_PIXEL_FORMAT,
            &extent
        );
        CHECK(app->renderTargetImage != NULL, "failed to create a render target image.");
    }

    // create a render target image view
    {
        const VkImageViewCreateInfo ci = {
            VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO,
            NULL,
            0,
            app->renderTargetImage->image,
            VK_IMAGE_VIEW_TYPE_2D,
            RENDER_TARGET_PIXEL_FORMAT,
            { 0 },
            { VK_IMAGE_ASPECT_COLOR_BIT, 0, 1, 0, 1 },
        };
        CHECK_VK(vkCreateImageView(app->device, &ci, NULL, &app->renderTargetImageView), "failed to create a render target image view.");
    }

    // create a render pass
    {
        const VkAttachmentDescription attchDescs[ATTACHMENTS_COUNT] = {
            {
                0,
                RENDER_TARGET_PIXEL_FORMAT,
                VK_SAMPLE_COUNT_1_BIT,
                VK_ATTACHMENT_LOAD_OP_CLEAR,
                VK_ATTACHMENT_STORE_OP_STORE,
                VK_ATTACHMENT_LOAD_OP_DONT_CARE,
                VK_ATTACHMENT_STORE_OP_DONT_CARE,
                VK_IMAGE_LAYOUT_UNDEFINED,
                VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL,
            },
        };
        const VkAttachmentReference attchRefs[] = {
            {
                0,
                VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL,
            },
        };
        const VkSubpassDescription subpassDescs[] = {
            {
                0,
                VK_PIPELINE_BIND_POINT_GRAPHICS,
                0,
                NULL,
                1,
                attchRefs,
                NULL,
                NULL,
                0,
                NULL,
            },
        };
        const VkRenderPassCreateInfo ci = {
            VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO,
            NULL,
            0,
            ATTACHMENTS_COUNT,
            attchDescs,
            1,
            subpassDescs,
            0,
            NULL,
        };
        CHECK_VK(vkCreateRenderPass(app->device, &ci, NULL, &app->renderPass), "failed to create a render pass.");
    }

    // create a frame buffer
    {
        const VkImageView attachments[ATTACHMENTS_COUNT] = { app->renderTargetImageView };
        const VkFramebufferCreateInfo ci = {
            VK_STRUCTURE_TYPE_FRAMEBUFFER_CREATE_INFO,
            NULL,
            0,
            app->renderPass,
            ATTACHMENTS_COUNT,
            attachments,
            WIDTH,
            HEIGHT,
            1,
        };
        CHECK_VK(vkCreateFramebuffer(app->device, &ci, NULL, &app->framebuffer), "failed to create a framebuffer");
    }

    // create a descriptor pool
    {
#define SIZES_COUNT 1
        const VkDescriptorPoolSize sizes[SIZES_COUNT] = {
            {
                VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER,
                1,
            },
        };
        const VkDescriptorPoolCreateInfo ci = {
            VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_CREATE_INFO,
            NULL,
            VK_DESCRIPTOR_POOL_CREATE_FREE_DESCRIPTOR_SET_BIT,
            1,
            SIZES_COUNT,
            sizes,
        };
        CHECK_VK(vkCreateDescriptorPool(app->device, &ci, NULL, &app->descPool), "failed to create a descriptor pool.");
#undef SIZES_COUNT
    }

#undef RENDER_TARGET_PIXEL_FORMAT
#undef ATTACHMENTS_COUNT

    return app;

#undef CHECK
#undef CHECK_VK
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

int create_pipeline(VulkanApp app, const char *fragShaderPath) {
#define CHECK_VK(p, m) ERROR_IF((p) != VK_SUCCESS, "create_pipeline()", (m), releasePipeline(app), 0)
#define CHECK(p, m)    ERROR_IF(!(p),              "create_pipeline()", (m), releasePipeline(app), 0)

    releasePipeline(app);

    // descriptor set layout
    {
#define BINDINGS_COUNT 1
        const VkDescriptorSetLayoutBinding bindings[BINDINGS_COUNT] = {
            {
                0,
                VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER,
                1,
                VK_SHADER_STAGE_VERTEX_BIT,
                NULL,
            },
        };
        const VkDescriptorSetLayoutCreateInfo ci = {
            VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_CREATE_INFO,
            NULL,
            0,
            BINDINGS_COUNT,
            bindings,
        };
        CHECK_VK(vkCreateDescriptorSetLayout(app->device, &ci, NULL, &app->descSetLayout), "failed to create a descriptor set layout.");
#undef BINDINGS_COUNT
    }

    // pipeline layout
    {
#define DESC_SET_LAYOUTS_COUNT 1
#define PUSH_CONSTANTS_COUNT 1
        const VkDescriptorSetLayout descSetLayouts[DESC_SET_LAYOUTS_COUNT] = { app->descSetLayout };
        const VkPushConstantRange pushConstants[PUSH_CONSTANTS_COUNT] = {
            {
                VK_SHADER_STAGE_VERTEX_BIT,
                0,
                sizeof(PushConstant),
            },
        };
        const VkPipelineLayoutCreateInfo ci = {
            VK_STRUCTURE_TYPE_PIPELINE_LAYOUT_CREATE_INFO,
            NULL,
            0,
            DESC_SET_LAYOUTS_COUNT,
            descSetLayouts,
            PUSH_CONSTANTS_COUNT,
            pushConstants,
        };
        CHECK_VK(vkCreatePipelineLayout(app->device, &ci, NULL, &app->pipelineLayout), "failed to create a pipeline layout.");
#undef PUSH_CONSTANTS_COUNT
#undef DESC_SET_LAYOUTS_COUNT
    }

    // shader modules
    {
        if (app->vertShader == NULL) {
            app->vertShader = createShaderModuleFromFile(app->device, "./shader.vert.spv");
            CHECK(app->vertShader != NULL, "failed to create a shader module from ./shader.vert.spv");
        }
        app->fragShader = createShaderModuleFromFile(app->device, fragShaderPath);
        CHECK(app->fragShader != NULL, "failed to create a shader module from ./shader.frag.spv");
    }

    // pipeline
    {
#define SHADERS_COUNT 2
#define VERT_INP_BIND_DESCS_COUNT 1
#define VERT_INP_ATTR_DESCS_COUNT 2
#define VIEWPORTS_COUNT 1
#define COLOR_BLEND_ATTACHMENTS_COUNT 1

        const VkPipelineShaderStageCreateInfo shaderCIs[SHADERS_COUNT] = {
            {
                VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO,
                NULL,
                0,
                VK_SHADER_STAGE_VERTEX_BIT,
                app->vertShader,
                "main",
                NULL,
            },
            {
                VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO,
                NULL,
                0,
                VK_SHADER_STAGE_FRAGMENT_BIT,
                app->fragShader,
                "main",
                NULL,
            },
        };

        const VkVertexInputBindingDescription vertInpBindDescs[VERT_INP_BIND_DESCS_COUNT] = {
            { 0, sizeof(float) * 5, VK_VERTEX_INPUT_RATE_VERTEX },
        };
        const VkVertexInputAttributeDescription vertInpAttrDescs[VERT_INP_ATTR_DESCS_COUNT] = {
            // position
            { 0, 0, VK_FORMAT_R32G32B32_SFLOAT, 0 },
            // uv
            { 1, 0, VK_FORMAT_R32G32_SFLOAT, sizeof(float) * 3 },
        };
        const VkPipelineVertexInputStateCreateInfo vertInpCI = {
            VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO,
            NULL,
            0,
            VERT_INP_BIND_DESCS_COUNT,
            vertInpBindDescs,
            VERT_INP_ATTR_DESCS_COUNT,
            vertInpAttrDescs,
        };

        const VkPipelineInputAssemblyStateCreateInfo inpAssemCI = {
            VK_STRUCTURE_TYPE_PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO,
            NULL,
            0,
            VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST,
            VK_FALSE,
        };

        const VkViewport viewports[VIEWPORTS_COUNT] = {
            { 0.0f, 0.0f, (float)WIDTH, (float)HEIGHT, 0.0f, 1.0f },
        };
        const VkRect2D scissors[VIEWPORTS_COUNT] = {
            { {0, 0}, {WIDTH, HEIGHT} },
        };
        const VkPipelineViewportStateCreateInfo viewportCI = {
            VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_STATE_CREATE_INFO,
            NULL,
            0,
            VIEWPORTS_COUNT,
            viewports,
            VIEWPORTS_COUNT,
            scissors,
        };

        // ラスタライゼーションステート
        const VkPipelineRasterizationStateCreateInfo rasterCI = {
            VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_CREATE_INFO,
            NULL,
            0,
            VK_FALSE,
            VK_FALSE,
            VK_POLYGON_MODE_FILL,
            VK_CULL_MODE_NONE,
            VK_FRONT_FACE_COUNTER_CLOCKWISE,
            VK_FALSE,
            0.0f,
            0.0f,
            0.0f,
            1.0f,
        };

        // マルチサンプルステート
        const VkPipelineMultisampleStateCreateInfo multisampleCI = {
            VK_STRUCTURE_TYPE_PIPELINE_MULTISAMPLE_STATE_CREATE_INFO,
            NULL,
            0,
            VK_SAMPLE_COUNT_1_BIT,
            VK_FALSE,
            0.0f,
            NULL,
            VK_FALSE,
            VK_FALSE,
        };

        // カラーブレンドステート
        const VkPipelineColorBlendAttachmentState colorBlendAttchs[COLOR_BLEND_ATTACHMENTS_COUNT] = {
            {
                VK_TRUE,
                VK_BLEND_FACTOR_SRC_ALPHA,
                VK_BLEND_FACTOR_ONE_MINUS_SRC_ALPHA,
                VK_BLEND_OP_ADD,
                VK_BLEND_FACTOR_SRC_ALPHA,
                VK_BLEND_FACTOR_ONE_MINUS_SRC_ALPHA,
                VK_BLEND_OP_ADD,
                VK_COLOR_COMPONENT_R_BIT | VK_COLOR_COMPONENT_G_BIT | VK_COLOR_COMPONENT_B_BIT | VK_COLOR_COMPONENT_A_BIT,
            }
        };
        const VkPipelineColorBlendStateCreateInfo colorBlendCI = {
            VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_STATE_CREATE_INFO,
            NULL,
            0,
            VK_FALSE,
            (VkLogicOp)0,
            COLOR_BLEND_ATTACHMENTS_COUNT,
            colorBlendAttchs,
            {0.0f, 0.0f, 0.0f, 0.0f},
        };

        const VkGraphicsPipelineCreateInfo ci = {
            VK_STRUCTURE_TYPE_GRAPHICS_PIPELINE_CREATE_INFO,
            NULL,
            0,
            SHADERS_COUNT,
            shaderCIs,
            &vertInpCI,
            &inpAssemCI,
            NULL,
            &viewportCI,
            &rasterCI,
            &multisampleCI,
            NULL,
            &colorBlendCI,
            NULL,
            app->pipelineLayout,
            app->renderPass,
            0,
            NULL,
            0,
        };
        CHECK_VK(vkCreateGraphicsPipelines(app->device, VK_NULL_HANDLE, 1, &ci, NULL, &app->pipeline), "failed to create a pipeline.");

#undef COLOR_BLEND_ATTACHMENTS_COUNT
#undef VIEWPORTS_COUNT
#undef VERT_INP_ATTR_DESCS_COUNT
#undef VERT_INP_BIND_DESCS_COUNT
#undef SHADERS_COUNT
    }

    return 1;

#undef CHECK
#undef CHECK_VK
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

int render(VulkanApp app) {
#define CHECK_VK(p, m) ERROR_IF((p) != VK_SUCCESS, "render()", (m), {}, 0)
#define CHECK(p, m)    ERROR_IF(!(p),              "render()", (m), {}, 0)

    if (app->pipeline == NULL) {
        printf("[ warning ] render(): pipeline not created.\n");
        return 0;
    }

    // allocate and start a command buffer
    VkCommandBuffer cmdBuffer = allocateAndStartCommandBuffer(app);
    CHECK(cmdBuffer != NULL, "failed to allocate or start a command buffer for rendering.");

    // start a render pass
    {
        const VkClearValue clearValues[] = {
            {{ 0.5f, 0.0f, 0.0f, 1.0f }},
        };
        const VkRenderPassBeginInfo bi = {
            VK_STRUCTURE_TYPE_RENDER_PASS_BEGIN_INFO,
            NULL,
            app->renderPass,
            app->framebuffer,
            { {0, 0}, {WIDTH, HEIGHT} },
            1,
            clearValues,
        };
        vkCmdBeginRenderPass(cmdBuffer, &bi, VK_SUBPASS_CONTENTS_INLINE);
    }

    // bind a pipeline
    vkCmdBindPipeline(cmdBuffer, VK_PIPELINE_BIND_POINT_GRAPHICS, app->pipeline);

    // end a render pass
    vkCmdEndRenderPass(cmdBuffer);

    // end and submit a command buffer
    CHECK(endAndSubmitCommandBuffer(app, cmdBuffer), "failed to end or submit a command buffer for rendering.");

    // wait for queue
    CHECK_VK(vkQueueWaitIdle(app->queue), "failed to wait queue.");

    return 1;

#undef CHECK
#undef CHECK_VK
}
