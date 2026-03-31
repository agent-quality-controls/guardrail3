use worker_domain_jobs::{Job, JobId, ProcessedJob};

pub trait JobQueue {
    fn pull_batch(&self, worker_id: &str, limit: usize) -> Vec<Job>;
    fn acknowledge(&self, job_id: &JobId);
    fn requeue(&self, job_id: &JobId, attempts: u8);
    fn dead_letter(&self, processed: &ProcessedJob);
}
