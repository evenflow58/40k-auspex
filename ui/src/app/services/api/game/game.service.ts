import { Injectable } from '@angular/core';
import { Observable } from 'rxjs';
import { BaseHttpService } from '../base-http/base-http.service';

@Injectable({
  providedIn: 'root'
})
export class GameService {
  constructor(private http: BaseHttpService) { }

  private serializeGame = (game: any) => ({
    size: game.size,
    attacker: {
      name: game.attacker.name,
      mission_type: game.attacker.missionType,
      turn_order: game.attacker.turnOrder,
      army_list: game.attacker.armyList,
    },
    defender: {
      name: game.defender.name,
      mission_type: game.defender.missionType,
      turn_order: game.defender.turnOrder,
      armyList: game.defender.armyList,
    }
  });

  public getGames = (): Observable<Array<{ id: string, name: string, date: Date }>> =>
    this.http.get<Array<{ id: string, name: string, date: Date }>>('game');

  public createGame = (name: string, game: any): Observable<{ id: string }> =>
    this.http.post('game', {
      name,
      game: this.serializeGame(game),
    });

  public updateGame = (id: string, name: string, game: any): Observable<{ id: string }> =>
    this.http.put(`game/${id}`, {
      name,
      game: this.serializeGame(game),
    });
}
