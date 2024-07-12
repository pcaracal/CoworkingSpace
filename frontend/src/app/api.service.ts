import { HttpClient } from '@angular/common/http';
import { Injectable } from '@angular/core';

@Injectable({
  providedIn: 'root'
})
export class ApiService {
  private _url = "http://localhost:8000";

  constructor(private _http: HttpClient) { }

  setToken(token: string) {
    localStorage.setItem("token", token);
  }

  getToken(): string {
    return localStorage.getItem("token") || "";
  }

  getHeader() {
    console.log("Authorization", "Bearer " + this.getToken());

    return {
      headers: {
        "Authorization": "Bearer " + this.getToken(),
        "Content-Type": "application/json"
      }
    };
  }

  getLogin() {
    return this._http.get(this._url + "/login", this.getHeader());
  }

  postLogin(email: string, password: string) {
    return this._http.post(this._url + "/login", { email, password });
  }

  postRegister(firstName: string, lastName: string, email: string, password: string) {
    return this._http.post(this._url + "/register", { first_name: firstName, last_name: lastName, email, password });
  }
}
