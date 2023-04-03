This is my implmentation of the
[Code Crafters "Build Your Own Redis" Challenge](https://codecrafters.io/challenges/redis).

It's a toy Redis clone that's capable of handling
basic commands like `PING`, `ECHO`, `SET` (with expiration) and `GET`.
I implemented a thread pool for handling concurrent requests,
used a hash-map for the in-memory store.
