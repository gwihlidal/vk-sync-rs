extern crate ash;

pub type BufferType = ash::vk::Buffer;
pub type ImageType = ash::vk::Image;
pub type ImageSubresourceRangeType = ash::vk::ImageSubresourceRange;

pub mod cmd;

/// Defines all potential resource usages
#[derive(Debug, Clone, PartialEq)]
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
#[derive(Debug, Clone, PartialEq)]
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
) -> (
    ash::vk::PipelineStageFlags,
    ash::vk::PipelineStageFlags,
    ash::vk::MemoryBarrier,
) {
    let mut src_stages = ash::vk::PipelineStageFlags::empty();
    let mut dst_stages = ash::vk::PipelineStageFlags::empty();

    let mut memory_barrier = ash::vk::MemoryBarrier {
        s_type: ash::vk::StructureType::MemoryBarrier,
        p_next: ::std::ptr::null(),
        src_access_mask: Default::default(),
        dst_access_mask: Default::default(),
    };

    for previous_access in &barrier.previous_accesses {
        let previous_info = get_access_info(previous_access);

        src_stages |= previous_info.stage_mask;

        // Add appropriate availability operations - for writes only.
        if is_write_access(previous_access) {
            memory_barrier.src_access_mask |= previous_info.access_mask;
        }
    }

    for next_access in &barrier.next_accesses {
        let next_info = get_access_info(next_access);

        dst_stages |= next_info.stage_mask;

        // Add visibility operations as necessary.
        // If the src access mask, this is a WAR hazard (or for some reason a "RAR"),
        // so the dst access mask can be safely zeroed as these don't need visibility.
        if memory_barrier.src_access_mask != ash::vk::AccessFlags::empty() {
            memory_barrier.dst_access_mask |= next_info.access_mask;
        }
    }

    // Ensure that the stage masks are valid if no stages were determined
    if src_stages == ash::vk::PipelineStageFlags::empty() {
        src_stages = ash::vk::PIPELINE_STAGE_TOP_OF_PIPE_BIT;
    }

    if dst_stages == ash::vk::PipelineStageFlags::empty() {
        dst_stages = ash::vk::PIPELINE_STAGE_BOTTOM_OF_PIPE_BIT;
    }

    (src_stages, dst_stages, memory_barrier)
}

// Mapping function that translates a buffer barrier into a set of source and
// destination pipeline stages, and a VkBufferMemoryBarrier, that can be used
// with Vulkan's synchronization methods.
pub fn get_buffer_memory_barrier(
    barrier: &BufferBarrier,
) -> (
    ash::vk::PipelineStageFlags,
    ash::vk::PipelineStageFlags,
    ash::vk::BufferMemoryBarrier,
) {
    let mut src_stages = ash::vk::PipelineStageFlags::empty();
    let mut dst_stages = ash::vk::PipelineStageFlags::empty();

    let mut buffer_barrier = ash::vk::BufferMemoryBarrier {
        s_type: ash::vk::StructureType::BufferMemoryBarrier,
        p_next: ::std::ptr::null(),
        src_access_mask: Default::default(),
        dst_access_mask: Default::default(),
        src_queue_family_index: barrier.src_queue_family_index,
        dst_queue_family_index: barrier.dst_queue_family_index,
        buffer: barrier.buffer,
        offset: barrier.offset as u64,
        size: barrier.size as u64,
    };

    for previous_access in &barrier.previous_accesses {
        let previous_info = get_access_info(previous_access);

        src_stages |= previous_info.stage_mask;

        // Add appropriate availability operations - for writes only.
        if is_write_access(previous_access) {
            buffer_barrier.src_access_mask |= previous_info.access_mask;
        }
    }

    for next_access in &barrier.next_accesses {
        let next_info = get_access_info(next_access);

        dst_stages |= next_info.stage_mask;

        // Add visibility operations as necessary.
        // If the src access mask, this is a WAR hazard (or for some reason a "RAR"),
        // so the dst access mask can be safely zeroed as these don't need visibility.
        if buffer_barrier.src_access_mask != ash::vk::AccessFlags::empty() {
            buffer_barrier.dst_access_mask |= next_info.access_mask;
        }
    }

    // Ensure that the stage masks are valid if no stages were determined
    if src_stages == ash::vk::PipelineStageFlags::empty() {
        src_stages = ash::vk::PIPELINE_STAGE_TOP_OF_PIPE_BIT;
    }

    if dst_stages == ash::vk::PipelineStageFlags::empty() {
        dst_stages = ash::vk::PIPELINE_STAGE_BOTTOM_OF_PIPE_BIT;
    }

    (src_stages, dst_stages, buffer_barrier)
}

