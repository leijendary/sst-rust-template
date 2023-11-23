CREATE TABLE sample (
    id bigint GENERATED ALWAYS AS IDENTITY (START 100000) PRIMARY KEY,
    name character varying(100) NOT NULL,
    description text,
    amount integer NOT NULL,
    version smallint NOT NULL DEFAULT 0,
    created_at timestamp with time zone NOT NULL DEFAULT now(),
    created_by text NOT NULL,
    last_modified_at timestamp with time zone NOT NULL DEFAULT now(),
    last_modified_by text NOT NULL,
    deleted_at timestamp with time zone,
    deleted_by text
);

CREATE UNIQUE INDEX sample_pkey ON sample(id int8_ops);
CREATE INDEX sample_created_at_id_idx ON sample(created_at timestamptz_ops DESC,id int8_ops DESC);
CREATE UNIQUE INDEX sample_name_key ON sample((lower(name::text)) text_ops) WHERE deleted_at IS NULL;

CREATE TABLE sample_translation (
    id bigint REFERENCES sample(id),
    name character varying(100) NOT NULL,
    description character varying(200),
    language character varying(4),
    ordinal smallint NOT NULL,
    CONSTRAINT sample_translation_pkey PRIMARY KEY (id, language)
);

CREATE UNIQUE INDEX sample_translation_pkey ON sample_translation(id int8_ops,language text_ops);
