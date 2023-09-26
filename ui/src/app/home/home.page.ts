import { Component } from '@angular/core';
import { AuthStateService } from '../services/authState/auth-state.service';

@Component({
  selector: 'app-home',
  templateUrl: 'home.page.html',
  styleUrls: ['home.page.scss'],
})
export class HomePage {
  user = this.authStateService.user;
  isLoggedIn = this.authStateService.isLoggedIn;

  constructor(public authStateService: AuthStateService) {}
}
