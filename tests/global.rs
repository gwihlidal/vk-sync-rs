//! Tests are based on the common synchronization examples on the Vulkan-Docs wiki: https://github.com/KhronosGroup/Vulkan-Docs/wiki/Synchronization-Examples.

extern crate ash;
extern crate vk_sync;

#[test]
fn compute_write_storage_compute_read_storage() {
	// Compute write to storage buffer/image, Compute read from storage buffer/image
	let global_barrier = vk_sync::GlobalBarrier {
		previous_accesses: vec![vk_sync::AccessType::ComputeShaderWrite],
		next_accesses: vec![vk_sync::AccessType::ComputeShaderReadOther],
	};

	let (src_mask, dst_mask, barrier) = vk_sync::get_memory_barrier(&global_barrier);

	assert_eq!(src_mask, ash::vk::PipelineStageFlags::COMPUTE_SHADER);
	assert_eq!(dst_mask, ash::vk::PipelineStageFlags::COMPUTE_SHADER);
	assert_eq!(barrier.src_access_mask, ash::vk::AccessFlags::SHADER_WRITE);
	assert_eq!(barrier.dst_access_mask, ash::vk::AccessFlags::SHADER_READ);
}

#[test]
fn compute_read_storage_compute_write_storage() {
	// Compute read from storage buffer, Compute write from storage buffer
	let global_barrier = vk_sync::GlobalBarrier {
		previous_accesses: vec![vk_sync::AccessType::ComputeShaderWrite],
		next_accesses: vec![vk_sync::AccessType::ComputeShaderReadOther],
	};

	let (src_mask, dst_mask, barrier) = vk_sync::get_memory_barrier(&global_barrier);

	assert_eq!(src_mask, ash::vk::PipelineStageFlags::COMPUTE_SHADER);
	assert_eq!(dst_mask, ash::vk::PipelineStageFlags::COMPUTE_SHADER);
	assert_eq!(barrier.src_access_mask, ash::vk::AccessFlags::SHADER_WRITE);
	assert_eq!(barrier.dst_access_mask, ash::vk::AccessFlags::SHADER_READ);
}

#[test]
fn compute_write_storage_graphics_read_index() {
	// Compute write to storage buffer, Graphics read as index buffer
	let global_barrier = vk_sync::GlobalBarrier {
		previous_accesses: vec![vk_sync::AccessType::ComputeShaderWrite],
		next_accesses: vec![vk_sync::AccessType::IndexBuffer],
	};

	let (src_mask, dst_mask, barrier) = vk_sync::get_memory_barrier(&global_barrier);

	assert_eq!(src_mask, ash::vk::PipelineStageFlags::COMPUTE_SHADER);
	assert_eq!(dst_mask, ash::vk::PipelineStageFlags::VERTEX_INPUT);
	assert_eq!(barrier.src_access_mask, ash::vk::AccessFlags::SHADER_WRITE);
	assert_eq!(barrier.dst_access_mask, ash::vk::AccessFlags::INDEX_READ);
}

#[test]
fn compute_write_storage_graphics_read_indirect() {
	// Compute write to storage buffer, Graphics read as indirect buffer
	let global_barrier = vk_sync::GlobalBarrier {
		previous_accesses: vec![vk_sync::AccessType::ComputeShaderWrite],
		next_accesses: vec![vk_sync::AccessType::IndirectBuffer],
	};

	let (src_mask, dst_mask, barrier) = vk_sync::get_memory_barrier(&global_barrier);

	assert_eq!(src_mask, ash::vk::PipelineStageFlags::COMPUTE_SHADER);
	assert_eq!(dst_mask, ash::vk::PipelineStageFlags::DRAW_INDIRECT);
	assert_eq!(barrier.src_access_mask, ash::vk::AccessFlags::SHADER_WRITE);
	assert_eq!(
		barrier.dst_access_mask,
		ash::vk::AccessFlags::INDIRECT_COMMAND_READ
	);
}

