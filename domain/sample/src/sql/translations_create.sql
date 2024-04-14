insert into sample_translation (id, name, description, language, ordinal)
select $1, * from unnest($2::text[], $3::text[], $4::text[], $5::smallint[])
returning name, description, language, ordinal