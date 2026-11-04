# CM0102-Rust-Tweaks roadmap

*Versão em português: [ROADMAP.pt.md](ROADMAP.pt.md)*

A tweaks `ddraw.dll` for CM 01/02 3.9.68, with an options picker. When it goes public, this repository replaces `cm0102-save2gb`. Updated on 11/01/2026.

What the community asks for and what's still open: [docs/OPORTUNIDADES.en.md](docs/OPORTUNIDADES.en.md).

Key: ✅ done · 🔨 in progress · ⏳ to do · 🔬 to research · 💡 idea, not decided

## Phase 0: the base ✅

- ✅ The original regen rule decoded ([docs/DISTRIBUICAO_DOS_REGENS.en.md](docs/DISTRIBUICAO_DOS_REGENS.en.md)): one regen per retirement, the contest by the highest random(score), the AI floor, the Training 20 step.
- ✅ The forwarding DLL, with each tweak turned on by a `tweaks.cfg` switch. Without the file, nothing changes.
- ✅ The project rule for modified exes:
  - the edges must match, otherwise the tweak never turns on;
  - a changed body (the Regen Tweak in the CLZ exes, for example) stays with the exe, unless `override_exe_changes = 1`.
- ✅ **Saves above 2 GB** (`save_over_2gb`), the same as save2gb.
- ✅ **No SetupReg and no administrator** (`portable_folder`, `clear_admin_flag`, 02/11/2026, asked by CLZ): the game folder is the exe's own and the `RUNASADMIN` NickDLL writes goes away (`WIN95` stays: without it v6.1 crashes creating a career), without touching NickDLL. Proven on a copy of CLZ v6.1; still to test: the first start with the flag already written (elevated).
- ✅ **Regen ages** from a table (`age_NN`, `contest_max_age`, `ages_out_of_game`).
- ✅ **Younger regens**, petermf23's idea: 22 or older, 6 years younger (`younger_from_age`, `younger_by`).
- ✅ **Clubless only without room** (`unattached = only_if_no_room`): every youngster from a nation in the game goes to the contest; over five seasons, regens without a club fall from 899 to 353 (factory) and from 233 to 77 (CLZ).
- ✅ **The contest by proportional draw** (`rule = new`): level × facilities × staff × room.
- ✅ The tools: `calibrar.py` (both rules with a save's real clubs), `regens_por_clube.py`, `simula_nota.py`.

## Phase 1: regen balance ✅

The configuration the DLL ships with.

**Decided:**

| Switch | Value | Why |
|---|---|---|
| `sigma` | 2500 | a wider level curve, above the target |
| `sigma_below` | 1200 | below the target the curve drops faster: with a single width, in Italy almost half the PA ≥ 140 youngsters went to clubs with reputation under 2000 |
| `staff_min`, `staff_max` | 0.8 and 1.2 | the best youth coach is worth 50% more than the worst; the human club counts the same as the AI |
| `temperature` | 1 | the plain proportional draw; randomness comes from surprise |
| `facilities_base`, `facilities_k` | 0.4 and 0.06 | Training 20 is worth 3.5 times Training 1, with no jump |
| `season_cap` | 2 | at most 2 contest youngsters per club every 365 days |
| `squad_limit` | 48 | big squads stay in the contest; the 2-a-year cap does the braking (Saturn uses 45) |
| `surprise` | 15% | the high-PA youngster born at a small club (Ronaldo at São Cristóvão), only among clubs in a competition; the market moves him later |
| ages | the original table + younger regens | petermf23's idea |
| `retire_idle_players` | on | retiring players nobody wants frees records, and more records mean more regens a year (implemented: 0x7a8050 replaced) |
| `idle_days`, `idle_min_age` | 180 days, age 28 | half a season without a club, from 28 |
| `idle_rep`, `idle_rep_per_year` | 1500, +250 a year | the chance rises with age (reputation 3000: 50% at 28, 100% at 34) |
| `protect_young_unemployed` | 0 | no protection: in a test career, protecting under-23s cut most of the recycling into regens (the original retires hundreds of youngsters clubless for over a year in March) |

✅ **Calibrated** on 11/01/2026 on England, Brazil and Italy (the numbers are in the README, § Calibration). What the simulator can't measure, real squads over a career, goes to the in-game proof.

## Phase 2: the in-game proof 🔨

The tests and each run's numbers: [docs/TESTES.en.md](docs/TESTES.en.md).

- ✅ An automatic holiday career on a test bench, hidden and windowed, with the DLL chained behind the working `ddraw.dll` (video only): each tweak shows in `tweaks.log`, careers get through a season and a half (almost two on CLZ) without hanging, and each regen goes to the log with age, club and chance.
- ✅ The in-game distribution against the original, on the factory database, the official October 2026 one and CLZ: the concentration goes away (at most 3 per club, against 13), as the simulator predicted.
- ✅ **How many more regens a year** with `retire_idle_players`: 772 retirements over five seasons on the factory database (about 150 a year); on CLZ, 9, because RetirePatchV4 blocks almost all of them.
- ✅ **The youngsters who were born without a club** (about 44%): `unattached = only_if_no_room` puts them in the contest; on the 2026 database, all 266 went to a club.
- ✅ Saves: the one above 2 GB loads with the switch and fails without it, on the project's exe and on CLZ. ⏳ Writing a save above 2 GB in a real career, with Lucas.
- ✅ NickPatcher and CLZ exes, with `override_exe_changes` at 0 and at 1.
- ✅ **The long career**: five seasons with 10 leagues (6 playable and 4 background), on the factory database and on CLZ (Nick's rules and ours, with `unattached`), with no crash; the log's `season` line gives the population every July ([docs/TESTES.en.md](docs/TESTES.en.md) §4.3).
- ✅ **Cap bug fixed** (11/02): the cap of 2 per 365 days only counted the squad and leaked when the club loaned out or sold the youngster (15 at Real Madrid in four years); now the DLL keeps every youngster it handed out, and the most per season is 2.
- 💡 **Piling up in nations not in the game:** even with the cap, one club gathers 17 (factory) to 24 (CLZ) regens in five years; on CLZ it's AS Douanes (Senegal, not in the game), where the regen doesn't go through the contest. Invisible to the player; a cap there is a future option.
- ✅ Ten-season careers and variants ([docs/TESTES.en.md](docs/TESTES.en.md) §4.4 and §4.5); `outside_competitions` for squad sizes (11/03, being measured).
- ✅ **Where each regen comes from** (`regen_origins`, 11/03): `regen_origins.csv` with each regen's slot and record, names of the moment, game and computer dates and the career; search and clearing in the configurator (§4.6).
- ⏳ The order of databases: the factory one (reference); CLZ, with Lucas; other community packs, when they're out.
- 💡 Don't cut the reputation of a youngster born without a club (the game cuts it to 1250 and the world one to 1, §4.6).

## Phase 2b: the exe snapshot 🔨

Today the DLL only notices changes in the routines it replaces. On CLZ, NickPatcher changed the caller of the idle rule (the retirement search, 0x7a5790), and that went unnoticed. To document behaviour without depending on other people's exes (Lucas's decision, 11/01/2026), in three layers:

1. ✅ **A snapshot of the whole exe** against the original 3.9.68: code function by function, data (`.rdata`, `.data`) in blocks, and new sections (like `.nick`), with the jumps into them. In the DLL, two snapshots: the file on disk (what the patcher wrote) and memory after the other DLLs load (what changes at run time). First as a standalone tool (`tools/retrato_exe.py`, ready: on CLZ it recognises 85 catalogue patches, among them RetirePatchV4, which zeroed the idle retirements, and lists the 16 thousand unnamed bytes by function), then inside the DLL (`src/retrato.rs`, ready: the `src/referencia.rs` table, generated by `tools/gerar_referencia.py`, holds the hash of the 3.9.68's 11,314 functions and 373 data blocks; the log says "the same code and data as the original 3.9.68" or how many functions and blocks changed, the entry point, new sections and the cases already identified by name; with `log = 1`, the list. On CLZ: 519 functions, 28 blocks, `.nick`, six known cases; in memory, NickDLL changes 1,205 more ranges at run time).
2. 🔨 **Naming the changes** (done outside the DLL: `tools/ler_retrato.py` gives each log line its function, source file and the catalogue patches applied there; the catalogue doesn't go inside the DLL, so Nick's data isn't redistributed; inside it, only the cases this project identified), from NickPatcher's catalogue (the community patches in `MiscPatches.zip`, with bytes and descriptions, and Nick's own options in `Patcher.cs`), read as data and crediting Nick+Co and the authors. Anything not in the catalogue shows as an unknown change, with the function's name.
3. 🔨 **What affects each tweak** (first part ready: if another DLL changes, at run time, code a tweak checked at start, the log warns; on CLZ, NickDLL hooks 0x540a50, the B-team routine the contest calls, and 0x5e8290, which the save container calls; both observed and harmless, now a note). Still to do, the call graph: from the original exe's call graph, who calls and who is called by the routine each tweak replaces; if any of them is changed, the log warns.

## Phase 3: the configurator (options picker) 🔨

The interface design and an interactive mock-up: [docs/CONFIGURADOR.en.md](docs/CONFIGURADOR.en.md) and `docs/configurador_maquete.html` (11/02). **The program prototype** (`configurador/`, Rust + egui, `tweaks-config.exe`) is ready since the night of 11/02 (§10 there): with the folder dialog and preset confirmation; the no-graphics-card test is left. The decisions that depend on Lucas are in §9 there. The exe status panel uses the states of [docs/COMPATIBILIDADE.en.md](docs/COMPATIBILIDADE.en.md): ✅ compatible, ⚠️ overrides (naming the patch), 🔁 left to the other, ❓ unknown.

A single `.exe`, with its own interface (not copying NickPatcher's). It doesn't need to be Tauri, but it should look good and be visual:
- each option with a **slider next to its input field**: drag or type, both move together, within the valid range;
- options grouped by tweak (save, ages, idle players, contest, nationality), with each one's help (the same text as the `.cfg`) on hover or in a side panel;
- visuals where they help: the age table as draggable bars, the level curve drawn, an example of each club's chance that changes as the user drags;
- reads and writes `tweaks.cfg` keeping the comments; the file stays the single source;
- presets ("Original", "Balanced"...);
- explains what "nation in the game" means: the league picked when starting the career, **playable or background** (for regens both count the same; the `NationLeagueSelected` byte is 2 for playable and 1 for background, and the regen code only checks it isn't 0); "not in the game" is a nation whose league wasn't picked. It applies to ages, to `unattached` and to the contest;
- shows, for the chosen exe, what each tweak will do: on, keeps the exe's (naming the change found, like the Regen Tweak) or doesn't fit;
- technology to be decided: a single exe, no installer and no dependency on anything installed (for example, Rust with an interface library that draws by itself).

## Phase 4: more tweaks 💡🔬

- 🔨 **Video, resolution and CD** (Lucas's request, 11/02; done, the visual test is left), three switches off by default, each checking the code's signature and keeping the exe's when it already changes it (CLZ has its own resolution, dynamic backgrounds and NoCD); nothing else goes into the DLL, so as not to conflict with NickPatcher:
  - `video = window`: the project's video layer (the game draws to our DirectDraw and the frame goes to an ordinary window): Print Screen, a resizable window with sharp scaling, nothing changes Windows' video mode. Works at 800×600;
  - `resolution = 1080`: the project's 1920×1080 resolution, applied in memory, together with the 30-row clamp in `Area::init` (without it, an old match's report shows the "Area.cpp 507" box);
  - `no_cd = 1`: the project's NoCD.
  - ✅ On 3.9.68, in hidden careers: the untouched `cm0102.exe` gets past the CD and runs at 1080 through the video layer; `video = window` alone at 800×600; on the exe that is already 1080, "already has it" ([docs/TESTES.en.md](docs/TESTES.en.md) §3.7). On CLZ the DLL does what is possible: it completes what the exe lacks and leaves the CD to LaMOHatesCD and the resolution to 6.1's NickDLL ([docs/COMPATIBILIDADE.en.md](docs/COMPATIBILIDADE.en.md)).
  - ⏳ With Lucas watching the screen: the window, Print Screen, an old match's report at 1080 (the 30-row limit) and, on CLZ, `video = window` (CLZ in a window with Windows' `ddraw.dll` stops at "16 bit colour"; the video layer should fix it), the resolutions and the dynamic backgrounds.

- ✅ **The `ARSE` line in `player_setup.cfg`** (a warning from the CM0102 session and Lucas's request of 11/04): the line marks everyone as a regen. The DLL recognises the career by a human manager with the mark and, in it, `season_cap` doesn't count by the mark; `tweaks.log` says so. Goes in the next version ([docs/TESTES.md](docs/TESTES.md) §3.10).
- 🔨 **Regen names from their own country** (`regen_names = own`, Lucas's request of 11/03). The names are in the database; the rule for when a country uses its own (only above 100) is in the exe. In the October 2026 data update, 100 of 231 nations still borrow; with the minimum of 20, 15 are left, almost all with no names at all (defunct nations). It also fixes the first name taken from the surnames table in language group 0. Goes in the next version. 💡 Later, if needed: a names database for the few nations with fewer than 20.
- 🔨 **Grey players at the club's level** (`grey_players = squad`, Lucas's request of 11/03: "these players must be strong at the team's level, not capped to be bad"; study and code by the CM0102 project's grey-players session, request P9). Off by default; the variation around the level is adjustable (± 5 by default). ✅ Proven on 3.9.68 (three careers to 14/09/2001) and on CLZ 6.1 with NickDLL ([docs/TESTES.en.md](docs/TESTES.en.md) §3.9). Goes in the next version, together with `multiple_instances`. 💡 Later: making a grey player real (click and sign), already studied in the CM0102 project.
- 🔬 **The 27 December hang** (the World Cup loop in `world_cup.cpp`, 0x92ea3b..0x92ea71; the one-byte workaround is Alan_v2's):
  - understand routine 0x92e940: what the 8 five-byte records are, who fills them and why there are national teams left over;
  - reproduce: a diagnostic mode in the DLL logs the whole list when the loop goes past a few rounds; automatic careers with an updated database (the community's 25/26, to be obtained with Lucas) up to 28/12/2001, with several seeds;
  - fix the cause, crediting Alan_v2 for finding the spot.
- 🔬 **Loans** (the market's most repeated complaint; **map in [docs/EMPRESTIMOS.en.md](docs/EMPRESTIMOS.en.md)**: the AI club rejects every loan bid in the bid response, and the prediction only accepts a listed player with enough reputation): map, in the routines already rewritten in Rust, who decides (the club that refuses, the player with "no intention of going", holiday mode getting stuck), log every offer and its outcome in an automatic career, and only then propose configurable rules.
- 🔬 **Club economy:** attendance and stadiums that don't grow with success, rare sponsorship, lower attendance in cups, AI not investing in facilities. Same method: map the routines (match income, finance events and stadium works are already in Rust in the project; see `OBRAS_DO_ESTADIO.md` there), measure and propose.
- 🔬 **Regen nationality, with defined rules** (to be decided with Lucas). There are two rules in use today [observed in the code]:
  - **the original** (0x7abcd7..0x7abd1d): if the youngster's nation has fewer than 100 people in the database (+0x88), 1 time in 4 he gets the nationality of another random nation, also a small one;
  - **the CLZ exes'** (author to be confirmed): every youngster with PA 120 or more gets the nationality of a random nation among those with reputation 5632 or more (+0x8e); the others keep their own. It concentrates the stars in the big nations.
  - The idea is a `nationality` tweak with modes and switches: the original; "strong nations" like CLZ, with adjustable minimum PA and reputation; and a draw proportional to the nation's reputation, with a chance of keeping the original nation. The code sits inside 0x7abac0 and would be replaced like the age block, respecting exes that already change it (CLZ).
- 🔬 **Staff develop for longer**, petermf23's idea (item 18 of his Polish divisions mod for 3.9.60):
  - managers, assistants, coaches, scouts and physios keep developing until about 55, instead of about 40;
  - improves the quality and variety of staff in long-term saves;
  - to do: find the threshold in the code (the development of non-player attributes), check the routine and expose the age in the `.cfg` (`staff_development_age`), crediting him.
- 💡 **Youngsters born without a club** go through the draw, only among lower-division clubs. Depends on the phase 2 measurement.
- 💡 **Facilities on PA**: on signing, PA + (Training − 10) × 0.5 ± 2. Off by default.
- 🔬 **Creating new records** to get more regens than retirements: growing `player.dat` and the per-person tables. A research project, with risk to the save; not in the first version.

## Phase 5: the release ⏳

- ⏳ Remove the private project's paths from the documents.
- ⏳ Release: a zip with `ddraw.dll`, `tweaks.cfg` and the READMEs, and the source in the repository.
- ⏳ `cm0102-save2gb` points here.
- ⏳ Forum post, with the draft approved by Lucas first.
- ⏳ Credits: xeno and Saturn (the Regen Tweak), petermf23 (younger regens and staff development), Nick (NickPatcher, the references), and the champman0102 community.

## Phase 6: the youth intake 🔬

FM's "youth intake": a batch of youngsters joins every club's academy each year, instead of one regen per retirement. It's the big goal of this repository, and it's ours alone: it means creating people, controlling the save and balancing the population. Decided with Lucas on 11/01/2026.

**What we don't do:** raise the 50-player limit. The 50 slots are inside the club record (+0xd7), and the rest of the record (coaches from +0x19f on) is read by fixed position across the whole game, the save and the editors. The intake works within the 50: a typical club has 30 to 40 players today, and the AI makes room.

**The pieces the game already has:** `regenerate_as_youth_player` (0x7abac0) turns a record into a youngster; `player_regen_sign_for_club` (0x7a6f10) puts him in a squad slot, with a contract. What's missing is the raw material, free records: the people tables have a fixed size, allocated on load.

1. 🔬 **Proof of concept**
   - **A pool of records at career creation.** The game already makes extra people when it creates a career: 1% (2% with an option), through a constant at 0x524dcf/0x524de1 (`database_trim_staff_tables` 0x524d70). The DLL raises that number and marks those records as the intake pool; the tables are born with room, and the save stores the counts. The proof: a career with 10% more people is created and runs stably on the test bench (time per day, memory, save).
   - **Our own item inside the save.** The DLL controls the container (the save2gb routines); the game only reads the items it knows. The proof: write and read a DLL item (the pool, the intake dates) and have the game load the save normally, with and without the DLL.
2. ⏳ **The intake:** on a date per nation, each club gets N youngsters from the pool, with N from facilities, the youth coach and a draw. PA comes from its own distribution (facilities 20 really produce more talent). All in the `.cfg`.
3. ⏳ **Squad room:** the AI releases, loans or retires the surplus; whoever ends up clubless returns to the pool through the idle rule. The human club gets the intake as a news item, and the youngsters appear in the squad.
4. ⏳ **Population balance:** intake, retirement and recycling into the pool, so the career doesn't swell decade after decade; measured in the simulator and in long careers on the test bench.

**Impacts to measure:** more people (day processing); bigger saves (already solved above 2 GB); too much talent in the world (PA inflation, national teams); a market flooded with youngsters (wages and fees); editors and tools (bigger counts, same format).

**Limit:** it applies to new careers, since the pool is born at creation. In an old save, the intake could only use recycled records.
