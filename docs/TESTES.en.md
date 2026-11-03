# Tests and results

*Versão em português: [TESTES.md](TESTES.md)*

Everything run to prove CM0102-Rust-Tweaks, from 11/01 to 11/02/2026 (the five-season careers on 11/02), with the numbers of each run. It is the basis for the forum post. **[measured]** = taken from the run's `tweaks.log`; **[inference]** = a conclusion from the numbers, still to confirm.

## 1. Summary

| Area | Status |
|---|---|
| The DLL starts, turns its parts on and respects the exe | ✅ on the original 3.9.68, on the exe with the Regen Tweak and on CLZ (NickPatcher + NickDLL) |
| Saves above 2 GB | ✅ on, it loads; off, it stops at the game's error, on both exes |
| CLZ with `override_exe_changes = 1` | ✅ stable: six runs in a row, two of them almost two seasons long |
| The exe snapshot and the dependency warning | ✅ recognises the original, the Regen Tweak and CLZ; finds NickDLL's and LaMOHatesCD's run-time hooks |
| The message boxes in the log | ✅ three real errors caught |
| Regen distribution | ✅ over five seasons, with 10 leagues (6 playable and 4 background), on the factory database and on CLZ (§4.3) |
| Stability in long careers | ✅ five careers of five seasons, with no crash, hang or error box |

## 2. Method

