//! Tests are based on the common synchronization examples on the Vulkan-Docs wiki: https://github.com/KhronosGroup/Vulkan-Docs/wiki/Synchronization-Examples.

extern crate vk_sync;
extern crate ash;

#[test]
fn compute_write_storage_compute_read_storage() {
    // Compute write to storage buffer/image, Compute read from storage buffer/image
}

#[test]
fn compute_read_storage_compute_write_storage() {
    // Compute read from storage buffer, Compute write from storage buffer
}

#[test]
fn compute_write_storage_graphics_read_index() {
    // Compute write to storage buffer, Graphics read as index buffer
}

#[test]
fn compute_write_storage_graphics_read_indirect() {
    // Compute write to storage buffer, Graphics read as indirect buffer
}
