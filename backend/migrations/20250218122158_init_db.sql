-- Add migration script here
CREATE TABLE IF NOT EXISTS "user" (
    id UUID PRIMARY KEY,
    password VARCHAR(255),
    username VARCHAR(255),
    admin boolean,
    cheater boolean
);

CREATE TABLE IF NOT EXISTS "user_statistics" (
    stat_id UUID PRIMARY KEY,
    highest_score smallint,
    victories smallint,
    games_played smallint,
    elo smallint,
    user_id UUID NOT NULL REFERENCES "user"(id)
);


CREATE TABLE IF NOT EXISTS "game" (
    game_id UUID PRIMARY KEY,
    score smallint,
    average_time real, 
    dimension smallint,
    time_limit smallint,
    user_id UUID NOT NULL REFERENCES "user"(id)
);

CREATE TABLE IF NOT EXISTS "statistics" (
    stat_id UUID PRIMARY KEY,
    total_timings real,
    total_score bigint,
    games_played bigint
);

CREATE TABLE IF NOT EXISTS "anomalous_games" (
    agame_id UUID PRIMARY KEY,
    moves jsonb,
    user_id UUID NOT NULL REFERENCES "user"(id)
);

CREATE TABLE IF NOT EXISTS "session" (
    ssid UUID PRIMARY KEY,
    expiry_date date,
    user_id UUID NOT NULL REFERENCES "user"(id)
);


CREATE OR REPLACE FUNCTION update_statistics_on_game_insert()
RETURNS TRIGGER AS $$
BEGIN
    UPDATE user_statistics
    SET
        games_played = games_played + 1,
        highest_score = GREATEST(highest_score, NEW.score)
    WHERE user_id = NEW.user_id;

    UPDATE statistics
    SET
        total_timings = total_timings + NEW.average_time,
        total_score = total_score + NEW.score,
        games_played = games_played + 1;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER game_insert_trigger
AFTER INSERT ON game
FOR EACH ROW EXECUTE FUNCTION update_statistics_on_game_insert();
