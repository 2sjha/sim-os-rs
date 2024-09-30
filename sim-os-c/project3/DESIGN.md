# Design for Priner Manager & Printer Semaphores Synchronization

## Printer Server

`CID = 0` computer instance acts the Printer Manager, which listens on a socket server, also initializes a bounded buffer(& its semaphores) for the socket connections it will recieve.

`CID > 0` computer instances will act as clients, as they start up they connect to the printer server, thus Printer must be started beforehand.

When the printer receives a new connection, The new connection is put in the bounded buffer, and communicators read the connection from the same.
Bounded Buffer semaphore design taken from the lecture slides.

## Communicators & Paper printer

Each communicator also has its own bounded buffer(msg_q & related semaphores) for the messages it will receive in the socket connection.
The simulated paper printer thread also has access to the sync_pc semaphore.

The design for the communicators and printer is as follows:

- Communicators: wait(msg_qfull); wait(msg_qmutex); Put msg in its msg_q & msg_count++ signal(msg_qmutex); signal(&sync_pc);
- Printer: wait(sync_pc); check each communicator for msg_count > 0; wait(msg_qmutex); Sequentially Read msg from msg_q & msg_count=0 signal(msg_qfull); signal(msg_qmutex);
