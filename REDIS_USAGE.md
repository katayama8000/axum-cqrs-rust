# Redis Usage Guide

This document explains how to interact with the Redis instance within this project's devcontainer.

## Prerequisites

- You must be inside the devcontainer.
- The Redis service name is `redis`.

## Connecting to Redis

To open the Redis interactive shell, run the following command in the devcontainer's terminal:

```bash
redis-cli -h redis
```

To exit the shell, type `exit` or press `Ctrl+C`.

For running a single command without entering the interactive shell, use the following format:

```bash
redis-cli -h redis <COMMAND>
```

## Basic Commands

### List All Keys

To get a list of all keys stored in Redis:

```bash
redis-cli -h redis KEYS "*"
```

### Check a Key's Data Type

To determine the data type of a specific key:

```bash
redis-cli -h redis TYPE 66lpFVq3bRFaE8SghL155F3bVQSzHZhpblZc
```

Example:
```bash
redis-cli -h redis TYPE "circles:list"
```

## Retrieving Values by Data Type

Based on the key's data type, use the appropriate command to get its value.

### String

```bash
redis-cli -h redis GET <key_name>
```
*In this project, circle details are often stored as JSON strings.*
Example:
```bash
redis-cli -h redis GET "circle:123qwe567asf890zxc"
```

### List

```bash
redis-cli -h redis LRANGE <key_name> 0 -1
```
*The `0 -1` part means get all elements from the list.*
Example:
```bash
redis-cli -h redis LRANGE "circles:list" 0 -1
```

### Hash

```bash
redis-cli -h redis HGETALL <key_name>
```

### Set

```bash
redis-cli -h redis SMEMBERS <key_name>
```

### Sorted Set (zset)

```bash
redis-cli -h redis ZRANGE <key_name> 0 -1 WITHSCORES
```
