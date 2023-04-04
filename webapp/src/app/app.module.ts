import { NgModule } from '@angular/core';
import { BrowserModule } from '@angular/platform-browser';

import { AppRoutingModule } from './app-routing.module';
import { AppComponent } from './app.component';
import { AnimeOstListComponent } from './anime-ost-list/anime-ost-list.component';
import { AnimeOstTableComponent } from './anime-ost-list/anime-ost-table/anime-ost-table.component';
import { HttpClientModule } from '@angular/common/http';
import { AnimeOstEntryComponent } from './anime-ost-list/anime-ost-table/anime-ost-entry/anime-ost-entry.component';
import { FormsModule, ReactiveFormsModule } from '@angular/forms';
import { AnimeOstModalComponent } from './anime-ost-modal/anime-ost-modal.component';
import { OstModalComponent } from './anime-ost-list/anime-ost-table/ost-modal/ost-modal.component';

@NgModule({
  declarations: [
    AppComponent,
    AnimeOstListComponent,
    AnimeOstTableComponent,
    AnimeOstEntryComponent,
    AnimeOstModalComponent,
    OstModalComponent
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
