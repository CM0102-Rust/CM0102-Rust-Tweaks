# regens: a different rule for how regens are distributed (prototype)

*Versão em português: [README.pt.md](README.pt.md)*

A regens-only `ddraw.dll`, built like the save2gb one (`save/`). It replaces the clubs' contest for a youth regen (`PLAYER_REGEN::pick_best_club_for_player`, 0x7ad0e0) with a proportional draw configured in `regens.cfg`. The rest of the game, including how the youngster is created, stays the same. The original rule is in [docs/DISTRIBUICAO_DOS_REGENS.en.md](docs/DISTRIBUICAO_DOS_REGENS.en.md).

The same DLL carries the save2gb **saves above 2 GB** fix (`save_over_2gb = 1` in the `.cfg`, on by default): the three container routines (`zopen` 0x9326e0, `zseek` 0x933080, `ztell` 0x933170) go to the ones in `src/conteiner.rs`, with the same signature as `save/`. So save2gb users swap one DLL for the other, with no need to chain two `ddraw.dll` files. Each part checks its own signature and turns on by itself. In the separate repository's copy, `conteiner.rs` goes in `src/` and the `#[path]` is removed, as in save2gb.

**Status: prototype for discussion.** It builds and checks the signature, but it hasn't run in the game yet. The curve in `regens.cfg` is a starting point, not a final proposal.

## Files

| File | What |
|---|---|
| `src/lib.rs` | The DLL: forwards `DirectDrawCreate` (to `ddraw_chain.dll` or the Windows one), reads `regens.cfg`, checks the signatures and redirects the save container and 0x7ad0e0 |
| `regens.cfg` | The rules; goes next to the exe. `rule = original` turns everything off |
| `tools/calibrar.py` | Out-of-game simulator: a country's real clubs, read from a save, contest several seasons of youngsters under the original rule and the new one |

Build: `cargo build --release` (output in `target/i686-pc-windows-msvc/release/ddraw.dll`).

Signature (FNV-1a of the pieces it depends on: the entry of 0x7ad0e0, its call site and the start of the routines the new one calls) [observed]:
- **matches** `cm0102.exe`, `cm0102-1080.exe`, the portable one and the three CLZ exes;
- **doesn't match** `cm0102_GDI.exe`, which has a different layout.

The body of 0x7ad0e0 is left out of the signature, so an exe with the Regen Tweak (0x7ad1bd) also works.

## The new rule

The eligible clubs are the same as in the original:
- the youngster isn't foreign at the club;
- it isn't a B team;
- it isn't barred by the Basque rule;
- the club can approach the player;
- squad below the limit.

Each club gets a **weight**, and the club is drawn **in proportion to its weight**, using the game's own random generator. No weight at all: the youngster stays clubless, as in the original.

    weight = (level × facilities × staff × room) ^ temperature

| Factor | Formula | Settings |
|---|---|---|
| Level | bell curve of the club's reputation around the target = target_k × PA (or the mix with CA) | `target_k`, `target_ca`, `sigma`, `level_floor` |
| Facilities | facilities_base + facilities_k × Training | `facilities_base`, `facilities_k` |
| Staff | from staff_min to staff_max by the best "Working with Youngsters" among the manager and the 5 coaches. Same for AI and human | `staff_min`, `staff_max` |
| Room | ((squad_limit − squad) / squad_limit) ^ squad_exp; with the squad at the limit, weight 0 | `squad_limit`, `squad_exp` |
| Per-club cap | weight 0 if the club already got `season_cap` regens in the last 365 days (counted in the save itself: the regen bit and the date he joined the club; no new data in the save) | `season_cap` |
| Temperature | exponent: 1 proportional, higher more predictable, lower more random | `temperature` |

With `log = 1`, each pick goes to `regens.log`: PA, club, reputation, Training, chance and how many clubs took part.

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
- In-game proof: an automatic holiday career on the test bench, with the original exe and this DLL, comparing the distribution by band (`tools/regens_por_clube.py`) with the original.
- Test with NickPatcher and CLZ.
