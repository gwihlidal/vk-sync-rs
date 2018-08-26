//! Tests are based on the common synchronization examples on the Vulkan-Docs wiki: https://github.com/KhronosGroup/Vulkan-Docs/wiki/Synchronization-Examples.

extern crate ash;
extern crate vk_sync;

#[test]
fn compute_write_storage_graphics_read_index_compute_read_uniform() {
    // Compute write to storage buffer, Graphics read as index buffer & Compute read as uniform buffer
    let global_barrier = vk_sync::GlobalBarrier {
        previous_accesses: vec![vk_sync::AccessType::ComputeShaderWrite],
        next_accesses: vec![
            vk_sync::AccessType::IndexBuffer,
            vk_sync::AccessType::ComputeShaderReadUniformBuffer,
        ],
    };

    let (src_mask, dst_mask, barrier) = vk_sync::get_memory_barrier(&global_barrier);

    assert_eq!(src_mask, ash::vk::PIPELINE_STAGE_COMPUTE_SHADER_BIT);
    assert_eq!(
        dst_mask,
        ash::vk::PIPELINE_STAGE_VERTEX_INPUT_BIT | ash::vk::PIPELINE_STAGE_COMPUTE_SHADER_BIT
    );
    assert_eq!(barrier.src_access_mask, ash::vk::ACCESS_SHADER_WRITE_BIT);
    assert_eq!(
        barrier.dst_access_mask,
        ash::vk::ACCESS_INDEX_READ_BIT | ash::vk::ACCESS_UNIFORM_READ_BIT
    );
}

#[test]
fn compute_write_texel_graphics_read_indirect_fragment_read_uniform() {
    // Compute write to storage texel buffer, Graphics read as indirect buffer & fragment read as uniform buffer
    let global_barrier = vk_sync::GlobalBarrier {
        previous_accesses: vec![vk_sync::AccessType::ComputeShaderWrite],
        next_accesses: vec![
            vk_sync::AccessType::IndirectBuffer,
            vk_sync::AccessType::FragmentShaderReadUniformBuffer,
        ],
    };

    let (src_mask, dst_mask, barrier) = vk_sync::get_memory_barrier(&global_barrier);

    assert_eq!(src_mask, ash::vk::PIPELINE_STAGE_COMPUTE_SHADER_BIT);
    assert_eq!(
        dst_mask,
        ash::vk::PIPELINE_STAGE_DRAW_INDIRECT_BIT | ash::vk::PIPELINE_STAGE_FRAGMENT_SHADER_BIT
    );
    assert_eq!(barrier.src_access_mask, ash::vk::ACCESS_SHADER_WRITE_BIT);
    assert_eq!(
        barrier.dst_access_mask,
        ash::vk::ACCESS_INDIRECT_COMMAND_READ_BIT | ash::vk::ACCESS_UNIFORM_READ_BIT
    );
}
