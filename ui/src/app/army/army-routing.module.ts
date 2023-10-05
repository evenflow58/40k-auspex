import { NgModule } from '@angular/core';
import { Routes, RouterModule } from '@angular/router';

import { ArmyPage } from './army.page';

const routes: Routes = [
  {
    path: '',
    component: ArmyPage,
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
];

@NgModule({
  imports: [RouterModule.forChild(routes)],
  exports: [RouterModule],
})
export class ArmyPageRoutingModule { }
