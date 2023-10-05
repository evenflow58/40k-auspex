import { ComponentFixture, TestBed } from '@angular/core/testing';
import { ArmyPage } from './army.page';

describe('ArmyPage', () => {
  let component: ArmyPage;
  let fixture: ComponentFixture<ArmyPage>;

  beforeEach(async(() => {
    fixture = TestBed.createComponent(ArmyPage);
    component = fixture.componentInstance;
    fixture.detectChanges();
  }));

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
