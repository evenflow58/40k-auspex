import { Component, OnInit } from '@angular/core';
import { SocialAuthService } from "@abacritt/angularx-social-login";
import { SocialUser } from "@abacritt/angularx-social-login";
import { AuthStateService } from './services/authState/auth-state.service';

@Component({
  selector: 'app-root',
  templateUrl: 'app.component.html',
  styleUrls: ['app.component.scss'],
})
export class AppComponent implements OnInit {
  constructor(
    private socialAuthService: SocialAuthService,
    private authStateService: AuthStateService) { }

  ngOnInit() {
    this.socialAuthService.authState.subscribe((user: SocialUser) => {
      debugger;
      this.authStateService.setUser(user);
    });
  }
}
