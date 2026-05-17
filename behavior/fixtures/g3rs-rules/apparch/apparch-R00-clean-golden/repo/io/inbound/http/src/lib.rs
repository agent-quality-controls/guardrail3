use types_core::OrderDto;

pub fn handle(input: OrderDto) {
    logic_service::process(input);
}
