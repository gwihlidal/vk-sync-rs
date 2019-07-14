# Changes

## 0.1.6 (2019-07-14)

* Removed inefficient Vec<AccessType> on barrier structs in favor of slice references.

## 0.1.5

* Updated to ash 0.29.

## 0.1.4

* Minor optimizations.

## 0.1.3

* Rust 2018 Edition.

## 0.1.2 (2018-11-17)

* Updated to ash 0.26
* Use default struct init from ash
* Made function pointer structs borrowed for performance
* Some minor cleanup

## 0.1.1 (2018-11-15)

* Updated to ash 0.25 (Vulkan 1.1)
* Added support for NVX generated commands
* Added support for read-only depth/stencil + writeable depth/stencil
* Added Copy and Default traits to AccessType and ImageLayout
* Added Debug, Default, and Clone traits to GlobalBarrier, BufferBarrier, and ImageBarrier

## 0.1.0 (2018-08-26)

* First release