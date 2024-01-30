-- Your SQL goes here
CREATE TABLE tokens (
    id int NOT NULL AUTO_INCREMENT,
    token VARCHAR(25) NOT NULL,
    account_id int NOT NULL,
    valid_until TIMESTAMP NOT NULL,
    PRIMARY KEY (id),
    FOREIGN KEY (account_id) REFERENCES account(id),
    CONSTRAINT un_token UNIQUE (token)
);