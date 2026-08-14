CREATE TABLE IF NOT EXISTS Flights (
    id            INTEGER PRIMARY KEY AUTOINCREMENT,
    dep           TEXT    NOT NULL,
    arr           TEXT    NOT NULL,
    dof           TEXT    NOT NULL,
    flight_number TEXT    NOT NULL,
    tail          TEXT    NOT NULL,
    pyld          INTEGER NOT NULL
);
