# CM0102-Rust-Tweaks

*Versão em português: [README.pt.md](README.pt.md)*

Tweaks for **Championship Manager 01/02 (3.9.68)** in a `ddraw.dll`, without changing the exe. Made with AI, from the Rust rewrite of the game (the CM0102 project). It runs with the original exe or the NickPatcher ones; each tweak checks the exe's code before turning on.

Each tweak has its own switch in `tweaks.cfg`, next to the exe. **They all start off:** without the `.cfg`, or without the line, the DLL changes nothing in the game. A configurator (the options picker) will read and write that same file.

| Switch | What it does |
|---|---|
| `save_over_2gb = 1` | **Saves above 2 GB** (up to 4 GB), the same fix as save2gb: the three container routines (`zopen` 0x9326e0, `zseek` 0x933080, `ztell` 0x933170) go to the ones in `src/conteiner.rs` |
| `ages = custom` | **Regen ages** from a table (§ Ages) |
| `rule = new` | The **clubs' contest** for a youth regen (`PLAYER_REGEN::pick_best_club_for_player`, 0x7ad0e0) becomes a proportional draw (§ The new rule). The original rule is in [docs/DISTRIBUICAO_DOS_REGENS.en.md](docs/DISTRIBUICAO_DOS_REGENS.en.md) |

When this repository goes public, it replaces `CM0102-Rust/cm0102-save2gb`, which will point here: every future change lives here. That's why the save fix is in this DLL; with a single DLL, nobody needs to chain two `ddraw.dll` files.

The plan is in [ROADMAP.md](ROADMAP.md).

**Status: prototype for discussion.** It builds and checks the signatures, but it hasn't run in the game yet. The regen curve is a starting point, not a final proposal.

## Files

| File | What |
|---|---|
| `src/lib.rs` | The DLL: forwards `DirectDrawCreate` (to `ddraw_chain.dll` or the Windows one), reads `tweaks.cfg`, checks the signatures and turns each tweak on |
| `src/conteiner.rs` | The saves above 2 GB container (a copy of `rust/src/conteiner.rs` from the CM0102 project; stable since save2gb) |
| `tweaks.cfg` | The options, in English and commented (it's the community's file); goes next to the exe |
| `tools/calibrar.py` | Out-of-game simulator: a country's real clubs, read from a save, contest several seasons of youngsters under the original rule and the new one (with the `.cfg` age table) |
| `tools/simula_nota.py` | The document's made-up clubs example (§7) |
| `tools/regens_por_clube.py` | Regens per club in a save, by reputation and Training band |
| `tools/sav.py` | Reads the items of a `.sav` |

Build: `cargo build --release` (output in `target/i686-pc-windows-msvc/release/ddraw.dll`).

Each tweak checks two signatures (FNV-1a) before turning on:
- **the edges**, what the hook uses (the entry, the return point, the routines it calls). If they don't match, the tweak never turns on;
- **the body**, the original routine the tweak replaces. If only the body differs, the exe already has another change there and, **as in the rest of the project, the exe's routine stays**, with a note in `tweaks.log`. `override_exe_changes = 1` overrides it.

On the exes we have [observed]:

| Exe | Save | Ages | Ages (not in the game) | Contest |
|---|---|---|---|---|
| `cm0102.exe`, `cm0102-1080.exe`, portable | on | on | on | on |
| the three CLZ exes | on | exe's | exe's | exe's (Regen Tweak) |
| `cm0102_GDI.exe` | never | never | never | never |

"Exe's" becomes "on" with `override_exe_changes = 1`. The log names the xeno/Saturn Regen Tweak when it finds it.

## Ages

