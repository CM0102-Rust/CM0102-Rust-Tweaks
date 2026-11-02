# Tests and results

*Versão em português: [TESTES.md](TESTES.md)*

Everything run to prove CM0102-Rust-Tweaks, from 11/01 to 11/02/2026, with the numbers of each run. It is the basis for the forum post. **[measured]** = taken from the run's `tweaks.log`; **[inference]** = a conclusion from the numbers, still to confirm.

## 1. Summary

| Area | Status |
|---|---|
| The DLL starts, turns its parts on and respects the exe | ✅ on the original 3.9.68, on the exe with the Regen Tweak and on CLZ (NickPatcher + NickDLL) |
| Saves above 2 GB | ✅ on, it loads; off, it stops at the game's error, on both exes |
| CLZ with `override_exe_changes = 1` | ✅ stable: six runs in a row, two of them almost two seasons long |
| The exe snapshot and the dependency warning | ✅ recognises the original, the Regen Tweak and CLZ; finds NickDLL's and LaMOHatesCD's run-time hooks |
| The message boxes in the log | ✅ three real errors caught |
| Regen distribution | ✅ over a season and a half; ⏳ still to do: the effect over several years, with more leagues |

## 2. Method

- **The game's own automatic holiday career** (`auto_setup.cfg` with `ON_HOLIDAY=YES`): the game creates the career and the human manager goes on holiday, with no clicks. `MINIMUM` database, playable leagues England, Brazil and Italy, match engine off, AI bids on.
- The game runs **hidden**, in a 960×540 window, with our working `ddraw.dll` chained behind (`ddraw_chain.dll`) only for video, with none of its routines replaced.
- **`log = 1`** in `tweaks.cfg`: one line per event (`regen`, `birth`, `idle`, `tick` every month, `crash`, `hang`, `msgbox`, `exe`, `runtime`).
- **One game at a time** (the game only allows one copy open), with the lab's memory guard (won't run with under 4 GB free).
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

## 5. Limits and next tests

- **A season and a half doesn't show the balance.** Still to do: the long career (5 to 10 seasons) with saves along the way: squad sizes, clubless players, the population, high-PA regens per year and where they are three years later, national teams.
- **One career each:** the game is random; the big numbers (the cap, the concentration) come from the rule, the small ones (the top PA) vary between careers.
- **More leagues:** the tests had three playable leagues and no background ones; still to do, a career with background leagues (which count as in the game for regens) and more countries.
- **`unattached` on CLZ:** the snapshot shows CLZ doesn't change that code; still to run.
- **Squad sizes** aren't in the log; they'll come from reading the saves.

## 6. How to reproduce

- `tweaks.cfg` with `log = 1`; `tweaks.log` appears next to the exe.
- The automatic career: `auto_setup.cfg` in the game folder with `LANGUAGE=ENG_UK`, `DATABASE=MINIMUM`, `FOREGROUND_LEAGUES=ENG BRA ITA`, `ENABLE_MATCH_ENGINE=NO`, `ENABLE_COMPUTER_BIDS=YES`, `ON_HOLIDAY=YES` (keep a copy of `game.cfg` first and delete `auto_setup.cfg` afterwards).
- `tools/comparar_logs.py` compares two logs; `tools/ler_retrato.py` names the snapshot lines; `tools/regens_por_clube.py` reads a save's regens.
