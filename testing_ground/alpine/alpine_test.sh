#!/usr/bin/env bash
set -euo pipefail

export LC_ALL=C
export LANG=C


ALPINE_VERSION="3.20.0"
ALPINE_MAJOR=$(echo "$ALPINE_VERSION" | cut -d. -f1,2)
ISO_NAME="alpine-standard-${ALPINE_VERSION}-x86_64.iso"
ISO_URL="https://dl-cdn.alpinelinux.org/alpine/v${ALPINE_MAJOR}/releases/x86_64/${ISO_NAME}"

ROOTFS_DIR="../rootfs"
BUILD_DIR="../build"
INITRAMFS_IMG="initramfs.cpio.gz"
OCTO_INIT="../../target/release/octo-init"


set -euo pipefail

export LC_ALL=C
export LANG=C

ALPINE_VERSION="3.20.0"
ALPINE_MAJOR=$(echo "$ALPINE_VERSION" | cut -d. -f1,2)
ISO_NAME="alpine-standard-${ALPINE_VERSION}-x86_64.iso"
ISO_URL="https://dl-cdn.alpinelinux.org/alpine/v${ALPINE_MAJOR}/releases/x86_64/${ISO_NAME}"

ROOTFS_DIR="rootfs"
BUILD_DIR="build"
INITRAMFS_IMG="initramfs.cpio.gz"

if [ -d "$ROOTFS_DIR" ] && [ -f "$ROOTFS_DIR/bin/sh" ]; then
    echo "[+] Existing rootfs found in '$ROOTFS_DIR'. Skipping download and extraction."
else
    echo "[+] Rootfs not found. Setting up working directories..."
    mkdir -p "$BUILD_DIR" "$ROOTFS_DIR"

    if [ ! -f "$BUILD_DIR/$ISO_NAME" ]; then
        echo "[+] Downloading Alpine Standard ISO..."
        curl -sSL "$ISO_URL" -o "$BUILD_DIR/$ISO_NAME"
    fi

    echo "[+] Extracting ISO contents..."
    bsdtar -xf "$BUILD_DIR/$ISO_NAME" -C "$BUILD_DIR" boot/vmlinuz-lts boot/initramfs-lts
    
    if [ -d "$BUILD_DIR/boot" ]; then
        mv -f "$BUILD_DIR/boot/"* "$BUILD_DIR/"
        rmdir "$BUILD_DIR/boot"
    fi

    echo "[+] Unpacking Alpine base rootfs..."
    (
        cd "$ROOTFS_DIR"
        gzip -dc "../$BUILD_DIR/initramfs-lts" | cpio -idmv > /dev/null 2>&1 || \
        cpio -idmv < "../$BUILD_DIR/initramfs-lts" > /dev/null 2>&1
    )
    
    echo "[+] Base rootfs ready."
fi

echo "[+] Installing custom init binary..."
if [ ! -f "$OCTO_INIT" ]; then
    echo "[-] Error: Custom init binary '$OCTO_INIT' not found! Build it first."
    exit 1
fi

cp "$OCTO_INIT" "$ROOTFS_DIR/init"
chmod +x "$ROOTFS_DIR/init"

echo "[+] Creating /etc/octo-init.conf..."
mkdir -p "$ROOTFS_DIR/etc"

cat << 'EOF' > "$ROOTFS_DIR/etc/octo-init.conf"
WAITFOR /sbin/mdev
TTY /bin/sh
EOF

echo "[+] Packing new initramfs image ($INITRAMFS_IMG)..."
(
    cd "$ROOTFS_DIR"
    find . -print0 | cpio --null -ov --format=newc | gzip -9 > "../$INITRAMFS_IMG"
)

echo "[+] Launching VM in QEMU..."

KERNEL_BIN="$BUILD_DIR/vmlinuz-lts"

qemu-system-x86_64 \
    -kernel "$KERNEL_BIN" \
    -initrd "$INITRAMFS_IMG" \
    -append "console=ttyS0 quiet poweroff=force" \
    -nographic \
    -no-reboot \
    -m 512M