import { Anime } from "./anime";
import { Ost } from "./ost";
import { User } from "./user";

export interface AnimeRowData {
    anime: Anime;
    users: Array<User>;
    ost: Array<Ost>;
}