# CM0102 Rust Tweaks 1.0

*Versão em português: [LANCAMENTO.md](LANCAMENTO.md)*

A tweaks `ddraw.dll` for Championship Manager 01/02 (3.9.68) and the `tweaks-config.exe` configurator. Nothing changes in the exe: the DLL replaces routines in memory when the game opens, and everything is switched on through `tweaks.cfg`. **Without the `.cfg`, or with everything off, the game is the original.** It works for new games and for saves in progress (from the load on).

## What's in it

| Group | What it does | Keys |
|---|---|---|
| **Save** | saves above 2 GB (up to 4 GB), the same as save2gb | `save_over_2gb` |
| **Install** | the game uses the exe's folder (no SetupReg); clears NickDLL's admin flag | `portable_folder`, `clear_admin_flag` |
| **Screen and CD** | 1920×1080 in memory; the video in an ordinary window (Print Screen, sharp scaling, no change to Windows' resolution); no CD | `resolution`, `video`, `no_cd` |
| **Regen ages** | the age regens are born at, by a table; "younger regens" (petermf23's idea) | `ages`, `age_14`…`age_35`, `younger_from_age`, `younger_by` |
| **Who goes to the contest** | every youngster from a nation in the game goes to the clubs' contest, and is only left clubless if there is no room | `unattached` |
| **The clubs' contest** | the youngster goes to a club his size, by a proportional draw (level, facilities, staff, room), at most 2 per club every 365 days; the weight of clubs outside the competitions | `rule`, `season_cap`, `outside_competitions` and the weights |
| **Idle players** | a clubless player nobody wants retires sooner, and his record becomes a regen | `retire_idle_players` and its limits |
| **Where each regen comes from** | `regen_origins.csv`: each regen, the club he went to, and whom he inherited the slot and the record from (name, age, CA and PA at retirement), with the game date, the computer's date and the career | `regen_origins` |
| **Log** | `tweaks.log` for tests and to send when something goes wrong | `log` |

Each key's guide, for players: [GUIA_DAS_CHAVES.en.md](GUIA_DAS_CHAVES.en.md).

## The configurator

`tweaks-config.exe`, a single exe, in English and Portuguese ([CONFIGURADOR.en.md](CONFIGURADOR.en.md)):
- each tweak's state on the chosen exe: compatible, overrides (naming the patch), left to the other, not compatible;
- the options with slider and field, each one's practical help and the `.cfg`'s technical detail;
- live charts: the ages (draggable bars), the chance of retiring by age, the level curve and the chances per club;
- presets (Original, Save only, Balanced, Balanced with everyone at a club), with the list of what changes;
- the regen history with a search by name and clearing;
- saves only the value of the lines changed and keeps a `.bak`.

## Compatibility

The rule is to do what is possible: each tweak checks its own place in the exe and, if it can't be used, only it stays out, with the reason in the log; the others turn on ([COMPATIBILIDADE.en.md](COMPATIBILIDADE.en.md)). With a NickPatcher exe, compatibility is limited to the changes applied to it: the warning in [NICKPATCHER.en.md](NICKPATCHER.en.md) says, option by option, what works together, what is one or the other and what each choice causes.

| Exe | Status |
|---|---|
| original 3.9.68 and the project's portable/1080 | everything turns on |
| exe with the Regen Tweak (xeno/Saturn) | the contest stays the exe's (or ours, with `override_exe_changes = 1`); the rest turns on |
| CLZ 6.1 (NickPatcher, NickDLL, LaMOHatesCD) | ages and the contest stay the exe's (or ours, with override); the resolution is left to NickDLL when it picks one; the CD to LaMOHatesCD; the rest turns on |

## What was tested

The numbers are in [TESTES.en.md](TESTES.en.md).
- Five- and ten-season automatic careers, with 10 leagues, on 3.9.68 and on CLZ, with no crash: the cap of 2 youngsters per club holds for 10 years; big clubs get 4% of the contest's youngsters (15% under the original) and end with the same stars; regens without a club are halved.
- Saves above 2 GB load on both exes.
- Screen and CD: on the untouched 3.9.68, the 1080 one, a "mixed" copy like CLZ's and one with another patch.
- Regen origins: 44 regens in a career, all with the slot and the record identified.

Not tested yet: the window, Print Screen and the 1080 report while watching the screen; `video = window` and `regen_origins` on CLZ; the career's link with the save's name (the automatic career doesn't save).

## Install and remove

1. Copy `ddraw.dll`, `tweaks.cfg` and `tweaks-config.exe` to the game folder (the one with `cm0102.exe`).
2. Open `tweaks-config.exe`, pick the options and save.
3. To remove: delete `ddraw.dll` (the game is back to the original; saves still work, except those above 2 GB).
