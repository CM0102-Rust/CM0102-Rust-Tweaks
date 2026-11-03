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

## 3. The inventory of CLZ 6.1's NickDLL

`tools/inventario_dll.py` looks, in a DLL's sections, for addresses that fall in each setting's code. It runs nothing and copies no code: it points where to look, and the confirmation is the memory snapshot (the `runtime` lines of `tweaks.log`). Reading NickDLL has its author's (dmcarmo) permission, and the CLZ version is edited by the CLZ team.

| Version (`D:\Games\CLZ Leagues Patch v6.1`) | What it touches in our code |
|---|---|
| `NickDLL.dll` of 11/02 22:08 (6.1's) | **the resolution:** 208 of our addresses; the "resolucao ampliada" (1280×720, 1280×800, 1280×1024, 1920×1080, through `NickDLL_config.json`) uses the same table of changes and refuses if it finds the bytes already changed. **The message box** (0x5e8290). **The B team** (0x540a50). Calls to 0x933610 (the save container) and to two places in regen creation (0x7abd90/96), outside the code we replace |
| `NickDLL.dllold` of 11/02 14:40 | the same |
| the one of 10/18 (the long careers', in `copias\bancada4\clz_tweaks`) | no resolution; the rest the same |
| `NickDLL.dlloriginal` of 10/08 | no resolution (only 3 Display addresses) and 0x7a8060 (inside the unemployed retirement) |
| `LaMOHatesCD.dll` | only the message box |

The fixes in 6.1's NickDLL (0x6847C0, 0x51F890, 0x768310, 0x522E00, 0x4A2540, 0x48F150, 0x4A4850, 0x4ACEB0, 0x6F46B0, 0x6E1090, 0x6CD35C, 0x6CD3FE, 0x6F3A67, by its log messages) don't fall in any Tweaks code [observed].

## 4. A new DLL (Davdav's and the next ones)

1. `python tools/inventario_dll.py NewDLL.dll`: the settings whose code it touches.
2. For each setting touched, read what it does there (without copying) and put it in the matrix: ✅, ⚠️, 🔁 or ❓.
3. A short career with `log = 1`: the `runtime` lines show what it actually changed in memory, and `check` says whether any of our hooks was overwritten.
4. If it does the same job as one of our settings (like NickDLL's resolution), the DLL recognises it and leaves it to it, as in `resolucao_da_nickdll` (`src/lib.rs`).

Credits: NickDLL (Nick and dmcarmo), NickPatcher (Nick), LaMOHatesCD (LaMO), Regen Tweak (xeno/Saturn), CLZ (the CLZ team).
