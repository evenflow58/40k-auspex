import { Component, OnInit } from '@angular/core';
import { ActivatedRoute } from '@angular/router';
import { Observable, map } from 'rxjs';
import { GameService } from 'src/app/services/api/game/game.service';
import { Phase } from './services/state.enum';
import { TrackStateService } from './services/track-state.service';

@Component({
  selector: 'app-track',
  templateUrl: './track.page.html',
  styleUrls: ['./track.page.scss'],
  providers:[TrackStateService]
})
export class TrackPage implements OnInit {
  phase$ = this.trackStateService.phase$;
  phases = Phase;

  constructor(
    private trackStateService: TrackStateService,
    private activatedRoute: ActivatedRoute,
    private gameService: GameService) { }

  ngOnInit() {
    let id = this.activatedRoute.snapshot.paramMap.get('id');

    if (id) this.gameService.getGame(id).subscribe(this.trackStateService.setGame);
    // else Not sure what to do here but something
  }

  get phase(): Observable<string> {
    return this.phase$.pipe(map(p => {
      switch (p) {
        case Phase.START:
          return 'Start';
        case Phase.COMMAND:
          return 'Command'
        case Phase.MOVEMENT:
          return 'Movement';
        case Phase.SHOOTING:
          return 'Shooting';
        case Phase.FIGHT:
          return 'Fight';
        default:
          return '';
      }
    }));    
  }
}
