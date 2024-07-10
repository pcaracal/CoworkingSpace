---
geometry: margin=1.5cm
---

# Dokumentation M223 LB-B

von Peter Schreivogel

\pagebreak

# 1. Anforderungen analysieren

## 1.1. Erweiterte Anforderungen

### A. Drei funktionalen Anforderungen

1. Als Besucher kann ich eine Login-Seite sehen
2. Als Mitglied kann ich sehen, welche Coworking Spaces wann gebucht sind.
3. Als Mitglied kann ich einen Grund für eine Buchung angeben, sodass der
   Administrator weiss wofür ich die Coworking Space brauche.

### B. Drei nicht-funktionalen Anforderungen

1. Passwörter werden nicht in Cleartext gespeichert, sondern müssen mittels
   eines Hashing-Algorithmus wie Argon2 gehashed werden
2. Coworking Spaces können nicht doppelt gebucht werden
3. Sämtliche Requests müssen geloggt werden um Fehler und Threat Actors zu entdecken

\pagebreak

## 1.2. Persona

### P1: Besucher

| Bild                          | Name        | Alter | Geschlecht | Beruf        | Grund             |
| ----------------------------- | ----------- | ----- | ---------- | ------------ | ----------------- |
| ![Besucher](img/besucher.jpg) | John Johnes | 35    | M          | SCRUM Master | Stand-up meetings |

### P2: Mitglied

| Bild                          | Name        | Alter | Geschlecht | Beruf                 | Grund                           |
| ----------------------------- | ----------- | ----- | ---------- | --------------------- | ------------------------------- |
| ![Mitglied](img/mitglied.jpg) | Jane Johnes | 24    | F          | Systemadministratorin | Systemmigration planen mit Team |

### P3: Administrator

| Bild                                    | Name          | Alter | Geschlecht | Beruf                  | Grund                                         |
| --------------------------------------- | ------------- | ----- | ---------- | ---------------------- | --------------------------------------------- |
| ![Administrator](img/administrator.jpg) | Batuhan Avsar | 19    | M          | Applikationsentwickler | Neue Web-Applikation an Investoren vorstellen |

![Anwendungsfalldiagramm](img/usecase.svg)

\pagebreak

# 2. Persistenzschicht planen

## 2.4. Fachklassendiagramm

\pagebreak

# 3. Schnittstelle planen

## 3.5. Schnittstellenplanung

## 3.6. Sequenzdiagramm
