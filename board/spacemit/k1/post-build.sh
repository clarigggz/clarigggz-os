#!/bin/bash
# post-build.sh: Inject the Clarigggz Engine as PID 1

TARGET_DIR=$1
CLARIGGGZ_ENGINE_BIN="clarigggz-engine"

echo "Configuring Clarigggz OS Payload..."

# 1. Remove all standard Linux userspace clutter
rm -rf ${TARGET_DIR}/etc/init.d
rm -rf ${TARGET_DIR}/etc/network
rm -rf ${TARGET_DIR}/usr/bin/busybox

# 2. Setup the Rust Engine as the init process
# We assume the engine has been compiled and exists in the artifacts directory
cp ${BINARIES_DIR}/${CLARIGGGZ_ENGINE_BIN} ${TARGET_DIR}/sbin/init
chmod +x ${TARGET_DIR}/sbin/init

# 3. Minimal /etc/fstab for the Unikernel
cat <<EOF > ${TARGET_DIR}/etc/fstab
/dev/root / auto defaults 1 1
EOF

echo "Clarigggz OS V2: Payload Injected (PID 1 ready)."
