=========
Ophiuchus
=========

An implementation of the Stellar Consensus Protocol (SCP) [MAZIERES]_.

``simple-scp``
--------------

A simple and network-less implementation of the SCP protocol on a single
slot.

Roadmap
-------

* implement and test ``simple-scp``
* implement and test ``scp-utils``, a real transport (probably based the noise
  framework or something similar). RPC vs gossip?
* implement and test ``ophiuchus``, a simple key-value store based on that stuff


License
-------

   Copyright (c) 2017 Peio Borthelle.

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version.

This program is distributed in the hope that it will be useful, but WITHOUT ANY
WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A
PARTICULAR PURPOSE. See the GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License along
with this program. If not, see http://www.gnu.org/licenses/.


.. [MAZIERES] David Mazières. *The Stellar Consensus Protocol: A Federated Model
   for Internet-level Consensus*. Stellar Development Foundation. 2016. `white
   paper`_.

.. _white paper: https://www.stellar.org/papers/stellar-consensus-protocol.pdf
