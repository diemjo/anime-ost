import { Injectable } from '@angular/core';
import { User } from '../models/user';
import { HttpClient, HttpHeaders } from '@angular/common/http';
import { Observable, of, throwError } from 'rxjs';
import { catchError, map, tap } from 'rxjs/operators';
import * as _ from 'lodash';
import { AnimeOstResponse, mapUsers } from '../models/response';

@Injectable({
  providedIn: 'root'
})
export class UserService {
  private usersUrl = '/api/users';

  constructor(
    private http: HttpClient,
  ) { }

  getUsers(): Observable<Array<User>> {
    const httpOptions = {
      headers: new HttpHeaders({
        'HTTP_REMOTE_USER': 'jo',
        'HTTP_REMOTE_NAME': 'joris',
        'HTTP_REMOTE_GROUPS': 'admin,yay',
        'HTTP_REMOTE_EMAIL': 'jo@example.com',
      })
    };
    return this.http.get<AnimeOstResponse<Array<User>>>(this.usersUrl, httpOptions)
      .pipe(
        map(resp => mapUsers(resp)),
        tap(users => this.log(`fetched ${(users as Array<User>).length} users`)),
        catchError(this.handleError<Array<User>>('getUsers', []))
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