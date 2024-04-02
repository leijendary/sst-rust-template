-- Table: sample
create table sample (
    id bigint generated always as identity primary key,
    name character varying(100) not null,
    description text,
    amount numeric(12, 2) not null,
    version smallint not null default 0,
    created_at timestamp with time zone not null default now(),
    created_by text not null,
    last_modified_at timestamp with time zone not null default now(),
    last_modified_by text not null,
    deleted_at timestamp with time zone,
    deleted_by text
);

-- Set random initial value for sample_id_seq.
select setval('sample_id_seq', (select floor(random() * 100000 + 99999)::bigint));

-- Unique constraint: sample.name
create unique index sample_name_key on sample(lower(name::text)) where deleted_at is null;

-- Index (desc): sample.created_at, sample.id
create index sample_created_at_id_idx on sample(created_at desc, id desc);

-- Table: sample_translation
create table sample_translation (
    id bigint references sample(id),
    name character varying(100) not null,
    description character varying(200),
    language character varying(4),
    ordinal smallint not null,
    constraint sample_translation_pkey primary key (id, language)
);

-- Index: sample_translation.id
create index sample_translation_id_idx on sample_translation(id);