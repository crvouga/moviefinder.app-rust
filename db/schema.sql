SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET transaction_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

--
-- Name: media_interaction_action; Type: TYPE; Schema: public; Owner: -
--

CREATE TYPE public.media_interaction_action AS ENUM (
    'add',
    'retract'
);


--
-- Name: media_interaction_name; Type: TYPE; Schema: public; Owner: -
--

CREATE TYPE public.media_interaction_name AS ENUM (
    'liked',
    'disliked',
    'interested',
    'not-interested',
    'seen',
    'not-seen'
);


SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: key_value; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.key_value (
    key text NOT NULL,
    value text,
    created_at_posix bigint DEFAULT EXTRACT(epoch FROM now()) NOT NULL,
    updated_at_posix bigint DEFAULT EXTRACT(epoch FROM now()) NOT NULL,
    deleted_at_posix bigint
);


--
-- Name: media_interaction; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.media_interaction (
    id text NOT NULL,
    media_id text NOT NULL,
    user_id text NOT NULL,
    interaction_name public.media_interaction_name NOT NULL,
    interaction_action public.media_interaction_action NOT NULL,
    created_at_posix bigint DEFAULT EXTRACT(epoch FROM now()) NOT NULL,
    updated_at_posix bigint DEFAULT EXTRACT(epoch FROM now()) NOT NULL,
    deleted_at_posix bigint
);


--
-- Name: schema_migrations; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.schema_migrations (
    version character varying NOT NULL
);


--
-- Name: key_value key_value_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.key_value
    ADD CONSTRAINT key_value_pkey PRIMARY KEY (key);


--
-- Name: media_interaction media_interaction_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.media_interaction
    ADD CONSTRAINT media_interaction_pkey PRIMARY KEY (id);


--
-- Name: schema_migrations schema_migrations_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.schema_migrations
    ADD CONSTRAINT schema_migrations_pkey PRIMARY KEY (version);


--
-- Name: idx_media_id_user_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_media_id_user_id ON public.media_interaction USING btree (media_id, user_id);


--
-- PostgreSQL database dump complete
--


--
-- Dbmate schema migrations
--

INSERT INTO public.schema_migrations (version) VALUES
    ('20241023214627'),
    ('20241213224638');
