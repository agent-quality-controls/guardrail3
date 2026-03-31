use worker_adapters_outbound_db::InMemoryJobStore;
use worker_adapters_outbound_sqs as _;
use worker_app_processor::process_batch;
use worker_domain_jobs::ProcessedJob;

pub fn run_poll_cycle(worker_id: &str, limit: usize) -> Vec<ProcessedJob> {
    let store = InMemoryJobStore::seeded();
    process_batch(&store, worker_id, limit)
}

#[cfg(test)]
mod tests {
    use super::run_poll_cycle;

    #[test]
    fn processes_seeded_jobs_for_active_worker() {
        let jobs = run_poll_cycle("worker-a", 10);
        assert_eq!(jobs.len(), 3);
    }
}
