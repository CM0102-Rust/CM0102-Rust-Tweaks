# Loans: how the game decides and our rules

*Versão em português: [EMPRESTIMOS.md](EMPRESTIMOS.md)*

The market's most repeated complaint on the forum ([OPORTUNIDADES.en.md](OPORTUNIDADES.en.md) §2): "the loan offer is rejected, no matter what wages or period or the player has no intention of going on loan" (JLa, t=1593). This document explains who decides in the original game, the rules this DLL proposes (the `loans` key), what the test careers measured and each key. **The rules are a proposal**: the idea is that the community plays, looks at the numbers and proposes changes (§7).

**[observed]** = read in the code; **[measured]** = automatic career; **[inference]** = a conclusion still to prove.

## 1. How the original game decides

| Routine | Name (community map) | What it decides |
|---|---|---|
| 0x84af00 | `predict_selling_club_response` | **the owner**: accepts, negotiates or refuses a bid or a loan request |
| 0x846920 | `staff_interest_in_joining_club` | **the willingness** of a player to go to a club (a score from 0 to 10) |
| 0x83cb40 | `assess_club_squad` | the squad assessment: who goes on the transfer and loan lists |
| 0x83f3a0 | `fill_shortlist_slot` | **whom the club looks for** in each slot of its shortlist |
| 0x84a020 | `make_club_bids` | the AI goes through its shortlist and makes bids and loan requests |
| 0x842740 | `try_loan_offer_for_player` | the AI's loan request |
| 0x598800 | the transfer budget | how much the club can spend |

1. **The owner almost always refuses** [observed]. An AI club only lends:
   - a player it put on its own loan list, with the asking club's reputation × 1.25 above the player's; or,
   - in a narrow case, the unhappy backup.

   Otherwise it refuses. **The wage and the period don't count.**
2. **The loan list has almost no youngsters** [observed]. The AI only lists a player at the end of the squad in poor form or tired, so a promising youngster who doesn't play sits still.
3. **The AI only asks when the player wants to go** [observed]. The willingness score must be 3.5 or more, and the request goes through a draw against the club's reputation / 10.
4. **"No intention of going"** comes out when the willingness score is below 1.0 [observed].
5. **The budget is compressed in some nations** [observed]:
   - Germany, France, Scotland and the Netherlands only use 25% of what's above 17.5 million;
   - Brazil, the Nordic nations and others, only part above 2.5 million;
   - England, Spain and Italy have no brake.

## 2. The idea of our rules

A loan is for three things:

- **Getting the youngster games**: a player up to 21 who isn't first team goes out to play, to a club where he'd be one of the two best in his position and whose Training isn't much worse.
- **Giving minutes to whoever is spare**: a squad player with someone to cover him goes where he'd play (a club with three good defensive midfielders lends one).
- **Going back home**: a player abroad is more willing to go to a club of his own nation, and the owner asks for less cover to let him go.

And one limit: **a Key player never goes on loan.** But a starter a much bigger club wants, whom the owner won't release, gets upset, and that moves the market too. The human club's decisions stay the player's.

## 3. The rules

**A. The owner's decision** (0x84af00, for every loan request to an AI club):
1. Key status: refuse.
2. **Minutes.** The player must be among the best of his line **at the asking club**, by CA:
   - a youngster (up to `loan_youth_age`): among the first `loan_youth_top`;
   - the others: among the first `loan_others_top`.

   The line is the one of his best position rating: goalkeeper, defence (sweeper, centre back, wing back), midfield (defensive, central, attacking) or attack. Nobody goes on loan to sit on the bench.
3. **Cover.** The owner must keep players of the same line with CA of at least `loan_cover_ratio`% of the loaned player's:
   - for a Regular starter: `loan_cover_regular` players;
   - for rotation and backup: `loan_cover_backup`;
   - for a youngster: none.
