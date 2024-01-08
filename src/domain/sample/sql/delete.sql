update sample
set
    version = version + 1,
    deleted_by = $3,
    deleted_at = now()
where id = $1 and version = $2 and deleted_at is null