#[test]
fn nothing_transfer_read() {
	// None, Transfer read from buffer
	let global_barrier = vk_sync::GlobalBarrier {
		previous_accesses: vec![vk_sync::AccessType::Nothing],
		next_accesses: vec![vk_sync::AccessType::TransferRead],
	};

	let (src_mask, dst_mask, barrier) = vk_sync::get_memory_barrier(&global_barrier);

	assert_eq!(src_mask, ash::vk::PipelineStageFlags::TOP_OF_PIPE);
	assert_eq!(dst_mask, ash::vk::PipelineStageFlags::TRANSFER);
	assert_eq!(barrier.src_access_mask, ash::vk::AccessFlags::empty());
	assert_eq!(barrier.dst_access_mask, ash::vk::AccessFlags::empty());
}

#[test]
fn transfer_write_graphics_read_vertex() {
	// Transfer write to buffer, Graphics read from vertex buffer
	let global_barrier = vk_sync::GlobalBarrier {
		previous_accesses: vec![vk_sync::AccessType::TransferWrite],
		next_accesses: vec![vk_sync::AccessType::VertexBuffer],
	};

	let (src_mask, dst_mask, barrier) = vk_sync::get_memory_barrier(&global_barrier);

	assert_eq!(src_mask, ash::vk::PipelineStageFlags::TRANSFER);
	assert_eq!(dst_mask, ash::vk::PipelineStageFlags::VERTEX_INPUT);
	assert_eq!(
		barrier.src_access_mask,
		ash::vk::AccessFlags::TRANSFER_WRITE
	);
	assert_eq!(
		barrier.dst_access_mask,
		ash::vk::AccessFlags::VERTEX_ATTRIBUTE_READ
	);
}

#[test]
fn full_pipeline_barrier() {
	// Full pipeline barrier
	let global_barrier = vk_sync::GlobalBarrier {
		previous_accesses: vec![vk_sync::AccessType::General],
		next_accesses: vec![vk_sync::AccessType::General],
	};

	let (src_mask, dst_mask, barrier) = vk_sync::get_memory_barrier(&global_barrier);

	assert_eq!(src_mask, ash::vk::PipelineStageFlags::ALL_COMMANDS);
	assert_eq!(dst_mask, ash::vk::PipelineStageFlags::ALL_COMMANDS);
	assert_eq!(
		barrier.src_access_mask,
		ash::vk::AccessFlags::MEMORY_READ | ash::vk::AccessFlags::MEMORY_WRITE
	);
	assert_eq!(
		barrier.dst_access_mask,
		ash::vk::AccessFlags::MEMORY_READ | ash::vk::AccessFlags::MEMORY_WRITE
	);
}

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

	assert_eq!(src_mask, ash::vk::PipelineStageFlags::COMPUTE_SHADER);
	assert_eq!(
		dst_mask,
		ash::vk::PipelineStageFlags::VERTEX_INPUT | ash::vk::PipelineStageFlags::COMPUTE_SHADER
	);
	assert_eq!(barrier.src_access_mask, ash::vk::AccessFlags::SHADER_WRITE);
	assert_eq!(
		barrier.dst_access_mask,
		ash::vk::AccessFlags::INDEX_READ | ash::vk::AccessFlags::UNIFORM_READ
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

	assert_eq!(src_mask, ash::vk::PipelineStageFlags::COMPUTE_SHADER);
	assert_eq!(
		dst_mask,
		ash::vk::PipelineStageFlags::DRAW_INDIRECT | ash::vk::PipelineStageFlags::FRAGMENT_SHADER
	);
	assert_eq!(barrier.src_access_mask, ash::vk::AccessFlags::SHADER_WRITE);
	assert_eq!(
		barrier.dst_access_mask,
		ash::vk::AccessFlags::INDIRECT_COMMAND_READ | ash::vk::AccessFlags::UNIFORM_READ
	);
}
