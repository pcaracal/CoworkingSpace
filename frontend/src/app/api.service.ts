import { HttpClient } from '@angular/common/http';
import { Injectable } from '@angular/core';

@Injectable({
  providedIn: 'root'
})
export class ApiService {
  private _url = "http://localhost:8000";


  setToken(token: string) {
    localStorage.setItem("token", token);
  }

  getToken(): string {
    return localStorage.getItem("token") || "";
  }


  constructor(private _http: HttpClient) { }

  postLogin(email: string, password: string) {
    return this._http.post(this._url + "/login", { email, password });
  }

  postRegister(firstName: string, lastName: string, email: string, password: string) {
    return this._http.post(this._url + "/register", { first_name: firstName, last_name: lastName, email, password });
  }


}
