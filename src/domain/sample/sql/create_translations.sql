insert into sample_translation (id, name, description, language, ordinal)
select * from unnest($1::int[], $2::text[], $3::text[], $4::text[], $5::smallint[])
returning name, description, language, ordinal