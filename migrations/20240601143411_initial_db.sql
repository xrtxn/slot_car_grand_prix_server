CREATE SCHEMA data;

CREATE TABLE IF NOT EXISTS data.users
(
    id			SERIAL		PRIMARY KEY,
    username    TEXT  		NOT NULL UNIQUE,
    password	TEXT		NOT NULL
);