4. **Training.** A youngster only goes to a club with Training of at least the owner's − `loan_training_margin`.
5. **Back home** (the destination is of the player's nation and the owner isn't): the cover asks one fewer, and Training doesn't count.
6. **Squad floor.** With `loan_squad_min` players or fewer at the club, the owner lends nobody.

**B. The loan list**: after every squad assessment, the owner lists whoever rule A would lend (without looking at the destination). That way the player shows up in the other clubs' shortlists.

**C. Who asks**:
- the AI's request needs willingness `loan_request_willingness` (the original needs 3.5);
- the AI tries `loan_requests` times more;
- the player's willingness to go on loan is multiplied:
  - by `loan_home`, for a club of his own nation while he's abroad;
  - by `loan_playing_time`, where he'd be 1st or 2nd of his line.

**D. The demand for youngsters** (0x83f3a0): when an AI club picks whom to ask for in a loan slot, a youngster who would be one of the `loan_youth_top` of his line there has his score multiplied by `loan_youth_demand`. He must also be going home or to Training within the margin. Without it, the spare adults now on offer win the slots and youth loans fall by half [measured].

**E. The blocked starter**: in a real decision (not the shortlist predictions), when the owner refuses to loan a starter (Key or Regular starter) to a club with reputation at least `loan_unhappiness_gap` above his own, the player loses `loan_unhappiness` contract happiness. Unhappy, he gets closer to asking to leave.

**F. The budget brake**: with `budget_caps = off`, no nation's budget is compressed (all like England).

## 4. What the careers measured

Automatic careers with every league in the database, July 2001 to July 2003, with no human club. Each number comes from a single career; the noise between identical careers is about ±5% and ±0.1 to 0.3 CA [measured].

| Per season (2001/02 · 2002/03) | Original | These rules |
|---|---|---|
| Loans | 3,255 · 4,042 | 5,467 · 5,469 |
| Promising youngsters loaned (up to 21, PA − CA ≥ 30) | 680 · 698 | 726 · 692 |
| Good players moving up (CA 120+ to a bigger club) | 259 · 387 | 456 · 920 |
| Starters leaving big clubs (CA 140+) | 70 · 104 | 136 · 182 |
| Transfer value (millions) | 1,253 · 1,464 | 1,630 · 1,978 |
| Promising players' growth (CA a year) | 11.56 · 10.55 | 11.78 · 10.70 |
| Clubs in the red at season end | 48.7% · 47.6% | 48.1% · 48.8% |

What each part did, one at a time [measured]:

- **Only the budget with no brake**: rich clubs that had a cap get +27% to +43% budget and spend more (from 104 to 143 million and from 133 to 166 million). Finances don't change.
- **A and B (the owner and the list)**: loans go up 70%, and good players moving to a bigger club double. **But youth loans fall by half**: the AI was asking for youngsters for the bench, and rule A refuses.
- **C (home and minutes)**: some 500 to 850 more loans a season.
- **D (the demand for youngsters)**: youth loans are back to the original's level: 681 · 653 with ×2, 726 · 692 with ×3.
- **E (the blocked starter)**, with 30 happiness and a 1,000 gap: 6 cases in two seasons. It's rare because a bigger club hardly ever asks for a starter on loan; the key stays for when transfers change that.
- **The squad floor**: without it, three of four careers crashed:
  - in one, a club lent until it had no team, and the game stopped with "Only 10 away players";
  - in another, a cup error;
  - in the third, a memory fault.

  With the floor at 20, none crashed.

## 5. The keys

In `tweaks.cfg`, or in the configurator's **Loans & budget brake** tab. Without `tweaks.cfg` nothing changes; with it, a missing key takes the "default" column. **The default is what was tested** in the §4 careers. In the "original" column, "no rule" means the original game has no such check: a youngster, for example, goes to any Training.

