//! Tests are based on the common synchronization examples on the Vulkan-Docs wiki: https://github.com/KhronosGroup/Vulkan-Docs/wiki/Synchronization-Examples.

extern crate ash;
extern crate vk_sync;

#[test]
fn compute_write_storage_fragment_read_sampled() {
	// Compute write to storage image, Graphics fragment read as sampled image
	let image_barrier = vk_sync::ImageBarrier {
		previous_accesses: vec![vk_sync::AccessType::ComputeShaderWrite],
		next_accesses: vec![
			vk_sync::AccessType::FragmentShaderReadSampledImageOrUniformTexelBuffer,
		],
		previous_layout: vk_sync::ImageLayout::Optimal,
		next_layout: vk_sync::ImageLayout::Optimal,
		discard_contents: false,
		src_queue_family_index: 0,
		dst_queue_family_index: 0,
		image: ash::vk::Image::null(),
		range: ash::vk::ImageSubresourceRange {
			aspect_mask: ash::vk::ImageAspectFlags::empty(),
			base_mip_level: 0,
			level_count: 1,
			base_array_layer: 0,
			layer_count: 1,
		},
	};

	let (src_mask, dst_mask, barrier) = vk_sync::get_image_memory_barrier(&image_barrier);

	assert_eq!(src_mask, ash::vk::PIPELINE_STAGE_COMPUTE_SHADER_BIT);
	assert_eq!(dst_mask, ash::vk::PIPELINE_STAGE_FRAGMENT_SHADER_BIT);
	assert_eq!(barrier.src_access_mask, ash::vk::ACCESS_SHADER_WRITE_BIT);
	assert_eq!(barrier.dst_access_mask, ash::vk::ACCESS_SHADER_READ_BIT);
	assert_eq!(barrier.old_layout, ash::vk::ImageLayout::General);
	assert_eq!(
		barrier.new_layout,
		ash::vk::ImageLayout::ShaderReadOnlyOptimal
	);
}

#[test]
fn graphics_write_color_compute_read_sampled() {
	// Graphics write to color attachment, Compute read from sampled image
	let image_barrier = vk_sync::ImageBarrier {
		previous_accesses: vec![vk_sync::AccessType::ColorAttachmentWrite],
		next_accesses: vec![vk_sync::AccessType::ComputeShaderReadSampledImageOrUniformTexelBuffer],
		previous_layout: vk_sync::ImageLayout::Optimal,
		next_layout: vk_sync::ImageLayout::Optimal,
		discard_contents: false,
		src_queue_family_index: 0,
		dst_queue_family_index: 0,
		image: ash::vk::Image::null(),
		range: ash::vk::ImageSubresourceRange {
			aspect_mask: ash::vk::ImageAspectFlags::empty(),
			base_mip_level: 0,
			level_count: 1,
			base_array_layer: 0,
			layer_count: 1,
		},
	};

	let (src_mask, dst_mask, barrier) = vk_sync::get_image_memory_barrier(&image_barrier);

	assert_eq!(
		src_mask,
		ash::vk::PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT
	);
	assert_eq!(dst_mask, ash::vk::PIPELINE_STAGE_COMPUTE_SHADER_BIT);
	assert_eq!(
		barrier.src_access_mask,
		ash::vk::ACCESS_COLOR_ATTACHMENT_WRITE_BIT
	);
	assert_eq!(barrier.dst_access_mask, ash::vk::ACCESS_SHADER_READ_BIT);
	assert_eq!(
		barrier.old_layout,
		ash::vk::ImageLayout::ColorAttachmentOptimal
	);
	assert_eq!(
		barrier.new_layout,
		ash::vk::ImageLayout::ShaderReadOnlyOptimal
	);
}

