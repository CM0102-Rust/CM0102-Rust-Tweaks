# Making a grey player permanent: the design

*Versão em português: [EFETIVAR_CINZENTOS.md](EFETIVAR_CINZENTOS.md)*

Lucas's request (11/16/2026): with every club playable (CLZ.DLL opens the state-league clubs, which live on grey players), the human manager needs to be able to keep the grey players in his own squad. This document is the design, before the code. It builds on the CM0102 project's study of 11/03 (its `docs/EFETIVAR_CINZENTOS.md`) and two exe studies of 11/16 (the profile's action menu and contract negotiation).

**[observed]** = read in the code; **[inference]** = to be proven.

## 1. Lucas's decisions

1. Only the grey players of the human manager's **own club**. The AI keeps using grey players as it does today.
2. Through a **"Make permanent"** item in the grey player's profile Action menu.
3. The personality is **drawn at random** when he's made permanent (the game gives a grey player 10 in everything).
4. He gets the **regen mark** (bit 0x40 of +0xb in the 79-byte record), so editors find him with the "Only regens" filter.
5. The contract is **negotiated** on the game's own screen, an **easy** negotiation: the player is won over by the role and never asks for much.
6. It works with the intake (`intake`) on or off.

## 2. What happens in the game

1. On the profile of a grey player in your club, the **Action** button appears with a single item, **Make permanent**. Today the game shows no Action for grey players.
2. On the click, the DLL turns the grey player into a real person: the same name, age, nation, positions and attributes, the random personality and the regen mark. He joins your squad on a short **temporary contract** and the profile reopens as an ordinary player's.
3. Right after, the game's **Set Role At Club** screen opens, then the contract offer. You pick the role and make the offer.
4. For him, the demand is the **game's default wage for that role** × `grey_signing_wage` (%), with no signing-on fee, bonuses or clauses; he never refuses to talk, and an offer that covers the demand is always accepted. The answer comes the next day (in the game, 1 to 14 days).
5. If you don't renew, the temporary contract runs out and he leaves the club like any player.

## 3. Where the person comes from

A grey player has no row in the per-person tables (contracts, discipline, injuries, statistics) and is rebuilt before every match [observed, 11/03 study]. Making him permanent uses a real person who is spare, in this order:

| Order | Source | When it exists | Cost |
|---|---|---|---|
| 1 | the **intake's dormant people** (retired, without a club, with a player record, outside the queue: `adormecidos` in `src/regens/fornada.rs`) | careers created with `intake = 1` | one dormant person fewer (thousands are left) |
| 2 | the **grey-player stock**: `grey_signing_stock` people created with the career, by the intake's mechanism | new careers created with `intake = 0` | none |
| 3 | the **regen queue's slot + a record from the regen pool** (the regen's path) | any save | one regen fewer; with no free record, the DLL says so and doesn't proceed |

## 4. The exe spots

| What | Where | How |
|---|---|---|
| The menu item | 0x874a58 (`33 F6 56 6A 04`), in the profile build 0x870700, right after the Action menu | for a grey player (`id ≥ [0xAE2364]`) of the human club: the Action button (`add_button_item` 0x804030), the area (`Display::add_area` 0x55fba0) and the item (`Display::add_item` 0x55f990) with id 0x50, unused [observed] |
| The click | `staff_screens_handle_profile_action` 0x888260 (`6A FF 68 9B 2F 96 00`) | id 0x50: make permanent, reopen the profile (0x8702a0) and open the contract; the rest goes to the original [observed] |
| The human and his club | `[0xb74c26 + 0x300 × [0xb77c7a]]`, club at +0x39 | [observed] |
| Joining the club | `player_regen_sign_for_club` 0x7a6f10 | the regen's contract (at a human club: reserves, status 6); the DLL shortens the end (+0x2d/+0x2f) [observed] |
| Opening the contract | `contract_screens_offer_contract_to_staff` 0x4dd0f0 `(staff, 0, 0, 1)` | opens Set Role, then the offer [observed] |
| The demand | `STAFF_CONTRACT::set_to_desired_contract` 0x85fc50 (`6A FF 68 76 2A 96 00`), after the original | only for players made permanent: wage = the role's `default_staff_contract` 0x864550 × `grey_signing_wage`; fee, bonuses and clauses zeroed [observed the spot; the effect, inference] |
| Always willing to talk | `CONTRACT_MANAGER::will_negotiate_offer` 0x4d4660 (`83 EC 10 53 8B 5C 24 18`) | returns 1 for them [observed] |
| Acceptance | `resolve_contract_offers` 0x4ce4e0 | an offer covering the demand has a ratio ≥ 0.9 and is accepted (3.15 × 0.9 > 2.75): nothing to hook [observed] |
| The answer the next day | `schedule_offer_decision` 0x4cd230 | optional: the decision date = tomorrow for them [observed the spot] |

All match byte for byte in the CLZ 6.1 exe (NickPatcher) [observed in the file; NickDLL may change them in memory: the DLL's hook check logs it].

## 5. The keys

| Key | Default | What it does |
|---|---|---|
| `grey_signing` | 0 (without the line) | 1 = the Make permanent item and the easy negotiation |
| `grey_signing_wage` | 80 | the demand, in % of the role's default wage |
| `grey_signing_days` | 30 | the length of the temporary contract, in days |
| `grey_signing_stock` | 300 | people created with the career for this, when the intake is off |

## 6. To check before and during the code

- Cleaning the reused person: the regen does it through `regenerate_as_youth_player` 0x7abac0, which also picks the club through the contest; this needs the same cleaning without the contest.
- The list of players made permanent who are still negotiating must survive the save (a file next to the save, like `.publico`, or a mark on the temporary contract).
- Whether `create_virtual_player` writes the club at the grey player's +0x39.
- The Set Role screen may refuse roles by the board's rating.
- The tests: make permanent, negotiate, save, load and check with the editor; with the intake on and off; on 3.9.68 and on CLZ.
