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
cargo run --bin main
```
Or, you can use the watch script to automatically restart the server when you make changes to the code: 

```bash
./watch.sh
```

### check version to see if the server is running
```bash
curl -X GET http://127.0.0.1:8080/version
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
  http://127.0.0.1:8080/circle
```

### find
```bash
curl -X GET http://127.0.0.1:8080/circle/HlPI7rLpLP5NqHNIecdtQVwpv4kCYfDF2PrE
``` 

### find all
```bash 
curl -X GET http://127.0.0.1:8080/circle
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
  http://127.0.0.1:8080/circle/{circle_id}
```

- ref
  - https://scrapbox.io/katayama8000/axum-cqrs-rust

