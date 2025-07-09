use std::{ time::Duration};
use tokio::time::sleep;
use rand;

use crate::{
    job::{Job, JobStatus},
    queue::JobQueue,
};

pub async fn start_worker(queue: JobQueue) {
    tokio::spawn(async move {
        loop {
            if let Some( job) = queue.pop_job() {
                println!("Processing job: {}", job.id);

                let success = simulate_job(&job).await;

                // Lock jobs once, and handle all logic inside the block
                let mut should_retry = false;
                let mut backoff = 0;

                {
                    let mut jobs = queue.jobs.lock().unwrap();

                    if let Some(stored_job) = jobs.get_mut(&job.id) {
                        if success {
                            stored_job.status = JobStatus::Completed;
                            println!("Job {} completed", job.id);
                        } else {
                            stored_job.attempts += 1;
                            if stored_job.attempts < 3 {
                                stored_job.status = JobStatus::Pending;
                                should_retry = true;
                                backoff = 2_u64.pow(stored_job.attempts);
                            } else {
                                stored_job.status = JobStatus::Failed;
                                println!("Job {} failed permanently", job.id);
                            }
                        }
                    }
                } 
                
                // Done as we can't hold the lock while we sleep. So we need to drop mutex and sleep
                // Retry logic after dropping the lock
                if should_retry {
                    queue.queue.lock().unwrap().push_back(job.id);
                    println!("Retrying job {} after {}s", job.id, backoff);
                    sleep(Duration::from_secs(backoff)).await;
                }
            } else {
                // If no job is found, wait a bit before checking again
                sleep(Duration::from_millis(500)).await;
            }
        }
    });
}

async fn simulate_job(job: &Job) -> bool {
    println!("Simulating: {:?}", job.task_type);

    // Simulate 1 second of work
    sleep(Duration::from_secs(1)).await;

    // 70% success rate
    rand::random::<f32>() > 0.3
}
