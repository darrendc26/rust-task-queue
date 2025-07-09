use std::collections:: { HashMap, VecDeque };
use std::sync::{ Arc, Mutex };
use uuid::Uuid;
use crate::job::{ Job, JobStatus };

#[derive(Clone)]
pub struct JobQueue {
    pub queue : Arc<Mutex<VecDeque<Uuid>>>,
    pub jobs : Arc<Mutex<HashMap<Uuid, Job>>>,
}

impl JobQueue {
    pub fn new() -> Self {
        Self{
            queue: Arc::new(Mutex::new(VecDeque::new())),
            jobs: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn push_job(&self, job: Job) {
        let id = job.id;
        self.queue.lock().unwrap().push_back(id);
        self.jobs.lock().unwrap().insert(id, job);
    }

    pub fn pop_job(&self) -> Option<Job> {
        let maybe_id = self.queue.lock().unwrap().pop_front();
        if let Some(id) = maybe_id {
            let mut jobs = self.jobs.lock().unwrap();
            if let Some(job) = jobs.get_mut(&id) {
                job.status = JobStatus::InProgress;
                return Some(job.clone());
            }
        }
        None
    }

    pub fn get_job(&self, id: Uuid) -> Option<Job> {
        let jobs = self.jobs.lock().unwrap();
        jobs.get(&id).map(|job| job.clone())
    }
}