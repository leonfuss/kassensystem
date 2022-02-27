DROP TABLE "users";
DROP TABLE "transactions";

CREATE TABLE "users" (
  id INTEGER NOT NULL PRIMARY KEY,
  name TEXT not NULL,
  email TEXT not NULL,
  date_created DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
  balance FLOAT NOT NULL
); 

CREATE TABLE "transactions" (
  id INTEGER NOT NULL PRIMARY KEY,
  description TEXT NOT NULL,
  date DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
  authorization INTEGER NOT NULL REFERENCES "users" (id),
  affected INTEGER NOT NULL REFERENCES "users" (id),
  amount FLOAT NOT NULL
);