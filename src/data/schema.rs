use rusqlite::Connection;

pub fn initialize(con: &Connection) {
    con.execute_batch(
        "
        BEGIN;
        CREATE TABLE IF NOT EXISTS activity (
            id            INTEGER PRIMARY KEY,
            name          TEXT NOT NULL,
            color         TEXT NOT NULL,
            symbol        TEXT NOT NULL,
            has_exercises INTEGER
        );

        CREATE TABLE IF NOT EXISTS exercise (
            id          INTEGER PRIMARY KEY,
            name        TEXT NOT NULL,
            description TEXT
        );

        CREATE TABLE IF NOT EXISTS activity_log (
            id          INTEGER PRIMARY KEY,
            activity_id INTEGER NOT NULL,
            date        INTEGER NOT NULL,
            comment     TEXT,
            FOREIGN KEY(activity_id) REFERENCES activity(id)
        );

        CREATE TABLE IF NOT EXISTS exercise_log (
            id           INTEGER PRIMARY KEY,
            exercise_id  INTEGER NOT NULL,
            activity_id  INTEGER NOT NULL,
            reps         TEXT,
            weight       TEXT,
            break        INTEGER,
            effort       INTEGER,
            comment      TEXT,
            FOREIGN KEY(exercise_id) REFERENCES exercise(id),
            FOREIGN KEY(activity_id) REFERENCES activity_log(id)
        );

        CREATE TABLE IF NOT EXISTS workout (
            id        INTEGER PRIMARY KEY,
            exercises TEXT,
            weights   TEXT,
            reps      TEXT,
            break     TEXT,
            comment   TEXT
        );
        COMMIT;
        ",
    )
    .unwrap();
}