- **The game's own automatic holiday career** (`auto_setup.cfg` with `ON_HOLIDAY=YES`): the game creates the career and the human manager goes on holiday, with no clicks. `MINIMUM` database, playable leagues England, Brazil and Italy, match engine off, AI bids on.
- The game runs **hidden**, in a 960×540 window, with our working `ddraw.dll` chained behind (`ddraw_chain.dll`) only for video, with none of its routines replaced.
- **`log = 1`** in `tweaks.cfg`: one line per event (`regen`, `birth`, `idle`, `tick` every month, `crash`, `hang`, `msgbox`, `exe`, `runtime`).
- **One game per folder**, with the lab's memory guard (won't run with under 4 GB free). The game only allows one copy open (the "Championship Manager 2001/02 Process" mutex); in the long careers each folder runs with `TWEAKS_MUTEX` = its own character, an environment variable for the tests only that changes the last letter of the mutex name, and up to three careers ran at once. Without the variable nothing changes.
- **The `season` line** (since 11/02): every July, a picture of the population, only reading memory: players and how many are without a club, squads (minimum, average and maximum in the clubs of running competitions), regens (how many, how many without a club, the most at one club, and which), those with PA ≥ 150 (how many, without a club, at a big club and the club's average reputation). `tools/temporadas.py` builds the per-season table.
- **The exes:**
  - untouched `cm0102.exe` 3.9.68 (snapshot only: it asks for the CD);
  - `cm0102-1080.exe`, the 3.9.68 with the CM0102 project's 1920×1080 resolution: the "original" careers;
  - `cm0102-1080-regentweak.exe`: the same with xeno/Saturn's Regen Tweak applied;
  - the **CLZ** exe (NickPatcher, NickDLL and LaMOHatesCD, 2025 database), a new exe name per run (the `RUNASADMIN` mark NickDLL writes to the registry).
- **The databases:** the 3.9.68 factory one; the update team's official October 2026 one (champman0102.co.uk), unpatched; CLZ's.

## 3. Stability

### 3.1 An exe already modified: the Regen Tweak

The exe with the Regen Tweak changes the contest's body (squad 45 at 0x7ad1bd). **[measured]**
- `override_exe_changes = 0`: the contest part keeps the exe's routine and the log says "keeping the exe's own routine"; the others turn on.
- `override_exe_changes = 1`: the contest switches to the new rule ("overriding it").
- In both cases the game carries on normally.

### 3.2 CLZ

| Run | Tweaks | Reached | Result |
|---|---|---|---|
| clz0, clz0b, clz0c | override 0 | 2025-338 | ✅ no failure (earlier DLL versions) |
| clz1 | override 1 | 2025-260 | ❌ access violation; old DLL, without crash logging, and Windows didn't keep the location |
| clz1b | override 1 | 2026-001 | ✅ |
| clzA | override 1 | 2026-033 | ❌ date stuck until the 15-minute limit; before the hang watchdog, no stack |
| clzB, clzC | override 1 | 2026-082, 2026-077 | ✅ |
| **clzLonga** | override 1 | **2027-185** | ✅ almost two seasons; 287 regens, 746 births |
| clzRef1, clzRef2 | no tweaks (reference) | 2026-093 | ✅ |
| clzD, clzE, clzF | override 1 | 2026-092, 2026-092, 2026-077 | ✅ |
| **clzLonga2** | override 1, with the watchdog, crash logging and message boxes | **2027-187** | ✅ 302 regens, 750 births, no error box |

**Conclusion:** after the two failures, six runs in a row without failure, two of them almost two seasons long. The failures were on earlier DLL versions; if they come back, today's log says where. [measured / inference]

**Idle players on CLZ:** zero retirements by the new rule in every run. Not a bug: CLZ's NickPatcher skips, in the retirement search (0x7a5790), anyone under 34 (RetirePatchV4, jump at 0x7a5806), and almost nobody reaches the rule. The log warns. [measured]

### 3.3 Saves above 2 GB

Test saves with items past 2 GB: the factory database one at 2.55 GB (187 items past 2 GB) and CLZ's (76 items past). **[measured]**

| Exe | `save_over_2gb = 1` | `save_over_2gb = 0` |
|---|---|---|
| 3.9.68 (1080) | loads: 187 items, 66 MB unpacked | game box: `friendly..cpp 4695` |
| CLZ | loads: 76 items, 77 MB | game box: `friendly..cpp 4634` |

A lesson in method: the first loads with the switch off seemed to pass, because the save's date comes from the start of the file and the error box is drawn by the game, invisible to a hidden test. Only logging the boxes (§3.5) told the two cases apart.

### 3.4 The exe snapshot

| Exe | On disk | In memory, after the exe's DLLs |
|---|---|---|
| untouched 3.9.68 | "the same code and data as the original 3.9.68" | nothing changed |
| Regen Tweak | 83 of 11,314 functions and 3 of 373 data blocks; the Regen Tweak named | nothing changed |
| CLZ | entry point in the new `.nick` section (2 MB); 519 functions and 28 blocks; six cases named (Regen Tweak, RetirePatchV4, AlwaysRegenToNewGoodCountry, the skipped branch for nations not in the game, the rare age bands) | NickDLL and LaMOHatesCD change 1,205 more ranges (10,090 bytes) |

From NickPatcher's catalogue (`tools/retrato_exe.py`, outside the game), CLZ has 85 recognised patches. In memory, two hooks fall in code the tweaks check; both were read and are harmless [measured]:
- **0x540a50 → NickDLL**: the B-team routine with the CLZ database's pair list; the contest uses it to leave B teams out, as the exe's own would.
- **0x5e8290 → LaMOHatesCD**: a message-box filter (the CD warning); other errors still show.

### 3.5 The game's message boxes

With `log = 1`, every box goes to the log before it shows. Real cases caught [measured]:

| Box | Cause |
|---|---|
| `Database..cpp 7784` | automatic career in Portuguese on the official October 2026 database: the language change fails (0x5339a0). In English (`ENG_UK`) it runs |
| `friendly..cpp 4695` / `4634` | save above 2 GB without `save_over_2gb` |
| "Insert the CD" | the untouched `cm0102.exe` 3.9.68 asks for the CD |
| `Cup..cpp 1187` | in a ten-season career with the tweaks, at 2001-331 (factory database): `Cup::process_fixture` (0x51a150) didn't find the fixture's home club in the cup tree (`find_tree_slot_for_fixture` returned -1). Code the DLL doesn't touch; the same setup, repeated, ran to 2011 |
| `match_eng..cpp 612` and `652` | on the official October 2026 database: "Only %d home players" (and the away side), a team without enough players for the match. 2 of 8 short careers to January 2002; in one of them, at 2001-248, the tweaks hadn't done anything in the game yet (no retirement, no youngster in the contest). The database has a competition club that starts with 0 players (the `season` line): it's the database, not the DLL |

Since 11/03 the log also records the detail the game builds before some boxes (the `msgbox-detail` line, the text at 0xde1f64), such as the match's teams in `match_eng`.

The hook chains to LaMOHatesCD's on CLZ and changes nothing in the game (loads and careers with it on ran normally).

### 3.6 No SetupReg and no administrator (CLZ v6.1)

A copy of CLZ 2026 V6.1 (the V5.3 exe, the group's new NickDLL, untouched), with the registry `Location` pointing to another install (the V5.4 one) [measured, 02/11/2026]:
- `portable_folder = 1`: the log says `portable folder: on (... Location is this folder: <the copy>)`, and the game opens and writes in the copy;
- `clear_admin_flag = 1`: NickDLL writes `WIN95 RUNASADMIN` for the copy's exe and the DLL removes `RUNASADMIN` at the first `DirectDrawCreate`; the next start doesn't ask for elevation. `WIN95` stays: without Windows 95 mode, v6.1's NickDLL crashes creating a career (heap fault in `ntdll`, right after `bra_setup Brasileiro Sub-21 B`, four times, with and without this DLL); with `__COMPAT_LAYER=WIN95 HIGHDPIAWARE`, not elevated, the career is created and saved (suggested by the CLZ team);
- windowed, with Windows' `ddraw.dll` CLZ stops at "Unable to initialise the graphics in windowed mode ... 16 bit colour" (box in the log): the test uses the working `ddraw.dll` as `ddraw_chain.dll`, for video only (`CM_SEM_DESVIOS=1`).

### 3.7 Video, resolution and CD

Short automatic careers in the October 2026 database's folder (English), without `ddraw_chain.dll` (the video layer is the DLL's own) [measured, 11/02/2026]:

| Exe | Switches | Result |
|---|---|---|
| untouched `cm0102.exe` 3.9.68 (asks for the CD) | `no_cd = 1`, `resolution = 1080` | "no CD: on", "resolution 1080: on", "video: window"; a 1920×1080 frame; the career runs to 2001-244 with no box; the memory snapshot: "no other DLL changed the exe's code" |
| `cm0102-portatil.exe` | `video = window` | an 800×600 frame in the window; to 2001-232 |
| `cm0102-1080.exe` (already has the CD and the resolution) | `no_cd = 1`, `resolution = 1080` | "this exe already has it" for both; only the 30-row limit goes in; to 2001-245 |
| CLZ (the exe's bytes only, not run) | — | CD: 38 of 66 places as in 3.9.68 and 28 already changed (14 of the 33 checks); resolution: 196 as in 3.9.68 and 5 already changed (the same NickPatcher code); the `Area::init` entry the same. With the do-what-is-possible rule, the DLL completes what is missing; the CD is left to `LaMOHatesCD.dll`, and the resolution to 6.1's NickDLL when it has a resolution chosen ([COMPATIBILIDADE.en.md](COMPATIBILIDADE.en.md)) |

After the do-what-is-possible rule (the resolution list completes what the exe already has; the CD check by check) [measured, 11/02 at night]:

| Exe | Result |
|---|---|
| untouched `cm0102.exe` | CD: 66 places changed now; resolution: 201; the 30-row limit; the windowed video; to 2001-216 |
| `cm0102-1080.exe` | CD: 0 now, 66 already in the exe; resolution: 0 now, 201 already in the exe; to 2001-215 |
| a "mixed" copy, like CLZ (14 CD checks and 5 resolution places already changed) | CD: 38 now, 28 already in the exe; resolution: 196 now, 5 already in the exe; the game at 1080; to 2001-215 |
| a "strange" copy (one resolution place with another patch's bytes) | the resolution refuses ("1 of 201 places ... another patch there; nothing was changed"); the CD and the limit go in; the game carries on at 800×600 to 2001-215 |

The test copies are generated by `resultados/misto.py` from `cm0102.exe`, in a test folder, and deleted afterwards.

Still to do: the test with the screen: the window, Print Screen and an old match's report at 1080.

## 4. Regens

### 4.1 Outside the game: the simulator

200 seasons with a save's real clubs, under the original rule and the new one. The numbers are in the [README](../README.md#calibration-simulator-testetelassav-200-seasons): the big clubs stop collecting the ordinary youngsters (England 8000+: from 1.58 to 0.26 a season) and keep their share of the stars (PA ≥ 140: 0.10 in both).

### 4.2 In the game

One career each, from the start date to the date in the table. "Big" = clubs with reputation 7000 or more. **[measured]**

| Career | Database | Reached | Regens in the contest | Clubs | Most at one club | Top 10 clubs' share | To big clubs | Average club reputation: PA < 100 / 100–129 / 130+ |
|---|---|---|---|---|---|---|---|---|
| original | factory | 2003-015 | 679 | 345 | 13 | 11% | 16% | 2861 / 5626 / 6440 |
| tweaks on | factory | 2003-022 | 480 | 343 | 3 | 4% | 4% | 2772 / 4507 / 5188 |
| tweaks on (2nd) | factory | 2003-027 | 466 | 334 | 3 | 5% | 5% | 2595 / 4568 / 6091 |
| original | October 2026 | 2002-180 | 293 | 198 | 8 | 18% | 9% | 2968 / 5413 / 5428 |
| tweaks on | October 2026 | 2002-180 | 152 | 122 | 2 | 13% | 3% | 3313 / 4528 / 5608 |
| on + `unattached = only_if_no_room` | October 2026 | 2002-180 | 266 | 192 | 2 | 8% | 2% | 2929 / 4093 / 5009 |
| CLZ, the exe's rule (Regen Tweak) | CLZ | 2026-093 | 59 and 68 | 40 and 46 | 6 and 4 | 46% and 41% | 2% and 1% | 4139 and 4030 / 5262 and 4929 / none |
| CLZ, override 1 | CLZ | 2027-187 | 302 | 228 | 3 | 9% | 4% | 3228 / 4535 / 7130 |

What the numbers show:
- **The concentration goes away.** Under the original rule, one club gets up to 13 regens in a season and a half and the big clubs take 16%; with the tweaks, nobody goes past 3 (the cap of 2 per 365 days) and the big clubs take 4% to 5%.
- **A youngster goes to a club his size.** The club's average reputation rises with PA under both rules; with the tweaks, the ordinary and middling youngsters spread over the middle clubs, and the high-PA ones still go to strong clubs.
- **19- and 20-year-olds.** With the age table, some youngsters are born older than 18 and, under the original rule of who enters the contest, start without a club: hence the drop from 293 to 152 in the contest on the 2026 database. With `unattached = only_if_no_room`, all 266 born in nations in the game went to the contest (the other 71 are from nations not in the game), spread over 192 clubs, and none was left without a club.
- **Idle players** (`retire_idle_players`): 35 to 37 retirements in the factory careers to 2003 and 16 in the 2026 ones to mid-2002; each record freed becomes a regen weeks later.

### 4.3 Long careers: five seasons, 10 leagues

Playable leagues England, Brazil, Italy, Spain, Germany and France; background Argentina, Portugal, Netherlands and Scotland. Factory database from 2001 to 2006 and CLZ from 2025 to 2030. Tweaks on with `unattached = only_if_no_room`; on the factory database with `protect_young_unemployed = 23` (from an earlier `.cfg`), on CLZ with today's default, 0. **[measured]**

In the factory database's first season, the tweaks career had far fewer regens than the original (318 against 1,119): protecting the under-23s cuts the recycling of clubless youngsters into regens. Over five years the difference goes away (7,573 against 7,476), and the `.cfg` default became 0.

**A bug found and fixed on the way:** the cap of 2 youngsters per club every 365 days only counted the regens still in the squad. A club that loaned out, sold or sent the youngster to its B team got more: in the first tweaks career, Real Valladolid and Real Madrid got 15 each from the contest in four years, almost all through the surprise, because in Spain only the top division runs and the surprise draws among some 20 clubs. Now the DLL keeps every youngster it handed out, with the date, and the cap counts that list (or the squad, whichever is higher: the list doesn't go into the save). "Tweaks v1" is the career with the bug; "v2", the fixed one.

**The contest, summed over the five seasons:**

| Career | Youngsters in the contest | Clubs that got some | Most at one club in a season | Who got most (5 years) | To big clubs | PA ≥ 150 in the contest | Idle retirements |
|---|---|---|---|---|---|---|---|
| original (factory) | 4,518 | 1,488 | 12 | Real Madrid, 26 | 15% | 93 | — |
| tweaks v1 (cap leaking) | 6,909 | 2,162 | 8 | Real Valladolid, 20 | 8% | 165 | 775 |
| **tweaks v2** | 6,955 | 2,188 | **2** | San Martín de Tucumán, 9 | 6% | 177 | 772 |
| CLZ, Nick's rules | 1,947 | 613 | 11 | River Plate, 23 | 14% | 14 | — |
| **CLZ, ours** (override 1) | 3,454 | 1,239 | **2** | ABC (RN), 9 | 5% | 37 | 9 |

(A 3 in one v2 season is two different clubs with the same name, "US Montagnarde", reputations 1000 and 540; each got 2.)

**The population in July of the last year** (the `season` line):

| Career | Players | Without a club | Average squad (max) | Regens | Regens without a club | Most regens at one club | PA ≥ 150: total / without a club / at a big club |
|---|---|---|---|---|---|---|---|
| original, 2006 | 36,518 | 3,220 | 30.9 (50) | 7,476 | 899 | 47 | 230 / 12 / 42 |
| tweaks v1, 2006 | 36,531 | 2,484 | 30.7 (48) | 7,490 | 372 | 18 | 223 / 1 / 66 |
| **tweaks v2, 2006** | 36,574 | 2,509 | 30.6 (48) | 7,573 | 353 | 17 | 244 / 2 / 72 |
| CLZ, Nick's, 2030 | 38,982 | 2,855 | 26.2 (46) | 5,258 | 233 | 27 | 165 / 2 / 18 |
| **CLZ, ours, 2030** | 38,989 | 2,687 | 26.0 (47) | 5,263 | 77 | 24 | 175 / 1 / 28 |

What the numbers show:
- **The cap is exact.** With the fix, no club gets more than 2 youngsters a season from the contest; over five years, the one that got most had 9 (against Real Madrid's 26 under the original and River Plate's 23 under Nick's rule). Big clubs get 5% to 6% (15% under the original, 14% under Nick's).
- **The population stays stable.** The number of players and of regens is the same under both rules, and so is the average squad; the tweaks don't make the world shrink or swell.
- **Fewer youngsters without a club.** With `unattached`, almost every regen from a nation in the game goes to the contest: the contest gets 50% (factory) to 80% (CLZ) more youngsters, spread over 50% (factory) to 100% (CLZ) more clubs, and regens without a club fall from 899 to 353 (factory) and from 233 to 77 (CLZ).
- **The stars still reach the big clubs**, now through the market: PA ≥ 150 regens at big clubs go up from 42 to 72 (factory) and from 18 to 28 (CLZ), even with the big clubs getting fewer from the contest.
- **What piling up is left is in nations not in the game.** Even with the cap, one club gathers 17 regens in five years on the factory database and 24 on CLZ (47 and 27 without the tweaks). The second CLZ career with our rules, now with the club's name in the `season` line, showed who: AS Douanes, of Dakar (Senegal, a nation not in the game), with 8, 13, 16 and 17 regens from 2027 to 2030, none of them through the contest (the club is in no `regen` line). In nations not in the game, the regen follows the original path and doesn't go through the cap; since those clubs play no running competition, nobody signs them and the regens pile up. Invisible to the player; a cap there too is a future option. In the first season the leader was MC Algiers (Algeria, also not in the game).
- **The repeat confirms it.** The second CLZ career with our rules gave the same picture as the first: at most 2 per club each season, big clubs with 4% to 6%, 5,259 regens (69 without a club) and 171 with PA ≥ 150 in 2030.
- **Idle players:** 772 retirements in five years on the factory database; 9 on CLZ, where RetirePatchV4 blocks almost all of them (the log warns).
- **Stability:** all five careers reached the date, from 1 h 23 min to 2 h 11 min each, with no crash, hang or error box.

### 4.4 Ten-season careers (2001 to 2011)

The same 10 leagues, factory database, the original against the tweaks as they ship (today's `tweaks.cfg`, with `protect_young_unemployed = 0`, plus `unattached = only_if_no_room`). Both reached 2011-202, in 2 h 48 min and 2 h 42 min, with no crash, hang or error box. **[measured]**

**The contest, summed over 10 seasons:**

| Career | Youngsters in the contest | Clubs that got some | Most at one club in a season | Who got most | To big clubs | Without a club | PA ≥ 150 in the contest | Idle retirements |
|---|---|---|---|---|---|---|---|---|
| original | 12,257 | 1,894 | 14 | Real Madrid and Milan, 57 each | 15% | 1 | 331 | — |
| tweaks | 18,932 | 2,745 | **2** | Burgos, 19 | **4%** | 84 | 587 | 2,278 |

**The population every July (the `season` line):**

| July | Original: without a club / average squad / regens without a club / most at one club | Tweaks: without a club / average squad / regens without a club / most at one club |
|---|---|---|
| 2002 | 1,008 / 32.5 / — / 7 | 675 / 32.7 / — / 2 |
| 2004 | 1,954 / 32.9 / — / 15 | 1,512 / 32.9 / — / 8 |
| 2006 | 3,168 / 30.9 / 913 / 23 (Tigre) | 2,418 / 30.9 / 367 / 15 (U. Las Palmas) |
| 2008 | 3,544 / 29.9 / — / 34 (Defensores de Belgrano) | 2,278 / 28.9 / 607 / 23 (U. Las Palmas) |
| 2011 | 3,282 / 30.3 / 1,785 / 40 (Tigre) | 2,079 / **27.6** / 869 / 30 (U. Las Palmas) |

In 2011: 18,875 regens under the original and 18,982 with the tweaks; PA ≥ 150: 868 (42 without a club, 245 at a big club) against 867 (9 without a club, 238 at a big club).

What the numbers show:
- **The cap holds for 10 years:** no club goes past 2 youngsters a season from the contest. Under the original, Real Madrid and Milan get 57 each, and one club gets 14 in a season.
- **Big clubs get 4% of the contest's youngsters (15% under the original) and end with the same number of stars** (238 against 245 PA ≥ 150 at big clubs): they reach them through the market.
- **Much less wasted talent:** regens without a club are halved (869 against 1,785), and PA ≥ 150 ones without a club go from 42 to 9.
- **What piling up is left comes from the market:** Universidad de Las Palmas gathers 30 regens, but only 2 came from the contest (the log lists every contest youngster and his club); under the original, Tigre gathers 40, almost all from the contest (30). Since 11/03 the `season` line separates the leading club's regens the contest gave it (`given by the contest`) from those who went through the contest to another club and arrived later by transfer or loan (`via the contest to another club`); before, "from the contest" lumped both together (in the career with `outside_competitions = 0.25`, Getafe had 15 "from the contest", but the contest only gave it 3).
- **Something to tune: the average squad of the competition clubs falls** from 32.9 (2004) to 27.6 (2011) with the tweaks; under the original it stays around 30. The total number of players is the same in both; with the tweaks, fewer free players (2,079 against 3,282) and the youngsters spread over 2,745 clubs, many outside the competitions. The §4.5 variants rule out idle retirements as the cause; a cap of 3 helps a little.
- **Without a club in the contest (84):** from 2007, 16 to 26 youngsters a season find no eligible club in their country (squads at the limit or the season cap): that is `unattached` working as asked ("clubless only without room").

### 4.5 Why squads shrink, and the "sink" club

**The variants (10 seasons each, the rest as in §4.4)** [measured]:

| July | Average squad: tweaks (§4.4) | without idle retirements | cap of 3 a season |
|---|---|---|---|
| 2004 | 32.9 | 32.8 | 32.9 |
| 2006 | 30.9 | 30.8 | 31.3 |
| 2008 | 28.9 | 28.9 | 29.7 |
| 2010 | 27.9 | 28.1 | 29.0 |
| 2011 | 27.6 | 27.8 | 28.7 |

- **Idle retirements are not the cause:** without them squads fall the same way.
- **A cap of 3 holds about one player per squad** (28.7 against 27.6 in 2011), with big clubs still at 4% to 10% of the contest. It doesn't fix it alone. The most likely cause is the youngsters spreading over clubs outside the competitions (2,745 clubs get some in the ten-year career). Lucas's decision (11/03): a new switch, `outside_competitions`, the weight of clubs outside the competitions (1 = as today; 0 = never); a ten-season career with 0.25 is running.

**A regen "sink" club (open):** in several careers with the tweaks, a club outside the competitions gathers many regens that didn't come from the contest (the `season` line shows how many came from it): Universidad de Las Palmas with 30 (§4.4) and, in other careers, Guiseley (English, non-league) with 13 to 49. In the original's careers, the club with most regens got most of them from the contest (Tigre, 30 of 40).

A suspicion, ruled out [measured]: the DLL's `PELA_DISPUTA` list (a `Vec` growing with each youngster on the game's thread) looked like the cause, since Guiseley showed up in the first three careers with it and went away without it. But the final career, with a fixed bitmap instead of the list, also had Guiseley (13 in July 2002). Count: the Guiseley sink appeared in 4 of 8 careers with the new DLL (with and without the list) and in 0 of 2 with the previous one; it's the career's path, which changes with any difference (the game seeds from the clock and reads uninitialised memory), not a bug in the list. The bitmap stayed: not allocating on the game's thread is the right practice.

Still to find out: whether those regens arrive by signing or by loan (on a loan, the player's +0x39 becomes the borrowing club), and whether the same happens under the original unnoticed (the original contest already concentrates so much that the sink doesn't reach the top). For that, a save in the middle of a career with the sink, read with `tools/regens_por_clube.py`.

A read of the running game's memory (from outside, read only; `resultados/deposito.py`), in the §4.5 final career in 2003: Guiseley had 16 players, **all regens**, aged 17 to 21. The non-league club replaced its whole squad with youngsters. Reading the contract's owner didn't give coherent values (the contract layout isn't right yet), so signing against loan is still open.

**And under the original?** One season of the original, read the same way in July 2002: the club with most regens was SC Kleve (Germany), with 6 players, **all regens**, none from the contest. It's the same phenomenon: a small club outside the competitions loses its database squad and rebuilds it with free youngsters. Under the original it hides behind the clubs the contest fills (Tigre with 30 from the contest); with the tweaks the contest is capped and the sink becomes the club with most regens. **It's the game, not the tweaks** [measured in this career; just one]. Guiseley doesn't even exist under that name in this original career: the set of clubs changes from one career to another.

### 4.6 Where each regen comes from (`regen_origins`)

**The proof (11/03), on the factory database**, with the checker `tools/conferir_origens.py`: it checks each regen in `regen_origins.csv` against a save of the same career written later. The regen must be a regen in the save, with the same name, nation and PA, and the PA must be that of the recorded record, the owner's. A deliberate mutation (a PA and a name changed in the CSV) gave 3 errors: the checker discriminates. **[measured]**

| Test | Regens | Unknown owners | Errors | Pool on load / on save |
|---|---|---|---|---|
| 1. New career, July to 12/01/2001, saved | 23 | 0 | 0 | — / 0 |
| 2. Loaded, to 05/30/2002, saved | 550 | 0 | 0 | 0 / 0 |
| 3. Loaded, to 07/29/2002, saved | 265 | 0 | 0 | 0 / 511, all with an owner |
| 4. Test 3 loaded **without** the `.regens`, to 10/07/2002 | 450 | 168 | 0 | 511: 326 owners recovered from the game, 185 unknown |
| 5. Test 3 loaded **with** the `.regens`, to 10/07/2002 | 450 | 0 | 0 | 511: 511 from the file / 77, all with an owner |

What these tests fixed, before the release:
- **The coach path.** A player who retires and stays on as a coach puts his record in the pool through 0x7a8350, whose 1st argument is the **person's index**, not the record. The DLL took that number for the record and almost never found the owner: hence the unknowns in a new career (1 in 32, Mark Evans, in the first test folder).
- **Loading.** In a loaded save, the records already in the pool lost their owner, and mid-year the pool holds hundreds (511 in July). Now, when the game is saved, the DLL writes an `X.regens` next to the save with the owners of the records in the pool and reads it on load. The save doesn't change. Without the file, it recovers the owner the queue wrote in the entry, when he still points to the record (326 of 511); the rest stay "unknown", and the line says why.
- **The career name.** The game writes to a `temp_save_file.sav` and renames it at the end (bit 0x20000000 of `game.cfg`), and `-load` arrives in lower case. The career came from `CreateFileA` and showed "temp_save_file.sav". Now it comes from loading and saving (the entries of 0x82a9c0 and 0x82e430, hooked with a trampoline) and uses the name as it is on disk.
- **A false line.** 0x7abac0 gives up when the person isn't retired or has a club. The DLL only records when the record at the front of the pool really went to the person.
- **Excel.** The CSV has a BOM (UTF-8): it opens with a double click. If the file is open in Excel (locked), the lines wait in memory and go in later; if the game closes first, they go to `regen_origins_pending.txt` and enter at the next start.

How the DLL finds the record's owner (changing nothing in the game or the save): it hooks the two calls through which a retired player's record enters the pool (the queue, 0x7a87d0 at 0x7a64c9 and 0x7a64e2, and the player who becomes a coach, 0x7a8350 at 0x7a6335) and notes, in a fixed DLL table indexed by the record, the owner's name, id, age, CA and PA **at that moment**. At creation (0x7abac0, called at 0x7a664f, the only call), it reads the slot (A) before it's overwritten and the record at the front of the pool, which is always the one the game uses. Freeing the game (0x82ff40) clears the table: another career starts clean. If any of these points differs from 3.9.68, recording doesn't turn on, and `tweaks.log` says where. In the CLZ exe and in the NickPatcher base exe, the points are intact in the file; the 6.1 NickDLL points at the load calls, which is why loading is hooked at the entry. The test in the CLZ game is still missing.

A finding on the way: a youngster born **without a club** and under 20 has his reputation cut by the game (0x7acf61..0x7acf94: current and home at most 1250, world 1), and is almost invisible to the AI. The PA is never touched, and the attribute normalisation (0x538ec0, 0x538bc0) applies to everyone, with or without a club.

## 5. Limits and next tests

- **Five seasons, one career per setup.** Still to do: repeat to see the variation between careers, and go to 10 seasons. National teams aren't in the `season` line yet.
- **One career each:** the game is random; the big numbers (the cap, the concentration) come from the rule, the small ones (the top PA) vary between careers.
- **Piling up in nations not in the game** (§4.3): decide whether a cap there too is worth it.
- **The order of the next databases** (Lucas, 11/03): the 3.9.68 factory database is the reference (the October 2026 one has Manchester City's reserve team short and stops at `match_eng..cpp 652`, without the tweaks too); then CLZ, with Lucas; then Davdav's new patch, when it's out (first `tools/inventario_dll.py`).

## 6. How to reproduce

- `tweaks.cfg` with `log = 1`; `tweaks.log` appears next to the exe.
- The automatic career: `auto_setup.cfg` in the game folder with `LANGUAGE=ENG_UK`, `DATABASE=MINIMUM`, `FOREGROUND_LEAGUES=ENG BRA ITA` (in the long ones: `ENG BRA ITA ESP GER FRA` and `BACKGROUND_LEAGUES=ARG POR NED SCO`), `ENABLE_MATCH_ENGINE=NO`, `ENABLE_COMPUTER_BIDS=YES`, `ON_HOLIDAY=YES` (keep a copy of `game.cfg` first and delete `auto_setup.cfg` afterwards).
- Each run's raw logs (named as in the tables: `clzLonga2.log`, `oficialSemVaga.log`, `longaOriginal.log`, `longaLigadoV1.log`, `longaLigado2.log`, `longaClzNick.log`, `longaClzNosso.log`, `longaClzNosso2.log`, `dezOriginal.log`, `dezAjustes.log`, `dezSemInativos.log`, `dezTeto3.log`, `controleDllNova.log`, `bisect*.log`...), the `.cfg` files used and the test runner (`prova_tweaks.py`) are in `resultados/`, outside git.
- `tools/comparar_logs.py` compares two logs; `tools/temporadas.py` builds the long careers' per-season table; `tools/ler_retrato.py` names the snapshot lines; `tools/regens_por_clube.py` reads a save's regens.
