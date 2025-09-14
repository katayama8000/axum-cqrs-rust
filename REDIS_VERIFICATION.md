# How to Verify Redis Integration

This document describes the steps to verify that the application is correctly integrated with Redis.

## Prerequisites

- The application, database, and Redis are running via `docker-compose up`.
- `redis-cli` is available in your environment (e.g., installed in the dev container).

## Verification Steps

### 1. Connect to the Redis Container

Execute the following command in your terminal to start `redis-cli` inside the `redis` container.

```bash
docker-compose exec redis redis-cli
```

### 2. Create or Update a Circle via the API

Use a tool like `curl` to create or update a circle.

**Example: Create a circle**
```bash
curl -X POST -H "Content-Type: application/json" -d '{"circle_name": "Test Circle", "capacity": 10}' http://127.0.0.1:3000/circle
```

### 3. Check the Data in Redis

In the terminal where `redis-cli` is running, execute the following commands to verify that the data has been saved or updated correctly.

#### List All Circle Keys

Check all the keys for the stored circles.

```redis-cli
KEYS "circle:*"
```

**Example output:**
```
1) "circle:01H8X2J4Y6Z4X2J4Y6Z4X2J4Y6"
```

#### Get Information for a Specific Circle

Use the key obtained from the `KEYS` command to get the information (in JSON format) for a specific circle.

```redis-cli
GET "circle:<circle_id>"
```

**Example output:**
```
"{\"id\":\"01H8X2J4Y6Z4X2J4Y6Z4X2J4Y6\",\"name\":\"Test Circle\",\"capacity\":10,\"version\":1}"
```

#### Display the Set of Circle IDs

Check the set of circle IDs, which is managed for the list API.

```redis-cli
SMEMBERS "circles:list"
```

**Example output:**
```
1) "01H8X2J4Y6Z4X2J4Y6Z4X2J4Y6"
```

### 4. Exit Redis-cli

When you are finished, exit `redis-cli` with the following command.

```redis-cli
exit
```