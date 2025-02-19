# SillyDB
## What it is?

It is an abstraction over a specific database library.

## Why?
To use a single trait and to abstract over the underlying database.

It will assume that a query can be run unchanged. If your queries don't satisfy this assumption, use an ORM