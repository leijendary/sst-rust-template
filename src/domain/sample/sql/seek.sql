select s.id, t.name, t.description, amount, created_at
from sample s
left join lateral (
    select name, description
    from sample_translation
    where id = s.id
    order by (language = $1)::int desc, ordinal
    limit 1
) t on true
where
    deleted_at is null
    and (s.name ilike concat('%%', $2::text, '%%') or t.name ilike concat('%%', $2::text, '%%'))
    and ($4 is null or $5 is null or (created_at, id) < ($4, $5))
order by created_at desc, id desc
limit $3