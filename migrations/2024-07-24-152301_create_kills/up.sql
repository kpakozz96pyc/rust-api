-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE kills
(
    id        uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    killer     VARCHAR NOT NULL,
    killed      VARCHAR NOT NULL
)