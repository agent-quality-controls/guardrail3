import { handle } from "../io/inbound/http";

export default function Page() {
  return <div>{handle().id}</div>;
}
