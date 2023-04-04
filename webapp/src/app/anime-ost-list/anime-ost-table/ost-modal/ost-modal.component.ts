import { Component, Input } from '@angular/core';
import * as _ from 'lodash';
import { debounceTime, distinctUntilChanged, Subject } from 'rxjs';
import { AnimeRowData } from 'src/app/models/anime-row';
import { Ost, OstType } from 'src/app/models/ost';
import { ModalService } from 'src/app/services/modal.service';
import { OstService } from 'src/app/services/ost.service';

@Component({
  selector: 'app-ost-modal',
  templateUrl: './ost-modal.component.html',
  styleUrls: ['./ost-modal.component.css']
})
export class OstModalComponent {
  @Input()
  row: AnimeRowData | undefined;
  editing: boolean = false;
  private ostChanged: Map<string, Subject<Ost>> = new Map();

  private modalId: string | undefined;

  constructor (private modalService: ModalService, private ostService: OstService) {}

  ngOnInit(): void {
    this.modalId = `ost-modal-${this.row?.anime.proxer_id}`;
    this.editing = this.modalService.getArg(this.modalId);
  }

  inputChanged(ost: Ost) {
    const ident = `${ost.proxer_id}/${ost.ost_type}/${ost.number}`;
    var subject: Subject<Ost> | undefined = this.ostChanged.get(ident);
    if (!subject) {
      console.log(`new Subject for ${JSON.stringify(ident)}`);
      console.log(`ident not in ${JSON.stringify(this.ostChanged)}`);
      subject = new Subject<Ost>();
      subject.pipe(
        debounceTime(500),
        distinctUntilChanged(),
        
      ).subscribe(ost => this.updateOst(ost));
      this.ostChanged.set(ident, subject);
    }
    subject.next(ost);
  }

  getOpenings(): Array<Ost> {
    return (this.row?.ost ?? [])
      .filter(o => o.ost_type == OstType.Opening)
      .sort((o1, o2) => o1.number - o2.number)
  }

  getEndings(): Array<Ost> {
    return (this.row?.ost ?? [])
      .filter(o => o.ost_type == OstType.Ending)
      .sort((o1, o2) => o1.number - o2.number)
  }

  updateOst(ost: Ost): void {
    console.log(`UpdateOst(${JSON.stringify(ost)})`);
    this.modalService.reopenOnCreate(this.modalId!, this.editing);
    this.ostService.postOst(ost);
  }

  addOst(ostType: OstType): void {
    if (_.isUndefined(this.row)) {
      console.error("Row is undefined")
      return;
    }
    const number = (ostType == OstType.Opening ? this.getOpenings().length : this.getEndings().length) + 1;
    const ost: Ost = {
      proxer_id: this.row.anime.proxer_id,
      ost_type: ostType,
      number: number,
      name: `${ostType} ${number}`,
      artist: null,
      video_url: null,
    }
    this.modalService.reopenOnCreate(this.modalId!, this.editing);
    this.ostService.postOst(ost);
  }

  removeOst(ost: Ost): void {
    this.modalService.reopenOnCreate(this.modalId!, this.editing);
    this.ostService.deleteOst(ost);
  }

  openingType = (): OstType => OstType.Opening
  endingType = (): OstType => OstType.Ending
}