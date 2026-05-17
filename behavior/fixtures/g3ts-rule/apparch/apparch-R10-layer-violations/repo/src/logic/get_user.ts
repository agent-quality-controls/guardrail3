import "react";
import { handle } from "../io/inbound/http";

export function getUser() {
  return handle();
}
