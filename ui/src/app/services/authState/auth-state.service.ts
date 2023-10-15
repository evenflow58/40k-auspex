import { Injectable } from '@angular/core';
import { SocialUser } from '@abacritt/angularx-social-login';
import { Observable, BehaviorSubject } from 'rxjs';

interface AuthState {
  user: SocialUser;
}

const intiialState: AuthState = {
  user: {} as SocialUser
};

@Injectable({
  providedIn: 'root'
})
export class AuthStateService {
  private _user = new BehaviorSubject<SocialUser | undefined>(undefined);
  private _isLoggedIn = new BehaviorSubject<boolean>(false);

  public get user(): Observable<SocialUser | undefined> {
    return this._user;
  }

  public get isLoggedIn(): Observable<boolean> {
    return this._isLoggedIn;
  }

  public setUser(user: SocialUser): void {
    this._user.next(user);
    this._isLoggedIn.next(!!user);
  }
}
