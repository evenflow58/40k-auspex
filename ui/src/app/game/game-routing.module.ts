import { NgModule } from '@angular/core';
import { Routes, RouterModule } from '@angular/router';

import { GamePage } from './game.page';

const routes: Routes = [
  {
    path: '',
    component: GamePage,
    children: [
      {
        path: '',
        redirectTo: 'list',
        pathMatch: 'full'
      },
      {
        path: 'list',
        loadChildren: () => import('./list/list.module').then(m => m.ListPageModule)
      },
      {
        path: 'edit',
        children: [
          {
            path: '',
            loadChildren: () => import('./edit/edit.module').then(m => m.EditPageModule),
          },
          {
            path: ':id',
            loadChildren: () => import('./edit/edit.module').then(m => m.EditPageModule),
          }
        ]
      }
    ]
  },
  {
    path: 'track/:id',
    loadChildren: () => import('./track/track.module').then( m => m.TrackPageModule)
  },
  {
    path: 'track',
    loadChildren: () => import('./track/track.module').then( m => m.TrackPageModule)
  },
];

@NgModule({
  imports: [RouterModule.forChild(routes)],
  exports: [RouterModule],
})
export class GamePageRoutingModule {}
