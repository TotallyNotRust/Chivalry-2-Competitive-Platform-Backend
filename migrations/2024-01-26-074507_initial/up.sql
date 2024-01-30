-- Your SQL goes here
-- If a user has recieved a punishment, such as a mute or ban, this will describe the punishment, and where to find evidence for it
CREATE TABLE punishment (
    id INT NOT NULL AUTO_INCREMENT,
    type TINYINT(4) NOT NULL,
    end_date DATE,
    evidence_path VARCHAR(16),  -- This will be the name of a file containing evidence related to this punishment
                                -- Users will want to know why theyre banned, so this will contain reports and a note by the admin.
    PRIMARY KEY (id)
);

CREATE TABLE account (
    id INT NOT NULL AUTO_INCREMENT,
    username VARCHAR(25) NOT NULL,
    email VARCHAR(255) NOT NULL,
    salted_password VARCHAR(60) NOT NULL,
    account_closed TINYINT(1) NOT NULL DEFAULT 0, /* If true, the account is closed and login will be denied. */
    punishment_id INT, -- TODO: I was smoking crack when i made this, this needs to change
    PRIMARY KEY (id),
    FOREIGN KEY (punishment_id) REFERENCES punishment(id)
);

/*  I split up account and account_ranked_info, since standing will be updated a lot,
    and i think its smarter to not need to update the entire account record.
    Its also possible i may want to expand the platform to have more games in the future,
    by having a seperate ranked info table, ill be able to have a record for each game the account plays */
CREATE TABLE account_ranked_info (
    id INT NOT NULL AUTO_INCREMENT,
    account_id INT NOT NULL,
    standing INT NOT NULL DEFAULT 1500,
    PRIMARY KEY (id),
    FOREIGN KEY (account_id) REFERENCES account(id) ON DELETE CASCADE
);