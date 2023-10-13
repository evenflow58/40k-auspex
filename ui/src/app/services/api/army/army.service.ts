import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Observable } from 'rxjs';
import { environment } from 'src/environments/environment';

@Injectable()
export class ArmyService {
  constructor(private http: HttpClient) { }

  public getArmies = (): Observable<Array<string>> =>
    this.http.get<Array<string>>(`${environment.apiUrl}/army`);
}
