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
- ✅ **Regen ages** from a table (`age_NN`, `contest_max_age`, `ages_out_of_game`).
- ✅ **Younger regens**, petermf23's idea: 22 or older, 6 years younger (`younger_from_age`, `younger_by`).
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
| `protect_young_unemployed` | 23 | under 23 the unemployed rule never retires him (the original could retire a regen nobody signed) |

✅ **Calibrated** on 11/01/2026 on England, Brazil and Italy (the numbers are in the README, § Calibration). What the simulator can't measure, real squads over a career, goes to the in-game proof.

## Phase 2: the in-game proof ⏳

- ⏳ An automatic holiday career on a test bench, with the original `cm0102.exe`, the DLL chained to the Windows `ddraw.dll`, always windowed. Check:
  - that `tweaks.log` shows each tweak on;
  - that the career gets through one or two seasons without hanging;
  - the regens with `log = 1`: each one's age, club and chance.
- ⏳ Compare the distribution by band with and without the DLL (`regens_por_clube.py`) against what the simulator predicted.
- ⏳ Measure **how many more regens a year** with `retire_idle_players`, and the age of those it retires.
- ⏳ Measure **who signs the youngsters born without a club** (about 44%). This decides the phase 4 idea.
- ⏳ Saves: write and load with and without the DLL, and a save above 2 GB.
- ⏳ NickPatcher and CLZ exes, with `override_exe_changes` at 0 and at 1.

## Phase 3: the configurator (options picker) ⏳

A single `.exe`, with its own interface (not copying NickPatcher's). It doesn't need to be Tauri, but it should look good and be visual:
- each option with a **slider next to its input field**: drag or type, both move together, within the valid range;
- options grouped by tweak (save, ages, idle players, contest, nationality), with each one's help (the same text as the `.cfg`) on hover or in a side panel;
- visuals where they help: the age table as draggable bars, the level curve drawn, an example of each club's chance that changes as the user drags;
- reads and writes `tweaks.cfg` keeping the comments; the file stays the single source;
- presets ("Original", "Balanced"...);
- shows, for the chosen exe, what each tweak will do: on, keeps the exe's (naming the change found, like the Regen Tweak) or doesn't fit;
- technology to be decided: a single exe, no installer and no dependency on anything installed (for example, Rust with an interface library that draws by itself).

## Phase 4: more tweaks 💡🔬

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