// Mapping function that translates an image barrier into a set of source and
// destination pipeline stages, and a VkBufferMemoryBarrier, that can be used
// with Vulkan's synchronization methods.
pub fn get_image_memory_barrier(
    barrier: &ImageBarrier,
) -> (
    ash::vk::PipelineStageFlags,
    ash::vk::PipelineStageFlags,
    ash::vk::ImageMemoryBarrier,
) {
    let mut src_stages = ash::vk::PipelineStageFlags::empty();
    let mut dst_stages = ash::vk::PipelineStageFlags::empty();

    let mut image_barrier = ash::vk::ImageMemoryBarrier {
        s_type: ash::vk::StructureType::ImageMemoryBarrier,
        p_next: ::std::ptr::null(),
        src_access_mask: Default::default(),
        dst_access_mask: Default::default(),
        old_layout: ash::vk::ImageLayout::Undefined,
        new_layout: ash::vk::ImageLayout::Undefined,
        src_queue_family_index: barrier.src_queue_family_index,
        dst_queue_family_index: barrier.dst_queue_family_index,
        image: barrier.image,
        subresource_range: barrier.range.clone(),
    };

    for previous_access in &barrier.previous_accesses {
        let previous_info = get_access_info(previous_access);

        src_stages |= previous_info.stage_mask;

        // Add appropriate availability operations - for writes only.
        if is_write_access(previous_access) {
            image_barrier.src_access_mask |= previous_info.access_mask;
        }

        if barrier.discard_contents {
            image_barrier.old_layout = ash::vk::ImageLayout::Undefined;
        } else {
            let layout = match barrier.previous_layout {
                ImageLayout::General => {
                    if *previous_access == AccessType::Present {
                        ash::vk::ImageLayout::PresentSrcKhr
                    } else {
                        ash::vk::ImageLayout::General
                    }
                }
                ImageLayout::Optimal => previous_info.image_layout,
                ImageLayout::GeneralAndPresentation => {
                    unimplemented!()
                    //layout = ash::vk::ImageLayout::VK_IMAGE_LAYOUT_SHARED_PRESENT_KHR
                }
            };

            image_barrier.old_layout = layout;
        }
    }

    for next_access in &barrier.next_accesses {
        let next_info = get_access_info(next_access);

        dst_stages |= next_info.stage_mask;

        // Add visibility operations as necessary.
        // If the src access mask, this is a WAR hazard (or for some reason a "RAR"),
        // so the dst access mask can be safely zeroed as these don't need visibility.
        if image_barrier.src_access_mask != ash::vk::AccessFlags::empty() {
            image_barrier.dst_access_mask |= next_info.access_mask;
        }

        let layout = match barrier.next_layout {
            ImageLayout::General => {
                if *next_access == AccessType::Present {
                    ash::vk::ImageLayout::PresentSrcKhr
                } else {
                    ash::vk::ImageLayout::General
                }
            }
            ImageLayout::Optimal => next_info.image_layout,
            ImageLayout::GeneralAndPresentation => {
                unimplemented!()
                //layout = ash::vk::ImageLayout::VK_IMAGE_LAYOUT_SHARED_PRESENT_KHR
            }
        };

        image_barrier.old_layout = layout;
    }

    // Ensure that the stage masks are valid if no stages were determined
    if src_stages == ash::vk::PipelineStageFlags::empty() {
        src_stages = ash::vk::PIPELINE_STAGE_TOP_OF_PIPE_BIT;
    }

    if dst_stages == ash::vk::PipelineStageFlags::empty() {
        dst_stages = ash::vk::PIPELINE_STAGE_BOTTOM_OF_PIPE_BIT;
    }

    (src_stages, dst_stages, image_barrier)
}

pub(crate) struct AccessInfo {
    pub(crate) stage_mask: ash::vk::PipelineStageFlags,
    pub(crate) access_mask: ash::vk::AccessFlags,
    pub(crate) image_layout: ash::vk::ImageLayout,
}