#[test]
fn graphics_write_depth_compute_read_sampled() {
	// Graphics write to color attachment, Compute read from sampled image
	let image_barrier = vk_sync::ImageBarrier {
		previous_accesses: vec![vk_sync::AccessType::DepthStencilAttachmentWrite],
		next_accesses: vec![vk_sync::AccessType::ComputeShaderReadSampledImageOrUniformTexelBuffer],
		previous_layout: vk_sync::ImageLayout::Optimal,
		next_layout: vk_sync::ImageLayout::Optimal,
		discard_contents: false,
		src_queue_family_index: 0,
		dst_queue_family_index: 0,
		image: ash::vk::Image::null(),
		range: ash::vk::ImageSubresourceRange {
			aspect_mask: ash::vk::ImageAspectFlags::empty(),
			base_mip_level: 0,
			level_count: 1,
			base_array_layer: 0,
			layer_count: 1,
		},
	};

	let (src_mask, dst_mask, barrier) = vk_sync::get_image_memory_barrier(&image_barrier);

	assert_eq!(
		src_mask,
		ash::vk::PIPELINE_STAGE_EARLY_FRAGMENT_TESTS_BIT
			| ash::vk::PIPELINE_STAGE_LATE_FRAGMENT_TESTS_BIT
	);
	assert_eq!(dst_mask, ash::vk::PIPELINE_STAGE_COMPUTE_SHADER_BIT);
	assert_eq!(
		barrier.src_access_mask,
		ash::vk::ACCESS_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT
	);
	assert_eq!(barrier.dst_access_mask, ash::vk::ACCESS_SHADER_READ_BIT);
	assert_eq!(
		barrier.old_layout,
		ash::vk::ImageLayout::DepthStencilAttachmentOptimal
	);
	assert_eq!(
		barrier.new_layout,
		ash::vk::ImageLayout::ShaderReadOnlyOptimal
	);
}

#[test]
fn graphics_write_depth_fragment_read_attachment() {
	// Graphics write to depth attachment, Graphics fragment read from input attachment
	let image_barrier = vk_sync::ImageBarrier {
		previous_accesses: vec![vk_sync::AccessType::DepthStencilAttachmentWrite],
		next_accesses: vec![vk_sync::AccessType::FragmentShaderReadDepthStencilInputAttachment],
		previous_layout: vk_sync::ImageLayout::Optimal,
		next_layout: vk_sync::ImageLayout::Optimal,
		discard_contents: false,
		src_queue_family_index: 0,
		dst_queue_family_index: 0,
		image: ash::vk::Image::null(),
		range: ash::vk::ImageSubresourceRange {
			aspect_mask: ash::vk::ImageAspectFlags::empty(),
			base_mip_level: 0,
			level_count: 1,
			base_array_layer: 0,
			layer_count: 1,
		},
	};

	let (src_mask, dst_mask, barrier) = vk_sync::get_image_memory_barrier(&image_barrier);

	assert_eq!(
		src_mask,
		ash::vk::PIPELINE_STAGE_EARLY_FRAGMENT_TESTS_BIT
			| ash::vk::PIPELINE_STAGE_LATE_FRAGMENT_TESTS_BIT
	);
	assert_eq!(dst_mask, ash::vk::PIPELINE_STAGE_FRAGMENT_SHADER_BIT);
	assert_eq!(
		barrier.src_access_mask,
		ash::vk::ACCESS_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT
	);
	assert_eq!(
		barrier.dst_access_mask,
		ash::vk::ACCESS_INPUT_ATTACHMENT_READ_BIT
	);
	assert_eq!(
		barrier.old_layout,
		ash::vk::ImageLayout::DepthStencilAttachmentOptimal
	);
	assert_eq!(
		barrier.new_layout,
		ash::vk::ImageLayout::DepthStencilReadOnlyOptimal
	);
}

