![progress-banner](https://app.codecrafters.io/progress/redis/c181b7d6-680c-4106-9996-9f3325071a27)

This is my implmentation of the
[Code Crafters "Build Your Own Redis" Challenge](https://codecrafters.io/challenges/redis).

It's a toy Redis clone that's capable of handling
basic commands like `PING`, `ECHO`, `SET` (with expiration) and `GET`.
I implemented a thread pool for handling concurrent requests,
used a hash-map for the in-memory store.
