CREATE TABLE members (
    id INT NOT NULL AUTO_INCREMENT,
    email VARCHAR(320) UNIQUE NOT NULL,
    hash CHAR(100),
    enabled BOOLEAN DEFAULT FALSE,
    ephem_token CHAR(64),

    PRIMARY KEY(id)
);

CREATE TABLE list_member_relation (
    list INT NOT NULL,
    member INT NOT NULL,

    PRIMARY KEY(list, member),

    FOREIGN KEY (list)
        REFERENCES lists(id)
        ON UPDATE CASCADE ON DELETE RESTRICT,
    FOREIGN KEY (member)
        REFERENCES members(id)
        ON UPDATE CASCADE ON DELETE CASCADE,

    INDEX( member )
);
