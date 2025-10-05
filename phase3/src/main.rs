use axum::{routing::get, Router, response::Json};
use hyper::Server; // Hyper 0.14 Server
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::Message;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::task;

type SharedRsi = Arc<Mutex<HashMap<String, Vec<Value>>>>;

#[tokio::main]
async fn main() {
    // ---------------------------
    // Shared in-memory store
    // ---------------------------
    let rsi_store: SharedRsi = Arc::new(Mutex::new(HashMap::new()));
    let rsi_store_clone = rsi_store.clone();

    // ---------------------------
    // HTTP endpoint for frontend
    // ---------------------------
    let app = Router::new().route(
        "/rsi-data",
        get(move || {
            let store = rsi_store_clone.lock().unwrap();
            async move { Json(store.clone()) } // must be async
        }),
    );

    // ---------------------------
    // Spawn HTTP server
    // ---------------------------
    task::spawn(async move {
        Server::bind(&"0.0.0.0:5000".parse().unwrap())
            .serve(app.into_make_service())
            .await
            .unwrap();
    });

    println!("Server running on http://0.0.0.0:5000");

    // ---------------------------
    // Kafka consumer
    // ---------------------------
    let consumer: StreamConsumer = rdkafka::ClientConfig::new()
        .set("bootstrap.servers", "127.0.0.1:9092")
        .set("group.id", "rsi-group")
        .set("auto.offset.reset", "earliest")
        .create()
        .expect("Consumer creation failed");

    consumer
        .subscribe(&["trade-data"])
        .expect("Can't subscribe to topic");

    // ---------------------------
    // Kafka producer
    // ---------------------------
    let producer: FutureProducer = rdkafka::ClientConfig::new()
        .set("bootstrap.servers", "127.0.0.1:9092")
        .create()
        .expect("Producer creation failed");

    println!("Listening to trade-data...");

    // ---------------------------
    // Consume trades & produce RSI
    // ---------------------------
    loop {
        match consumer.recv().await {
            Err(e) => eprintln!("Error receiving: {}", e),
            Ok(m) => {
                if let Some(payload_result) = m.payload_view::<str>() {
                    match payload_result {
                        Ok(payload) => {
                            if let Ok(trade) = serde_json::from_str::<Value>(payload) {
                                let token_address = trade["token_address"]
                                    .as_str()
                                    .unwrap_or("unknown")
                                    .to_string();

                                // -------------------
                                // Placeholder RSI
                                // Replace with 14-period calculation later
                                // -------------------
                                let fake_rsi = 50.0;

                                let rsi_record = json!({
                                    "token_address": token_address,
                                    "price_in_sol": trade["price_in_sol"],
                                    "rsi": fake_rsi,
                                    "timestamp": trade["block_time"]
                                });

                                // Produce to rsi-data topic
                                let payload_string = rsi_record.to_string();
                                let record = FutureRecord::to("rsi-data")
                                    .payload(&payload_string)
                                    .key("rsi-key");

                                match producer.send(record, Duration::from_secs(0)).await {
                                    Ok(_) => println!("Produced RSI for {}", token_address),
                                    Err((e, _)) => eprintln!("Failed to produce: {:?}", e),
                                }

                                // Store in memory
                                let mut store = rsi_store.lock().unwrap();
                                store
                                    .entry(token_address)
                                    .or_insert_with(Vec::new)
                                    .push(rsi_record);
                            }
                        }
                        Err(e) => eprintln!("Payload error: {}", e),
                    }
                }
            }
        }
    }
}
