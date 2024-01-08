update sample
set
    name = $3,
    description = $4,
    amount = $5,
    version = version + 1,
    last_modified_at = now(),
    last_modified_by = $6
where id = $1 and version = $2
returning id, name, description, amount, version, created_at