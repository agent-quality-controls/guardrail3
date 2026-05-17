import { getUser } from "../../logic/get_user";

export interface DbPort {
  save(): void;
}

export const save = (_user: { id: string }) => getUser();
