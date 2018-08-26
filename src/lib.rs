extern crate ash;

pub type BufferType = ash::vk::Buffer;
pub type ImageType = ash::vk::Image;
pub type ImageSubresourceRangeType = ash::vk::ImageSubresourceRange;

pub mod cmd;

/// Defines all potential resource usages
pub enum AccessType {
    /// No access. Useful primarily for initialization
    Nothing,

    /// Command buffer read operation as defined by NVX_device_generated_commands
    CommandBufferReadNVX,

    /// Read as an indirect buffer for drawing or dispatch
    IndirectBuffer,

    /// Read as an index buffer for drawing
    IndexBuffer,

    /// Read as a vertex buffer for drawing
    VertexBuffer,

    /// Read as a uniform buffer in a vertex shader
    VertexShaderReadUniformBuffer,

    /// Read as a sampled image/uniform texel buffer in a vertex shader
    VertexShaderReadSampledImageOrUniformTexelBuffer,

    /// Read as any other resource in a vertex shader
    VertexShaderReadOther,

    /// Read as a uniform buffer in a tessellation control shader
    TessellationControlShaderReadUniformBuffer,

    /// Read as a sampled image/uniform texel buffer  in a tessellation control shader
    TessellationControlShaderReadSampledImageOrUniformTexelBuffer,

    /// Read as any other resource in a tessellation control shader
    TessellationControlShaderReadOther,

    /// Read as a uniform buffer in a tessellation evaluation shader
    TessellationEvaluationShaderReadUniformBuffer,

    /// Read as a sampled image/uniform texel buffer in a tessellation evaluation shader
    TessellationEvaluationShaderReadSampledImageOrUniformTexelBuffer,

    /// Read as any other resource in a tessellation evaluation shader
    TessellationEvaluationShaderReadOther,

    /// Read as a uniform buffer in a geometry shader
    GeometryShaderReadUniformBuffer,

    /// Read as a sampled image/uniform texel buffer in a geometry shader
    GeometryShaderReadSampledImageOrUniformTexelBuffer,

    /// Read as any other resource in a geometry shader
    GeometryShaderReadOther,

    /// Read as a uniform buffer in a fragment shader
    FragmentShaderReadUniformBuffer,

    /// Read as a sampled image/uniform texel buffer in a fragment shader
    FragmentShaderReadSampledImageOrUniformTexelBuffer,

    /// Read as an input attachment with a color format in a fragment shader
    FragmentShaderReadColorInputAttachment,

    /// Read as an input attachment with a depth/stencil format in a fragment shader
    FragmentShaderReadDepthStencilInputAttachment,

    /// Read as any other resource in a fragment shader
    FragmentShaderReadOther,

    /// Read by blending/logic operations or subpass load operations
    ColorAttachmentRead,

    /// Read by depth/stencil tests or subpass load operations
    DepthStencilAttachmentRead,

    /// Read as a uniform buffer in a compute shader
    ComputeShaderReadUniformBuffer,

    /// Read as a sampled image/uniform texel buffer in a compute shader
    ComputeShaderReadSampledImageOrUniformTexelBuffer,

    /// Read as any other resource in a compute shader
    ComputeShaderReadOther,

    /// Read as a uniform buffer in any shader
    AnyShaderReadUniformBuffer,

    /// Read as a uniform buffer in any shader, or a vertex buffer
    AnyShaderReadUniformBufferOrVertexBuffer,

    /// Read as a sampled image in any shader
    AnyShaderReadSampledImageOrUniformTexelBuffer,

    /// Read as any other resource (excluding attachments) in any shader
    AnyShaderReadOther,

    /// Read as the source of a transfer operation
    TransferRead,

    /// Read on the host
    HostRead,

    /// Read by the presentation engine (i.e. vkQueuePresentKHR)
    Present,

    /// Command buffer write operation as defined by NVX_device_generated_commands
    CommandBufferWriteNVX,

    /// Written as any resource in a vertex shader
    VertexShaderWrite,

    /// Written as any resource in a tessellation control shader
    TessellationControlShaderWrite,

    /// Written as any resource in a tessellation evaluation shader
    TessellationEvaluationShaderWrite,

    /// Written as any resource in a geometry shader
    GeometryShaderWrite,

    /// Written as any resource in a fragment shader
    FragmentShaderWrite,

    /// Written as a color attachment during rendering, or via a subpass store op
    ColorAttachmentWrite,

    /// Written as a depth/stencil attachment during rendering, or via a subpass store op
    DepthStencilAttachmentWrite,

    /// Written as a depth aspect of a depth/stencil attachment during rendering, whilst the stencil aspect is read-only. Requires VK_KHR_maintenance2 to be enabled.
    DepthAttachmentWriteStencilReadOnly,

    /// Written as a stencil aspect of a depth/stencil attachment during rendering, whilst the depth aspect is read-only. Requires VK_KHR_maintenance2 to be enabled.
    StencilAttachmentWriteDepthReadOnly,

    /// Written as any resource in a compute shader
    ComputeShaderWrite,

    /// Written as any resource in any shader
    AnyShaderWrite,

