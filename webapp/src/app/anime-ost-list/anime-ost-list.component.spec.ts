import { ComponentFixture, TestBed } from '@angular/core/testing';

import { AnimeOstListComponent } from './anime-ost-list.component';

describe('AnimeOstListComponent', () => {
  let component: AnimeOstListComponent;
  let fixture: ComponentFixture<AnimeOstListComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ AnimeOstListComponent ]
    })
    .compileComponents();

    fixture = TestBed.createComponent(AnimeOstListComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
