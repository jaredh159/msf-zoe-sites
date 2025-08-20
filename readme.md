- FileServer https://api.rocket.rs/v0.5/rocket/fs/struct.FileServer
- RawHtml

# deps

- tailwindcss cli, used a standalone release:
  https://github.com/tailwindlabs/tailwindcss/releases/download/v4.1.12/tailwindcss-macos-arm64

## db

schema:

```sql
CREATE TABLE teachings (
    id        INTEGER PRIMARY KEY AUTOINCREMENT,
    title     TEXT NOT NULL,
    speaker   TEXT NOT NULL,
    context   TEXT,
    filename  TEXT NOT NULL,
    filesize  INTEGER NOT NULL,
    duration  INTEGER NOT NULL,
    date      TEXT NOT NULL
) STRICT;
```
