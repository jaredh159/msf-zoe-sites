# msf-zoe

MSF and Zoe websites. (8/2025+)

## add teaching

To access the teaching upload form at `/add-teaching-5b2e3090`, you must set an
authentication cookie in the browser console:

```javascript
document.cookie = 'auth=<token>; path=/; expires=Fri, 31 Dec 2037 23:59:59 GMT';
```

## deps

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
