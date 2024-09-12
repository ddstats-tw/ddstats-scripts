-- Create all tables.
CREATE TABLE IF NOT EXISTS maps (
    map VARCHAR(128) NOT NULL,
    server VARCHAR(32) NOT NULL,
    points INTEGER NOT NULL DEFAULT 0,
    stars INTEGER NOT NULL DEFAULT 0,
    mapper VARCHAR(128) NOT NULL,
    timestamp TIMESTAMP DEFAULT NULL,
    PRIMARY KEY (Map)
);

CREATE TABLE IF NOT EXISTS mapinfo (
    map VARCHAR(128) NOT NULL,
    width INTEGER NOT NULL DEFAULT 0,
    height INTEGER NOT NULL DEFAULT 0,
    death BOOLEAN NOT NULL DEFAULT false,
    through BOOLEAN NOT NULL DEFAULT false,
    jump BOOLEAN NOT NULL DEFAULT false,
    dfreeze BOOLEAN NOT NULL DEFAULT false,
    ehook_start BOOLEAN NOT NULL DEFAULT false,
    hit_end BOOLEAN NOT NULL DEFAULT false,
    solo_start BOOLEAN NOT NULL DEFAULT false,
    tele_gun BOOLEAN NOT NULL DEFAULT false,
    tele_grenade BOOLEAN NOT NULL DEFAULT false,
    tele_laser BOOLEAN NOT NULL DEFAULT false,
    npc_start BOOLEAN NOT NULL DEFAULT false,
    super_start BOOLEAN NOT NULL DEFAULT false,
    jetpack_start BOOLEAN NOT NULL DEFAULT false,
    walljump BOOLEAN NOT NULL DEFAULT false,
    nph_start BOOLEAN NOT NULL DEFAULT false,
    weapon_shotgun BOOLEAN NOT NULL DEFAULT false,
    weapon_grenade BOOLEAN NOT NULL DEFAULT false,
    powerup_ninja BOOLEAN NOT NULL DEFAULT false,
    weapon_rifle BOOLEAN NOT NULL DEFAULT false,
    laser_stop BOOLEAN NOT NULL DEFAULT false,
    crazy_shotgun BOOLEAN NOT NULL DEFAULT false,
    dragger BOOLEAN NOT NULL DEFAULT false,
    door BOOLEAN NOT NULL DEFAULT false,
    switch_timed BOOLEAN NOT NULL DEFAULT false,
    switch BOOLEAN NOT NULL DEFAULT false,
    stop BOOLEAN NOT NULL DEFAULT false,
    through_all BOOLEAN NOT NULL DEFAULT false,
    tune BOOLEAN NOT NULL DEFAULT false,
    oldlaser BOOLEAN NOT NULL DEFAULT false,
    teleinevil BOOLEAN NOT NULL DEFAULT false,
    telein BOOLEAN NOT NULL DEFAULT false,
    telecheck BOOLEAN NOT NULL DEFAULT false,
    teleinweapon BOOLEAN NOT NULL DEFAULT false,
    teleinhook BOOLEAN NOT NULL DEFAULT false,
    checkpoint_first BOOLEAN NOT NULL DEFAULT false,
    bonus BOOLEAN NOT NULL DEFAULT false,
    boost BOOLEAN NOT NULL DEFAULT false,
    plasmaf BOOLEAN NOT NULL DEFAULT false,
    plasmae BOOLEAN NOT NULL DEFAULT false,
    plasmau BOOLEAN NOT NULL DEFAULT false,
    PRIMARY KEY (map)
);

CREATE TABLE IF NOT EXISTS teamrace (
    map VARCHAR(128) NOT NULL,
    name VARCHAR(16) NOT NULL,
    time FLOAT NOT NULL DEFAULT 0,
    id bytea NOT NULL,
    timestamp TIMESTAMP NOT NULL DEFAULT current_timestamp,
    PRIMARY KEY (name, id)
);