pub(crate) fn get_access_info(access_type: &AccessType) -> AccessInfo {
    match access_type {
        AccessType::Nothing => AccessInfo {
            stage_mask: ash::vk::PipelineStageFlags::empty(),
            access_mask: ash::vk::AccessFlags::empty(),
            image_layout: ash::vk::ImageLayout::Undefined,
        },
        /*CommandBufferReadNVX => {
            AccessInfo {
                stage_mask: ash::vk::PIPELINE_STAGE_COMMAND_PROCESS_BIT_NVX,
                access_mask: ash::vk::ACCESS_COMMAND_PROCESS_READ_BIT_NVX,
                image_layout: ash::vk::ImageLayout::Undefined,
            }
        }*/
        AccessType::IndirectBuffer => AccessInfo {
            stage_mask: ash::vk::PIPELINE_STAGE_DRAW_INDIRECT_BIT,
            access_mask: ash::vk::ACCESS_INDIRECT_COMMAND_READ_BIT,
            image_layout: ash::vk::ImageLayout::Undefined,
        },
        AccessType::IndexBuffer => AccessInfo {
            stage_mask: ash::vk::PIPELINE_STAGE_VERTEX_INPUT_BIT,
            access_mask: ash::vk::ACCESS_INDEX_READ_BIT,
            image_layout: ash::vk::ImageLayout::Undefined,
        },
        AccessType::VertexBuffer => AccessInfo {
            stage_mask: ash::vk::PIPELINE_STAGE_VERTEX_INPUT_BIT,
            access_mask: ash::vk::ACCESS_VERTEX_ATTRIBUTE_READ_BIT,
            image_layout: ash::vk::ImageLayout::Undefined,
        },
        AccessType::VertexShaderReadUniformBuffer => AccessInfo {
            stage_mask: ash::vk::PIPELINE_STAGE_VERTEX_SHADER_BIT,
            access_mask: ash::vk::ACCESS_SHADER_READ_BIT,
            image_layout: ash::vk::ImageLayout::Undefined,
        },
        AccessType::VertexShaderReadSampledImageOrUniformTexelBuffer => AccessInfo {
            stage_mask: ash::vk::PIPELINE_STAGE_VERTEX_SHADER_BIT,
            access_mask: ash::vk::ACCESS_MEMORY_READ_BIT,
            image_layout: ash::vk::ImageLayout::Undefined,
        },
        AccessType::VertexShaderReadOther => AccessInfo {
            stage_mask: ash::vk::PIPELINE_STAGE_VERTEX_SHADER_BIT,
            access_mask: ash::vk::ACCESS_MEMORY_READ_BIT,
            image_layout: ash::vk::ImageLayout::General,
        },
        AccessType::TessellationControlShaderReadUniformBuffer => AccessInfo {
            stage_mask: ash::vk::PIPELINE_STAGE_TESSELLATION_CONTROL_SHADER_BIT,
            access_mask: ash::vk::ACCESS_UNIFORM_READ_BIT,
            image_layout: ash::vk::ImageLayout::Undefined,
        },
        AccessType::TessellationControlShaderReadSampledImageOrUniformTexelBuffer => AccessInfo {
            stage_mask: ash::vk::PIPELINE_STAGE_TESSELLATION_CONTROL_SHADER_BIT,
            access_mask: ash::vk::ACCESS_SHADER_READ_BIT,
            image_layout: ash::vk::ImageLayout::ShaderReadOnlyOptimal,
        },
        AccessType::TessellationControlShaderReadOther => AccessInfo {
            stage_mask: ash::vk::PIPELINE_STAGE_TESSELLATION_CONTROL_SHADER_BIT,
            access_mask: ash::vk::ACCESS_SHADER_READ_BIT,
            image_layout: ash::vk::ImageLayout::General,
        },
        AccessType::TessellationEvaluationShaderReadUniformBuffer => AccessInfo {
            stage_mask: ash::vk::PIPELINE_STAGE_TESSELLATION_EVALUATION_SHADER_BIT,
            access_mask: ash::vk::ACCESS_UNIFORM_READ_BIT,
            image_layout: ash::vk::ImageLayout::Undefined,
        },
        AccessType::TessellationEvaluationShaderReadSampledImageOrUniformTexelBuffer => {
            AccessInfo {
                stage_mask: ash::vk::PIPELINE_STAGE_TESSELLATION_EVALUATION_SHADER_BIT,
                access_mask: ash::vk::ACCESS_SHADER_READ_BIT,
                image_layout: ash::vk::ImageLayout::ShaderReadOnlyOptimal,
            }
        }
        AccessType::TessellationEvaluationShaderReadOther => AccessInfo {
            stage_mask: ash::vk::PIPELINE_STAGE_TESSELLATION_EVALUATION_SHADER_BIT,
            access_mask: ash::vk::ACCESS_SHADER_READ_BIT,
            image_layout: ash::vk::ImageLayout::General,
        },
        AccessType::GeometryShaderReadUniformBuffer => AccessInfo {
            stage_mask: ash::vk::PIPELINE_STAGE_GEOMETRY_SHADER_BIT,
            access_mask: ash::vk::ACCESS_UNIFORM_READ_BIT,
            image_layout: ash::vk::ImageLayout::Undefined,
        },
        AccessType::GeometryShaderReadSampledImageOrUniformTexelBuffer => AccessInfo {
            stage_mask: ash::vk::PIPELINE_STAGE_GEOMETRY_SHADER_BIT,
            access_mask: ash::vk::ACCESS_SHADER_READ_BIT,
            image_layout: ash::vk::ImageLayout::ShaderReadOnlyOptimal,
        },
        AccessType::GeometryShaderReadOther => AccessInfo {
            stage_mask: ash::vk::PIPELINE_STAGE_GEOMETRY_SHADER_BIT,
            access_mask: ash::vk::ACCESS_SHADER_READ_BIT,
            image_layout: ash::vk::ImageLayout::General,
        },
        AccessType::FragmentShaderReadUniformBuffer => AccessInfo {
            stage_mask: ash::vk::PIPELINE_STAGE_FRAGMENT_SHADER_BIT,
            access_mask: ash::vk::ACCESS_UNIFORM_READ_BIT,
            image_layout: ash::vk::ImageLayout::Undefined,
        },
        AccessType::FragmentShaderReadSampledImageOrUniformTexelBuffer => AccessInfo {
            stage_mask: ash::vk::PIPELINE_STAGE_FRAGMENT_SHADER_BIT,
            access_mask: ash::vk::ACCESS_SHADER_READ_BIT,
            image_layout: ash::vk::ImageLayout::ShaderReadOnlyOptimal,
        },
        AccessType::FragmentShaderReadColorInputAttachment => AccessInfo {
            stage_mask: ash::vk::PIPELINE_STAGE_FRAGMENT_SHADER_BIT,
            access_mask: ash::vk::ACCESS_INPUT_ATTACHMENT_READ_BIT,
            image_layout: ash::vk::ImageLayout::ShaderReadOnlyOptimal,
        },
        AccessType::FragmentShaderReadDepthStencilInputAttachment => AccessInfo {
            stage_mask: ash::vk::PIPELINE_STAGE_FRAGMENT_SHADER_BIT,
            access_mask: ash::vk::ACCESS_INPUT_ATTACHMENT_READ_BIT,
            image_layout: ash::vk::ImageLayout::DepthStencilReadOnlyOptimal,
        },
        AccessType::FragmentShaderReadOther => AccessInfo {
            stage_mask: ash::vk::PIPELINE_STAGE_FRAGMENT_SHADER_BIT,
            access_mask: ash::vk::ACCESS_SHADER_READ_BIT,
            image_layout: ash::vk::ImageLayout::General,
        },
        AccessType::ColorAttachmentRead => AccessInfo {
            stage_mask: ash::vk::PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT,
            access_mask: ash::vk::ACCESS_COLOR_ATTACHMENT_READ_BIT,
            image_layout: ash::vk::ImageLayout::ColorAttachmentOptimal,
        },
        AccessType::DepthStencilAttachmentRead => AccessInfo {
            stage_mask: ash::vk::PIPELINE_STAGE_EARLY_FRAGMENT_TESTS_BIT
                | ash::vk::PIPELINE_STAGE_LATE_FRAGMENT_TESTS_BIT,
            access_mask: ash::vk::ACCESS_DEPTH_STENCIL_ATTACHMENT_READ_BIT,
            image_layout: ash::vk::ImageLayout::DepthStencilReadOnlyOptimal,
        },
        AccessType::ComputeShaderReadUniformBuffer => AccessInfo {
            stage_mask: ash::vk::PIPELINE_STAGE_COMPUTE_SHADER_BIT,
            access_mask: ash::vk::ACCESS_UNIFORM_READ_BIT,
            image_layout: ash::vk::ImageLayout::Undefined,
        },
        AccessType::ComputeShaderReadSampledImageOrUniformTexelBuffer => AccessInfo {
            stage_mask: ash::vk::PIPELINE_STAGE_COMPUTE_SHADER_BIT,
            access_mask: ash::vk::ACCESS_SHADER_READ_BIT,
            image_layout: ash::vk::ImageLayout::ShaderReadOnlyOptimal,
        },
        AccessType::ComputeShaderReadOther => AccessInfo {
            stage_mask: ash::vk::PIPELINE_STAGE_COMPUTE_SHADER_BIT,
            access_mask: ash::vk::ACCESS_SHADER_READ_BIT,
            image_layout: ash::vk::ImageLayout::General,
        },
        AccessType::AnyShaderReadUniformBuffer => AccessInfo {
            stage_mask: ash::vk::PIPELINE_STAGE_ALL_COMMANDS_BIT,
            access_mask: ash::vk::ACCESS_UNIFORM_READ_BIT,
            image_layout: ash::vk::ImageLayout::Undefined,
        },
        AccessType::AnyShaderReadUniformBufferOrVertexBuffer => AccessInfo {
            stage_mask: ash::vk::PIPELINE_STAGE_ALL_COMMANDS_BIT,
            access_mask: ash::vk::ACCESS_UNIFORM_READ_BIT
                | ash::vk::ACCESS_VERTEX_ATTRIBUTE_READ_BIT,
            image_layout: ash::vk::ImageLayout::Undefined,
        },
        AccessType::AnyShaderReadSampledImageOrUniformTexelBuffer => AccessInfo {
            stage_mask: ash::vk::PIPELINE_STAGE_ALL_COMMANDS_BIT,
            access_mask: ash::vk::ACCESS_SHADER_READ_BIT,
            image_layout: ash::vk::ImageLayout::ShaderReadOnlyOptimal,
        },
        AccessType::AnyShaderReadOther => AccessInfo {
            stage_mask: ash::vk::PIPELINE_STAGE_ALL_COMMANDS_BIT,
            access_mask: ash::vk::ACCESS_SHADER_READ_BIT,
            image_layout: ash::vk::ImageLayout::General,
        },
        AccessType::TransferRead => AccessInfo {
            stage_mask: ash::vk::PIPELINE_STAGE_TRANSFER_BIT,
            access_mask: ash::vk::ACCESS_TRANSFER_READ_BIT,
            image_layout: ash::vk::ImageLayout::TransferSrcOptimal,
        },
        AccessType::HostRead => AccessInfo {
            stage_mask: ash::vk::PIPELINE_STAGE_HOST_BIT,
            access_mask: ash::vk::ACCESS_HOST_READ_BIT,
            image_layout: ash::vk::ImageLayout::General,
        },
        AccessType::Present => AccessInfo {
            stage_mask: ash::vk::PipelineStageFlags::empty(),
            access_mask: ash::vk::AccessFlags::empty(),
            image_layout: ash::vk::ImageLayout::PresentSrcKhr,
        },
        /*AccessType::CommandBufferWriteNVX => {
            AccessInfo {
                stage_mask: ash::vk::PIPELINE_STAGE_COMMAND_PROCESS_BIT_NVX,
                access_mask: ash::vk::ACCESS_COMMAND_PROCESS_WRITE_BIT_NVX,
                image_layout: ash::vk::ImageLayout::Undefined,
            }
        }*/
        AccessType::VertexShaderWrite => AccessInfo {
            stage_mask: ash::vk::PIPELINE_STAGE_VERTEX_SHADER_BIT,
            access_mask: ash::vk::ACCESS_SHADER_WRITE_BIT,
            image_layout: ash::vk::ImageLayout::General,
        },
        AccessType::TessellationControlShaderWrite => AccessInfo {
            stage_mask: ash::vk::PIPELINE_STAGE_TESSELLATION_CONTROL_SHADER_BIT,
            access_mask: ash::vk::ACCESS_SHADER_WRITE_BIT,
            image_layout: ash::vk::ImageLayout::General,
        },
        AccessType::TessellationEvaluationShaderWrite => AccessInfo {
            stage_mask: ash::vk::PIPELINE_STAGE_TESSELLATION_EVALUATION_SHADER_BIT,
            access_mask: ash::vk::ACCESS_SHADER_WRITE_BIT,
            image_layout: ash::vk::ImageLayout::General,
        },
        AccessType::GeometryShaderWrite => AccessInfo {
            stage_mask: ash::vk::PIPELINE_STAGE_GEOMETRY_SHADER_BIT,
            access_mask: ash::vk::ACCESS_SHADER_WRITE_BIT,
            image_layout: ash::vk::ImageLayout::General,
        },
        AccessType::FragmentShaderWrite => AccessInfo {
            stage_mask: ash::vk::PIPELINE_STAGE_FRAGMENT_SHADER_BIT,
            access_mask: ash::vk::ACCESS_SHADER_WRITE_BIT,
            image_layout: ash::vk::ImageLayout::General,
        },
        AccessType::ColorAttachmentWrite => AccessInfo {
            stage_mask: ash::vk::PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT,
            access_mask: ash::vk::ACCESS_COLOR_ATTACHMENT_WRITE_BIT,
            image_layout: ash::vk::ImageLayout::ColorAttachmentOptimal,
        },
        AccessType::DepthStencilAttachmentWrite => AccessInfo {
            stage_mask: ash::vk::PIPELINE_STAGE_EARLY_FRAGMENT_TESTS_BIT
                | ash::vk::PIPELINE_STAGE_LATE_FRAGMENT_TESTS_BIT,
            access_mask: ash::vk::ACCESS_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT,
            image_layout: ash::vk::ImageLayout::DepthStencilAttachmentOptimal,
        },
        /*AccessType::DepthAttachmentWriteStencilReadOnly => {
            AccessInfo {
                stage_mask: ash::vk::PIPELINE_STAGE_EARLY_FRAGMENT_TESTS_BIT | ash::vk::PIPELINE_STAGE_LATE_FRAGMENT_TESTS_BIT,
                access_mask: ash::vk::ACCESS_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT | ash::vk::ACCESS_DEPTH_STENCIL_ATTACHMENT_READ_BIT,
                image_layout: ash::vk::ImageLayout::IMAGE_LAYOUT_DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL_KHR,
            }
        }*/
        /*AccessType::StencilAttachmentWriteDepthReadOnly => {
            AccessInfo {
                stage_mask: ash::vk::PIPELINE_STAGE_EARLY_FRAGMENT_TESTS_BIT | ash::vk::PIPELINE_STAGE_LATE_FRAGMENT_TESTS_BIT,
                access_mask: ash::vk::ACCESS_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT | ash::vk::ACCESS_DEPTH_STENCIL_ATTACHMENT_READ_BIT,
                image_layout: ash::vk::ImageLayout::IMAGE_LAYOUT_DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL_KHR,
            }
        }*/
        AccessType::ComputeShaderWrite => AccessInfo {
            stage_mask: ash::vk::PIPELINE_STAGE_COMPUTE_SHADER_BIT,
            access_mask: ash::vk::ACCESS_SHADER_WRITE_BIT,
            image_layout: ash::vk::ImageLayout::General,
        },
        AccessType::AnyShaderWrite => AccessInfo {
            stage_mask: ash::vk::PIPELINE_STAGE_ALL_COMMANDS_BIT,
            access_mask: ash::vk::ACCESS_SHADER_WRITE_BIT,
            image_layout: ash::vk::ImageLayout::General,
        },
        AccessType::TransferWrite => AccessInfo {
            stage_mask: ash::vk::PIPELINE_STAGE_TRANSFER_BIT,
            access_mask: ash::vk::ACCESS_TRANSFER_WRITE_BIT,
            image_layout: ash::vk::ImageLayout::TransferDstOptimal,
        },
        AccessType::HostWrite => AccessInfo {
            stage_mask: ash::vk::PIPELINE_STAGE_HOST_BIT,
            access_mask: ash::vk::ACCESS_HOST_WRITE_BIT,
            image_layout: ash::vk::ImageLayout::General,
        },
        AccessType::ColorAttachmentReadWrite => AccessInfo {
            stage_mask: ash::vk::PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT,
            access_mask: ash::vk::ACCESS_COLOR_ATTACHMENT_READ_BIT
                | ash::vk::ACCESS_COLOR_ATTACHMENT_WRITE_BIT,
            image_layout: ash::vk::ImageLayout::ColorAttachmentOptimal,
        },
        AccessType::General => AccessInfo {
            stage_mask: ash::vk::PIPELINE_STAGE_ALL_COMMANDS_BIT,
            access_mask: ash::vk::ACCESS_MEMORY_READ_BIT | ash::vk::ACCESS_MEMORY_WRITE_BIT,
            image_layout: ash::vk::ImageLayout::General,
        },
        _ => unimplemented!(),
    }
}

pub fn is_write_access(access_type: &AccessType) -> bool {
    unimplemented!()
}

pub fn is_read_access(access_type: &AccessType) -> bool {
    unimplemented!()
}