#[test]
fn graphics_write_depth_fragment_read_sampled() {
	// Graphics write to depth attachment, Graphics fragment read from sampled image
	let image_barrier = vk_sync::ImageBarrier {
		previous_accesses: vec![vk_sync::AccessType::DepthStencilAttachmentWrite],
		next_accesses: vec![
			vk_sync::AccessType::FragmentShaderReadSampledImageOrUniformTexelBuffer,
		],
		previous_layout: vk_sync::ImageLayout::Optimal,
		next_layout: vk_sync::ImageLayout::Optimal,
		discard_contents: false,
		src_queue_family_index: 0,
		dst_queue_family_index: 0,
		image: ash::vk::Image::null(),
		range: ash::vk::ImageSubresourceRange {
			aspect_mask: ash::vk::ImageAspectFlags::empty(),
			base_mip_level: 0,
			level_count: 1,
			base_array_layer: 0,
			layer_count: 1,
		},
	};

	let (src_mask, dst_mask, barrier) = vk_sync::get_image_memory_barrier(&image_barrier);

	assert_eq!(
		src_mask,
		ash::vk::PIPELINE_STAGE_EARLY_FRAGMENT_TESTS_BIT
			| ash::vk::PIPELINE_STAGE_LATE_FRAGMENT_TESTS_BIT
	);
	assert_eq!(dst_mask, ash::vk::PIPELINE_STAGE_FRAGMENT_SHADER_BIT);
	assert_eq!(
		barrier.src_access_mask,
		ash::vk::ACCESS_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT
	);
	assert_eq!(barrier.dst_access_mask, ash::vk::ACCESS_SHADER_READ_BIT);
	assert_eq!(
		barrier.old_layout,
		ash::vk::ImageLayout::DepthStencilAttachmentOptimal
	);
	assert_eq!(
		barrier.new_layout,
		ash::vk::ImageLayout::ShaderReadOnlyOptimal
	);
}

#[test]
fn graphics_write_color_fragment_read_attachment() {
	// Graphics write to color attachment, Graphics fragment read from input attachment
	let image_barrier = vk_sync::ImageBarrier {
		previous_accesses: vec![vk_sync::AccessType::ColorAttachmentWrite],
		next_accesses: vec![vk_sync::AccessType::FragmentShaderReadColorInputAttachment],
		previous_layout: vk_sync::ImageLayout::Optimal,
		next_layout: vk_sync::ImageLayout::Optimal,
		discard_contents: false,
		src_queue_family_index: 0,
		dst_queue_family_index: 0,
		image: ash::vk::Image::null(),
		range: ash::vk::ImageSubresourceRange {
			aspect_mask: ash::vk::ImageAspectFlags::empty(),
			base_mip_level: 0,
			level_count: 1,
			base_array_layer: 0,
			layer_count: 1,
		},
	};

	let (src_mask, dst_mask, barrier) = vk_sync::get_image_memory_barrier(&image_barrier);

	assert_eq!(
		src_mask,
		ash::vk::PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT
	);
	assert_eq!(dst_mask, ash::vk::PIPELINE_STAGE_FRAGMENT_SHADER_BIT);
	assert_eq!(
		barrier.src_access_mask,
		ash::vk::ACCESS_COLOR_ATTACHMENT_WRITE_BIT
	);
	assert_eq!(
		barrier.dst_access_mask,
		ash::vk::ACCESS_INPUT_ATTACHMENT_READ_BIT
	);
	assert_eq!(
		barrier.old_layout,
		ash::vk::ImageLayout::ColorAttachmentOptimal
	);
	assert_eq!(
		barrier.new_layout,
		ash::vk::ImageLayout::ShaderReadOnlyOptimal
	);
}