CREATE TABLE IF NOT EXISTS race (
    map VARCHAR(128) NOT NULL,
    name VARCHAR(16) NOT NULL,
    time FLOAT NOT NULL DEFAULT 0,
    timestamp TIMESTAMP NOT NULL DEFAULT current_timestamp,
    server VARCHAR(4) NOT NULL DEFAULT '',
    cp1 FLOAT NOT NULL DEFAULT 0,
    cp2 FLOAT NOT NULL DEFAULT 0,
    cp3 FLOAT NOT NULL DEFAULT 0,
    cp4 FLOAT NOT NULL DEFAULT 0,
    cp5 FLOAT NOT NULL DEFAULT 0,
    cp6 FLOAT NOT NULL DEFAULT 0,
    cp7 FLOAT NOT NULL DEFAULT 0,
    cp8 FLOAT NOT NULL DEFAULT 0,
    cp9 FLOAT NOT NULL DEFAULT 0,
    cp10 FLOAT NOT NULL DEFAULT 0,
    cp11 FLOAT NOT NULL DEFAULT 0,
    cp12 FLOAT NOT NULL DEFAULT 0,
    cp13 FLOAT NOT NULL DEFAULT 0,
    cp14 FLOAT NOT NULL DEFAULT 0,
    cp15 FLOAT NOT NULL DEFAULT 0,
    cp16 FLOAT NOT NULL DEFAULT 0,
    cp17 FLOAT NOT NULL DEFAULT 0,
    cp18 FLOAT NOT NULL DEFAULT 0,
    cp19 FLOAT NOT NULL DEFAULT 0,
    cp20 FLOAT NOT NULL DEFAULT 0,
    cp21 FLOAT NOT NULL DEFAULT 0,
    cp22 FLOAT NOT NULL DEFAULT 0,
    cp23 FLOAT NOT NULL DEFAULT 0,
    cp24 FLOAT NOT NULL DEFAULT 0,
    cp25 FLOAT NOT NULL DEFAULT 0,
    PRIMARY KEY (map, name, time, timestamp, server)
);

CREATE TABLE IF NOT EXISTS playtime (
    date DATE NOT NULL,
    location VARCHAR(8) NOT NULL DEFAULT 'unknown',
    gametype VARCHAR(32) NOT NULL,
    map VARCHAR(128) NOT NULL,
    name VARCHAR(15) NOT NULL,
    clan VARCHAR(15) NOT NULL,
    country INTEGER NOT NULL,
    skin_name VARCHAR(32),
    skin_color_body INTEGER,
    skin_color_feet INTEGER,
    time INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS playtime_processed (
    date DATE PRIMARY KEY
);

CREATE TABLE IF NOT EXISTS maps_playtime (
    map VARCHAR(128) PRIMARY KEY NOT NULL,
    seconds BIGINT NOT NULL DEFAULT 0,
    mostaddicted VARCHAR(32) NOT NULL,
    mostaddicted_seconds INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS playtime_maps (
    name VARCHAR(16) NOT NULL,
    map VARCHAR(128) NOT NULL,
    seconds_played INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (map, name)
);

CREATE TABLE IF NOT EXISTS playtime_maps_processed (
    date DATE PRIMARY KEY
);

CREATE TABLE IF NOT EXISTS rankings (
    rank INTEGER NOT NULL,
    timestamp timestamp NOT NULL, 
    name VARCHAR(16) NOT NULL,
    time FLOAT NOT NULL,
    map VARCHAR(128) NOT NULL,
    server VARCHAR(4) NOT NULL DEFAULT ''
);

CREATE TABLE IF NOT EXISTS teamrankings (
    rank INTEGER NOT NULL,
    timestamp TIMESTAMP NOT NULL, 
    id BYTEA NOT NULL,
    players VARCHAR(16)[] NOT NULL,
    time FLOAT NOT NULL,
    map VARCHAR(128) NOT NULL,
    server VARCHAR(4) NOT NULL DEFAULT ''
);

CREATE TABLE IF NOT EXISTS mapstats (
    map VARCHAR(128) NOT NULL PRIMARY KEY,
    finishes INTEGER DEFAULT 0,
    finishes_rank INTEGER,
    median_time FLOAT,
    top_time FLOAT,
    top_time_team FLOAT
);

CREATE TABLE IF NOT EXISTS rankedpoints (
    date DATE NOT NULL,
    name VARCHAR(16) NOT NULL,
    rankpoints INTEGER,
    teampoints INTEGER,
    PRIMARY KEY (date, name)
);

CREATE TABLE IF NOT EXISTS rankedpoints_processed (
    date DATE NOT NULL PRIMARY KEY
);

CREATE TABLE IF NOT EXISTS rankpoints (
    rank INTEGER NOT NULL,
    points INTEGER NOT NULL,
    PRIMARY KEY(rank)
);

INSERT INTO rankpoints (rank, points) VALUES
    (1, 25),
    (2, 18),
    (3, 15),
    (4, 12),
    (5, 10),
    (6, 8),
    (7, 6),
    (8, 4),
    (9, 2),
    (10, 1)
ON CONFLICT DO NOTHING;

CREATE TABLE players (
	name VARCHAR(15) NOT NULL,
	points INTEGER NOT NULL,
	clan VARCHAR(15),
	country INTEGER,
	skin_name VARCHAR(32),
	skin_color_body INTEGER,
	skin_color_feet INTEGER,
	PRIMARY KEY(name)
);
