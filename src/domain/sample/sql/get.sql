select
    s.id,
    coalesce(t.name, s.name) name,
    coalesce(t.description, s.description) description,
    amount,
    version,
    created_at
from sample s
left join lateral (
    select name, description
    from sample_translation
    where id = s.id
    order by (language = $3)::int desc, ordinal
    limit 1
) t on $2
where id = $1 and deleted_at is null