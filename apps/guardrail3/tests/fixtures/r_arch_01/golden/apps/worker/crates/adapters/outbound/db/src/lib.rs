use worker_domain_jobs::{Job, JobStatus};
use worker_ports_outbound_queue::JobQueue;

#[derive(Clone, Debug)]
pub struct InMemoryJobStore {
    jobs: Vec<Job>,
}

impl InMemoryJobStore {
    pub fn seeded() -> Self {
        let mut jobs = Vec::new();

        if let Ok(job_id) = worker_domain_jobs::JobId::new("job_a") {
            if let Ok(job) = Job::new(
                job_id,
                "acme",
                worker_domain_jobs::JobKind::BillingDigest,
                "billing:digest",
            ) {
                jobs.push(job.lock_to("worker-a"));
            }
        }

        if let Ok(job_id) = worker_domain_jobs::JobId::new("job_b") {
            if let Ok(job) = Job::new(
                job_id,
                "acme",
                worker_domain_jobs::JobKind::NotificationFanout,
                "notify:weekly",
            ) {
                jobs.push(job);
            }
        }

        if let Ok(job_id) = worker_domain_jobs::JobId::new("job_c") {
            if let Ok(job) = Job::new(
                job_id,
                "beta",
                worker_domain_jobs::JobKind::ContentReindex,
                "search:backfill",
            ) {
                jobs.push(job.with_attempts(2, 3));
            }
        }

        Self { jobs }
    }
}

impl JobQueue for InMemoryJobStore {
    fn pull_batch(&self, worker_id: &str, limit: usize) -> Vec<Job> {
        self.jobs
            .iter()
            .filter(|job| {
                !matches!(job.status, JobStatus::Completed)
                    && match &job.locked_by {
                        Some(locked_by) => locked_by == worker_id,
                        None => true,
                    }
            })
            .take(limit)
            .cloned()
            .collect()
    }

    fn acknowledge(&self, _job_id: &worker_domain_jobs::JobId) {}

    fn requeue(&self, _job_id: &worker_domain_jobs::JobId, _attempts: u8) {}

    fn dead_letter(&self, _processed: &worker_domain_jobs::ProcessedJob) {}
}
