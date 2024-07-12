import { NgIf } from '@angular/common';
import { Component, OnInit } from '@angular/core';
import { ApiService } from '../api.service';
import { FormsModule } from '@angular/forms';

@Component({
  selector: 'app-login',
  standalone: true,
  imports: [NgIf, FormsModule],
  templateUrl: './login.component.html',
  styleUrl: './login.component.scss'
})
export class LoginComponent implements OnInit {
  isRegister = false;

  firstName = '';
  lastName = '';
  email = '';
  password = '';

  constructor(private api: ApiService) { }

  ngOnInit() { }

  onLogin(event: Event) {
    event.preventDefault();
    console.log("onLogin()", this.email, this.password);
    if (this.email && this.password) {
      this.api.postLogin(this.email, this.password).subscribe((response: any) => {
        console.log("onLogin() response", response);
        this.api.setToken(response.token);
      });
    }
  }

  onRegister(event: Event) {
    event.preventDefault();
    console.log("onRegister()", this.firstName, this.lastName, this.email, this.password);

    if (this.firstName && this.lastName && this.email && this.password) {
      this.api.postRegister(this.firstName, this.lastName, this.email, this.password).subscribe((response: any) => {
        console.log("onRegister() response", response);
        this.api.setToken(response.token);
      });
    }
  }

}
