CREATE TABLE IF NOT EXISTS accounts
(
  id          INTEGER PRIMARY KEY NOT NULL,
  user_id     INTEGER,
  name        VARCHAR(250) NOT NULL,

  FOREIGN KEY (user_id) REFERENCES users(id)
);

CREATE TABLE IF NOT EXISTS transactions
(
  id          INTEGER PRIMARY KEY NOT NULL,
  account_id  INTEGER,
  date        DATETIME NOT NULL,
  memo        VARCHAR(250),
  inflow      INTEGER DEFAULT 0,
  outflow     INTEGER DEFAULT 0,
  cleared     BOOLEAN NOT NULL DEFAULT 0,

  FOREIGN KEY (account_id) REFERENCES accounts(id));
