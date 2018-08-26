use ash;

use super::AccessType;
use super::BufferBarrier;
use super::GlobalBarrier;
use super::ImageBarrier;
use super::ImageLayout;
use super::get_access_info;

// Simplified wrapper around vkCmdPipelineBarrier.
// The mapping functions defined above are used to translate the passed in
// barrier definitions into a set of pipeline stages and native Vulkan memory
// barriers to be passed to vkCmdPipelineBarrier.
// commandBuffer is passed unmodified to vkCmdPipelineBarrier.
pub fn pipeline_barrier(
    command_buffer: ash::vk::CommandBuffer,
    global_barrier: Option<GlobalBarrier>,
    buffer_barriers: &[BufferBarrier],
    image_barriers: &[ImageBarrier],
) {

}

// Wrapper around vkCmdSetEvent.
// Sets an event when the accesses defined by pPrevAccesses are completed.
// commandBuffer and event are passed unmodified to vkCmdSetEvent.
pub fn set_event(
    device: ash::vk::cmds::DeviceFnV1_0,
    command_buffer: ash::vk::CommandBuffer,
    event: ash::vk::Event,
    previous_accesses: &[AccessType],
) {
    let mut stage_mask = ash::vk::PIPELINE_STAGE_TOP_OF_PIPE_BIT;
    for previous_access in previous_accesses {
        let previous_info = get_access_info(previous_access);
        stage_mask |= previous_info.stage_mask;
    }

    unsafe {
        device.cmd_set_event(command_buffer, event, stage_mask);
    }
}

// Wrapper around vkCmdResetEvent.
// Resets an event when the accesses defined by pPrevAccesses are completed.
// command_buffer and event are passed unmodified to vkCmdResetEvent.
pub fn reset_event(
    command_buffer: ash::vk::CommandBuffer,
    event: ash::vk::Event,
    previous_accesses: &[AccessType],
) {

}

// Simplified wrapper around vkCmdWaitEvents.
// The mapping functions defined above are used to translate the passed in
// barrier definitions into a set of pipeline stages and native Vulkan memory
// barriers to be passed to vkCmdPipelineBarrier.
// commandBuffer, eventCount, and pEvents are passed unmodified to
// vkCmdWaitEvents.
pub fn wait_events(
    command_buffer: ash::vk::CommandBuffer,
    events: &[ash::vk::Event],
    global_barrier: Option<GlobalBarrier>,
    buffer_barriers: &[BufferBarrier],
    image_barriers: &[ImageBarrier],
) {

}
