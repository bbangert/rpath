====================================================
rpath -- a small tool to manipulate the shell's PATH
====================================================

``rpath`` is a small tool to view and manipulate the shell's ``PATH`` than the
standard view.

Running
=======


Listing the ``PATH``:

.. code-block:: bash

    $ rpath list
    Location: Path
    0: /home/ben/go/bin
    1: /usr/local/haskell/ghc-7.8.3-x86_64/bin
    2: /usr/local/sbin
    3: /usr/local/bin
    4: /usr/sbin
    5: /usr/bin
    6: /sbin
    7: /bin
    8: /usr/games
    9: /usr/local/games

Removing an element from the ``PATH``:

.. code-block:: bash

    $ export PATH=`rpath remove 8`
    $ rpath list
    Location: Path
    0: /home/ben/go/bin
    1: /usr/local/haskell/ghc-7.8.3-x86_64/bin
    2: /usr/local/sbin
    3: /usr/local/bin
    4: /usr/sbin
    5: /usr/bin
    6: /sbin
    7: /bin
    8: /usr/local/games

Adding an element to the ``PATH``:

.. code-block:: bash

    $ export PATH=`rpath insert 2 /opt/local/sbin`
    $ rpath list
    Location: Path
    0: /home/ben/go/bin
    1: /usr/local/haskell/ghc-7.8.3-x86_64/bin
    2: /opt/local/sbin
    3: /usr/local/sbin
    4: /usr/local/bin
    5: /usr/sbin
    6: /usr/bin
    7: /sbin
    8: /bin
    9: /usr/local/games

Replacing an element of the ``PATH``:

.. code-block:: bash

    $ export PATH=`rpath replace 2 /debug/local/bin`
    $ rpath list
    Location: Path
    0: /home/ben/go/bin
    1: /usr/local/haskell/ghc-7.8.3-x86_64/bin
    2: /debug/local/bin
    3: /usr/local/sbin
    4: /usr/local/bin
    5: /usr/sbin
    6: /usr/bin
    7: /sbin
    8: /bin
    9: /usr/local/games

Building
========

Rust 1.0 and cargo should be installed, then checkout this project and:

.. code-block:: bash

    $ cargo build --release

The ``rpath`` binary will then be under the ``target/release`` directory.
