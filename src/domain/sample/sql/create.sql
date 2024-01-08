insert into sample (name, description, amount, created_by, last_modified_by)
values ($1, $2, $3, $4, $5)
returning id, name, description, amount, version, created_at