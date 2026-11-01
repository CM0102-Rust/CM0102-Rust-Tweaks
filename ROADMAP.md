# CM0102-Rust-Tweaks roadmap

*Versão em português: [ROADMAP.pt.md](ROADMAP.pt.md)*

A tweaks `ddraw.dll` for CM 01/02 3.9.68, with an options picker. When it goes public, this repository replaces `cm0102-save2gb`. Updated on 11/01/2026.

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

## Phase 1: regen balance 🔨

The configuration the DLL ships with.

**Decided:**

| Switch | Value | Why |
|---|---|---|
| `sigma` | 2500 | a wider level curve |
| `facilities_base`, `facilities_k` | 0.4 and 0.06 | Training 20 is worth 3.5 times Training 1, with no jump |
| `season_cap` | 2 | at most 2 contest youngsters per club every 365 days |
| `squad_limit` | 46 | like Saturn (45), one more |
| `surprise` | 15% | the high-PA youngster born at a small club (Ronaldo at São Cristóvão), only among clubs in a competition; the market moves him later |
| ages | the original table + younger regens | petermf23's idea |

**Open:**
- ⏳ **Randomness** (`temperature`, proposed 1, the plain proportional draw): with surprise, it stops being the randomness knob.
- ⏳ **Staff** (`staff_min` and `staff_max`, now 0.8 to 1.2): confirm the weight.
- ⏳ **Calibrate** the full configuration on England, Brazil and a mid-size league (Italy or Portugal), and settle the numbers.

## Phase 2: the in-game proof ⏳

- ⏳ An automatic holiday career on a test bench, with the original `cm0102.exe`, the DLL chained to the Windows `ddraw.dll`, always windowed. Check:
  - that `tweaks.log` shows each tweak on;
  - that the career gets through one or two seasons without hanging;
  - the regens with `log = 1`: each one's age, club and chance.
- ⏳ Compare the distribution by band with and without the DLL (`regens_por_clube.py`) against what the simulator predicted.
- ⏳ Measure **who signs the youngsters born without a club** (about 44%). This decides the phase 4 idea.
- ⏳ Saves: write and load with and without the DLL, and a save above 2 GB.
- ⏳ NickPatcher and CLZ exes, with `override_exe_changes` at 0 and at 1.

## Phase 3: the configurator (options picker) ⏳

A single `.exe`, with its own interface (not copying NickPatcher's):
- reads and writes `tweaks.cfg` keeping the comments; the file stays the single source;
- checks each value's range and offers presets ("Original", "Balanced"...);
- shows, for the chosen exe, what each tweak will do: on, keeps the exe's (naming the change found, like the Regen Tweak) or doesn't fit;
- optional: for the regen curve, the simulator table with a save's clubs.

## Phase 4: more tweaks 💡🔬

- 🔬 **Staff develop for longer**, petermf23's idea (item 18 of his Polish divisions mod for 3.9.60):
  - managers, assistants, coaches, scouts and physios keep developing until about 55, instead of about 40;
  - improves the quality and variety of staff in long-term saves;
  - to do: find the threshold in the code (the development of non-player attributes), check the routine and expose the age in the `.cfg` (`staff_development_age`), crediting him.
- ⏳ **Retire idle players** (`retire_idle_players`, approved on 11/01; the numbers are under discussion): bring forward the retirement of players who have been without a club for a long time, aged 30 or more and with low CA.
  - It frees player records, and more records mean more regens per year.
  - The total population doesn't change, nor does the save format.
  - The retirement rule (0x7a7310) is already decoded in the CM0102 project.
- 💡 **Youngsters born without a club** go through the draw, only among lower-division clubs. Depends on the phase 2 measurement.
- 💡 **Facilities on PA**: on signing, PA + (Training − 10) × 0.5 ± 2. Off by default.
- 🔬 **Creating new records** to get more regens than retirements: growing `player.dat` and the per-person tables. A research project, with risk to the save; not in the first version.

## Phase 5: the release ⏳

- ⏳ Remove the private project's paths from the documents.
- ⏳ Release: a zip with `ddraw.dll`, `tweaks.cfg` and the READMEs, and the source in the repository.
- ⏳ `cm0102-save2gb` points here.
- ⏳ Forum post, with the draft approved by Lucas first.
- ⏳ Credits: xeno and Saturn (the Regen Tweak), petermf23 (younger regens and staff development), Nick (NickPatcher, the references), and the champman0102 community.
