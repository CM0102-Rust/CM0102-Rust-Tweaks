# Grey players: making them permanent and the nickname

*Versão em português: [EFETIVAR_CINZENTOS.md](EFETIVAR_CINZENTOS.md)*

Lucas's request (16/11/2026): with every club playable (CLZ.DLL opens up the state-league clubs, which live on grey players), the human manager needs to be able to keep the grey players in his own squad. On 17/11 the grey players' nickname came in, to avoid long compound names. This document describes how it turned out; the proofs are in [TESTES.en.md](TESTES.en.md) §6.

**[observed]** = read in the code or in the game's memory; **[inference]** = still to prove.

## 1. Lucas's decisions

1. Only the grey players of the human manager's **own club**. The AI keeps using grey players as it does today.
2. Through a **"Make permanent"** item in the grey player's profile Action menu.
3. The personality is **drawn at random** when he's made permanent (the game gives a grey player 10 in everything).
4. The player gets the **regen mark** (bit 0x40 of +0xb in the 79-byte record), so editors find him under the "Only regens" filter.
5. **No negotiation** (17/11): the first version opened the game's contract screen, and no player made permanent accepted the offer. The player arrives on a **normal 12-month contract** with a wage in line with the squad.
6. He **stays in the team he was in**: first team or reserves.
7. It works with the intake (`intake`) on or off.
8. Grey players get a one-word **nickname** (`grey_players_names = nickname`), at every club.

## 2. What happens in the game

1. On the profile of a grey player in your club, the **Action** button appears with a single item, **Make permanent** (in Portuguese, Efetivar). The game doesn't show Action for grey players.
   - The grey player must belong to your club (or your national team) by the owner the game writes in him, not just sit in a place of your squad. The original sometimes leaves the same grey player row in two squads (a week with continental matches); up to 1.8 this showed the item on AI grey players' profiles, and making one permanent made a copy, leaving the grey player there (forum report of 11/20, fixed in 1.8.1).
2. On the click, the DLL turns the grey player into a real person, with the same name (and nickname), age, nation, positions and attributes.
   - His CA is at least the club's level (the grey players' one, `grey_players = squad`).
   - His PA gets room by age: up to 18, +30 to +50; 19 to 21, +15 to +35; 22 to 24, +5 to +20; after that, 0 to +8.
   - His personality is drawn and he carries the regen mark.
3. He signs a **normal** contract (not the monthly one) with your club for `grey_signing_months` months (12). The wage is the median wage of the squad's non-regen players × `grey_signing_wage` % (100), with the game's minimum (55).
4. He stays in the team the grey player was in: in the first team, he moves up through the game's own routine; in the reserves, he stays there.
5. The grey player's place is freed. At the next match the game creates another grey player, and the DLL makes sure he isn't born with the signed player's name: the game's draw is fixed per club and place.

## 3. Where the person comes from

A grey player has no row in the per-person tables (contracts, discipline, injuries, statistics) and is rebuilt before every match [observed, 03/11 study]. Making one permanent uses a spare real person, in this order:

| Order | Source | When it exists | Cost |
|---|---|---|---|
| 1 | the **intake's dormant people** (retired, without a club, with a player record, out of the queue: `adormecidos` in `src/regens/fornada.rs`) | a career created with `intake = 1` | one dormant person fewer (thousands are left) |
| 2 | the **head of the regen queue + a reserve player record** from the regen | any save | one regen fewer; with no free record, the DLL logs it and signs nobody |

The person becomes a regen through the game's own routine (`regenerate_as_youth_player` 0x7abac0), without the clubs' contest, and then takes on the grey player's identity.

Not done yet: an own stock (`grey_signing_stock`, about 300 people created with the career) for when the intake is off, without costing regens.

## 4. The nickname (`grey_players_names = nickname`)

The game draws 20 names from the club's nation and keeps the one with the highest weight in the database. In CLZ, compound Brazilian names have a high weight ("Pedro Henrique", "da Silva Santos"): 7,668 of the 9,270 Brazilian second names are compound.

With the nickname, after the game creates the grey player:

