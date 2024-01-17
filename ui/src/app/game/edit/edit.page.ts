import { Component, OnDestroy, OnInit } from '@angular/core';
import { FormControl, FormGroup, Validators } from '@angular/forms';
import { ActivatedRoute, Router } from '@angular/router';
import { ToastController } from '@ionic/angular';
import { lastValueFrom, Subscription } from 'rxjs';
import { GameService } from 'src/app/services/api/game/game.service';

@Component({
  selector: 'game-edit',
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
      name: new FormControl('Player 1'),
      missionType: new FormControl('fixed'),
      turnOrder: new FormControl('1', Validators.required),
      playerType: new FormControl('attacker', Validators.required),
      armyList: new FormControl(''),
    }),
    player2: new FormGroup({
      name: new FormControl('Player 2'),
      missionType: new FormControl('fixed'),
      turnOrder: new FormControl('2', Validators.required),
      playerType: new FormControl('defender', Validators.required),
      armyList: new FormControl(''),
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

    if (this.id) {
      this.gameService.getGame(this.id).subscribe(game => {
        this.listForm.patchValue({
          name: game.name,
          size: game.size,
          player1: {
            name: game.attacker.name,
            missionType: game.attacker.missionType,
            turnOrder: game.attacker.turnOrder,
            playerType: game.attacker.playerType,
            armyList: game.attacker.armyList,
          },
          player2: {
            name: game.defender.name,
            missionType: game.defender.missionType,
            turnOrder: game.defender.turnOrder,
            playerType: game.defender.playerType,
            armyList: game.defender.armyList,
          },
        });
      });
    }

    // Add subscriptions to allow the form to flip options when one is selected.
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

  public async saveGame() {
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
        armyList: attacker?.armyList,
      },
      defender: {
        name: defender?.name,
        turnOrder: parseInt(defender?.turnOrder as string),
        missionType: defender?.missionType,
        playerTypes: defender?.playerType,
        armyList: defender?.armyList,
      },
    }
    
    try {
      debugger;
      if (this.id) {
        const { id } = await lastValueFrom(this.gameService.createGame(name, game));
        debugger;
        this.router.navigate(
          [`./${id}`],
          { relativeTo: this.activatedRoute }
        );
      } else await lastValueFrom(this.gameService.updateGame(this.id as string, name, game));
    } catch (e) {
      console.error(e);
      this.isToastOpen = true;
    }
  }
}
