import { Component, OnInit } from '@angular/core';
import { ActivatedRoute, Router } from '@angular/router';
import { LoadingController } from '@ionic/angular';
import { finalize, Observable, of, switchMap, tap } from 'rxjs';
import { GameService } from 'src/app/services/api/game/game.service';
import { Game } from 'src/app/models/game';

@Component({
  selector: 'game-list',
  templateUrl: './list.page.html',
  styleUrls: ['./list.page.scss'],
})
export class ListPage implements OnInit {
  public games$!: Observable<Array<Game>>;
  private loader: HTMLIonLoadingElement | undefined;

  constructor(
    private router: Router,
    private activatedRoute: ActivatedRoute,
    private gameService: GameService,
    private loadingController: LoadingController,
  ) { }

  ngOnInit(): void {
    this.games$ = of(null).pipe(
      tap(() => this.showLoading()),
      switchMap(() => this.gameService.getGames()),
      finalize(() => this.loader?.dismiss()),
    )
  }

  private async showLoading() {
    this.loader = await this.loadingController.create({ message: 'Loading...' });
    this.loader.present();
  }

  public navigateToEdit() {
    this.router.navigate(
      ['..', 'edit'],
      { relativeTo: this.activatedRoute }
    );
  }
}
