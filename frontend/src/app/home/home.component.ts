import { NgIf } from '@angular/common';
import { Component, OnInit } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { ApiService } from '../api.service';
import { LoginComponent } from "../login/login.component";

interface User {
  id: number;
  is_admin: boolean;
  first_name: string;
  last_name: string;
  email: string;
  password: string;
  created_at: string;
}

@Component({
  selector: 'app-home',
  standalone: true,
  imports: [NgIf, FormsModule, LoginComponent],
  templateUrl: './home.component.html',
  styleUrl: './home.component.scss'
})
export class HomeComponent implements OnInit {
  isLoggedIn = false;
  user: User = {
    id: 0,
    is_admin: false,
    first_name: "",
    last_name: "",
    email: "",
    password: "",
    created_at: ""
  };

  constructor(private api: ApiService) { }

  ngOnInit() {
    this.api.getLogin().subscribe((response: any) => {
      console.log("getLogin() response", response);

      if (response) {
        this.isLoggedIn = true;
        this.user = response;
      }
    });
  }

  onLogout() {
    this.api.setToken("");
    window.location.reload();
  }
}
