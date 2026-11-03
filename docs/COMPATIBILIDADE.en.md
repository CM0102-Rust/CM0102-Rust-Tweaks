# Compatibility with other patches

*Versão em português: [COMPATIBILIDADE.md](COMPATIBILIDADE.md)*

What each Tweaks setting does when the exe, or a DLL next to it, has already changed the same code. **The rule is to do what is possible:** each setting checks its own code on its own, and one that can't be used stays out, with the reason in `tweaks.log`. The other settings turn on normally, and the game is never stopped because of it. **[observed]** = seen in the code, the bytes or the log; **[inference]** = to be confirmed in the game.

For NickPatcher users, the option-by-option warning is in [NICKPATCHER.en.md](NICKPATCHER.en.md).

## 1. The states

The same ones the configurator will show ([CONFIGURADOR.en.md](CONFIGURADOR.en.md)):

| State | When | Example |
|---|---|---|
| ✅ compatible | the code is as in 3.9.68, or the exe already has this same change | the save above 2 GB on any exe |
| ⚠️ overrides | the exe has a **known** change there, and `override_exe_changes = 1` says to replace it with ours | the contest over the Regen Tweak |
| 🔁 left to the other | a **known** change does the same job; ours stays out, and the exe's or the other DLL's applies | the resolution of CLZ 6.1's NickDLL; LaMOHatesCD's CD |
| ❓ unknown | the code differs from 3.9.68 in a way the catalogue doesn't recognise; only this setting stays out | someone's new patch |

"Known" means named by the exe snapshot (`src/retrato.rs`, the known cases), by the NickPatcher catalogue (`tools/retrato_exe.py`) or by this document.

## 2. The matrix

| Setting | 3.9.68 (and the CM0102 project's exes) | Regen Tweak (xeno/Saturn) | CLZ 6.1: exe (NickPatcher) | CLZ 6.1: NickDLL and LaMOHatesCD |
|---|---|---|---|---|
| `save_over_2gb` | ✅ | ✅ | ✅ | ✅ NickDLL calls 0x933610 (another container routine), which the setting doesn't replace |
| `portable_folder`, `clear_admin_flag` | ✅ | ✅ | ✅ | ✅ made for CLZ |
| `ages` | ✅ | ✅ | 🔁 NickPatcher changed the rare age bands; ⚠️ with override 1 | ✅ |
| `ages_out_of_game` | ✅ | ✅ | 🔁 the branch is skipped in the exe; ⚠️ with override 1 (the branch still doesn't run) | ✅ |
| `rule = new` (the contest) | ✅ | 🔁 the Regen Tweak (squad 45); ⚠️ with override 1 | 🔁 / ⚠️ the same | ✅ NickDLL replaces the B-team routine (0x540a50) with its own, with the database's list; the contest uses it as it would the exe's |
| `retire_idle_players` | ✅ | ✅ | ✅ on, but RetirePatchV4 lets nobody under 34 reach the rule (the log warns) | ✅ |
| `unattached` | ✅ | ✅ | ✅ | ✅ |
| `no_cd` | ✅ | ✅ | ✅ check by check: the exe already has 14 of the 33; the DLL completes the others | 🔁 with `LaMOHatesCD.dll` next to the exe, left to it |
| `resolution = 1080` | ✅ (on the 1080 exe, "already there") | ✅ | ✅ the exe has 5 of the 201 places already changed (the same NickPatcher code); the DLL completes the others | 🔁 with 6.1's NickDLL and `"resolucao"` other than 800x600 in `NickDLL_config.json`, left to it |
| the 30-row limit (with `resolution`, `video` or NickDLL's resolution) | ✅ | ✅ | ✅ | ✅ no NickDLL fix touches `Area::init` |
| `video = window` | ✅ | ✅ | ❓ not tested: CLZ in a window with Windows' `ddraw.dll` stops at "16 bit colour"; the video layer should fix it [inference] | ❓ |
| `log = 1` (logging message boxes, 0x5e8290) | ✅ | ✅ | ✅ | ⚠️ LaMOHatesCD and 6.1's NickDLL (`hook_msgbox`) also hook 0x5e8290; the log chains to whatever hook is already there; if NickDLL installs its own later and checks for the original bytes, its own may refuse [inference, to confirm in the game] |

## 3. CLZ 6.1's NickDLL

What CLZ 6.1's NickDLL and LaMOHatesCD change in the code the Tweaks use, observed through the memory snapshot (the `runtime` lines of `tweaks.log`) and in the tests:

| DLL | What it changes in our code |
|---|---|
| 6.1's NickDLL | **the resolution**, when a `"resolucao"` is chosen in `NickDLL_config.json`: then ours stays out (it's one or the other); **the message box** (0x5e8290): our message box log chains to it; **the B team** (0x540a50): the contest uses its routine to leave B teams out. The other tweaks aren't touched |
| LaMOHatesCD | only the message box |
