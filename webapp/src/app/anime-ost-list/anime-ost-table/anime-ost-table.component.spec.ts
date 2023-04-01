import { ComponentFixture, TestBed } from '@angular/core/testing';

import { AnimeOstTableComponent } from './anime-ost-table.component';

describe('AnimeOstTableComponent', () => {
  let component: AnimeOstTableComponent;
  let fixture: ComponentFixture<AnimeOstTableComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ AnimeOstTableComponent ]
    })
    .compileComponents();

    fixture = TestBed.createComponent(AnimeOstTableComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
