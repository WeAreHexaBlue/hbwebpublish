# HexaBlue Website News Publisher

Small tool to publish articles on the HexaBlue website's "News" page.

## Usage

```sh
hbpub init [dir]
hbpub publish [dir] [--preserve]
hbpub delete <slug> <locale>
```

`init` creates a `meta.toml`, `body.html`, `short.html`, and `images/` directory. `publish` sends the article to the HexaBlue news API. `delete` removes an article by slug and locale.

## Polls

Articles can optionally include polls. Add one or more `[[polls]]` tables to `meta.toml`:

```toml
[[polls]]
key = "favorite-project"
question = "Which project do you like most?"
options = ["dibsy", "Server Eggs"]
ends_at = "2026-12-31T23:59:59Z"
```

- `key`: lowercase letters, numbers, and hyphens only. Must be unique per article.
- `question`: at most 200 characters.
- `options`: 2 to 12 unique labels, each at most 120 characters.
- `ends_at`: optional ISO-8601 date. A date in the past means the poll is already finished.
- An article can have at most 10 polls.

Polls are optional. If `meta.toml` has no `[[polls]]` tables, the `polls` field is omitted from the request entirely, so any existing polls on the article are left untouched.

Polls are upserted by `key`. Options are matched by position, so reducing the number of options deletes the trailing options and their votes. There is currently no API to delete a single poll; deleting the article also deletes its polls.

### Inline placement

By default a poll renders after the article body. To place a poll inline, put its marker on its own line/block in `body.html`:

```html
<poll key="favorite-project"></poll>
```

The marker must not be inside a `<p>` (or any other inline context); place it between blocks. `body.html` is sent to the API verbatim, so each marker must match the `key` of a poll defined in `meta.toml`. Polls without a marker still render after the article body.
