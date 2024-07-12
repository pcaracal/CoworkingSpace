create table user (
  id integer primary key autoincrement,
  is_admin boolean default false,
  first_name text not null,
  last_name text not null,
  email text not null,
  password text not null,
  created_at datetime default current_timestamp
);

create table room (
  id integer primary key autoincrement,
  name text not null
);

insert into room (name) values ('Room 1'), ('Room 2'), ('Room 3');

create table booking (
  id integer primary key autoincrement,
  reason text not null,
  duration integer not null,
  status text not null,
  date text not null,
  fk_room_id integer not null,
  fk_user_id integer not null,
  created_at datetime default current_timestamp,
  foreign key (fk_room_id) references room(id),
  foreign key (fk_user_id) references user(id)
);
