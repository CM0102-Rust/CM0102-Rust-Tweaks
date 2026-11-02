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

The hook chains to LaMOHatesCD's on CLZ and changes nothing in the game (loads and careers with it on ran normally).

### 3.6 No SetupReg and no administrator (CLZ v6.1)

A copy of CLZ 2026 V6.1 (the V5.3 exe, the group's new NickDLL, untouched), with the registry `Location` pointing to another install (the V5.4 one) [measured, 02/11/2026]:
- `portable_folder = 1`: the log says `portable folder: on (... Location is this folder: <the copy>)`, and the game opens and writes in the copy;
- `clear_admin_flag = 1`: NickDLL writes `WIN95 RUNASADMIN` for the copy's exe and the DLL removes `RUNASADMIN` at the first `DirectDrawCreate`; the next start doesn't ask for elevation. `WIN95` stays: without Windows 95 mode, v6.1's NickDLL crashes creating a career (heap fault in `ntdll`, right after `bra_setup Brasileiro Sub-21 B`, four times, with and without this DLL); with `__COMPAT_LAYER=WIN95 HIGHDPIAWARE`, not elevated, the career is created and saved (suggested by the CLZ team);
- windowed, with Windows' `ddraw.dll` CLZ stops at "Unable to initialise the graphics in windowed mode ... 16 bit colour" (box in the log): the test uses the working `ddraw.dll` as `ddraw_chain.dll`, for video only (`CM_SEM_DESVIOS=1`).

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
- **What piling up is left comes from the market.** Even with the cap, one club gathers 17 regens in five years on the factory database and 24 on CLZ (47 and 27 without the tweaks): youngsters signed later, which the contest doesn't control. A second CLZ career, with the club's name in the `season` line, is running to say who it is. [inference, to confirm]
- **Idle players:** 772 retirements in five years on the factory database; 9 on CLZ, where RetirePatchV4 blocks almost all of them (the log warns).
- **Stability:** all five careers reached the date, from 1 h 23 min to 2 h 11 min each, with no crash, hang or error box.

## 5. Limits and next tests

- **Five seasons, one career per setup.** Still to do: repeat to see the variation between careers, and go to 10 seasons. National teams aren't in the `season` line yet.
- **One career each:** the game is random; the big numbers (the cap, the concentration) come from the rule, the small ones (the top PA) vary between careers.
- **Piling up through the market** (§4.3): find out who gathers the signed regens and whether a tweak is worth it.

## 6. How to reproduce

- `tweaks.cfg` with `log = 1`; `tweaks.log` appears next to the exe.
- The automatic career: `auto_setup.cfg` in the game folder with `LANGUAGE=ENG_UK`, `DATABASE=MINIMUM`, `FOREGROUND_LEAGUES=ENG BRA ITA` (in the long ones: `ENG BRA ITA ESP GER FRA` and `BACKGROUND_LEAGUES=ARG POR NED SCO`), `ENABLE_MATCH_ENGINE=NO`, `ENABLE_COMPUTER_BIDS=YES`, `ON_HOLIDAY=YES` (keep a copy of `game.cfg` first and delete `auto_setup.cfg` afterwards).
- Each run's raw logs (named as in the tables: `clzLonga2.log`, `oficialSemVaga.log`, `longaOriginal.log`, `longaLigadoV1.log`, `longaLigado2.log`, `longaClzNick.log`, `longaClzNosso.log`...), the `.cfg` files used and the test runner (`prova_tweaks.py`) are in `resultados/`, outside git.
- `tools/comparar_logs.py` compares two logs; `tools/temporadas.py` builds the long careers' per-season table; `tools/ler_retrato.py` names the snapshot lines; `tools/regens_por_clube.py` reads a save's regens.
