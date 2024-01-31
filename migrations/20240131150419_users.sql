CREATE TABLE IF NOT EXISTS users
(
    id          INTEGER PRIMARY KEY NOT NULL,
    name        VARCHAR(250)        NOT NULL,
    email       VARCHAR(250)        NOT NULL,
    password    VARCHAR(25)         NOT NULL,
    active      BOOLEAN             NOT NULL DEFAULT 0
);
