CREATE TABLE IF NOT EXISTS data.score (
	id			SERIAL		PRIMARY KEY,
	userid    	INTEGER		NOT NULL REFERENCES data.users(id),
	build		SMALLINT	NOT NULL,
	track_id 	SMALLINT	NOT NULL,
	timescore	INTEGER		NOT NULL,
	carbody_id	SMALLINT	NOT NULL,
	car_color	SMALLINT	NOT NULL,
	data		TEXT		NOT NULL
);
