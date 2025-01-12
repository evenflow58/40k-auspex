import { HttpClient, HttpHeaders } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { environment } from 'src/environments/environment';
import { AuthStateService } from '../../authState/auth-state.service';

@Injectable({
  providedIn: 'root'
})
export class BaseHttpService {
  private baseUrl = environment.apiUrl;
  private idToken = '';

  constructor(
    private http: HttpClient,
    protected authState: AuthStateService
  ) { 
    authState.user.subscribe(async user => {
      this.idToken = user?.idToken || '';
    });
  }

  private getRequestOptions = () => 
  ({
    headers: new HttpHeaders({
      'Content-Type': 'application/json',
      'Accept': 'application/json',
      'Access-Control-Allow-Headers': '*',
      'Access-Control-Allow-Origin': '*',
      'Authorization': `${this.idToken}`
    })
  })

  public get = <T>(url: string) =>
    this.http
      .get<T>(
        `${this.baseUrl}${url}`,
        this.getRequestOptions()
      );

  public put = <T>(url: string, body: any) =>
    this.http
      .put<T>(
        `${this.baseUrl}${url}`,
        body,
        this.getRequestOptions()
      );

  public post = <T>(url: string, body: any) =>
    this.http
      .post<T>(
        `${this.baseUrl}${url}`,
        body,
        this.getRequestOptions()
      );

  public delete = <T>(url: string) =>
    this.http
      .delete<T>(
        `${this.baseUrl}${url}`,
        this.getRequestOptions()
      );
}
