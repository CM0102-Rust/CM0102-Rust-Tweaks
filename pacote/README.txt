CM0102 Rust Tweaks {versao} - for Championship Manager 01/02 3.9.68
WORK IN PROGRESS: a test release, not a final one. ALWAYS BACK UP YOUR SAVES AND THE GAME FOLDER BEFORE TRYING IT.

INSTALL
Copy ddraw.dll, tweaks.cfg, tweaks-config.exe and intake_news.txt into the folder of cm0102.exe and open
tweaks-config.exe to choose what to turn on. Without tweaks.cfg nothing changes in the game. To remove: delete
ddraw.dll. Using NickPatcher or NickDLL? Read NICKPATCHER.en.md first. Every option is explained in
GUIA_DAS_CHAVES.en.md (and in the configurator: hover the "?").

NEW IN 1.3: THE YOUTH INTAKE, ON BY DEFAULT (only in a NEW career)
Football Manager's "Youth Intake" in CM 01/02. A regen is a regen: the intake changes only two things.
- THE DATE: regens are born on intake day (each nation's season end, or a fixed day), not one by one through the
  season. Whoever retires waits in the game's own queue until then.
- THE NUMBER: every club with squad room gets 3 regens (intake_per_club), up to 50 in the squad. Which club gets
  each regen is still decided by the curve on the club contest tab. When retirements aren't enough, the DLL creates
  the missing regens from a hidden reserve made with the career, each one a mix of two retired players of the same
  position from the club's country (no clones), born through the game's own routine.
- THE MESSAGE: your club gets an inbox message from the youth coach with each youngster and his view of him. The
  phrases are in intake_news.txt, in 19 languages: you can edit them.
Tested over 10 seasons with every league of the original game (1,134 clubs): every club gets its quota, no regen
"sink" clubs, Basque clubs (Athletic, Eibar, Real Sociedad) get Basque youngsters, the reserve lasts (42,000 of
50,000 left after 10 years), saving and loading keep the intake going. The intake_stock / intake_pa_* keys of 1.2
are gone: a 1.2 tweaks.cfg with intake_stock above 0 still turns the intake on.
Also on by default now: save_over_2gb (saves above 2 GB). To play the original way, set intake = 0.

Problems: send tweaks.log (next to the exe) and say what happened.
Source code, documentation and tests: https://github.com/CM0102-Rust/CM0102-Rust-Tweaks
License: MIT (LICENSE). The game, its executable and its database are not included.
