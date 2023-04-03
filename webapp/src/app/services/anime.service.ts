import { HttpClient, HttpHeaders } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { catchError, map, Observable, of, tap } from 'rxjs';
import { Anime } from '../models/anime';
import { AnimeOstResponse, mapAnime, mapUserAnime } from '../models/response';
import { UserAnime } from '../models/useranime';

@Injectable({
  providedIn: 'root'
})
export class AnimeService {
  private animeUrl = '/api/anime';
  private userAnimeUrl = '/api/useranime';

  constructor(
    private http: HttpClient,
  ) { }

  getAnime(): Observable<Array<Anime>> {
    return this.http.get<AnimeOstResponse<Array<Anime>>>(this.animeUrl)
      .pipe(
        map(resp => mapAnime(resp)),
        tap(_ => this.log('fetched anime')),
        catchError(this.handleError<Array<Anime>>('getAnime', []))
      );
  }

  getUserAnime(): Observable<Array<UserAnime>> {
    return this.http.get<AnimeOstResponse<Array<UserAnime>>>(this.userAnimeUrl)
      .pipe(
        map(resp => mapUserAnime(resp)),
        tap(_ => this.log('fetched user anime')),
        catchError(this.handleError<Array<UserAnime>>('getUserAnime', []))
      );
  }

  private handleError<T>(operation = 'operation', result?: T) {
    return (error: any): Observable<T> => {
      this.log(`${operation} failed: ${error.message}`);
      return of(result as T);
    }
  }

  private log(message: string) {
    console.log(message);
  }
}
