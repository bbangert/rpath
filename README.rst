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

    $ export PATH=`rpath rm 8`
    $ rpath list
    0: /home/ben/go/bin
    1: /usr/local/haskell/ghc-7.8.3-x86_64/bin
    2: /usr/local/sbin
    3: /usr/local/bin
    4: /usr/sbin
    5: /usr/bin
    6: /sbin
    7: /bin
    8: /usr/local/games

Building
========

Rust 1.0 and cargo should be installed, then checkout this project and:

.. code-block:: bash

    $ cargo build --release

The ``rpath`` binary will then be under the ``target/release`` directory.
