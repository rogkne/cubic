.. _create vm:

Getting Started
===============

Select a Linux Distribution
---------------------------

Cubic can create a virtual machine for you from a common Linux distribution.
First of all, you need to decide which distribution you want to use.
You can list all supported distributions with:

.. code-block::

    $ cubic images
    Name                Tags                       Arch         Size   Cached
    almalinux:10        stable, latest             amd64   547.0 MiB       no
    archlinux:rolling   stable, latest             amd64   530.8 MiB       no
    debian:13           trixie, stable, latest     amd64   413.6 MiB       no
    fedora:44           stable, latest             amd64   556.7 MiB       no
    gentoo:rolling      stable, latest             amd64     1.6 GiB       no
    opensuse:15.6       stable, latest             amd64   694.9 MiB       no
    rockylinux:10       stable, latest             amd64   519.8 MiB       no
    ubuntu:26.04        resolute, stable, latest   amd64   407.8 MiB       no
    [...]

The list above is shortened to one release per distribution.
Run the command to see every image.
By default Cubic only lists images for the architecture of your host.
Add ``--all`` to see the other architectures too.

Use the name of a row or swap its version for one of its tags, so
``ubuntu:26.04``, ``ubuntu:resolute`` and ``ubuntu:latest`` are the same image.
Every distribution carries the same two tags.
The tag ``latest`` is the newest release.
The tag ``stable`` is the newest long term release, so the last LTS for Ubuntu
and the current stable for Debian.
A rolling release such as ``archlinux:rolling`` has no version and uses the same
image for both.
A plain distribution name is a shortcut for ``stable``, so ``--image ubuntu``
gives you the last LTS.
Both tags keep working release after release, so you never have to edit a
version into your scripts.

Create the Virtual Machine
--------------------------

In the next step you can create the virtual machine with a single command:

.. code-block::

    $ cubic create example --image debian

List all Virtual Machines
-------------------------

You can list all your virtual machines with the following command:

.. code-block::

    $ cubic instances
    Name      Arch    CPUs    Memory   Disk Used   Disk Total   Running
    example   amd64      4   4.0 GiB   941.2 MiB    100.0 GiB        no

The machine is created but not started yet, so ``Running`` is ``no``.
Add ``--all`` to see the process id of each running machine.
Cubic picks the number of CPUs and the memory size from the resources of your
host, so your values can differ.

Additional Settings
-------------------

You can also override the default virtual machine settings and define the number of CPUs, the memory size and disk size.
For example to create a virtual machine with 4 CPU cores, 4 GiB of memory and 10 GiB of disk storage use:

.. code-block::

    $ cubic create example --image debian --cpus 4 --memory 4G --disk 10G


Start the virtual machine
-------------------------

Once the virtual machine was created you can start it with:

.. code-block::

    $ cubic start example

Connect with SSH
----------------

When the virtual machine is running you can simply connect over SSH by:

.. code-block::

    $ cubic ssh example
