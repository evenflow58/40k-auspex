import { Injectable } from '@angular/core';
import { Observable } from 'rxjs';
import { environment } from 'src/environments/environment';
import { BaseHttpService } from '../base-http/base-http.service';

@Injectable()
export class ArmyService {
  constructor(private http: BaseHttpService) { }

  public getArmies = (): Observable<Array<{ name: string, factions: Array<string> }>> =>
    this.http.get<Array<{ name: string, factions: Array<string> }>>('army');
}
