import { Component } from '@angular/core';
import { Anime } from 'src/app/models/anime';
import { Ost } from 'src/app/models/ost';
import { User } from 'src/app/models/user';
import { UserAnime } from 'src/app/models/useranime';
import { AnimeService } from 'src/app/services/anime.service';
import { OstService } from 'src/app/services/ost.service';
import { UserService } from 'src/app/services/user.service';
import * as _ from 'lodash';
import { forkJoin } from 'rxjs';

@Component({
  selector: 'app-anime-ost-table',
  templateUrl: './anime-ost-table.component.html',
  styleUrls: ['./anime-ost-table.component.css']
})
export class AnimeOstTableComponent {
  private userList: Array<User> = [];
  private animeList: Array<Anime> = [];
  private userAnimeList: Array<UserAnime> = [];
  private ostList: Array<Ost> = [];
  selectedUserList: Array<User> = [];
  rowDataList: Array<RowData> = [];

  allUsersSelected: boolean = true;
  userCheckboxes: Array<UserCheckbox> = [];
  onlyCommonCheckbox: boolean = false;

  constructor(
    private userService: UserService,
    private animeService: AnimeService,
    private ostService: OstService,
  ) {}

  ngOnInit(): void {
      this.userService.onUsers().subscribe(userList => this.onUsers(userList));
      this.animeService.onAnime().subscribe(animeList => this.onAnime(animeList));
      this.animeService.onUserAnime().subscribe(userAnimeList => this.onUserAnime(userAnimeList));
      this.ostService.onOst().subscribe(ostList => this.onOst(ostList));
  }

  ngOnUpdate(): void {
    console.log("Updated Table");
  }

  onUsers(userList: Array<User>): void {
    this.userList = userList;
    this.selectedUserList = userList;
    this.setUserCheckboxes();
    this.updateRowData();
  };

  onAnime(animeList: Array<Anime>): void {
    this.animeList = animeList.sort((a1, a2) => a1.proxer_name.toLowerCase().localeCompare(a2.proxer_name.toLowerCase()))
    this.updateRowData();
  }

  onUserAnime(userAnimeList: Array<UserAnime>): void {
    this.userAnimeList = userAnimeList;
    this.updateRowData();
  }

  onOst(ostList: Array<Ost>): void {
    this.ostList = ostList;
    this.updateRowData();
  }

  updateRowData(): void {
    this.setSelectedUserList();
    const rows: Array<RowData | undefined> = this.animeList.map(anime => {
      const userAnimeEntries = this.userAnimeList
        .filter(ua => ua.proxer_id == anime.proxer_id);
      const users = this.selectedUserList.filter(user => userAnimeEntries.some(e => e.user_id == user.user_id));
      if (users.length==0) {
        return undefined;
      }
      //console.log(`checkbox(${this.onlyCommonCheckbox}): ${users.length}/${this.selectedUserList.length} for ${anime.proxer_name}`);
      if (this.onlyCommonCheckbox && users.length != this.selectedUserList.length) {
        return undefined;
      }
      const ost = this.ostList.filter(ost => ost.proxer_id == anime.proxer_id);
      return {
        anime,
        userAnimeEntries,
        users,
        ost
      }
    })
    this.rowDataList = rows.filter((r): r is RowData => r !== undefined);
  }

  getUserAnimeFor(user_id: number, rowData: RowData): UserAnime | undefined {
    const entries = rowData.userAnimeEntries.filter(ua => ua.user_id === user_id);
    return entries[0];
  }

  getOstFor(proxer_id: number): Array<Ost> {
    const entries = this.ostList?.filter(o => o.proxer_id === proxer_id) ?? [];
    return entries;
  }

  setUserCheckboxes(): void {
    this.userCheckboxes = this.userList.map(user => { return {
      id: user.user_id,
      value: user.user_name,
      isSelected: true,
    }})
  }

  checkAllCheckboxes() {
    this.userCheckboxes.forEach(checkbox => checkbox.isSelected = this.allUsersSelected);
    this.updateRowData()
  }

  checkAllCheckboxesSelected(): void {
    this.allUsersSelected = this.userCheckboxes
      .every(checkbox => checkbox.isSelected);
    this.updateRowData()
  }

  setSelectedUserList(): void {
    const selectedUsers = this.userCheckboxes
      .filter(checkbox => checkbox.isSelected)
      .map(checkbox => checkbox.id);
    this.selectedUserList = this.userList.filter(user => selectedUsers.includes(user.user_id))
  }
}

interface UserCheckbox {
  id: number,
  value: string,
  isSelected: boolean,   
}

interface RowData {
  anime: Anime,
  userAnimeEntries: Array<UserAnime>,
  users: Array<User>
  ost: Array<Ost>,
}