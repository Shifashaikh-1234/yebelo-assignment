This project is a **full-stack application** designed to automate quotations and analyze token trades with RSI (Relative Strength Index). It covers **data ingestion, real-time trade processing, RSI calculation, and frontend visualization**.

The project is divided into **four phases**:

1. Phase 1 – Redpanda/Kafka Setup  
2. Phase 2 – Data Ingestion  
3. Phase 3 – RSI Service (Rust Backend)  
4. Phase 4 – Next.js Frontend Dashboard  

---

## Phase 1 – Redpanda/Kafka Setup

### Objective
Set up a Kafka-compatible broker using Redpanda to handle real-time trade data streams.

### Steps
1. **Install Docker and Docker Compose** (https://www.docker.com/).  
2. **Start Redpanda broker**:

```bash
docker-compose up -d
Create Kafka topics:

bash
Copy code
rpk topic create trade-data
rpk topic create rsi-data
Notes
trade-data topic stores incoming trades.

rsi-data topic stores calculated RSI values per token.

Redpanda is used as it is fully Kafka API compatible and lightweight.

Phase 2 – Data Ingestion
Objective
Publish trade data to the trade-data topic for downstream processing.

Implementation
ingest.py reads trades_data.csv (or any CSV of trades).

Each row is converted to JSON and published to Kafka using rdkafka.

The script continuously publishes trades for testing.

Example Output
kotlin
Copy code
Published trade to trade-data: {'trade_id': 1, 'product': 'apple', 'qty': 10, 'price': 15}
Published trade to trade-data: {'trade_id': 2, 'product': 'banana', 'qty': 5, 'price': 8}
Run
bash
Copy code
python ingest.py
Phase 3 – RSI Service (Rust Backend)
Objective
Consume trades from Kafka, calculate RSI per token, produce to rsi-data, and expose via HTTP API.

Tech Stack
Rust + Tokio for async runtime.

Axum for HTTP routing.

rdkafka for Kafka consumer/producer.

Serde / serde_json for JSON handling.

In-memory storage (Arc<Mutex<HashMap<String, Vec<Value>>>>) for simplicity.

Features
Kafka Consumer: Listens to trade-data.

RSI Calculation: Placeholder 14-period RSI (50.0) stored per token.

Kafka Producer: Publishes RSI messages to rsi-data.

HTTP API: /rsi-data endpoint serves the latest RSI values for the frontend.

Run
Build & run Rust service:

bash
Copy code
cargo run
Access API:

bash
Copy code
curl http://localhost:5000/rsi-data
Example Output
json
Copy code
{
  "FCuk4XWLR6fAJFTcQoMrm3KeywSt2X6wK4Ufh4Xjpump": [
    {
      "token_address": "FCuk4XWLR6fAJFTcQoMrm3KeywSt2X6wK4Ufh4Xjpump",
      "price_in_sol": 0.0000000327,
      "rsi": 50.0,
      "timestamp": 1757782395
    }
  ]
}
Phase 4 – Next.js Frontend Dashboard
Objective
Visualize token trades and RSI in real-time.

Tech Stack
Next.js 13+ (App Router)

React 18

Chart libraries (Recharts or equivalent)

Axios / Fetch for API requests

Features
Token Selector: Choose tokens to visualize.

Price Chart: Display token price trends.

RSI Chart: Show RSI values per token.

Real-time Updates: Poll /rsi-data endpoint continuously.

Client Components: All React Hooks (useEffect, useState) run in "use client" components.

Run
Install dependencies:

bash
Copy code
npm install
Start development server:

bash
Copy code
npm run dev
Open in browser:

arduino
Copy code
http://localhost:3000
Charts update as new RSI data is produced.

