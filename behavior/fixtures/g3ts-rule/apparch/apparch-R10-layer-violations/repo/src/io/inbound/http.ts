import { save } from "../outbound/db";

export interface HttpPort {
  run(): void;
}

export function handle() {
  return save({ id: "1" });
}