#[test]
fn graphics_write_color_fragment_read_sampled() {
	// Graphics write to color attachment, Graphics fragment read from input attachment
	let image_barrier = vk_sync::ImageBarrier {
		previous_accesses: vec![vk_sync::AccessType::ColorAttachmentWrite],
		next_accesses: vec![
			vk_sync::AccessType::FragmentShaderReadSampledImageOrUniformTexelBuffer,
		],
		previous_layout: vk_sync::ImageLayout::Optimal,
		next_layout: vk_sync::ImageLayout::Optimal,
		discard_contents: false,
		src_queue_family_index: 0,
		dst_queue_family_index: 0,
		image: ash::vk::Image::null(),
		range: ash::vk::ImageSubresourceRange {
			aspect_mask: ash::vk::ImageAspectFlags::empty(),
			base_mip_level: 0,
			level_count: 1,
			base_array_layer: 0,
			layer_count: 1,
		},
	};

	let (src_mask, dst_mask, barrier) = vk_sync::get_image_memory_barrier(&image_barrier);

	assert_eq!(
		src_mask,
		ash::vk::PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT
	);
	assert_eq!(dst_mask, ash::vk::PIPELINE_STAGE_FRAGMENT_SHADER_BIT);
	assert_eq!(
		barrier.src_access_mask,
		ash::vk::ACCESS_COLOR_ATTACHMENT_WRITE_BIT
	);
	assert_eq!(barrier.dst_access_mask, ash::vk::ACCESS_SHADER_READ_BIT);
	assert_eq!(
		barrier.old_layout,
		ash::vk::ImageLayout::ColorAttachmentOptimal
	);
	assert_eq!(
		barrier.new_layout,
		ash::vk::ImageLayout::ShaderReadOnlyOptimal
	);
}

#[test]
fn graphics_write_color_vertex_read_sampled() {
	// Graphics write to color attachment, Graphics vertex read from sampled image
	let image_barrier = vk_sync::ImageBarrier {
		previous_accesses: vec![vk_sync::AccessType::ColorAttachmentWrite],
		next_accesses: vec![vk_sync::AccessType::VertexShaderReadSampledImageOrUniformTexelBuffer],
		previous_layout: vk_sync::ImageLayout::Optimal,
		next_layout: vk_sync::ImageLayout::Optimal,
		discard_contents: false,
		src_queue_family_index: 0,
		dst_queue_family_index: 0,
		image: ash::vk::Image::null(),
		range: ash::vk::ImageSubresourceRange {
			aspect_mask: ash::vk::ImageAspectFlags::empty(),
			base_mip_level: 0,
			level_count: 1,
			base_array_layer: 0,
			layer_count: 1,
		},
	};

	let (src_mask, dst_mask, barrier) = vk_sync::get_image_memory_barrier(&image_barrier);

	assert_eq!(
		src_mask,
		ash::vk::PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT
	);
	assert_eq!(dst_mask, ash::vk::PIPELINE_STAGE_VERTEX_SHADER_BIT);
	assert_eq!(
		barrier.src_access_mask,
		ash::vk::ACCESS_COLOR_ATTACHMENT_WRITE_BIT
	);
	assert_eq!(barrier.dst_access_mask, ash::vk::ACCESS_SHADER_READ_BIT);
	assert_eq!(
		barrier.old_layout,
		ash::vk::ImageLayout::ColorAttachmentOptimal
	);
	assert_eq!(
		barrier.new_layout,
		ash::vk::ImageLayout::ShaderReadOnlyOptimal
	);
}

#[test]
fn graphics_read_sampled_graphics_write_color() {
	// Graphics fragment read from sampled image, Graphics write to color attachment
	let image_barrier = vk_sync::ImageBarrier {
		previous_accesses: vec![
			vk_sync::AccessType::FragmentShaderReadSampledImageOrUniformTexelBuffer,
		],
		next_accesses: vec![vk_sync::AccessType::ColorAttachmentWrite],
		previous_layout: vk_sync::ImageLayout::Optimal,
		next_layout: vk_sync::ImageLayout::Optimal,
		discard_contents: false,
		src_queue_family_index: 0,
		dst_queue_family_index: 0,
		image: ash::vk::Image::null(),
		range: ash::vk::ImageSubresourceRange {
			aspect_mask: ash::vk::ImageAspectFlags::empty(),
			base_mip_level: 0,
			level_count: 1,
			base_array_layer: 0,
			layer_count: 1,
		},
	};

	let (src_mask, dst_mask, barrier) = vk_sync::get_image_memory_barrier(&image_barrier);

	assert_eq!(src_mask, ash::vk::PIPELINE_STAGE_FRAGMENT_SHADER_BIT);
	assert_eq!(
		dst_mask,
		ash::vk::PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT
	);
	assert_eq!(barrier.src_access_mask, ash::vk::AccessFlags::empty());
	assert_eq!(barrier.dst_access_mask, ash::vk::AccessFlags::empty());
	assert_eq!(
		barrier.old_layout,
		ash::vk::ImageLayout::ShaderReadOnlyOptimal
	);
	assert_eq!(
		barrier.new_layout,
		ash::vk::ImageLayout::ColorAttachmentOptimal
	);
}

