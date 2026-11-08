# For NickPatcher users: what works together and what doesn't

*Versão em português: [NICKPATCHER.md](NICKPATCHER.md)*

The Tweaks' compatibility with an exe changed by NickPatcher (or by the community patches that come with it) holds **as far as the changes applied allow**. Before turning on, each tweak checks the part of the exe it uses. If a NickPatcher option already changed that part, the tweak doesn't turn on over it: the exe's version stays, with a warning in `tweaks.log` and in the configurator. The other tweaks turn on normally, and the game is never stopped because of it. With `override_exe_changes = 1`, the ages, contest, idle-player and save tweaks replace the exe's version with the Tweaks', and that NickPatcher option's effect is lost.

The table below crosses NickPatcher's options with each tweak's code, from NickPatcher's own data (the `Patcher.cs` options and the `MiscPatches.zip` patches, read with `tools/exe/nickpatcher_vs_tweaks.py`).

## Options that change the same place (one or the other)

| NickPatcher option | Tweak affected | Without override | With `override_exe_changes = 1` |
|---|---|---|---|
| **Regen fixes** (NickPatcher option) or **Regen tweak 2** (Saturn): no regens to clubs with 45 players or more | the clubs' contest (`rule = new`) | the exe's contest stays (with the 45 limit); ours doesn't turn on | our contest turns on (the squad limit becomes our `squad_limit`) |
| **Regen fixes** or **Younger Regens** (Saturn): skip the age draw of nations not in the game | `ages_out_of_game` | the exe's stays | our hook goes in, but the branch is still skipped: in practice the same as the exe's |
| **Regen CA level reset** (Saturn): changes the regens' age bands and CA level | ages (`ages`) | the exe's stay (Saturn's bands) | our age table turns on; Saturn's CA level reset no longer applies |
| **Android11Patch**: makes the game run on Android 11 (changes the routine that opens saves) | saves above 2 GB (`save_over_2gb`) | the exe's routine stays: saves above 2 GB don't turn on | our save turns on; the Android fix is lost (don't use it on Android) |
| **1280x800** and NickPatcher's other resolutions, **Tapani Space Maker**, **Wales Patch**, **Fix World Cup pre-2000** and **English League North relegation**: use the same exe space as our resolution | `resolution = 1080` | our resolution doesn't turn on (the exe's stays) | — (the resolution has no override: half the changes would break the screens) |

## Options that work together but change the result

| NickPatcher option | What happens to the Tweaks |
|---|---|
| **RetirePatchV4**: nobody retires before 34 | idle retirements (`retire_idle_players`) turn on, but almost nobody reaches them; fewer new regens |
| **NoMoreRetirements**: nobody retires | without retirements no regens are born: ages, the contest, `unattached` and `regen_origins` have nothing to do |
| **AlwaysRegenToNewGoodCountry**: a regen with PA 120 or more moves to a strong nation | works together; the contest uses the new nation, and `regen_origins` records the regen's final nation |
| **Tapani's new regen code** (`tapaninewregencode`, **TurnOffTapaniRegens**, **Regens mega boost**): another attribute generator at the end of regen creation | works together; our ages, contest and `regen_origins` act before or around it; the final attributes are those of Tapani's code |
| **UseCommonNamesForBrazilRegens**, **IncreasePregensMultiplier** | work together (names; players generated at the start of the career) |
| NickPatcher's **NoCD** | works with our `no_cd`: the checks the exe already has stay, the DLL completes the others |

## NickDLL (packs like CLZ)

NickDLL changes the exe in memory after it opens. Those with the "extended resolution" (`"resolucao"` in `NickDLL_config.json`) take care of the resolution: with a resolution chosen there, our `resolution` stays out. It also replaces the B-team routine (the contest uses its version, no problem) and may hook the message box (our `log` chains; to be confirmed in the game). Details in [COMPATIBILIDADE.en.md](COMPATIBILIDADE.en.md).

**Basque regens and the newest NickDLL (November 2026).** The Tweaks contest (`rule = new`) has its own rule for Basque regens: Basque by first or second nationality, and the original's 5 clubs (Athletic and its B team, Real Sociedad and its B team, Alavés) plus the ones the game itself marks as "signs only Basques" (Eibar, Barakaldo, Sestao and 18 more); when choosing a regen's club, a Basque is not a foreigner in Spain. We tested it with the newest NickDLL: the two coexist. With `rule = original`, the choice stays with the exe and whatever sits next to it.

**NickPatcher v2.33** (11/04/2026): only international cups and the 2026 hosts; nothing in common with Tweaks' code.

## How to know what applies to your exe

Open `tweaks-config.exe` in the game folder: the Start tab shows each tweak as **compatible**, **attention: overrides** (naming the NickPatcher option), **left to the other** or **not compatible**, and the yellow band at the top warns when a tweak you turned on doesn't fit your exe.
