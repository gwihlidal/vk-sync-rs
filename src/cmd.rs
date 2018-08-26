use ash;

use super::GlobalBarrier;
use super::BufferBarrier;
use super::ImageBarrier;
use super::AccessType;
use super::ImageLayout;

// Simplified wrapper around vkCmdPipelineBarrier.
// The mapping functions defined above are used to translate the passed in
// barrier definitions into a set of pipeline stages and native Vulkan memory
// barriers to be passed to vkCmdPipelineBarrier.
// commandBuffer is passed unmodified to vkCmdPipelineBarrier.
pub fn pipeline_barrier(
    command_buffer: ash::vk::CommandBuffer,
    global_barrier: Option<GlobalBarrier>,
    buffer_barriers: &[BufferBarrier],
    image_barriers: &[ImageBarrier]) {

}

// Wrapper around vkCmdSetEvent.
// Sets an event when the accesses defined by pPrevAccesses are completed.
// commandBuffer and event are passed unmodified to vkCmdSetEvent.
pub fn set_event(
    command_buffer: ash::vk::CommandBuffer,
    event: ash::vk::Event,
    previous_accesses: &[AccessType]) {

}

// Wrapper around vkCmdResetEvent.
// Resets an event when the accesses defined by pPrevAccesses are completed.
// command_buffer and event are passed unmodified to vkCmdResetEvent.
pub fn reset_event(
    command_buffer: ash::vk::CommandBuffer,
    event: ash::vk::Event,
    previous_accesses: &[AccessType]) {

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
    image_barriers: &[ImageBarrier]) {

}