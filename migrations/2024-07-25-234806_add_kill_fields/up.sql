-- Your SQL goes here
ALTER TABLE kills
ADD COLUMN range DOUBLE PRECISION NOT NULL;
ALTER TABLE kills
ADD COLUMN server VARCHAR NOT NULL default 'TBD';