1. First and second names become one word (2 to 12 letters, no full stop; up to 10 draws each).
2. The nickname (+0xc, which in a grey player points to the empty row 0 of the nickname table) gets a nickname row from the nation, one word long and not yet used by any real person in the database. The draw is the same as for names: 20 draws in the nation's range, keeping the one with the highest weight.
3. The game shows the nickname instead of the name, as it does for real players.

A nation with fewer than 20 nicknames gets only one-word first and second names. A player made permanent keeps his nickname, and the row then counts as used. The draw uses the `rand()` the game seeds per club: the grey player in the same place keeps his nickname from one match to the next.

## 5. The points in the exe

| What | Where | How |
|---|---|---|
| The menu item | 0x874a58 (`33 F6 56 6A 04`), in the profile build 0x870700, right after the Action menu | for a grey player (`id ≥ [0xAE2364]`) of the human club: the Action button (`add_button_item` 0x804030), the area (`Display::add_area` 0x55fba0) and the item (`Display::add_item` 0x55f990) with id 0x50 |
| The click | `staff_screens_handle_profile_action` 0x888260 (`6A FF 68 9B 2F 96 00`) | id 0x50: make permanent; the rest goes to the original |
| The human and his club | `[0xb74c26 + 0x300 × [0xb77c7a]]`, club at +0x39 | |
| The grey player's owner | +0x39 the club (0x90e33d) or +0x24 the national team (0x90e330), written by `create_virtual_player` | must be the human's club or national team: `relocate_virtual_players` (0x90e9f6) can leave the same row in two squads |
| The person becomes a regen | the two club calls of 0x7abac0 (0x7acea2, the contest; 0x7acf4f, the nation list) | return 0 while making permanent |
| Putting him in the club | `player_regen_sign_for_club` 0x7a6f10 | at a human club: reserves, monthly contract, default wage |
| The contract | `CONTRACT_MANAGER::get_contract` 0x4cf5a0 | the DLL sets the type to normal (+0x35, 2), the end (+0x2d day, +0x2f year) and the wage (+0xc) |
| Moving up to the first team | `move_to_first_team` 0x4cc0b0 | when the grey player was in the first team (bit 0 of +0x5e) |
| The grey player's name | `create_virtual_player` 0x90e0a0 (`6A FF 68 A0 55 96 00`, thiscall, `ret 0x1c`) | after the original: the nickname, and a name that doesn't repeat a real squad player's |
| The name draws | 0x539c60 (first name), 0x53a1b0 (second name), per-nation index `[0xae2338]` + id × 36 | nicknames: first row at +8, count at +0x20; table `[0xae23f8]`, 60-byte rows, nation at +0x37, weight at +0x3b |

**Trap:** the contract manager is `[0xae1050]`, and the contract array is `[[0xae1050]]`. Passing `0xae1050` as `this` writes to the wrong memory; it was why the contract stayed monthly (16/11).

## 6. The keys

| Key | Default | What it does |
|---|---|---|
| `grey_signing` | 0 (without the line) | 1 = the Make permanent item |
| `grey_signing_months` | 12 | the signed player's contract length, in months (1 to 60) |
| `grey_signing_wage` | 100 | the signed player's wage, in % of the squad's median without regens (10 to 300) |
| `grey_players_names` | original | `nickname` = the grey players' one-word nickname |

The grey players' surprise (`grey_players_surprise`, 5%, +15 to +35 CA above the level) is in [GUIA_DAS_CHAVES.en.md](GUIA_DAS_CHAVES.en.md).

At the human manager's national team (with `manage_any_team = 1`), the player made permanent is called up with no club and no contract: see [SELECOES.en.md](SELECOES.en.md) §3.

## 7. Proofs and what's left

Proven (TESTES §6):

- making permanent on screen, in CLZ v6;
- saved and loaded: a first-team and a reserve player in CLZ (intake off, the person from the regen queue) and a reserve player on 3.9.68 with the intake on (the person from the dormant ones);
- the nickname in a hidden 3.9.68 career and in CLZ.

Still to do: the own stock for when the intake is off.
