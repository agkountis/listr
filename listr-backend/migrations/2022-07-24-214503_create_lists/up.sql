-- Your SQL goes here
DROP TABLE IF EXISTS lists;
DROP TABLE IF EXISTS items;

CREATE TABLE lists(
    id SERIAL PRIMARY KEY,
    name VARCHAR(80) NOT NULL
);

CREATE TABLE items(
    id SERIAL PRIMARY KEY,
    list_id SERIAL REFERENCES lists(id),
    data VARCHAR(80) NOT NULL
);