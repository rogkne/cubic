.. _templates:

Create VMs from a Template
==========================

A template is a TOML file with default values for a new virtual machine. It lets
you recreate the same machine again and again without retyping a long command
line, and it is easy to share with a friend or a customer.

Write a Template
----------------

A template is a plain TOML file. Every field except ``version`` is optional and
falls back to the Cubic default when it is left out:

.. code-block::

    version = 1              # mandatory

    image = "debian"         # optional
    user = "john"            # optional
    cpus = 4                 # optional
    memory = "4G"            # optional
    disk = "100G"            # optional
    isolate = false          # optional

    ports = [                # optional
      "8000:80",             # forward guest HTTP port to host TCP port 8000
      "5353:53/udp"          # forward guest DNS port to host UDP port 5353
    ]

    run = [                  # optional, commands run once on the first boot
      "sudo apt update",
      "sudo apt full-upgrade -y",
      "sudo apt install vim"
    ]

The fields use the same form as the command line, for example ``4G`` for memory
and ``8000:80`` for a forwarded port.

The ``run`` list holds shell commands that run once inside the guest on the first
boot, so a template can install software and start services for you.

Create a VM from a Template
---------------------------

Pass the template to ``cubic create`` or ``cubic run`` with ``--template``:

.. code-block::

    $ cubic create my-instance --template ./my-template.toml

Because the template has no instance name, you can create as many machines from it
as you like:

.. code-block::

    $ cubic create web-1 --template ./my-template.toml
    $ cubic create web-2 --template ./my-template.toml

Override Template Values
------------------------

Every command line argument overrides the matching value in the template. This is
handy to reuse a template but change one detail, for example the image or the
number of CPUs:

.. code-block::

    $ cubic create my-instance --template ./my-template.toml --image ubuntu --cpus 8

A template that sets ``isolate = true`` keeps the machine off the network. Use
``--no-isolate`` to give a single machine network access anyway:

.. code-block::

    $ cubic create my-instance --template ./my-template.toml --no-isolate
