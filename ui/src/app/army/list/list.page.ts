import { Component, OnInit } from '@angular/core';
import { Router, ActivatedRoute } from '@angular/router';
import { Observable, of, tap, finalize, switchMap } from 'rxjs';
import { LoadingController } from '@ionic/angular';
import { ArmyService } from '../../services/api';

@Component({
  selector: 'army-list',
  templateUrl: './list.page.html',
  styleUrls: ['./list.page.scss'],
})
export class ListPage implements OnInit {
  armies$!: Observable<Array<{ name: string, factions: Array<string> }>>;
  isArmyListOpen: boolean = false;
  isFactionListOpen: boolean = false;
  areListsOpen = () => !(this.isArmyListOpen || this.isFactionListOpen);
  factions: Array<string> = new Array<string>();
  private loader: HTMLIonLoadingElement | undefined;
  private selectedArmyName: string = '';

  constructor(
    private router: Router,
    private activatedRoute: ActivatedRoute,
    private armyService: ArmyService,
    private loadingController: LoadingController,
  ) { }

  ngOnInit(): void {
    this.armies$ = of(null).pipe(
      tap(() => this.showLoading()),
      switchMap(() => this.armyService.getArmies()),
      finalize(() => this.loader?.dismiss()),
    )
  }

  private async showLoading() {
    this.loader = await this.loadingController.create({ message: 'Loading...' });
    this.loader.present();
  }

  public navigateToEdit() {
    this.router.navigate(
      ['../edit'],
      { relativeTo: this.activatedRoute }
    );
  }
}
