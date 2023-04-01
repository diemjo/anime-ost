import { ComponentFixture, TestBed } from '@angular/core/testing';

import { AnimeOstEntryComponent } from './anime-ost-entry.component';

describe('AnimeOstEntryComponent', () => {
  let component: AnimeOstEntryComponent;
  let fixture: ComponentFixture<AnimeOstEntryComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ AnimeOstEntryComponent ]
    })
    .compileComponents();

    fixture = TestBed.createComponent(AnimeOstEntryComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
