import { Component, OnInit } from '@angular/core';
import { SocialAuthService, SocialUser } from "@abacritt/angularx-social-login";
import { AuthStateService } from './services/authState/auth-state.service'; 

@Component({
  selector: 'app-root',
  templateUrl: 'app.component.html',
  styleUrls: ['app.component.scss'],
})
export class AppComponent implements OnInit {
  user  = this.authStateService.user;
  isLoggedIn = this.authStateService.isLoggedIn;

  constructor(
    private socialAuthService: SocialAuthService,
    private authStateService: AuthStateService) { }

  ngOnInit() {
    this.socialAuthService.authState.subscribe((user: SocialUser) => {
      this.authStateService.setUser(user);
      console.log(user?.idToken);
    });
  }
}
