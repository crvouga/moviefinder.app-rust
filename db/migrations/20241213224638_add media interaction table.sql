-- migrate:up

CREATE TYPE media_interaction_action AS ENUM ('add', 'retract');

CREATE TYPE media_interaction_name AS ENUM ('liked', 'disliked', 'interested', 'not-interested', 'seen', 'not-seen');

CREATE TABLE media_interaction (
    id TEXT PRIMARY KEY,
    media_id TEXT NOT NULL,
    user_id TEXT NOT NULL,
    interaction_name media_interaction_name NOT NULL,
    interaction_action media_interaction_action NOT NULL,
    created_at_posix BIGINT NOT NULL DEFAULT (EXTRACT(EPOCH FROM NOW())),
    updated_at_posix BIGINT NOT NULL DEFAULT (EXTRACT(EPOCH FROM NOW())),
    deleted_at_posix BIGINT DEFAULT NULL
);

CREATE INDEX idx_media_id_user_id ON media_interaction(media_id, user_id);


-- migrate:down

DROP INDEX idx_media_id_user_id;
DROP TABLE media_interaction;
DROP TYPE media_interaction_action;
DROP TYPE media_interaction_name;

