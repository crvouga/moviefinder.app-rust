-- migrate:up

CREATE TABLE key_value (
    key TEXT PRIMARY KEY,
    value TEXT,
    created_at_posix BIGINT NOT NULL DEFAULT (strftime('%s', 'now')),
    updated_at_posix BIGINT NOT NULL DEFAULT (strftime('%s', 'now')),
    deleted_at_posix BIGINT DEFAULT NULL
);

-- migrate:down

DROP TABLE key_value;