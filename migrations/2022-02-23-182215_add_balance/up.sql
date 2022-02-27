DROP TABLE "users";

CREATE TABLE "users" (
  id INTEGER NOT NULL PRIMARY KEY,
  name TEXT not NULL,
  email TEXT not NULL,
  date_created INTEGER NOT NULL,
  balance FLOAT NOT NULL
); 

CREATE TABLE "transactions" (
  id INTEGER NOT NULL PRIMARY KEY,
  description TEXT NOT NULL,
  date INTEGER NOT NULL,
  authorization_id INTEGER NOT NULL REFERENCES "users" (id),
  affected_id INTEGER NOT NULL REFERENCES "users" (id),
  amount FLOAT NOT NULL
);
