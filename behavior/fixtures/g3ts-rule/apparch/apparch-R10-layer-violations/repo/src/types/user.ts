import "next/navigation";
import { getUser } from "../logic/get_user";

export function makeUser() {
  return getUser();
}
