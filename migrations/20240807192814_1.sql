CREATE TYPE testing.location_kind AS ENUM ('prophecy', 'pope_msg');

CREATE TABLE testing.location_v2 (
    kind testing.location_kind,
    guild_id bigint,
    channel_id bigint
    PRIMARY KEY (guild_id, channel_id)
);

CREATE TABLE testing.prophecy (
    id bigint PRIMARY KEY,
    content text
);

CREATE TABLE testing.default_response (
    id bigint PRIMARY KEY,
    content text
);


INSERT INTO testing.prophecy (id, content) VALUES
(1, 'The heat death of the universe.'),
(2, '2006 HONDA CIVIC'),
(3, 'Dr. jj Jr.');