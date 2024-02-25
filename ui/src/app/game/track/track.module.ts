import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';

import { IonicModule } from '@ionic/angular';

import { TrackPageRoutingModule } from './track-routing.module';

import { TrackPage } from './track.page';
import { StartComponent } from './start/start.component';
import { CommandComponent } from './command/command.component';
import { MovementComponent } from './movement/movement.component';
import { ShootingComponent } from './shooting/shooting.component';
import { FightComponent } from './fight/fight.component';
import { ChargeComponent } from './charge/charge.component';
import { MissionSelectComponent } from './mission-select/mission-select.component';

@NgModule({
  imports: [
    CommonModule,
    FormsModule,
    IonicModule,
    TrackPageRoutingModule
  ],
  declarations: [
    TrackPage,
    StartComponent,
    CommandComponent,
    MovementComponent,
    ShootingComponent,
    ChargeComponent,
    FightComponent,
    MissionSelectComponent,
  ]
})
export class TrackPageModule {}
