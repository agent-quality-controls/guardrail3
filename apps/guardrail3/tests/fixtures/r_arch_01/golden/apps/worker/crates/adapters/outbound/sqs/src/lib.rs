use std::cell::RefCell;

use worker_domain_jobs::{Job, JobId, ProcessedJob};
use worker_ports_outbound_queue::JobQueue;

#[derive(Default)]
pub struct MemoryBackedQueue {
    jobs: RefCell<Vec<Job>>,
    dead_lettered_ids: RefCell<Vec<String>>,
}

impl MemoryBackedQueue {
    pub fn with_jobs(jobs: Vec<Job>) -> Self {
        Self {
            jobs: RefCell::new(jobs),
            dead_lettered_ids: RefCell::new(Vec::new()),
        }
    }

    pub fn dead_lettered_ids(&self) -> Vec<String> {
        self.dead_lettered_ids.borrow().clone()
    }
}

impl JobQueue for MemoryBackedQueue {
    fn pull_batch(&self, _worker_id: &str, limit: usize) -> Vec<Job> {
        self.jobs.borrow().iter().take(limit).cloned().collect()
    }

    fn acknowledge(&self, job_id: &JobId) {
        self.jobs
            .borrow_mut()
            .retain(|job| job.id.as_str() != job_id.as_str());
    }

    fn requeue(&self, job_id: &JobId, attempts: u8) {
        let mut jobs = self.jobs.borrow_mut();
        for job in jobs.iter_mut() {
            if job.id.as_str() == job_id.as_str() {
                job.attempts = attempts;
                job.status = worker_domain_jobs::JobStatus::Retrying;
            }
        }
    }

    fn dead_letter(&self, processed: &ProcessedJob) {
        self.dead_lettered_ids
            .borrow_mut()
            .push(processed.id.as_str().to_owned());
    }
}
