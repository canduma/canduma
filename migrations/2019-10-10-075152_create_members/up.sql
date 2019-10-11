CREATE TABLE teams (
      team_id SERIAL PRIMARY KEY,
      name VARCHAR NOT NULL
    );

    CREATE TABLE members (
      member_id SERIAL PRIMARY KEY,
      name VARCHAR NOT NULL,
      knockouts INT NOT NULL DEFAULT 0,
      team_id INT NOT NULL,
      FOREIGN KEY (team_id) REFERENCES teams(team_id)
    );

    INSERT INTO teams(team_id, name) VALUES (1, 'Heroes');
    INSERT INTO members(name, knockouts, team_id) VALUES ('Link', 14, 1);
    INSERT INTO members(name, knockouts, team_id) VALUES ('Mario', 11, 1);
    INSERT INTO members(name, knockouts, team_id) VALUES ('Kirby', 8, 1);

    INSERT INTO teams(team_id, name) VALUES (2, 'Villains');
    INSERT INTO members(name, knockouts, team_id) VALUES ('Ganondorf', 8, 2);
    INSERT INTO members(name, knockouts, team_id) VALUES ('Bowser', 11, 2);
    INSERT INTO members(name, knockouts, team_id) VALUES ('Mewtwo', 12, 2);