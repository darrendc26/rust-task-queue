// use std::{thread::sleep, time::Duration};
use tokio:: { net::TcpListener };
// use job::{Job, JobStatus, TaskType};
use queue::JobQueue;
use worker::start_worker;
use api::create_router;
// use uuid::Uuid;
// use serde_json::json;

mod job;
mod queue;
mod worker;
mod api;

#[tokio::main]
async fn main() {
    let queue = JobQueue::new();

    start_worker(queue.clone()).await;  

    let app = create_router(queue.clone());
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    
   let listener = TcpListener::bind(&addr).await.unwrap();
axum::serve(listener, app).await.unwrap();
}