    /// Written as the destination of a transfer operation
    TransferWrite,

    /// Written on the host
    HostWrite,

    /// Read or written as a color attachment during rendering
    ColorAttachmentReadWrite,

    /// Covers any access - useful for debug, generally avoid for performance reasons
    General,
}

/// Defines a handful of layout options for images.
/// Rather than a list of all possible image layouts, this reduced list is
/// correlated with the access types to map to the correct Vulkan layouts.
/// `Optimal` is usually preferred.
pub enum ImageLayout {
    // Choose the most optimal layout for each usage. Performs layout transitions as appropriate for the access.
    Optimal,

    // Layout accessible by all Vulkan access types on a device - no layout transitions except for presentation
    General,

    // As `General`, but also allows presentation engines to access it - no layout transitions. // Requires VK_KHR_shared_presentable_image to be enabled. Can only be used for shared presentable images (i.e. single-buffered swap chains).
    GeneralAndPresentation,
}                       

// Global barriers define a set of accesses on multiple resources at once.
// If a buffer or image doesn't require a queue ownership transfer, or an image
// doesn't require a layout transition (e.g. you're using one of the `General`
// layouts) then a global barrier should be preferred.
// Simply define the previous and next access types of resources affected.
pub struct GlobalBarrier {
    pub previous_accesses: Vec<AccessType>,
    pub next_accesses: Vec<AccessType>,
}

/*
Buffer barriers should only be used when a queue family ownership transfer
is required - prefer global barriers at all other times.
Access types are defined in the same way as for a global memory barrier, but
they only affect the buffer range identified by buffer, offset and size,
rather than all resources.
srcQueueFamilyIndex and dstQueueFamilyIndex will be passed unmodified into a
VkBufferMemoryBarrier.
A buffer barrier defining a queue ownership transfer needs to be executed
twice - once by a queue in the source queue family, and then once again by a
queue in the destination queue family, with a semaphore guaranteeing
execution order between them.
*/
pub struct BufferBarrier {
    pub previous_accesses: Vec<AccessType>,
    pub next_accesses: Vec<AccessType>,
    pub src_queue_family_index: u32,
    pub dst_queue_family_index: u32,
    pub buffer: BufferType,
    pub offset: usize,
    pub size: usize,
}

/*
Image barriers should only be used when a queue family ownership transfer
or an image layout transition is required - prefer global barriers at all
other times.
In general it is better to use image barriers with THSVS_IMAGE_LAYOUT_OPTIMAL
than it is to use global barriers with images using either of the
THSVS_IMAGE_LAYOUT_GENERAL* layouts.
Access types are defined in the same way as for a global memory barrier, but
they only affect the image subresource range identified by image and
subresourceRange, rather than all resources.
srcQueueFamilyIndex, dstQueueFamilyIndex, image, and subresourceRange will
be passed unmodified into a VkImageMemoryBarrier.
An image barrier defining a queue ownership transfer needs to be executed
twice - once by a queue in the source queue family, and then once again by a
queue in the destination queue family, with a semaphore guaranteeing
execution order between them.
If discardContents is set to true, the contents of the image become
undefined after the barrier is executed, which can result in a performance
boost over attempting to preserve the contents.
This is particularly useful for transient images where the contents are
going to be immediately overwritten. A good example of when to use this is
when an application re-uses a presented image after vkAcquireNextImageKHR.
*/
pub struct ImageBarrier {
    pub previous_accesses: Vec<AccessType>,
    pub next_accesses: Vec<AccessType>,
    pub previous_layout: ImageLayout,
    pub next_layout: ImageLayout,
    pub discard_contents: bool,
    pub src_queue_family_index: u32,
    pub dst_queue_family_index: u32,
    pub image: ImageType,
    pub range: ImageSubresourceRangeType,
}

// Mapping function that translates a global barrier into a set of source and
// destination pipeline stages, and a VkMemoryBarrier, that can be used with
// Vulkan's synchronization methods.
pub fn get_memory_barrier(
    barrier: &GlobalBarrier,
    src_stages: ash::vk::PipelineStageFlags,
    dst_stages: ash::vk::PipelineStageFlags) /*-> ash::vk::MemoryBarrier*/ {

}

// Mapping function that translates a buffer barrier into a set of source and
// destination pipeline stages, and a VkBufferMemoryBarrier, that can be used
// with Vulkan's synchronization methods.
pub fn get_buffer_memory_barrier(
    barrier: &BufferBarrier,
    src_stages: ash::vk::PipelineStageFlags,
    dst_stages: ash::vk::PipelineStageFlags) /*-> ash::vk::BufferMemoryBarrier*/ {

}

// Mapping function that translates an image barrier into a set of source and
// destination pipeline stages, and a VkBufferMemoryBarrier, that can be used
// with Vulkan's synchronization methods.
pub fn get_image_memory_barrier(
    barrier: &ImageBarrier,
    src_stages: ash::vk::PipelineStageFlags,
    dst_stages: ash::vk::PipelineStageFlags) /*-> ash::vk::ImageMemoryBarrier*/ {

}
