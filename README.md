# `jig` - A CLI Toolbox

[![Ask DeepWiki](https://deepwiki.com/badge.svg)](https://deepwiki.com/daveio/jig)

## Project

`jig` is a utility which collects tools for various tasks into one place. It merges all my disparate tools into a single CLI toolbox, making it easier to manage and use them, and teaches me Rust.

## Command Tree

```mermaid
---
config:
  theme: dark
  layout: elk
---
graph LR
  jig[jig] --> init(init)
  jig --> crypto(crypto)
    crypto --> cryptoEncrypt(encrypt)
    crypto --> cryptoDecrypt(decrypt)
    crypto --> cryptoPublic(public)
  jig --> generate(generate)
    generate --> generateHex(hex)
    generate --> generatePassword(password)
    generate --> generateKey(key)
      generateKey --> generateKeyCrypto(crypto)
      generateKey --> generateKeyWireguard(wireguard)
    generate --> generateJwt(jwt)
  jig --> network(network)
    network --> networkDns(dns)
      networkDns --> networkDnsFlush(flush)
      networkDns --> networkDnsLookup(lookup)
      networkDns --> networkDnsSec(sec)
  jig --> domain(domain)
    domain --> domainCheck(check)
    domain --> domainExpiry(expiry))
    domain --> domainNs(ns)
  jig --> tls(tls)
    tls --> tlsCert(cert)
    tls --> tlsCiphers(ciphers)
  jig --> api(api)
    api --> apiTicket(ticket)
      apiTicket --> apiTicketTitle(title)
      apiTicket --> apiTicketDescription(description)
      apiTicket --> apiTicketEnrich(enrich)
    api --> apiImage(image)
      apiImage --> apiImageAlt(alt)
      apiImage --> apiImageOptimise(optimise)
    api --> apiToken(token)
      apiToken --> apiTokenInfo(info)
      apiToken --> apiTokenRevoke(revoke)
      apiToken --> apiTokenUsage(usage)
    api --> apiPing(ping)
  jig --> mcp(mcp)
  jig --> dance(dance)
  jig --> terminal(terminal)
    terminal --> terminalXKCD(xkcd)
  jig --> project(project)
    project --> projectNew(new)
    project --> projectUpdate(update)
    project --> projectBump(bump)
    project --> projectDependabot(dependabot)
  jig --> git(git)
    git --> gitClone(clone)
    git --> gitBinary(binary)
      gitBinary --> gitBinaryGet(get)
      gitBinary --> gitBinaryUpdate(update)
    git --> gitSecrets(secrets)
    git --> gitCommit(commit)
    git --> gitYank(yank)
    git --> gitLatest(latest)
  jig --> workspace(workspace)
    workspace --> workspaceSwitch(switch)
    workspace --> workspaceList(list)
    workspace --> workspaceHook(hook)
  jig --> ai(ai)
    ai --> aiRename(rename)
      aiRename --> aiRenameImage(image)

  style dance stroke-dasharray: 2 3,stroke-width: 5px
```

## Config

Config is in `yaml` format. We use `saphyr` with `serde` for YAML operations.

### Full Config

```yaml
api: #                          API key configuration. optional.
  domainr: DOMAINR_API_KEY #      def: none. optional.
dns: #                          DNS configuration. optional.
  nameserver: 8.8.8.8 #           def: system resolver. optional.
jwt: #                          JWT configuration. optional.
  env: JIG_JWT_SECRET #           def: JIG_JWT_SECRET. optional.
  key: JWT_SECRET_VALUE #         def: main key. secret for JWTs. optional.
  order: #                        def: env, key. first wins. optional.
    - env #                         top priority
    - key #                         final priority
secret: #                       we use a secret in many places. required.
  env: JIG_SECRET_KEY #           def: JIG_SECRET_KEY. optional.
  file: ~/.jig.key #              def: none. file containing key. optional.
  key: AGE-SECRET-KEY-[...] #     def: generated. unencrypted key. required.
  order: #                        def: env, file, key. first wins. optional.
    - env #                         top priority
    - file #                        second priority
    - key #                         final priority
```

### Minimal Config

```yaml
secret:
  key: AGE-SECRET-KEY-[...]
```

## Library Notes

- `rmcp`
  - <https://hackmd.io/@Hamze/SytKkZP01l>
- `serde`
  - YAML: <https://lib.rs/crates/saphyr>
  - Other formats: <https://serde.rs/#data-formats>
- `spinoff`
  - `noise`: single character fade spinner
  - `aesthetic`: multi character spinner

## Shared Utilities

### `prepare_image_for_claude`

Shrinks images to under 5 MB for Claude compatibility.

- If image is over `2048px` on the long edge, resize to `2048px` on the long edge.
- Convert to WebP format with `image-webp`.
  - Start with lossless compression.
  - If still over 5 MB, use lossy compression with a quality of 90.
  - If still over 5 MB, use lossy compression with a quality of 75.
  - If still over 5 MB, use lossy compression with a quality of 50.
  - If still over 5 MB, use lossy compression with a quality of 25.
  - Abort with an error.
- If successful, return WebP image data as struct or bytes.

### `ask_claude`

- `PROMPT` - Prompt to send to Claude.
- `ASSOCIATED_DATA` - Associated data to send to Claude. Optional.
- `IMAGE` | `FILENAME` - Image data to send to Claude. Optional.

Calls `prepare_image_for_claude` to ensure the image is compatible, then sends the prompt, associated data, and any image to Claude.

## Commands

`clap` supports command shortening to the point of disambiguation.

```rust
// Clap Derive API
#[command(infer_subcommands = true)]
```

### `jig ai`

AI-powered utilities.

#### `jig ai rename`

AI renaming operations.

##### `jig ai rename image`

AI-powered image renaming.

- `[FILENAME_OR_GLOB]`: File or glob pattern to rename images. Defaults to all `*.jpg`, `*.jpeg`, `*.png`, `*.webp` in the current directory. Multiple filenames/globs can be specified.

Flow:

- Use the `ask_claude` utility to send an image to Claude for filename generation.
  - Syntax: `a_few_words-YYYYMMDD-HHMMss.ext`
- Rename the image.
- Throw away any temp files.

### `jig api`

Call the `dave.io` API.

Base URL: `https://dave.io/`

#### `jig api image`

Image processing operations.

##### `jig api image alt`

Generate alt text for images.

Flow:

- Puts the image through `prepare_image_for_claude`.
- Sends it to `/api/ai/alt` via POST.

##### `jig api image optimise`

Optimise image files.

Flow:

- Sends image to `/api/images/optimise` via POST.

#### `jig api ping`

API health checks.

Flow:

- Dead simple. Fetches `/api/ping` and shows the data from the response.
  - The API returns JSON.

#### `jig api ticket`

Ticket management operations.

##### `jig api ticket description`

Generate ticket descriptions from a title.

- `[TITLE]`: Title of the ticket to generate a description for. Required.
- `-i` / `--input`: File containing the title. Optional.
- `-s` / `--stdin`: Read title from `stdin`. Optional.

Flow:

- Calls `/api/tickets/description` with the title.

##### `jig api ticket enrich`

Enrich ticket information.

##### `jig api ticket title`

Generate ticket title from a description.

- `[DESCRIPTION]`: Description of the ticket to generate a title for. Required.
- `-i` / `--input`: File containing the description. Optional.
- `-s` / `--stdin`: Read description from `stdin`. Optional.

Flow:

- Calls `/api/tickets/title` with the description.

#### `jig api token`

Token management operations.

##### `jig api token info`

Get token information.

- `[UUID]`: Token UUID to get information for. Required.

Flow:

- Calls `/api/tokens/$UUID` with the UUID.

##### `jig api token revoke`

Revoke tokens.

- `[UUID]`: Token UUID to revoke. Required.

Flow:

- Calls `/api/tokens/$UUID/revoke` with the UUID.

##### `jig api token usage`

Check token usage.

- `[UUID]`: Token UUID to get usage information for. Required.

Flow:

- Calls `/api/tokens/$UUID/usage` with the UUID.

### `jig crypto`

Encryption and decryption operations.

#### `jig crypto decrypt`

`-i` / `--input`: File of ciphertext to read.
`-o` / `--output`: File of plaintext to write. May be binary.
`-k` / `--key` `[KEY]`: Override key from configuration or env.

Decrypt data using `age` encryption.

Default: ciphertext in via `stdin`, plaintext out via `stdout`, information via `stderr`.

#### `jig crypto encrypt`

`-i` / `--input`: File of plaintext to read. May be binary.
`-o` / `--output`: File of ciphertext to write.
`-k` / `--key` `[KEY]`: Override key from configuration or env.

Encrypt data using `age` encryption.

Default: plaintext in via `stdin`, ciphertext out via `stdout`, information via `stderr`.

#### `jig crypto public`

`-k` / `--key` `[KEY]`: Private key to process

Prints the public key ('recipient' in `age` terms) associated with a private key. Uses the configured private key by default.

### `jig dance`

Easter egg command with terminal effects.

- Not present in `--help`.
- Not exposed via MCP.

See [Easter Egg Nonsense](#easter-egg-nonsense) for more details.

### `jig domain`

Domain management and information tools.

#### `jig domain check`

- `-d` / `--domaincheck`: Shell out to `domaincheck`. Must be installed and on `$PATH`.
- `[QUERY]`: Domain query. Required.

Check domain availability.

Flow:

- Use the Domainr API with RapidAPI credentials.
- Consider only TLDs on the 'Cloudflare TLDs' list.

Improvements:

- Extract logic from `domaincheck` CLI crate.

**Cloudflare TLDs:**

> `ac` `academy` `accountant` `accountants` `actor` `adult` `agency` `ai` `airforce` `apartments` `app` `army` `associates` `attorney` `auction` `audio` `baby` `band` `bar` `bargains` `beer` `bet` `bid` `bike` `bingo` `biz` `black` `blog` `blue` `boo` `boston` `boutique` `broker` `build` `builders` `business` `cab` `cafe` `cam` `camera` `camp` `capital` `cards` `care` `careers` `casa` `cash` `casino` `catering` `cc` `center` `ceo` `chat` `cheap` `christmas` `church` `city` `claims` `cleaning` `clinic` `clothing` `cloud` `club` `co` `co.uk` `coach` `codes` `coffee` `college` `com` `com.ai` `com.co` `community` `company` `compare` `computer` `condos` `construction` `consulting` `contact` `contractors` `cooking` `cool` `coupons` `credit` `creditcard` `cricket` `cruises` `dad` `dance` `date` `dating` `day` `dealer` `deals` `degree` `delivery` `democrat` `dental` `dentist` `design` `dev` `diamonds` `diet` `digital` `direct` `directory` `discount` `doctor` `dog` `domains` `download` `education` `email` `energy` `engineer` `engineering` `enterprises` `equipment` `esq` `estate` `events` `exchange` `expert` `exposed` `express` `fail` `faith` `family` `fan` `fans` `farm` `fashion` `feedback` `finance` `financial` `fish` `fishing` `fit` `fitness` `flights` `florist` `flowers` `fm` `foo` `football` `forex` `forsale` `forum` `foundation` `fun` `fund` `furniture` `futbol` `fyi` `gallery` `game` `games` `garden` `gifts` `gives` `glass` `global` `gmbh` `gold` `golf` `graphics` `gratis` `green` `gripe` `group` `guide` `guitars` `guru` `haus` `health` `healthcare` `help` `hockey` `holdings` `holiday` `horse` `hospital` `host` `hosting` `house` `how` `icu` `immo` `immobilien` `inc` `industries` `info` `ink` `institute` `insure` `international` `investments` `io` `irish` `jetzt` `jewelry` `kaufen` `kim` `kitchen` `land` `lawyer` `lease` `legal` `lgbt` `life` `lighting` `limited` `limo` `link` `live` `loan` `loans` `lol` `love` `ltd` `luxe` `maison` `management` `market` `marketing` `markets` `mba` `me` `me.uk` `media` `memorial` `men` `miami` `mobi` `moda` `mom` `money` `monster` `mortgage` `mov` `movie` `navy` `net` `net.ai` `net.co` `net.uk` `network` `new` `news` `nexus` `ngo` `ninja` `nom.co` `observer` `off.ai` `ong` `online` `org` `org.ai` `org.uk` `organic` `page` `partners` `parts` `party` `pet` `phd` `photography` `photos` `pics` `pictures` `pink` `pizza` `place` `plumbing` `plus` `porn` `press` `pro` `productions` `prof` `promo` `properties` `protection` `pub` `racing` `realty` `recipes` `red` `rehab` `reise` `reisen` `rent` `rentals` `repair` `report` `republican` `rest` `restaurant` `review` `reviews` `rip` `rocks` `rodeo` `rsvp` `run` `sale` `salon` `sarl` `school` `schule` `science` `security` `select` `services` `sex` `sh` `shoes` `shop` `shopping` `show` `singles` `site` `ski` `soccer` `social` `software` `solar` `solutions` `soy` `space` `storage` `store` `stream` `studio` `style` `supplies` `supply` `support` `surf` `surgery` `systems` `tax` `taxi` `team` `tech` `technology` `tennis` `theater` `theatre` `tienda` `tips` `tires` `today` `tools` `tours` `town` `toys` `trade` `trading` `training` `travel` `tv` `uk` `university` `uno` `us` `vacations` `ventures` `vet` `viajes` `video` `villas` `vin` `vip` `vision` `vodka` `voyage` `watch` `webcam` `website` `wedding` `wiki` `win` `wine` `work` `works` `world` `wtf` `xxx` `xyz` `yoga` `zone`

#### `jig domain expiry`

Check domain expiration dates.

- `[DOMAIN]`: Domain to check. Required.

Use RDAP with `rdap-icann-client`.

If RDAP fails, provide a clickable URL for WHOIS.

#### `jig domain ns`

Check nameserver information.

- `[DOMAIN]`: Domain to check. Required.

Use RDAP with `rdap-icann-client`.

If RDAP fails, do a NS lookup.

### `jig generate`

Generation utilities.

`generate` can be deterministic with `-k` / `--keyed` `[name]`

- Uses encryption key and `[name]` to generate deterministic output
- Use `-s` / `--seed` `[value]` to use custom value instead of encryption key
  - Run `argon2` on `[value]` to get data to actually use

#### `jig generate hex`

- `[LENGTH]`: hex length to generate. Defaults to 16 bytes / 32 chars.

Generate cryptographically secure random hexadecimal values.

#### `jig generate jwt`

Generate JSON Web Tokens.

Applies random UUID as token ID using `uuid`.

- `--subject [subject]`: Token subject (e.g., "ai:alt", "api:tokens", required)
- `--description [text]`: Human-readable token desc (default: generated)
- `--expiry [duration]`: Expiration time (e.g., "1h", "7d", "30m", default: 1h)
- `--claim [key=value]`: Add custom claims, can be specified multiple times
- `--secret [secret]`: JWT signing secret (or use config/env)
- `--algorithm [alg]`: Signing algorithm (default: HS256)

Secret priority:

- `--secret`
- JWT secret resolution from config
- Return an error

#### `jig generate key`

Generate cryptographic keys.

##### `jig generate key crypto`

`-s` / `--set`: Sets key in configuration file after generation.

Generate encryption keys for native `age`-based encryption.

##### `jig generate key wireguard`

Generate WireGuard private and public keys.

#### `jig generate password`

- `-e` / `--emoji`: Include emoji. Experimental. Uses a subset of non-ZWJ emoji from the RGI list. Warns user to be able to reset their password if the site doesn't use Unicode for passwords. Emoji count as one character.
- `-x` / `--xkcd`: Use `correct horse battery staple` format from [xkcd](https://xkcd.com/936). Uses `chbs`.
- `[LENGTH]`: password length to generate. Defaults to 16. In `--xkcd` mode, the number of words, defaulting to 4.

Generate cryptographically secure random passwords with a safe alphabet.

Prints password entropy and general security at the end with `zxcvbn` and `chbs`. Repeats until the `zxcvbn` score is above `2`, telling the user what is going on.

A minimum of one item from each of the four (five if emoji is enabled) character sets.

Alphabet: `A-Z`, `a-z`, `0-9`, `@%^-_,.~`

With `--emoji`: Also include single-width non-ZWJ, RGI emoji

Emoji list: 😀, 😃, 😄, 😁, 😆, 😅, 😂, 🤣, 😊, 😇, 🙂, 🙃, 😉, 😌, 😍, 🥰, 😘, 😗, 😙, 😚, 😋, 😛, 😜, 🤪, 😝, 🤑, 🤗, 🤭, 🤫, 🤔, 🤐, 🤨, 😐, 😑, 😶, 😏, 😒, 🙄, 😬, 🤥, 😌, 😔, 😪, 🤤, 😴, 😷, 🤒, 🤕, 🤢, 🤮, 🤧, 🥵, 🥶, 🥴, 😵, 🤯, 🤠, 🥳, 😎, 🤓, 🧐, 😕, 😟, 🙁, ☹️, 😮, 😯, 😲, 😳, 🥺, 😦, 😧, 😨, 😰, 😥, 😢, 😭, 😱, 😖, 😣, 😞, 😓, 😩, 😫, 🥱, 😤, 😡, 😠, 🤬, 😈, 👿, 💀, ☠️, 💩, 🤡, 👹, 👺, 👻

### `jig git`

Git utilities and enhancements.

#### `jig git binary`

Binary file management.

##### `jig git binary get`

Retrieve binary files.

##### `jig git binary update`

Update binary files.

#### `jig git clone`

Enhanced git cloning.

#### `jig git commit`

AI-assisted commit messages.

Reimplement `oco` so we don't have to shell out to it.

#### `jig git latest`

Get latest repository information.

#### `jig git secrets`

Secret scanning and management.

#### `jig git yank`

Yank/remove commits.

### `jig init`

`-c` / `--clobber` : Overwrite existing config (with a new key!)

Creates initial config file. Also sets up / ensures shell integration for `jig workspace`.

### `jig mcp`

Model Context Protocol server functionality.

`jig` will offer a `stdio` [Model Context Protocol (MCP)](https://modelcontextprotocol.org) server, allowing other tools - particularly AI agents - to interact with `jig` and use its features.

This will be implemented 'eventually'.

The MCP tool may be extended to a remote MCP in future, if I figure out how to compile `jig` to WASM and import it from my [personal site and API](https://github.com/daveio/dave-io) at <https://dave.io>. This would also allow me to provide the <https://dave.io/api> endpoints as MCP endpoints too.

### `jig network`

Network utilities and diagnostics.

#### `jig network dns`

DNS operations and utilities.

##### `jig network dns flush`

Flush DNS cache. Detects operating system and runs commands accordingly.

##### `jig network dns lookup`

- `[TYPE]`: Record type, `A`, `MX`, `TXT`, etc.
- `[QUERY]`: Domain to query.
- `--root`: Use root servers.
- `--server`: Use specific nameserver.

Uses system resolver unless `--root` or `--server` are specified.

`--root` and `--server` cannot be specified together.

Perform DNS lookups.

##### `jig network dns sec`

- `[DOMAIN]`: domain to check.

Check DNSSEC configuration for `[DOMAIN]`.

### `jig project`

Project management utilities.

#### `jig project bump`

Bump project versions.

#### `jig project dependabot`

Dependabot configuration.

#### `jig project new`

Create new projects.

#### `jig project update`

Update project dependencies.

### `jig terminal`

Terminal utilities and enhancements.

#### `jig terminal xkcd`

Display XKCD comics in terminal.

### `jig tls`

TLS/SSL utilities and diagnostics.

#### `jig tls cert`

Certificate operations and analysis.

#### `jig tls ciphers`

Cipher suite analysis.

### `jig workspace`

Workspace management and switching.

#### `jig workspace hook`

`[SHELL]`: Shell; we support `bash`, `zsh`, and `fish`.

Used to hook into the user's shell by executing whenever the prompt renders. Not generally called by a human.

#### `jig workspace list`

List available workspaces.

#### `jig workspace switch`

Switch between workspaces.

## Vendoring

Vendoring has been disabled. To re-enable, create `.cargo/config.toml`:

```toml
[source.crates-io]
replace-with = "vendored-sources"

[source.vendored-sources]
directory = "vendor"
```

## Easter Egg Nonsense

- `tachyonfx` as the primary effects engine.
  - `tachyonfx` integrates nicely with `ratatui`.
- `tui-rain` for atmospheric background effects.
- `firework-rs` for explosive moments.
- `rascii_art` to convert a profile photo into animated ASCII art.
- `rusty-termcolor` typewriter effects for text reveals.
- `spinoff` spinners for loading sequences.

### ASCII Art Generation

`rascii_art`

- <https://github.com/UTFeight/RASCII>
- Advanced image to ASCII art converter
- An advanced image to ASCII art tool and crate that supports colored ASCII generation, custom dimensions, and multiple character sets including block, emoji, default, russian, and slight variants

### Atmospheric Effects

`tui-rain`

- <https://github.com/levilutz/tui-rain>
- Rain and atmospheric effects widget
- A simple stateless ratatui widget that generates various rain effects including Matrix rain, normal rain, snow, and emoji floods

### Core Effects Engine

`tachyonfx`

- <https://github.com/junkdog/tachyonfx>
- The primary shader-like effects engine for ratatui applications
- A ratatui library for creating shader-like effects in terminal UIs with color transformations, animations, and complex effect combinations

### Explosive Visual Effects

`firework-rs`

- <https://github.com/Wayoung7/firework-rs>
- ASCII art firework simulator
- A cross-platform ASCII-art firework simulator that provides colorful displays, smooth animations, and a simple particle system

### Progress Indicators

`spinoff`

- <https://github.com/ad4mx/spinoff>
- Terminal spinner library
- An easy-to-use, robust library for displaying spinners in the terminal with over 80 spinner variants, custom colors, and multiple output stream support

### Text Effects

`rusty-termcolor`

- <https://github.com/rusty-libraries/rusty-termcolor>
- Terminal text formatting and effects
- A lightweight terminal manipulation library that provides color manipulation, text effects (typewriter, loading bar, wiggle, matrix), text formatting, and terminal control functions
