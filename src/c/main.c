#include "app.h"

void ff_delete_vulkan_app(VulkanApp app) {
    deleteVulkanApp(app);
}

VulkanApp ff_create_vulkan_app(void) {
    return createVulkanApp();
}

int ff_render(VulkanApp app, uint64_t *time) {
    return render(app, time);
}
