import { Injectable } from '@angular/core';
import { ComponentStore } from '@ngrx/component-store'
import { Observable } from 'rxjs';
import { Phase } from './state.enum';
import { State } from './state.model';
import { Game } from '../../../models/game'

@Injectable()
export class TrackStateService extends ComponentStore<State> {

  constructor() { 
    super({
      phase: Phase.START
    })
  }

  readonly phase$: Observable<Phase> = this.select(state => state.phase);

  readonly setPhase = this.updater((state: State, phase: Phase) => ({
    ...state,
    phase,
  }));

  readonly setGame = this.updater((state: State, game: Game) => ({
    ...state,
    game
  }))
}
