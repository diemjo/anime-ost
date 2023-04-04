import { ComponentFixture, TestBed } from '@angular/core/testing';

import { AnimeOstModalComponent } from './anime-ost-modal.component';

describe('AnimeOstModalComponent', () => {
  let component: AnimeOstModalComponent;
  let fixture: ComponentFixture<AnimeOstModalComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ AnimeOstModalComponent ]
    })
    .compileComponents();

    fixture = TestBed.createComponent(AnimeOstModalComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
