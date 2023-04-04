import { HttpClient, HttpHeaders } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { catchError, map, Observable, of, startWith, Subject, switchMap, tap } from 'rxjs';
import { Ost } from '../models/ost';
import { AnimeOstResponse, mapOst } from '../models/response';

@Injectable({
  providedIn: 'root'
})
export class OstService {
  private ostUrl = '/api/ost';
  private ostUpdated$ = new Subject<void>();
  private ost$: Observable<Array<Ost>>;

  constructor(
    private http: HttpClient,
  ) {
    this.ost$ = this.ostUpdated$.pipe(
      startWith({}),
      switchMap(() => this.loadOst())
    );
  }

  onOst(): Observable<Array<Ost>> {
    return this.ost$;
  }

  private loadOst(): Observable<Array<Ost>> {
    return this.http.get<AnimeOstResponse<Array<Ost>>>(this.ostUrl)
      .pipe(
        map(resp => mapOst(resp)),
        tap(_ => this.log('fetched ost')),
        catchError(this.handleError<Array<Ost>>('getOst', []))
      );
  }

  postOst(ost: Ost): void {
    const httpOptions = {
      headers: new HttpHeaders({
        'content-type': 'application/json',
      })
    };
    this.http.post<Ost>(`${this.ostUrl}/${ost.proxer_id}/${ost.ost_type.toLowerCase()}/${ost.number}`, JSON.stringify(ost), httpOptions)
      .pipe(
        tap(_ => this.log('posted ost')),
        catchError(this.handleError<void>('postOst'))
      ).subscribe(() => {
        this.ostUpdated$.next();
      });
  }

  deleteOst(ost: Ost): void {
    this.http.delete<Ost>(`${this.ostUrl}/${ost.proxer_id}/${ost.ost_type.toLowerCase()}/${ost.number}`)
      .pipe(
        tap(_ => this.log('deleted ost')),
        catchError(this.handleError<void>('deleteOst'))
      ).subscribe(() => {
        this.ostUpdated$.next();
      });
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
