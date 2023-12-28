import { Component, Input, OnInit } from '@angular/core';
import { AbstractControl, ControlContainer } from '@angular/forms';

@Component({
  selector: 'player-form',
  templateUrl: './player-form.component.html',
  styleUrls: ['./player-form.component.scss'],
})
export class PlayerFormComponent implements OnInit {
  constructor(public controlContainer: ControlContainer) { }

  @Input() placeholder = '';
  @Input() missionTypes = new Array();

  public playerForm: any | null = null;

  ngOnInit(): void {
    this.playerForm = this.controlContainer.control;
  }
}
