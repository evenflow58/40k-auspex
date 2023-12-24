import { Injectable } from '@angular/core';
import { Observable } from 'rxjs';
import { BaseHttpService } from '../base-http/base-http.service';

@Injectable()
export class ArmyService {
  constructor(private http: BaseHttpService) { }

  public getArmies = (): Observable<Array<{ name: string, factions: Array<string> }>> =>
    this.http.get<Array<{ name: string, factions: Array<string> }>>('army');

  public serializeList = (name: string, army: string): Observable<{ id: string }> =>
    this.http.post('army', { name, army});
}
