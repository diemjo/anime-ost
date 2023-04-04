import { HttpClient, HttpHeaders } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { catchError, map, Observable, of, startWith, Subject, switchMap, tap } from 'rxjs';
import { Anime } from '../models/anime';
import { AnimeOstResponse, mapAnime, mapUserAnime } from '../models/response';
import { UserAnime } from '../models/useranime';

@Injectable({
  providedIn: 'root'
})
export class AnimeService {
  private animeUrl = '/api/anime';
  private userAnimeUrl = '/api/useranime';

  private animeUpdated$ = new Subject<void>();
  private anime$: Observable<Array<Anime>>;

  private userAnimeUpdated$ = new Subject<void>();
  private userAnime$: Observable<Array<UserAnime>>;

  constructor(
    private http: HttpClient,
  ) {
    this.anime$ = this.animeUpdated$.pipe(
      startWith({}),
      switchMap(() => this.loadAnime())
    );
    this.userAnime$ = this.userAnimeUpdated$.pipe(
      startWith({}),
      switchMap(() => this.loadUserAnime())
    );
  }

  onAnime(): Observable<Array<Anime>> {
    return this.anime$;
  }

  private loadAnime(): Observable<Array<Anime>> {
    return this.http.get<AnimeOstResponse<Array<Anime>>>(this.animeUrl)
      .pipe(
        map(resp => mapAnime(resp)),
        tap(_ => this.log('fetched anime')),
        catchError(this.handleError<Array<Anime>>('getAnime', []))
      );
  }

  onUserAnime(): Observable<Array<UserAnime>> {
    return this.userAnime$;
  }

  private loadUserAnime(): Observable<Array<UserAnime>> {
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