The youngster's age is drawn in a block of `regenerate_as_youth_player` (0x7ac597..0x7ac6f6), which also caps CA by age. With `ages = custom`, the DLL replaces that block with a small machine-code stub that calls `sortear_idade`: the age comes from the `.cfg` table `age_NN = chance, CA cap` (ages 14 to 35), and that age's CA cap is applied, except `ca_cap_skip`% of the time. The age then goes through the rest of the routine as in the original (birth year, attribute adjustments by age). And `contest_max_age` (default 18) replaces the "under 19 enters the contest" cut (the byte at 0x7ace69).

- **Younger regens, the idea of petermf23** (item 14 of his Polish divisions mod for 3.9.60): a youngster drawn at `younger_from_age` (22) or older is made `younger_by` (6) years younger (22 → 16, 26 → 20), so he gets a development period. The rule applies after the draw, in the table and for nations not in the game. In playable nations, 22 or older is very rare (about 0.2%); for nations not in the game, the original draws mostly 23 to 27, and that's where the rule matters (and the younger age also leads to the CA formula for younger players). There, with `ages_out_of_game = original`, the DLL reproduces the original draw and applies the rule. On in the shipped `.cfg`; `younger_by = 0` turns it off.
- The `.cfg` default table is the original distribution: 16 to 20 almost always (17 and 18 at 29% each), 15 and 21 to 26 very rare.
- A deliberate difference: the CA cap becomes one value per age. In the original, the cap at 17 and 18 depends on the band that was drawn (80 or 90, 100 or 110).
- Nations not in the game with reputation < 350 have their own draw in the original: mostly 23 to 27, sometimes 17 to 23, and there the age picks the CA formula (younger, lower CA), with no cap. With `ages_out_of_game = table` they use the table too (0x7abeb1..0x7abeea); the CA formula by age stays the original one, and the table's caps don't apply there. The default is `original`.
- **The CLZ exes already change the ages** [observed in the bytes]: the very rare 15-to-18 band becomes 14 to 17 (0x7ac5b1), the rare 21-to-26 band becomes 20 to 25 (0x7ac699), and nations not in the game use the playable nations' draw (`jmp` at 0x7abeab). They also carry the full Regen Tweak (0x7ad1bf `0x2d` and 0x7a6f48 `jl`) and another change to the youngsters' nationality switch (0x7abcd7..0x7abd1c). By default the DLL leaves those routines to the exe.

## The new rule

The eligible clubs are the same as in the original:
- the youngster isn't foreign at the club;
- it isn't a B team;
- it isn't barred by the Basque rule;
- the club can approach the player;
- squad below the limit.

Each club gets a **weight**, and the club is drawn **in proportion to its weight**, using the game's own random generator. No weight at all: the youngster stays clubless, as in the original.

**The random sequence changes:** the new rule uses 2 draws from the generator per youngster; the original uses 1 per eligible club (and sometimes more, in the score bonuses). The age table also uses other draws. For a mod that's expected, but a career with the same seed (`-seed`) won't come out the same as the game without the DLL.

    weight = (level × facilities × staff × room) ^ temperature

| Factor | Formula | Settings |
|---|---|---|
| Level | bell curve of the club's reputation around the target = target_k × PA (or the mix with CA) | `target_k`, `target_ca`, `sigma`, `level_floor` |
| Facilities | facilities_base + facilities_k × Training | `facilities_base`, `facilities_k` |
| Staff | from staff_min to staff_max by the best "Working with Youngsters" among the manager and the 5 coaches. Same for AI and human | `staff_min`, `staff_max` |
| Room | ((squad_limit − squad) / squad_limit) ^ squad_exp; with the squad at the limit, weight 0 | `squad_limit`, `squad_exp` |
| Per-club cap | weight 0 if the club already got `season_cap` regens in the last 365 days (counted in the save itself: the regen bit and the date he joined the club; no new data in the save) | `season_cap` |
| Temperature | exponent: 1 proportional, higher more predictable, lower more random | `temperature` |

With `log = 1`, each pick goes to `tweaks.log`: PA, club, reputation, Training, chance and how many clubs took part.

## Calibration (England, `TesteTelas.sav`, 200 simulated seasons)

