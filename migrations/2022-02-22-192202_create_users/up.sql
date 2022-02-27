CREATE TABLE "users" (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  name TEXT NOT NULL,
  email TEXT NOT NULL,
  date_created TEXT NOT NULL
);

INSERT INTO 
  "users"(name,email,date_created)
VALUES
  ("Leon", "leon.fuss@icloud.com", "23-02-22");
