use super::*;
use ash;

/// Simplified wrapper around `vkCmdPipelineBarrier`.
/// The mapping functions defined above are used to translate the passed in
/// barrier definitions into a set of pipeline stages and native Vulkan memory
/// barriers to be passed to `vkCmdPipelineBarrier`.
/// `command_buffer` is passed unmodified to `vkCmdPipelineBarrier`.
pub fn pipeline_barrier(
	device: &ash::vk::DeviceFnV1_0,
	command_buffer: ash::vk::CommandBuffer,
	global_barrier: Option<GlobalBarrier>,
	buffer_barriers: &[BufferBarrier],
	image_barriers: &[ImageBarrier],
) {
	let mut src_stage_mask = ash::vk::PipelineStageFlags::TOP_OF_PIPE;
	let mut dst_stage_mask = ash::vk::PipelineStageFlags::BOTTOM_OF_PIPE;

	let mut vk_memory_barriers: Vec<ash::vk::MemoryBarrier> = Vec::with_capacity(1);
	let mut vk_buffer_barriers: Vec<ash::vk::BufferMemoryBarrier> =
		Vec::with_capacity(buffer_barriers.len());
	let mut vk_image_barriers: Vec<ash::vk::ImageMemoryBarrier> =
		Vec::with_capacity(image_barriers.len());

	// Global memory barrier
	if let Some(ref barrier) = global_barrier {
		let (src_mask, dst_mask, barrier) = get_memory_barrier(barrier);
		src_stage_mask |= src_mask;
		dst_stage_mask |= dst_mask;
		vk_memory_barriers.push(barrier);
	}

	// Buffer memory barriers
	for buffer_barrier in buffer_barriers {
		let (src_mask, dst_mask, barrier) = get_buffer_memory_barrier(buffer_barrier);
		src_stage_mask |= src_mask;
		dst_stage_mask |= dst_mask;
		vk_buffer_barriers.push(barrier);
	}

	// Image memory barriers
	for image_barrier in image_barriers {
		let (src_mask, dst_mask, barrier) = get_image_memory_barrier(image_barrier);
		src_stage_mask |= src_mask;
		dst_stage_mask |= dst_mask;
		vk_image_barriers.push(barrier);
	}

	unsafe {
		device.cmd_pipeline_barrier(
			command_buffer,
			src_stage_mask,
			dst_stage_mask,
			ash::vk::DependencyFlags::empty(),
			vk_memory_barriers.len() as u32,
			vk_memory_barriers.as_ptr(),
			vk_buffer_barriers.len() as u32,
			vk_buffer_barriers.as_ptr(),
			vk_image_barriers.len() as u32,
			vk_image_barriers.as_ptr(),
		);
	}
}

/// Wrapper around `vkCmdSetEvent`.
/// Sets an event when the accesses defined by `previous_accesses` are completed.
/// `command_buffer` and `event` are passed unmodified to `vkCmdSetEvent`.
pub fn set_event(
	device: &ash::vk::DeviceFnV1_0,
	command_buffer: ash::vk::CommandBuffer,
	event: ash::vk::Event,
	previous_accesses: &[AccessType],
) {
	let mut stage_mask = ash::vk::PipelineStageFlags::TOP_OF_PIPE;
	for previous_access in previous_accesses {
		let previous_info = get_access_info(*previous_access);
		stage_mask |= previous_info.stage_mask;
	}

	unsafe {
		device.cmd_set_event(command_buffer, event, stage_mask);
	}
}

/// Wrapper around `vkCmdResetEvent`.
/// Resets an event when the accesses defined by `previous_accesses` are completed.
/// `command_buffer` and `event` are passed unmodified to `vkCmdResetEvent`.
pub fn reset_event(
	device: &ash::vk::DeviceFnV1_0,
	command_buffer: ash::vk::CommandBuffer,
	event: ash::vk::Event,
	previous_accesses: &[AccessType],
) {
	let mut stage_mask = ash::vk::PipelineStageFlags::TOP_OF_PIPE;
	for previous_access in previous_accesses {
		let previous_info = get_access_info(*previous_access);
		stage_mask |= previous_info.stage_mask;
	}

	unsafe {
		device.cmd_reset_event(command_buffer, event, stage_mask);
	}
}

/// Simplified wrapper around `vkCmdWaitEvents`.
/// The mapping functions defined above are used to translate the passed in
/// barrier definitions into a set of pipeline stages and native Vulkan memory
/// barriers to be passed to `vkCmdPipelineBarrier`.
///
/// `commandBuffer` and `events` are passed unmodified to `vkCmdWaitEvents`.
pub fn wait_events(
	device: &ash::vk::DeviceFnV1_0,
	command_buffer: ash::vk::CommandBuffer,
	events: &[ash::vk::Event],
	global_barrier: Option<GlobalBarrier>,
	buffer_barriers: &[BufferBarrier],
	image_barriers: &[ImageBarrier],
) {
	let mut src_stage_mask = ash::vk::PipelineStageFlags::TOP_OF_PIPE;
	let mut dst_stage_mask = ash::vk::PipelineStageFlags::BOTTOM_OF_PIPE;

	let mut vk_memory_barriers: Vec<ash::vk::MemoryBarrier> = Vec::with_capacity(1);
	let mut vk_buffer_barriers: Vec<ash::vk::BufferMemoryBarrier> =
		Vec::with_capacity(buffer_barriers.len());
	let mut vk_image_barriers: Vec<ash::vk::ImageMemoryBarrier> =
		Vec::with_capacity(image_barriers.len());

	// Global memory barrier
	if let Some(ref barrier) = global_barrier {
		let (src_mask, dst_mask, barrier) = get_memory_barrier(barrier);
		src_stage_mask |= src_mask;
		dst_stage_mask |= dst_mask;
		vk_memory_barriers.push(barrier);
	}

	// Buffer memory barriers
	for buffer_barrier in buffer_barriers {
		let (src_mask, dst_mask, barrier) = get_buffer_memory_barrier(buffer_barrier);
		src_stage_mask |= src_mask;
		dst_stage_mask |= dst_mask;
		vk_buffer_barriers.push(barrier);
	}

	// Image memory barriers
	for image_barrier in image_barriers {
		let (src_mask, dst_mask, barrier) = get_image_memory_barrier(image_barrier);
		src_stage_mask |= src_mask;
		dst_stage_mask |= dst_mask;
		vk_image_barriers.push(barrier);
	}

	unsafe {
		device.cmd_wait_events(
			command_buffer,
			events.len() as u32,
			events.as_ptr(),
			src_stage_mask,
			dst_stage_mask,
			vk_memory_barriers.len() as u32,
			vk_memory_barriers.as_ptr(),
			vk_buffer_barriers.len() as u32,
			vk_buffer_barriers.as_ptr(),
			vk_image_barriers.len() as u32,
			vk_image_barriers.as_ptr(),
		);
	}
}
