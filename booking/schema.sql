
-- used to make time interval index
CREATE EXTENSION IF NOT EXISTS btree_gist;

DROP TABLE IF EXISTS place;
CREATE TABLE IF NOT EXISTS place (
    id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    name TEXT NOT NULL UNIQUE,
    capacity INTEGER NOT NULL DEFAULT 1 CONSTRAINT positive_capacity CHECK (capacity > 0),
    is_open BOOLEAN NOT NULL DEFAULT TRUE
);

DROP TABLE IF EXISTS reservation;
CREATE TABLE IF NOT EXISTS reservation (
    id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    timespan TSTZRANGE NOT NULL,
    fk__reservation__place INTEGER REFERENCES place(id),
    EXCLUDE USING GIST (fk__reservation__place WITH =, timespan WITH &&)
);

INSERT INTO place(name, capacity, is_open)
VALUES
    ('A1', 22, true),
    ('A2', 10, false),
    ('B', 4, true);

INSERT INTO reservation(timespan, fk__reservation__place)
VALUES
    (TSTZRANGE('2022-04-01 14:30UTC'::TIMESTAMP, '2022-04-01 14:30UTC'::TIMESTAMP + INTERVAL '1 hour'), 1),
    (TSTZRANGE(now(), now() + INTERVAL '1 hour'), 1),
    ('[2022-04-01 11:00, 2022-04-01 13:00)', 2);

