import { Component, ElementRef, Input } from '@angular/core';
import { ModalService } from 'src/app/services/modal.service';

@Component({
  selector: 'app-anime-ost-modal',
  templateUrl: './anime-ost-modal.component.html',
  styleUrls: ['./anime-ost-modal.component.css']
})
export class AnimeOstModalComponent {
  @Input()
  id: string | undefined;

  isOpen: boolean = false;
  arg: any;

  private element: any;

  constructor (private modalService: ModalService, private el: ElementRef) {
    this.element = el.nativeElement;
  }

  ngOnInit() {
    // console.log(`${this.id} INIT`);
    // add self (this modal instance) to the modal service so it can be opened from any component
    this.modalService.add(this);

    // move element to bottom of page (just before </body>) so it can be displayed above everything else
    document.body.appendChild(this.element);

    // close modal on background click
    this.element.addEventListener('click', (el: any) => {
        if (el.target.className === 'anime-ost-modal') {
            this.close();
        }
    });
  }

  ngOnDestroy() {
    // console.log(`${this.id} DESTROY`);
    // remove self from modal service
    this.modalService.remove(this);

    // remove modal element from html
    this.element.remove();
  }

  open() {
    this.element.style.display = 'block';
    document.body.classList.add('anime-ost-modal-open');
    this.isOpen = true;
    console.log(`Openend modal: ${this.id}`);
  }

  close() {
    this.element.style.display = 'none';
    document.body.classList.remove('anime-ost-modal-open');
    this.isOpen = false;
    console.log(`Closed modal: ${this.id}`);
  }
}
