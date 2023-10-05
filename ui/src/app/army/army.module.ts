import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { RouterModule } from '@angular/router';

import { IonicModule } from '@ionic/angular';

import { ArmyPageRoutingModule } from './army-routing.module';

import { ArmyPage } from './army.page';

@NgModule({
  imports: [
    CommonModule,
    FormsModule,
    IonicModule,
    ArmyPageRoutingModule,
    RouterModule
  ],
  declarations: [ArmyPage]
})
export class ArmyPageModule {}