#[test]
fn transfer_write_image_fragment_read_sampled() {
	// Transfer write to image, Graphics fragment read from sampled image
	let image_barrier = vk_sync::ImageBarrier {
		previous_accesses: vec![vk_sync::AccessType::TransferWrite],
		next_accesses: vec![
			vk_sync::AccessType::FragmentShaderReadSampledImageOrUniformTexelBuffer,
		],
		previous_layout: vk_sync::ImageLayout::Optimal,
		next_layout: vk_sync::ImageLayout::Optimal,
		discard_contents: false,
		src_queue_family_index: 0,
		dst_queue_family_index: 0,
		image: ash::vk::Image::null(),
		range: ash::vk::ImageSubresourceRange {
			aspect_mask: ash::vk::ImageAspectFlags::empty(),
			base_mip_level: 0,
			level_count: 1,
			base_array_layer: 0,
			layer_count: 1,
		},
	};

	let (src_mask, dst_mask, barrier) = vk_sync::get_image_memory_barrier(&image_barrier);

	assert_eq!(src_mask, ash::vk::PIPELINE_STAGE_TRANSFER_BIT);
	assert_eq!(dst_mask, ash::vk::PIPELINE_STAGE_FRAGMENT_SHADER_BIT);
	assert_eq!(barrier.src_access_mask, ash::vk::ACCESS_TRANSFER_WRITE_BIT);
	assert_eq!(barrier.dst_access_mask, ash::vk::ACCESS_SHADER_READ_BIT);
	assert_eq!(barrier.old_layout, ash::vk::ImageLayout::TransferDstOptimal);
	assert_eq!(
		barrier.new_layout,
		ash::vk::ImageLayout::ShaderReadOnlyOptimal
	);
}

#[test]
fn graphics_write_color_presentation() {
	// Graphics color attachment write, Presentation
	let image_barrier = vk_sync::ImageBarrier {
		previous_accesses: vec![vk_sync::AccessType::ColorAttachmentWrite],
		next_accesses: vec![vk_sync::AccessType::Present],
		previous_layout: vk_sync::ImageLayout::Optimal,
		next_layout: vk_sync::ImageLayout::Optimal,
		discard_contents: false,
		src_queue_family_index: 0,
		dst_queue_family_index: 0,
		image: ash::vk::Image::null(),
		range: ash::vk::ImageSubresourceRange {
			aspect_mask: ash::vk::ImageAspectFlags::empty(),
			base_mip_level: 0,
			level_count: 1,
			base_array_layer: 0,
			layer_count: 1,
		},
	};

	let (src_mask, dst_mask, barrier) = vk_sync::get_image_memory_barrier(&image_barrier);

	assert_eq!(
		src_mask,
		ash::vk::PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT
	);
	assert_eq!(dst_mask, ash::vk::PIPELINE_STAGE_BOTTOM_OF_PIPE_BIT);
	assert_eq!(
		barrier.src_access_mask,
		ash::vk::ACCESS_COLOR_ATTACHMENT_WRITE_BIT
	);
	assert_eq!(barrier.dst_access_mask, ash::vk::AccessFlags::empty());
	assert_eq!(
		barrier.old_layout,
		ash::vk::ImageLayout::ColorAttachmentOptimal
	);
	assert_eq!(barrier.new_layout, ash::vk::ImageLayout::PresentSrcKhr);
}
