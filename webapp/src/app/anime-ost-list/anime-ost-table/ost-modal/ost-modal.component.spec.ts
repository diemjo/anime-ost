import { ComponentFixture, TestBed } from '@angular/core/testing';

import { OstModalComponent } from './ost-modal.component';

describe('OstModalComponent', () => {
  let component: OstModalComponent;
  let fixture: ComponentFixture<OstModalComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ OstModalComponent ]
    })
    .compileComponents();

    fixture = TestBed.createComponent(OstModalComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
