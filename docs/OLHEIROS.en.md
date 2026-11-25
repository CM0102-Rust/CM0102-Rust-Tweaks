# Scouts: fame in the judgement

*Versão em português: [OLHEIROS.md](OLHEIROS.md)*

Forum report (11/25/2026): a few seasons in, almost no five-star scout reports, although CM Scout shows regens with PA close to 200. At the start of the game, many players got five stars and "excellent signing".

## 1. How the game judges

`match_man_judge_player_ability` (0x6e86d0, cdecl, 5 arguments: who judges, the player, the club, the mode and the reputation, or −1 for the game to fetch it) estimates a player's CA (mode 2) or PA:

`estimate = ((real value + noise) × p + r × (20 − p)) / 20`

- `p` = the scout's judging / 2 + 10, from 10 to 20;
- `r` = the player's reputation as the judge sees it (`database_get_staff_reputation_for_viewer` 0x5407e0) / 50; for PA, minus (age − 23)², or (age − 26)² for a goalkeeper;
- the noise shrinks with the scout's judging.

A scout with judging 10 sees 75% ability and 25% fame; only one with 20 sees pure ability. At the start of the game, the good players are the database's stars, with high reputations: fame helps. Years later, the good ones are young unknown regens: fame works against them. A regen with PA 190 and reputation 300 comes out, for a scout with judging 10, near 144.

## 2. The stars

The report (`PLAYER_SEARCH::report_on_player` 0x7b62a0) gives stars by the score of `match_man_score_player_for_role` (0x6e2f20), which calls the judgement above for CA and for PA. The score is compared with a target of the manager's club (`player_search_get_club_target_rating` 0x7bca90: the club's reputation, the league's, the professional status, the squad and the finances, from 1,750 to 35,000). Five stars: above the target and above the club's reputation × 2 + 10,750 (for potential, + 10,500). As the club grows, the bar goes up; that's the game's design and stays.

## 3. The option (`scout_reputation`, 0 to 100; 100 = the original)

Only for the scouts of human managers' clubs (`0x600a50`): when the game asks for the reputation (the 5th argument −1), the DLL passes in its place the blend `k × reputation + (1 − k) × value × 50`. With 0, the fame term becomes the value itself, and only the noise is left, which already depends on the scout's judging. The AI (the market, the shortlists) keeps judging as in the original. Off (100) in the package. Code in `src/jogo/olheiros.rs`.

**Not measured in a career:** the automatic career sends no scouts. The proof is the code reading and a hidden career with `scout_reputation = 0` with no crash ([TESTES.en.md](TESTES.en.md)).
