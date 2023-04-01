import { throwError } from "rxjs";
import { Anime } from "./anime";
import { Ost } from "./ost";
import { User } from "./user";
import { UserAnime } from "./useranime";

export interface AnimeOstResponse<T> {
    messages: Array<string>;
    data: T,
}

export function mapUsers(response: AnimeOstResponse<Array<User>>): Array<User> {
    if (response.messages.length > 0) {
      throw throwError(() => new Error(`Error getting users: ${response.messages.join(', ')}`))
    }
    return response.data
  }

export function mapAnime(response: AnimeOstResponse<Array<Anime>>): Array<Anime> {
    if (response.messages.length > 0) {
      throw throwError(() => new Error(`Error getting anime: ${response.messages.join(', ')}`))
    }
    return response.data
  }

export function mapUserAnime(response: AnimeOstResponse<Array<UserAnime>>): Array<UserAnime> {
    if (response.messages.length > 0) {
      throw throwError(() => new Error(`Error getting user anime: ${response.messages.join(', ')}`))
    }
    return response.data
  }

export function mapOst(response: AnimeOstResponse<Array<Ost>>): Array<Ost> {
    if (response.messages.length > 0) {
      throw throwError(() => new Error(`Error getting ost: ${response.messages.join(', ')}`))
    }
    return response.data
  }