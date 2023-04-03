import { HttpClient, HttpHeaders } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { catchError, map, Observable, of, tap } from 'rxjs';
import { Ost } from '../models/ost';
import { AnimeOstResponse, mapOst } from '../models/response';

@Injectable({
  providedIn: 'root'
})
export class OstService {
  private ostUrl = '/api/ost';

  constructor(
    private http: HttpClient,
  ) { }

  getOst(): Observable<Array<Ost>> {
    return this.http.get<AnimeOstResponse<Array<Ost>>>(this.ostUrl)
      .pipe(
        map(resp => mapOst(resp)),
        tap(_ => this.log('fetched ost')),
        catchError(this.handleError<Array<Ost>>('getOst', []))
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
