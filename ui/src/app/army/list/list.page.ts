import { Component, OnInit } from '@angular/core';
import { BehaviorSubject, Observable } from 'rxjs';
import { ArmyService } from '../../services/api';

@Component({
  selector: 'app-list',
  templateUrl: './list.page.html',
  styleUrls: ['./list.page.scss'],
})
export class ListPage implements OnInit {
  armies$!: Observable<Array<string>>;

  constructor(private armyService: ArmyService) { }

  ngOnInit(): void {
    debugger;
    this.armies$ = this.armyService.getArmies();
  }
}
