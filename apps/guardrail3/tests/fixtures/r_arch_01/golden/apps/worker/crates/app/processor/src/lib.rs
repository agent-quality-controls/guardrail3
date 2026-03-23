use worker_domain_jobs::{Job, JobDisposition, JobKind, JobStatus, ProcessedJob};
use worker_ports_outbound_queue::JobQueue;

pub fn process_batch(queue: &impl JobQueue, worker_id: &str, limit: usize) -> Vec<ProcessedJob> {
    let jobs = queue.pull_batch(worker_id, limit);
    let mut processed = Vec::with_capacity(jobs.len());

    for job in jobs {
        let outcome = process_job(job);
        match outcome.disposition {
            JobDisposition::Acked | JobDisposition::Skipped => queue.acknowledge(&outcome.id),
            JobDisposition::Requeued => queue.requeue(&outcome.id, outcome.attempts_after_run),
            JobDisposition::DeadLetter => queue.dead_letter(&outcome),
        }
        processed.push(outcome);
    }

    processed
}

fn process_job(job: Job) -> ProcessedJob {
    if matches!(job.status, JobStatus::Completed) {
        return ProcessedJob {
            id: job.id,
            disposition: JobDisposition::Skipped,
            attempts_after_run: job.attempts,
        };
    }

    let next_attempt = job.attempts.saturating_add(1);
    let should_fail_soft = matches!(job.kind, JobKind::NotificationFanout) && next_attempt == 1;
    let disposition = if should_fail_soft {
        JobDisposition::Requeued
    } else if next_attempt >= job.max_attempts {
        JobDisposition::DeadLetter
    } else {
        JobDisposition::Acked
    };

    ProcessedJob {
        id: job.id,
        disposition,
        attempts_after_run: next_attempt,
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use worker_domain_jobs::{Job, JobId, JobKind, JobStatus, ProcessedJob};
    use worker_ports_outbound_queue::JobQueue;

    use super::process_batch;

    #[derive(Default)]
    struct FakeQueue {
        jobs: RefCell<Vec<Job>>,
        acked: RefCell<Vec<String>>,
        requeued: RefCell<Vec<(String, u8)>>,
        dead_lettered: RefCell<Vec<String>>,
    }

    impl JobQueue for FakeQueue {
        fn pull_batch(&self, _worker_id: &str, limit: usize) -> Vec<Job> {
            self.jobs.borrow().iter().take(limit).cloned().collect()
        }

        fn acknowledge(&self, job_id: &JobId) {
            self.acked.borrow_mut().push(job_id.as_str().to_owned());
        }

        fn requeue(&self, job_id: &JobId, attempts: u8) {
            self.requeued
                .borrow_mut()
                .push((job_id.as_str().to_owned(), attempts));
        }

        fn dead_letter(&self, processed: &ProcessedJob) {
            self.dead_lettered
                .borrow_mut()
                .push(processed.id.as_str().to_owned());
        }
    }

    #[test]
    fn requeues_first_notification_attempt() {
        let queue = FakeQueue {
            jobs: RefCell::new(vec![
                Job::new(
                    JobId::new("job_1").expect("valid id"),
                    "acme",
                    JobKind::NotificationFanout,
                    "fanout:daily-brief",
                )
                .expect("valid job"),
            ]),
            ..FakeQueue::default()
        };

        let processed = process_batch(&queue, "worker-a", 5);

        assert_eq!(processed[0].attempts_after_run, 1);
        assert_eq!(queue.requeued.borrow().as_slice(), &[("job_1".to_owned(), 1)]);
    }

    #[test]
    fn skips_jobs_that_already_finished() {
        let mut finished_job = Job::new(
            JobId::new("job_2").expect("valid id"),
            "acme",
            JobKind::BillingDigest,
            "billing:week-12",
        )
        .expect("valid job");
        finished_job.status = JobStatus::Completed;

        let queue = FakeQueue {
            jobs: RefCell::new(vec![finished_job]),
            ..FakeQueue::default()
        };

        let processed = process_batch(&queue, "worker-a", 5);

        assert_eq!(processed[0].disposition, worker_domain_jobs::JobDisposition::Skipped);
        assert_eq!(queue.acked.borrow().as_slice(), &["job_2".to_owned()]);
    }
}