Parameters:
- 687 clubs;
- 52 youngsters per season in the contest (in `Valencia.sav`, England got ~88 regens per season, and ~56% go into the contest);
- PA drawn from English players aged 31 or over;
- squads reset to the save's each season [simplification].

| Reputation | Clubs | Original: youngsters per club per season | New (default) | Original: PA ≥ 140 | New: PA ≥ 140 |
|---|---|---|---|---|---|
| 0–999 | 499 | 0.00 | 0.03 | 0.000 | 0.000 |
| 1000–1999 | 83 | 0.09 | 0.05 | 0.000 | 0.000 |
| 2000–2999 | 24 | 0.40 | 0.14 | 0.000 | 0.001 |
| 3000–3999 | 21 | 0.28 | 0.28 | 0.002 | 0.004 |
| 4000–4999 | 27 | 0.22 | 0.42 | 0.020 | 0.019 |
| 5000–5999 | 13 | 0.27 | 0.40 | 0.038 | 0.039 |
| 6000–6999 | 13 | 0.54 | 0.37 | 0.083 | 0.068 |
| 7000–7999 | 3 | 1.26 | 0.31 | 0.110 | 0.140 |
| 8000–8999 | 2 | 1.45 | 0.21 | 0.083 | 0.102 |
| 9000+ | 2 | 1.96 | 0.09 | 0.138 | 0.062 |

Under the original, Liverpool, Arsenal and Manchester United get ~2 per season each, mostly ordinary youngsters. Under the new rule, the big clubs get fewer youngsters overall, but their share of the PA ≥ 140 ones is similar (less at 9000+, because those have 47 to 48 in the squad). Mid clubs, 4000 to 6999, get more.

Two attempts dropped [observed in the simulator]:
- **5% level floor and squad limit 45:** the big clubs dropped to zero (they were above 45) and about 30 of the 52 youngsters went to the 499 amateur clubs. With hundreds of small clubs, a small floor added up becomes the biggest slice of the draw.
- **Temperature 2:** concentrates more on clubs of the right size; the 7000+ clubs get twice the PA ≥ 140 youngsters.

Run: `python tools/calibrar.py TesteTelas.sav Inglaterra --humano Fulham --temporadas 200 [--cfg other.cfg]`.

## To discuss

1. **The target.** Where the youngster "should" go: by the level he'll reach (PA, the default) or the one he has now (CA, `target_ca`)? A target of 50 × PA puts a PA 180 youngster at a 9000 club.
2. **The width (`sigma`).** At 1500, a club 3000 away from the target gets ~14% of the top weight. Narrower: fewer surprises.
3. **Big clubs and stars.** How many of the high-PA youngsters should the big clubs take? Right now the new rule keeps their share close to the original.
4. **The weight of facilities.** Right now Training 20 is worth 2.7 times Training 1 (1.5 against 0.55). Stronger?
5. **Staff.** Right now it ranges from 0.8 to 1.2. The human club counts the same as the AI, through its coaches. Stronger?
6. **The per-club cap** (3 per 365 days) and the squad limit (50).
7. **Randomness (`temperature`).**
8. **Outside the prototype:**
   - the ~44% of youngsters born without a club could go through the same draw with a lower weight (an adjustment after the daily queue, 0x7a65d0);
   - the youngster's PA could shift a few points with the Training of the club that gets him (on signing, 0x7a6f10).

## Next steps

- Settle the curve in the discussion.
- The configurator: an `.exe` that reads and writes `tweaks.cfg` (keeping the comments), checks the ranges, offers presets and shows, for the chosen exe, what each tweak will do (on, keeps the exe's, or doesn't fit).
- At release: `cm0102-save2gb` will point here.
- In-game proof: an automatic holiday career on the test bench, with the original exe and this DLL, comparing the distribution by band (`tools/regens_por_clube.py`) with the original.
- Test with NickPatcher and CLZ.
