import { Component, Input } from '@angular/core';
import { Anime } from 'src/app/models/anime';
import { Ost } from 'src/app/models/ost';
import { User } from 'src/app/models/user';
import { UserAnime } from 'src/app/models/useranime';
import { ModalService } from 'src/app/services/modal.service';

@Component({
  selector: 'app-anime-ost-entry',
  templateUrl: './anime-ost-entry.component.html',
  styleUrls: ['./anime-ost-entry.component.css']
})

export class AnimeOstEntryComponent {
  @Input()
  user: User | undefined;

  @Input()
  anime: Anime | undefined;

  @Input()
  userAnime: UserAnime | undefined;

  @Input()
  ostList: Array<Ost> | undefined;
  
  constructor(private modalService: ModalService) {}

  ngOnInit(): void {

  }

  openOstModal(): void {
    this.modalService.open(`ost-modal-${this.anime?.proxer_id}`, false)
  }
}
