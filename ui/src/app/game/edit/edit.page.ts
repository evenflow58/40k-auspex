import { Component, OnDestroy, OnInit } from '@angular/core';
import { FormControl, FormGroup, Validators } from '@angular/forms';
import { ActivatedRoute, Router } from '@angular/router';
import { ToastController } from '@ionic/angular';
import { lastValueFrom, Subscription } from 'rxjs';
import { GameService } from 'src/app/services/api/game/game.service';

@Component({
  selector: 'app-edit',
  templateUrl: './edit.page.html',
  styleUrls: ['./edit.page.scss'],
})
export class EditPage implements OnInit, OnDestroy {
  public id: string | null = null;
  public missionTypes = new Array(
    {
      name: "Fixed",
      value: "fixed"
    },
    {
      name: "Tactical",
      value: "tactical"
    },
  );
  public listForm = new FormGroup({
    name: new FormControl(`${new Date().toLocaleDateString()} Game`, Validators.required),
    size: new FormControl('incursion', Validators.required),
    player1: new FormGroup({
      name: new FormControl(''),
      missionType: new FormControl('fixed'),
      turnOrder: new FormControl('1', Validators.required),
      playerType: new FormControl('attacker', Validators.required),
    }),
    player2: new FormGroup({
      name: new FormControl(''),
      missionType: new FormControl('fixed'),
      turnOrder: new FormControl('2', Validators.required),
      playerType: new FormControl('defender', Validators.required),
    }),
  });
  public isToastOpen = false;

  private subscriptions = new Array<Subscription | undefined>;

  constructor(
    private router: Router,
    private activatedRoute: ActivatedRoute,
    private gameService: GameService,
    private toastController: ToastController,
  ) { }

  ngOnInit() {
    this.id = this.activatedRoute.snapshot.paramMap.get('id');

    this.subscriptions.concat([
      this
        .listForm
        .get("player1")
        ?.get("turnOrder")
        ?.valueChanges
        .subscribe(p1TurnOrder => 
          this
            .listForm
            .get("player2")
            ?.get("turnOrder")
            ?.setValue(p1TurnOrder === "1" ? "2" : "1", { emitEvent: false })),

      this
        .listForm
        .get("player2")
        ?.get("turnOrder")
        ?.valueChanges
        .subscribe(p1TurnOrder => 
          this
            .listForm
            .get("player1")
            ?.get("turnOrder")
            ?.setValue(p1TurnOrder === "1" ? "2" : "1", { emitEvent: false })),

      this
        .listForm
        .get("player1")
        ?.get("playerType")
        ?.valueChanges
        .subscribe(p1TurnOrder => 
          this
            .listForm
            .get("player2")
            ?.get("playerType")
            ?.setValue(p1TurnOrder === "attacker" ? "defender" : "attacker", { emitEvent: false })),
  
      this
        .listForm
        .get("player2")
        ?.get("playerType")
        ?.valueChanges
        .subscribe(p1TurnOrder => 
          this
            .listForm
            .get("player1")
            ?.get("playerType")
            ?.setValue(p1TurnOrder === "attacker" ? "defender" : "attacker", { emitEvent: false })),
    ]);
  }

  ngOnDestroy(): void {
    this.subscriptions?.forEach(subscription => subscription?.unsubscribe());
  }

  public setOpen(isOpen: boolean) {
    this.isToastOpen = isOpen;
  }

  public async createGame() {
    const { name, size, player1, player2 } = this.listForm.value;
    if (!name || !size || !player1 || !player2) throw new Error();

    const players = [player1, player2];
    const attacker = players.find(player => player.playerType === 'attacker');
    const defender = players.find(player => player.playerType === 'defender');
    const game = {
      size,
      attacker: {
        name: attacker?.name,
        turnOrder: parseInt(attacker?.turnOrder as string),
        missionType: attacker?.missionType,
        playerTypes: attacker?.playerType,
      },
      defender: {
        name: defender?.name,
        turnOrder: parseInt(defender?.turnOrder as string),
        missionType: defender?.missionType,
        playerTypes: defender?.playerType,
      },
    }
    
    try {
      const { id } = await lastValueFrom(this.gameService.createGame(name, game));
      this.router.navigate(
        [`./${id}`],
        { relativeTo: this.activatedRoute }
      );
    } catch (e) {
      console.error(e);
      this.isToastOpen = true;
    }
  }
}
