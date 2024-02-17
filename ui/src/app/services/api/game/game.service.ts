import { Injectable } from '@angular/core';
import { Observable, map } from 'rxjs';
import { BaseHttpService } from '../base-http/base-http.service';
import { Game, Player } from '../../../models';

@Injectable({
  providedIn: 'root'
})
export class GameService {
  constructor(private http: BaseHttpService) { }

  private mapGame = (game: any): Game => ({
    id: game.id,
    name: game.name,
    attacker: this.mapPlayer(game.attacker, 'attacker'),
    defender: this.mapPlayer(game.defender, 'defender'),
    size: game.size,
  });

  private mapPlayer = (player: any, playerType: string): Player => ({
    army: player.army,
    armyList: player.army_list,
    currentMission: player.current_mission,
    discardedMissions: player.discarded_missions,
    missionType: player.mission_type,
    id: player.id,
    name: player.name,
    turnOrder: player.turn_order.toString(),
    playerType,
  });

  private serializeGame = (game: any) => ({
    name: game.name,
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

  public getGames = (): Observable<Array<{ id: string, name: string }>> =>
    this.http.get<any>('game');

  public getGame = (id: string): Observable<Game> =>
    this.http.get<any>(`game/${id}`).pipe(map(resp => this.mapGame(resp.game)));

  public createGame = (game: any): Observable<{ id: string }> =>
    this.http.post('game', {
      game: this.serializeGame(game),
    });

  public updateGame = (id: string, name: string, game: any): Observable<{ id: string }> =>
    this.http.put(`game/${id}`, {
      name,
      game: this.serializeGame(game),
    });
}
