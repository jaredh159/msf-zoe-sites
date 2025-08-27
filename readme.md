# msf-zoe

> MSF and Zoe websites. (8/2025+)

## deployment, infra

Deployed on a $4/mo DigitalOcean droplet, the MSF (not Gertrude) account.

Domains are pointed at cloudflare for caching. Cloudflare account is using the email
addres/account I created when messing around with Tech Lockdown:
`jared.thomas.henderson+techlockdown@gmail.com`

Both domains are on the msf aws account, `jared+msfaws@netrivet.com`.

https://dash.cloudflare.com/4927bfdc7fcb7265c45b69b977d6bb4a/home/domains

To deploy, run `just deploy`, but changes will lag behind CF cache. You can manually purge
it from the CF dashboard.

## rust version

Created w/ rust 1.89.0, which is installed on the VM, because cross-compilation wasn't
working due to C deps from aws or something. This should be pinned to only use 1.89.0
because I'd rather not update rust on the server, as there is very little space on the
disk. If you do need to update rust, you might have to clear out the target/ dir to free
up space.

## add teaching

To access the teaching upload form at `/add-teaching-5b2e3090`, you must set an
authentication cookie in the browser console:

```javascript
document.cookie = 'auth=<token>; path=/; expires=Fri, 31 Dec 2037 23:59:59 GMT';
```

## edit teaching db records

ssh into server, then: `sqlite3 --box teachings.db`

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
