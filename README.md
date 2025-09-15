# axum-cqrs-rust

## Tech stack
- Rust
- Axum
- MariaDB
- Redis
- Kafka
- Debezium
- Docker

## Architecture
- CQRS
- Event Sourcing

## How to run
1. Run the devcontainer.
2. Then, start the server with the following command:

```bash
cargo run
```

### check version to see if the server is running
```bash
curl -X GET http://127.0.0.1:3000/version
``` 

### create 
```bash
curl -X POST \
  -H "Content-Type: application/json" \
  -d '{
        "circle_name": "music club",
        "capacity": 10,
        "owner_name": "John Lennon",
        "owner_age": 21,
        "owner_grade": 3,
        "owner_major": "Music"
      }' \
  http://127.0.0.1:3000/circle
```

### find
```bash
curl -X GET http://127.0.0.1:3000/circle/{circle_id}
``` 

### find all
```bash 
curl -X GET http://127.0.0.1:3000/circle
```

### update
```bash
curl -X PUT \
  -H "Content-Type: application/json" \
  -d '{
        "circle_name": "football club",
        "capacity": 15,
        "version": 2
      }' \
  http://127.0.0.1:3000/circle/{circle_id}
```

