# Command DB Schema

## Circle Table

```sql
CREATE TABLE circle_table (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
```

## Circle Event Table

```sql
CREATE TABLE circle_event_table (
    id UUID PRIMARY KEY, -- event_id
    circle_id UUID NOT NULL,
    version INT NOT NULL,
    event_type TEXT NOT NULL,
    event_data JSONB NOT NULL,
    occurred_at TIMESTAMPTZ DEFAULT NOW(),
    CONSTRAINT fk_circle FOREIGN KEY (circle_id) REFERENCES circle_table(id)
);
```

# Query DB Schema

## Circle Read Model Table

```sql
CREATE TABLE circle_read_model (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    capacity INT NOT NULL,
    owner_id UUID NOT NULL,
    members JSONB NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
```
