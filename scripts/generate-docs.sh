#!/bin/bash
set -euo pipefail

version="$1"

CMDS=(run create instances images ports show modify console ssh scp exec start \
    stop restart rename clone snapshot restore delete prune completions)

function generate_cmd_doc() {
    name="$1"
    ref="$2"
    file="$3"
    shift 3
    underline="${name//?/=}"

    echo -e ".. $ref:\n\n$name\n$underline\n\n.. code-block::\n\n    \$ $name --help" > "$file"
    cargo run -- "$@" --help | sed "s/^/    /" >> "$file"
}

# Set version
sed "s/^release = .*$/release = '$version'/g" -i docs/conf.py

# Create Reference Doc Directory
mkdir -p docs/reference

# Generate cubic help
generate_cmd_doc "cubic" "_ref_cubic" "docs/reference/cubic.rst"

# Generate cubic subcommands help
for cmd in "${CMDS[@]}"; do
    generate_cmd_doc "cubic $cmd" "_ref_cubic_$cmd" "docs/reference/$cmd.rst" "$cmd"
done

# Generate reference/index.rst as the Command Reference landing page
cat > docs/reference/index.rst << 'REFEOF'
Command Reference
=================

.. toctree::
   :hidden:

   cubic
REFEOF

for cmd in "${CMDS[@]}"; do
    echo "   $cmd" >> docs/reference/index.rst
done

# Generate index.rst with a single root toctree
cat > docs/index.rst << 'EOF'
Cubic
=====

.. toctree::
   :caption: How-To
   :hidden:

   howto/install
   howto/shell_completions
   howto/getting_started
   howto/snapshots
   howto/templates
   howto/http_server
   howto/ssh_connect
   howto/console_login
   howto/environment_variables

.. toctree::
   :caption: Troubleshooting
   :hidden:

   troubleshooting/qemu_not_found
   troubleshooting/recover_disk

.. toctree::
   :caption: Internals
   :hidden:

   internals/how_it_works
   internals/security
   internals/qemu_detection

.. toctree::
   :caption: Command Reference
   :hidden:

EOF

for cmd in "${CMDS[@]}"; do
    echo "   reference/$cmd" >> docs/index.rst
done

cat >> docs/index.rst << 'EOF'

Cubic spins up Linux virtual machines on Linux, macOS and Windows with a single command.

Every distribution comes as an official image and is ready to use within seconds, so you skip the long installation. Cubic keeps things simple and secure by acting as lightweight glue over proven tools. No privileged system service is required and every VM runs as your normal user.
Cubic is built on top of ``QEMU``, ``EDK2``, official Linux distribution images and ``cloud-init``.

Features
---------

**Fast and simple**

* Creates a VM and opens a shell in one command
* Boots official Linux distribution images in seconds
* Written in Rust

**Runs anywhere**

* Runs on **Linux**, **macOS** and **Windows** hosts
* Ships **Alma Linux**, **Arch Linux**, **Debian**, **Fedora**, **Gentoo**, **OpenSUSE**, **Rocky Linux** and **Ubuntu**
* Runs **amd64** and **arm64** guests
* Accelerates every VM with **KVM** (Linux), **Hypervisor** (macOS) and **WHPX** (Windows)

**Everyday work**

* Forwards ports from a VM to the host
* Copies files between host and VM and between two VMs
* Executes single commands in a VM
* Creates VM instances from reusable templates
* Snapshots a VM disk and restores it later
* Clones and renames VM instances
* Runs temporary VM instances that are deleted after use
* Isolates a VM from the network with one flag

**Safe by default**

* Runs every VM as a normal user process without a privileged system service
* Verifies every image against the checksum of the distribution
* Protects every VM with its own SSH key and a locked password
* Encrypts the QEMU control channels with mutual TLS

Source Code
===========

The source code of Cubic is on `Github`_.

.. _Github: https://github.com/cubic-vm/cubic
EOF
