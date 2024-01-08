select id, name, description, amount, created_at
from sample
where deleted_at is null and name ilike concat('%%', $1::text, '%%')
order by created_at desc
limit $2
offset $3;