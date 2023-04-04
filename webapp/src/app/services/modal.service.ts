import { Injectable } from '@angular/core';
import { isUndefined } from 'lodash';
import { AnimeOstModalComponent } from '../anime-ost-modal/anime-ost-modal.component';

@Injectable({
  providedIn: 'root'
})
export class ModalService {
  private modals: Array<AnimeOstModalComponent> = [];
  private reopenModal: Array<ModalArg> = [];

  constructor() { }

  add(modal: AnimeOstModalComponent) {
    // ensure component has a unique id attribute
    if (isUndefined(modal.id) || this.modals.find(x => x.id === modal.id)) {
      throw new Error('modal must have a unique id attribute');
    }

    // add modal to array of active modals
    this.modals.push(modal);
    const modalArg = this.reopenModal.find(m => m.id == modal.id);
    if (modalArg) {
        this.open(modalArg.id, modalArg.arg);
        this.reopenModal = this.reopenModal.filter(ma => ma.id !== modal.id);
    }
  }

  reopenOnCreate(id: string, editing: boolean) {
    this.reopenModal.push({id: id, arg: editing});
  }

  getArg(id: string): any {
    const modal = this.modals.find(m => m.id === id);
    return modal?.arg;
  }

  remove(modal: AnimeOstModalComponent) {
    // remove modal from array of active modals
    this.modals = this.modals.filter(m => m === modal);
  }

  open(id: string, arg: any) {
    // open modal specified by id
    const modal = this.modals.find(m => m.id === id);

    if (isUndefined(modal)) {
        throw new Error(`modal '${id}' not found`);
    }

    modal.arg = arg;
    modal.open();
  }

  close() {
    // close the modal that is currently open
    const modal = this.modals.find(m => m.isOpen);
    modal?.close();
  }
}

interface ModalArg {
  id: string,
  arg: any,
}