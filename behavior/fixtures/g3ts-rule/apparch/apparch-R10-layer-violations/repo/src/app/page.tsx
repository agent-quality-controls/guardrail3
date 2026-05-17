import { save } from "../io/outbound/db";

export default function Page() {
  save({ id: "1" });
  return <div />;
}
