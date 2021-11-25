CREATE TABLE user_ (
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    name varchar(128) NOT NULL,
    password TEXT NOT NULL
);

CREATE TABLE file_ (
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    name varchar(256) NOT NULL,
    uploader uuid NOT NULL REFERENCES user_(id),
    times_downloaded integer NOT NULL DEFAULT 0
);