| Key | Default | Original | What it does |
|---|---|---|---|
| `loans` | on | off | turns on rules A to E; off = the original game's loans |
| `loan_youth_age` | 21 | no rule | up to this age, a player who isn't Key counts as a youngster |
| `loan_youth_top` | 2 | no rule | a youngster only goes where he'd be 1st to Nth of his line |
| `loan_others_top` | 2 | no rule | the same for the others |
| `loan_training_margin` | 3 | no rule | a youngster only goes to Training ≥ the owner's − this (0 = equal or better) |
| `loan_cover_regular` | 2 | no rule | cover to lend a Regular starter |
| `loan_cover_backup` | 1 | no rule | cover to lend rotation and backup |
| `loan_cover_ratio` | 90 | no rule | who covers: CA of at least this % of his |
| `loan_squad_min` | 20 | 0 | with this many players or fewer, the owner doesn't lend |
| `loan_request_willingness` | 2.75 | 3.5 | the minimum willingness for the AI to ask |
| `loan_requests` | 2 | 1 | times the original's requests (1, 2, 4, 8...) |
| `loan_home` | 2 | 1 | willingness to go back home, times |
| `loan_playing_time` | 1.3 | 1 | willingness to go where he'd be 1st or 2nd of his line, times |
| `loan_youth_demand` | 3 | 1 | the score of a youngster who'd play there, when picking a loan slot's target |
| `loan_unhappiness` | 30 | 0 | the happiness the blocked starter loses |
| `loan_unhappiness_gap` | 1000 | no rule | how much bigger (reputation) the asking club must be |
| `budget_caps` | off | original | off = no nation with a compressed budget |

With `log = 1`, the `tick` line in `tweaks.log` shows, every game month:
- `loan_listed`: the loan-list markings;
- `loan_real_yes` and `loan_real_no`: the owner's decisions, not counting the shortlist predictions;
- `young_sought`: the times a youngster got the edge in a loan slot;
- `unhappy`: the blocked starters who got upset.

## 6. What didn't go in, and what isn't known yet

- **The blocked starter is rare** (6 cases in two seasons): a bigger club hardly ever asks for a starter on loan. To make it happen more, the way is transfers (the next study) or bigger values in the keys.
- **Minutes hardly change a youngster's growth** [measured]:
  - in the original, a loaned promising youngster gains +9.9 CA with 0 league games and +10.8 with 10 to 19;
  - in every career, the original included, **a loaned youngster grows about 2.5 CA a year less than one who stays**, and almost all go to smaller clubs, which train worse;
  - the game's engine develops through Training, age and potential.

  That's why there's a Training margin. With `loan_training_margin = 0`, each youngster goes to a better place, but a third of them are no longer loaned.
- **Not measured yet**: long careers (10 seasons), careers with a human club and the community's databases (such as CLZ).
- **The human club's loans** stay as in the game: the human club decides what it lends, and a human request to an AI club goes through rule A.

## 7. How to propose a change

Each rule is a key, and the original is always a possible value: you can turn off just one part. To propose:

1. Change the keys in the configurator and play (or run a holiday career).
2. With `log = 1`, send `tweaks.log` with the proposal: the `tick` line's counters show what the rules did.
3. Say what in football the rule should reflect. That's how these were born: the youngster gets games, whoever is spare plays, the player goes back home, the Key player stays, and the blocked starter gets upset.

## Technical notes

Where each rule hooks into the exe:

| Rule | Place in the exe |
|---|---|
| The owner's decision | wraps 0x84af00 (the trampoline checks the first 6 bytes) |
| The squad assessment | the call at 0x83a763 |
| The willingness | wraps 0x846920 |
| The request threshold | the `fcomp` at 0x8427f0 |
| The request draw | the `sar edx, 2` at 0x84aaf9 |
| The demand for youngsters | the final score draw at 0x840610 (`push 0x11; call 0x90cfc0`) |
| The blocked starter | the contract's happiness (+0x3d), in 0x84af00's real decision |
| The budget with no brake | the jump at 0x599307 |

An exe that differs from 3.9.68 at any of these places leaves that part off, and `tweaks.log` says which.

The full market map (the routines, the draws, the original's defects) is in the CM0102 project, `docs/MAPA_MERCADO.md`.
