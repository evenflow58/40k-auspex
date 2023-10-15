import { Component } from '@angular/core';
import { Army } from './models/army';

@Component({
  selector: 'army-list',
  templateUrl: './army-list.component.html',
  styleUrls: ['./army-list.component.scss'],
})
export class ArmyListComponent {
  public armies: Army[];

  constructor() {
    this.armies = [
      new Army('Grey Knights'),
    ];
  }
}
