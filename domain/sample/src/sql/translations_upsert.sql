insert into sample_translation (id, name, description, language, ordinal)
select $1, * from unnest($2::text[], $3::text[], $4::text[], $5::smallint[])
on conflict (id, language)
do update
set
    name = excluded.name,
    description = excluded.description,
    language = excluded.language,
    ordinal = excluded.ordinal
returning name, description, language, ordinal