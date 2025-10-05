import pandas as pd
import json
import time
from confluent_kafka import Producer

# Kafka / Redpanda configuration
conf = {'bootstrap.servers': '127.0.0.1:9092'}
producer = Producer(conf)

def delivery_report(err, msg):
    if err is not None:
        print(f"Delivery failed: {err}")
    else:
        print(f"Delivered to {msg.topic()} [{msg.partition()}] at offset {msg.offset()}")

# Read CSV
csv_file = "trades_data.csv"
df = pd.read_csv(csv_file)

print(f"Loaded {len(df)} trades from {csv_file}")

# Produce messages
for i, row in df.iterrows():
    # Convert each row (trade) to a JSON object
    record = row.to_dict()

    # Ensure JSON serializable (e.g. convert numpy types)
    record_json = json.dumps(record, default=str)

    # Send to topic 'trade-data'
    producer.produce(
        topic="trade-data",
        value=record_json,
        callback=delivery_report
    )

    # Throttle publishing slightly
    time.sleep(0.05)
    producer.poll(0)

producer.flush()
print("\nAll trades successfully published to topic 'trade-data'!")
