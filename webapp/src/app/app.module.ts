import { NgModule } from '@angular/core';
import { BrowserModule } from '@angular/platform-browser';

import { AppRoutingModule } from './app-routing.module';
import { AppComponent } from './app.component';
import { AnimeOstListComponent } from './anime-ost-list/anime-ost-list.component';
import { AnimeOstTableComponent } from './anime-ost-list/anime-ost-table/anime-ost-table.component';
import { HttpClientModule } from '@angular/common/http';
import { AnimeOstEntryComponent } from './anime-ost-list/anime-ost-table/anime-ost-entry/anime-ost-entry.component';
import { FormsModule, ReactiveFormsModule } from '@angular/forms';

@NgModule({
  declarations: [
    AppComponent,
    AnimeOstListComponent,
    AnimeOstTableComponent,
    AnimeOstEntryComponent
  ],
  imports: [
    BrowserModule,
    AppRoutingModule,
    HttpClientModule,
    FormsModule,
    ReactiveFormsModule,
  ],
  providers: [],
  bootstrap: [AppComponent]
})
export class AppModule { }
