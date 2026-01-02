# Clarigggz OS (V2)

Strategic Pivot: Pragmatic Sovereignty.

## Architecture

- **Kernel**: Standard Linux 6.x (Bianbu/SpacemiT fork). Stripped to absolute minimum.
- **Userland**: Deleted. No Systemd. No GNU Utils. No Bash.
- **PID 1**: The [Clarigggz Engine](../clarigggz-engine) is the init process. It owns the hardware directly.

## Strategy

We utilize the vendor-provided Linux Kernel (`linux-k1x`) to leverage the existing driver ecosystem (ISP, GPU, PMIC) while maintaining a minimal, high-performance footprint.

## Build System

- **Toolchain**: Buildroot / Yocto (Targeting SpacemiT K1).
- **Payload**: The Rust Engine compiled as the sole userspace binary.

## Optimization Targets

- **Boot Time**: <2 seconds.
- **Display**: Direct DRM/KMS.
- **Camera**: V4L2 integration.
- **Vector**: RVV 1.0 (X60 Cores).
