import { Component, OnInit } from '@angular/core';
import { FormControl, FormGroup, Validators } from '@angular/forms';
import { ActivatedRoute } from '@angular/router';
import { lastValueFrom } from 'rxjs';
import { ArmyService } from 'src/app/services/api';

@Component({
  selector: 'army-edit',
  templateUrl: './edit.page.html',
  styleUrls: ['./edit.page.scss'],
})
export class EditPage implements OnInit {
  public id: string | null = null;
  public listForm = new FormGroup({
    name: new FormControl('', [Validators.required, ]),
    listString: new FormControl('', [Validators.required]),
  });

  constructor(
    private route: ActivatedRoute,
    private armyService: ArmyService,
  ) { }

  ngOnInit() {
    this.id = this.route.snapshot.paramMap.get('id');
  }

  public async serializeList() {
    const { name, listString } = this.listForm.value;
    if (!name || !listString) throw new Error();
    await lastValueFrom(this.armyService.serializeList(name, listString));
  }
}
