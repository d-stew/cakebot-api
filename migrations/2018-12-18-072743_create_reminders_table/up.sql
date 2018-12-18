CREATE TABLE reminders (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  remind_date DATE NOT NULL,
  note TEXT,
  buffer INT
);  