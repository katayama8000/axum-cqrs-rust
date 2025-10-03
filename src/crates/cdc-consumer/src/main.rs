use rdkafka::consumer::{BaseConsumer, Consumer};
use rdkafka::config::ClientConfig;
use redis::Commands;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct DebeziumPayload {
    before: Option<serde_json::Value>,
    after: Option<serde_json::Value>,
    op: String,
}

#[derive(Deserialize, Debug)]
struct CircleSnapshot {
    circle_id: String,
    state: serde_json::Value,
}

fn main() {
    let kafka_brokers = "kafka:9092";
    let redis_url = "redis://redis:6379";
    let topics = ["mysql-server.mydatabase.circle_snapshots"];

    let consumer: BaseConsumer = ClientConfig::new()
        .set("bootstrap.servers", kafka_brokers)
        .set("group.id", "cdc-consumer-group")
        .create()
        .expect("Failed to create Kafka consumer");

    consumer.subscribe(&topics).expect("Failed to subscribe to topics");

    let mut redis_conn = redis::Client::open(redis_url)
        .expect("Failed to create Redis client")
        .get_connection()
        .expect("Failed to get Redis connection");

    println!("Starting CDC consumer...");

    loop {
        for msg in consumer.iter() {
            match msg {
                Ok(msg) => {
                    if let Some(payload) = msg.payload() {
                        let debezium_payload: Result<DebeziumPayload, _> = serde_json::from_slice(payload);
                        match debezium_payload {
                            Ok(debezium_payload) => {
                                if let Some(after) = debezium_payload.after {
                                    let snapshot: Result<CircleSnapshot, _> = serde_json::from_value(after);
                                    match snapshot {
                                        Ok(snapshot) => {
                                            let circle_id = snapshot.circle_id;
                                            let state = snapshot.state.to_string();
                                            println!("Updating Redis for circle_id: {}", circle_id);
                                            let _: () = redis_conn.set(format!("circle:{}", circle_id), state).expect("Failed to set Redis key");
                                        }
                                        Err(e) => eprintln!("Failed to deserialize snapshot: {:?}", e),
                                    }
                                }
                            }
                            Err(e) => eprintln!("Failed to deserialize Debezium payload: {:?}", e),
                        }
                    }
                }
                Err(e) => eprintln!("Kafka error: {:?}", e),
            }
        }
    }
}
