-- Add up migration script here
create extension if not exists "uuid-ossp";

-- While `created_at` can just be `default now()`, setting `updated_at` on update requires a trigger which
-- is a lot of boilerplate. These two functions save us from writing that every time as instead we can just do
--
-- select trigger_updated_at('<table name>');
--
-- after a `CREATE TABLE`.
create or replace function set_updated_at()
    returns trigger as
$$
begin
    NEW.updated_at = now();
    return NEW;
end;
$$ language plpgsql;

create or replace function trigger_updated_at(tablename regclass)
    returns void as
$$
begin
    execute format('CREATE TRIGGER set_updated_at
        BEFORE UPDATE
        ON %s
        FOR EACH ROW
        WHEN (OLD is distinct from NEW)
    EXECUTE FUNCTION set_updated_at();', tablename);
end;
$$ language plpgsql;

-- Finally, this is a text collation that sorts text case-insensitively, useful for `UNIQUE` indexes
-- over things like usernames and emails, without needing to remember to do case-conversion.
create collation case_insensitive (provider = icu, locale = 'und-u-ks-level2', deterministic = false);

