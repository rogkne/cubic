[![Cubic](https://github.com/cubic-vm/cubic/blob/main/cubic.svg)](https://github.com/cubic-vm/cubic)
:star: Please star us on [GitHub](https://github.com/cubic-vm/cubic) to promote the project!

[![github.com](https://github.com/cubic-vm/cubic/actions/workflows/build.yml/badge.svg)](https://github.com/cubic-vm/cubic/actions/workflows/build.yml)
[![crates.io](https://img.shields.io/crates/v/cubic.svg)](https://crates.io/crates/cubic)
[![snapcraft.io](https://snapcraft.io/cubic/badge.svg)](https://snapcraft.io/cubic)


Cubic is a lightweight command-line manager for virtual machines with a focus on simplicity and security.

It has a simple, daemonless, and rootless design. All Cubic virtual machines run isolated in the user context.
Cubic is built on top of `QEMU`, `KVM`, and `cloud-init`.

[![Get it from the Snap Store](https://snapcraft.io/en/dark/install.svg)](https://snapcraft.io/cubic)

# :monocle_face: Why use Cubic?

Cubic is a simple tool that may be used for various purposes, such as:

- Developing and testing software on different Linux distributions
- Installing software on a virtual machine to avoid polluting the host system
- Running untrusted software on an isolated virtual machine

# :fire: Features

- Simple command-line interface
- Supports the following guest OS:
  - **Alma Linux**
  - **Arch Linux**
  - **Debian**
  - **Fedora**
  - **Gentoo**
  - **OpenSUSE**
  - **Rocky Linux**
  - **Ubuntu**
- Supports the following host OS: **Linux**, **macOS**, **Windows**
- Supports **amd64** and **arm64** CPU architectures
- Supports hardware acceleration with **KVM** (Linux), **Hypervisor** (macOS), and **Hyper-V** (Windows)
- Daemonless design which does not require root privileges
- Written in Rust

# :rocket: Quick Start

A virtual machine instance can be created with a single command. This example
creates an instance from a Ubuntu image with the name `quickstart`.
```
$ cubic run quickstart --image ubuntu:noble
Welcome to Ubuntu 24.04.4 LTS (GNU/Linux 6.8.0-101-generic x86_64)

 * Documentation:  https://help.ubuntu.com
 * Management:     https://landscape.canonical.com
 * Support:        https://ubuntu.com/pro

This system has been minimized by removing packages and content that are
not required on a system that users do not log into.

To restore this content, you can run the 'unminimize' command.

The programs included with the Ubuntu system are free software;
the exact distribution terms for each program are described in the
individual files in /usr/share/doc/*/copyright.

Ubuntu comes with ABSOLUTELY NO WARRANTY, to the extent permitted by
applicable law.

cubic@quickstart:~$
```

Use `cubic images` to list all supported images.

# :dizzy: How to install Cubic?

**Linux** (Snap) and **Windows** (WSL2):
```
sudo snap install cubic
sudo snap connect cubic:kvm
```

**macOS** (homebrew)
```
brew install cubic-vm/cubic/cubic
```

**Cargo** (requires *qemu-system-x86*, *qemu-system-arm* and *qemu-img* on the host)
```
rustup toolchain add 1.92.0
cargo install cubic
```

See the [install](https://cubic-vm.org/install.html) instructions for more information.

# :bulb: How to use Cubic?

Cubic has a simple CLI:
```
$ cubic --help
Cubic is a lightweight command line manager for virtual machines. It has a
simple, daemonless and rootless design. All Cubic virtual machines run isolated
in the user context. Cubic is built on top of QEMU, KVM and cloud-init.

Examples:

  Create a new VM instance with:
  $ cubic create example --image ubuntu:noble
  Open a shell in the VM instance:
  $ cubic ssh example

  Alternatively, use `run` to execute the above commands in a single command:
  $ cubic run example --image ubuntu:noble

  Show all supported VM images:
  $ cubic images

  List previously created VM instances:
  $ cubic instances

  Show information about a VM instance:
  $ cubic show <instance>

  Execute a command in a VM instance:
  $ cubic exec <instance> <shell command>

  Transfer files and directories between host and VM instance:
  $ cubic scp <path/to/host/file> <instance>:<path/to/guest/file>
  See `cubic scp --help` for more examples

For more information, visit: https://cubic-vm.org/
The source code is located at: https://github.com/cubic-vm/cubic


Usage: cubic [OPTIONS] <COMMAND>

Commands:
  run          Create and start VM instances
  create       Create VM instances
  instances    List VM instances
  images       List VM images
  ports        List ports for VM instances
  show         Show VM images and instances
  modify       Modify VM instances
  console      Open VM instance console
  ssh          Connect to VM instances
  scp          Copy data between host and VM instances
  exec         Execute commands on VM instances
  start        Start VM instances
  stop         Stop VM instances
  restart      Restart VM instances
  rename       Rename VM instances
  clone        Clone VM instances
  delete       Delete VM instances
  prune        Clear caches
  completions  Generate shell completion scripts

Options:
  -v, --verbose  Increase logging output
  -q, --quiet    Reduce logging output
  -h, --help     Print help
  -V, --version  Print version
```

# :hammer: How to Build Cubic from Source?

See [CONTRIBUTING.md](CONTRIBUTING.md) for instructions on setting up a development
environment and building the project.

# :speech_balloon: How to contribute to Cubic?

We are actively looking for help to improve Cubic. You can help in various ways:

- :girl: Increase Cubic's user base by installing and using it!
- :star: Star us on [Github](https://github.com/cubic-vm/cubic) to promote the project!
- :beetle: If you found a bug or you are interested in a feature, please create an [issue on Github](https://github.com/cubic-vm/cubic/issues)!
- :construction_worker: If you are a developer and you want to submit a change, please have a look at the [contribution page](CONTRIBUTING.md)!
- :pencil: If you are a technical writer and you want to improve the documentation, please have a look at the [contribution page](CONTRIBUTING.md)!

# :page_with_curl: License

Cubic is dual-licensed under [Apache](LICENSE-APACHE) and [MIT](LICENSE-MIT).

# :balance_scale: How does Cubic compare to other tools?

- **[Multipass](https://multipass.run)** — developed by Canonical and released under GPL-3.0,
  Multipass is a CLI VM manager similar to Cubic in concept, but backed by a privileged
  background daemon and focused primarily on Ubuntu images. Cubic is daemonless, supports a
  wider range of Linux distributions, and is released under a permissive license.

- **[Vagrant](https://www.vagrantup.com)** — developed by HashiCorp and distributed under
  the Business Source License, Vagrant provisions VMs through a pluggable hypervisor such as
  VirtualBox or VMware and uses community-maintained boxes rather than official vendor images.
  Its declarative Vagrantfile format is well suited for teams sharing reproducible
  environments. Cubic skips the extra toolchain and boots directly from official cloud images
  using QEMU.

- **[VirtualBox](https://www.virtualbox.org)** — developed by Oracle and released under
  GPL-2.0, VirtualBox is a graphical VM manager that requires a kernel module and elevated
  privileges to install. It supports a broad range of guest operating systems including
  Windows and macOS. Cubic trades the GUI for a lightweight CLI, requires no kernel module,
  and runs entirely as an unprivileged user.

- **[Docker](https://www.docker.com)** — developed by Docker Inc. and released under
  Apache-2.0, Docker runs Linux containers that share the host kernel rather than full VMs,
  making it fast and well suited for packaging and deploying applications. The Docker daemon
  runs as root by default and Docker Desktop requires a commercial license for larger
  organisations. Cubic provides stronger isolation through full VMs and is rootless and
  daemonless by design.

- **[Podman](https://podman.io)** — a community project backed by Red Hat and released under
  Apache-2.0, Podman is rootless and daemonless like Cubic and a popular drop-in replacement
  for Docker. The key difference is the isolation model: Podman runs containers that share
  the host kernel, while Cubic runs full VMs with a separate kernel per instance.

- **[UTM](https://mac.getutm.app)** — a community-driven project released under MIT, UTM is
  a graphical QEMU-based VM manager for macOS and iOS that can also emulate foreign CPU
  architectures such as x86 on Apple Silicon. Cubic and UTM share the same QEMU foundation,
  but Cubic is a cross-platform CLI tool that automates provisioning through cloud-init
  rather than relying on manual guest installation.
