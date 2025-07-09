use axum::{ extract::{ Path, State },
 http::StatusCode,
 response::{IntoResponse }, 
 routing::{get, post }, 
 Json, Router };
 use serde::{ Deserialize, Serialize };
//  use std::sync::Arc;
 use uuid::Uuid;
 use crate:: { job::{ Job, JobStatus, TaskType },
    queue::JobQueue };

#[derive(Serialize, Deserialize)]
pub struct CreateJobRequest {
    task_type : TaskType,
    payload : serde_json::Value,
}

#[derive(Serialize, Deserialize)]
pub struct JobResultResponse {
    id : Uuid,
    status : JobStatus,
    attempts : u32,
}  

pub fn create_router(queue: JobQueue) -> Router {
    Router::new()
        .route("/jobs", post(create_job))
        .route("/jobs/{id}", get(get_job)).with_state(queue)
}

pub async fn create_job(
    State(queue): State<JobQueue>,
    Json(request): Json<CreateJobRequest>,
) -> impl IntoResponse {
    let job = Job {
        id: Uuid::new_v4(),
        task_type: request.task_type,
        status: JobStatus::Pending,
        payload: request.payload,
        attempts: 0,
    };
    let id = job.id;
    queue.push_job(job);
    (
        StatusCode::CREATED,
        Json(JobResultResponse {
            id,
            status: JobStatus::Pending,
            attempts: 0,
        }),
    )
}

pub async fn get_job(
    State(queue): State<JobQueue>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    if let Some(job) = queue.get_job(id) {
        let response = JobResultResponse {
            id: job.id,
            status: job.status,
            attempts: job.attempts,
        };
        (StatusCode::OK, Json(response)).into_response()
    } else {
        (StatusCode::NOT_FOUND, "Job not found").into_response()
    }
}

