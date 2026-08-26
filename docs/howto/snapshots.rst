.. _snapshots:

Snapshot and Restore a VM
=========================

A snapshot saves the disk of a virtual machine so you can go back to it later.
This is useful before an upgrade, before you try out a risky change, or to reset
a test machine to a known state.

Snapshots of the same machine share every block that did not change, so keeping
several of them stays cheap.

Create a Snapshot
-----------------

A snapshot can only be taken while the machine is stopped:

.. code-block::

    $ cubic stop example --wait
    $ cubic snapshot example/clean

A snapshot is always addressed as ``<instance>/<snapshot>``, so ``example/clean``
is the snapshot ``clean`` of the machine ``example``.

List the Snapshots
------------------

``cubic show`` lists the snapshots of a machine:

.. code-block::

    $ cubic show example
    Running:     no
    Arch:        amd64
    CPUs:        4
    Memory:      4.0 GiB
    Disk Used:   941.2 MiB
    Disk Total:  100.0 GiB
    User:        alice
    Isolated:    no
    SSH Port:    10022
    Snapshots:   clean
                 before-upgrade

Restore a Snapshot
------------------

Restoring rolls the disk back to the state it had when the snapshot was taken:

.. code-block::

    $ cubic restore example/clean

Everything written since the snapshot is lost. A running machine is stopped
first, so make sure you no longer need its current state. Add ``--yes`` to skip
the confirmation.

The settings of a machine, such as CPUs, memory and forwarded ports, are not
part of a snapshot and stay as they are.

Delete a Snapshot
-----------------

A snapshot you no longer need can be deleted without touching the machine:

.. code-block::

    $ cubic delete example/clean

Deleting the machine itself removes all of its snapshots as well:

.. code-block::

    $ cubic delete example
