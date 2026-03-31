use worker_adapters_outbound_db as _;
use worker_adapters_outbound_sqs as _;
use worker_app_processor as _;
use worker_domain_jobs as _;

fn main() {
    let processed = worker_adapters_inbound_poller::run_poll_cycle("worker-a", 10);
    println!("processed {} jobs", processed.len());
}
