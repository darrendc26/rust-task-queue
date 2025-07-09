# Rust Task Queue

A lightweight, asynchronous background job queue written in Rust, inspired by systems like Celery and Sidekiq.  
Supports retries with exponential backoff, job status tracking, and a REST API built with Axum.

---

## Features

- Submit background jobs via HTTP
- In-memory job queue (`VecDeque`)
- Async job processing with `tokio`
- Retry on failure (configurable)
- Exponential backoff delay
- Track job status by ID
- JSON payload support

---

## Job Types

The system supports dynamic task types with pluggable logic. Default examples include:

- `SendEmail` – simulate sending an email
- `ResizeImage` – simulate image processing

---

## Getting Started

### Prerequisites
- Rust (1.70+ recommended)
- Cargo
- `curl` or Postman (for testing API)

---

### 🔧 Install & Run

```bash
git clone https://github.com/darrendc26/rust-task-queue.git
cd task-queue
cargo run
