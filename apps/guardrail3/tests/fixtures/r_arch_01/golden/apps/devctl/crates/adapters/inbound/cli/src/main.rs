use devctl_adapters_outbound_fs as _;
use devctl_app_core as _;

fn main() {
    println!("{}", devctl_adapters_inbound_cli::render_doctor_summary("."));
}
