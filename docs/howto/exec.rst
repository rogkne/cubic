.. _exec command:

Run a Command
=============

``cubic exec`` runs one command inside a VM instance and prints its output on
the host. A stopped VM instance is started first, so a single command is enough
to get an answer out of a guest.

Run a Command
-------------

.. code-block::

    $ cubic exec demo "uname -sr"
    Linux 7.0.0-30-generic

Quote the command, so the shell of the host passes it on instead of expanding
it.

Run Several Commands
--------------------

The command reaches the guest through its shell, so the usual operators work
inside the quotes:

.. code-block::

    $ cubic exec demo "sudo apt update && sudo apt install -y vim"

Check the Result
----------------

``cubic exec`` reports whether it reached the guest, not what the command
decided there. A command that fails inside the guest still leaves ``cubic exec``
successful, so ask the guest itself when a script depends on the answer:

.. code-block::

    $ cubic exec demo "test -f /etc/cubic-tutorial && echo present || echo missing"
    missing

Run a Command at Creation
-------------------------

``cubic exec`` runs whenever you like and needs a guest that already exists.
``cubic create --exec`` is the other one, a command cloud-init runs as root once
on the first boot, which is where package installs for a fresh VM instance
belong:

.. code-block::

    $ cubic create builder --image ubuntu --exec "apt install -y build-essential"

:ref:`template file` sets the same thing with the ``run`` list.

Related
-------

* :ref:`forward env vars` of the host into a VM instance
* :ref:`copy files` in and out of a VM instance
* :ref:`template file` to run commands on the first boot of every VM instance
