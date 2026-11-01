# Opportunities: what the community complains about and asks for

*Versão em português: [OPORTUNIDADES.md](OPORTUNIDADES.md)*

A reading of the champman0102.co.uk forum on 11/01/2026, by script, public pages only and without logging in. The idea is not to repeat what NickPatcher and the other authors already do, and to go after what's still open and only our Rust base makes possible.

**Scope (Lucas's decision, 11/01/2026):** this repository is the DLL front. The fixes and what we learn here will be consolidated into the CM0102 project once it's 100% Rust. **Competition structures are out:** davdav already does that, and does it well (the restructure patch). In the "Where we have an edge" table, the chosen fronts are the December crash (study), loans and the economy.

## Method

- **10 sections:** Technical Support, Patches, The Workshop, Dressing Room, Databases, Tactics, Talents/Tips/Training, Research and Archives. 1,330 topics in total.
- **944 topics read** (the first page; for megathreads, also the last two): outside Tactics, those with some discussion or a title hinting at a request or complaint; in Tactics, the most discussed ones and those about the match engine.
- **Themes by keyword**, weighted by engagement (replies and views), and a reading of the excerpts that look like requests ("is it possible", "would be nice", "wish"...).
- **The most valuable source:** xeno's "Summary of improvement ideas, bugs" list ([t=2410](https://champman0102.co.uk/viewtopic.php?t=2410), 222 replies). It has 104 items, each marked as fixed (FIXED, almost always by Tapani, Saturn and Nick+Co patches), bug, feature or annoyance. Also "How would you improve the game?" ([t=1593](https://champman0102.co.uk/viewtopic.php?t=1593)), with the voice of ordinary players.

## Where the community lives (what engages most)

| Topic | Replies | Views |
|---|---|---|
| Retro database update (Season 1984/85) | 5,922 | 554k |
| Nick's CM0102Patcher | 3,441 | 927k |
| [WIP/TESTING] 2026-27 restructures patch | 1,423 | 127k |
| Data Update - Technical Issues (2023-2026) | 1,301 | 248k |
| Installing patches (Nick+Co, Saturn, Tapani) | 1,171 | 480k |
| Starter Kit FAQ | 1,127 | 371k |
| User Tactics, Tactic Benchmark League | 853 and 684 | 334k and 268k |
| Game Crashing | 829 | 150k |
| [RESOURCES] Offsets | 788 | 254k |

The community revolves around **three activities**: updating the database (current and retro seasons), installing and combining patches, and studying tactics as a sport (benchmark leagues). The rest of the engagement is support: installing, getting the game to run, fixing crashes.

## What others have already solved (don't repeat)

According to xeno's list, the Tapani, Saturn and Nick+Co patches (gathered in NickPatcher) have already solved, among others:
- fixtures without clashes and match rescheduling;
- visible human manager attributes, squad numbers;
- club funds above 3 billion;
- better AI tactics, better default formations;
- transfer windows per country, end-of-season dates;
- AI buying and selling decisions, values and wages, club budgets;
- loans (Saturn's "loan boost"), the Regen Tweak (squad 45);
- less predictable retirements, managers and chairmen retiring later;
- prize and TV money in leagues that had none;
- several competitions repurposed (Club World Cup, Gold Cup, Libertadores and Copa Argentina in the slots of defunct cups);
- immediate suspensions, shorter work permits, the positioning penalty by age.

Resolution and bigger screens also have patches (Nick, Tapani, melanzalko). Saves above 2 GB (item 104) were solved by us, in save2gb.

## What's still open

### 1. Competition structures: the biggest demand
- "Ability to change League Structures is the big thing for me - particularly the ability to Increase/Decrease team numbers and make more leagues playable" (SteveHarle, t=1593). MLS, for example, has only 12 teams in the original game.
- davdav's restructure patch has 1,423 replies, plus 335 in its support topic. The Champions League and Europa League formats (t=3206, t=10794), the English National League playoffs (xeno 82) and defunct cups turned into new ones (xeno 79–81) are all recurring requests.
- Today all of this is done by swapping pointers and ids written into the exe ("dozens of places in the code", see the project's `CAMPOS.md`). Every change is a fragile patch.
- Open bugs: Dutch relegation (xeno 5.1), the FIFA ranking (35), small nations missing from qualifiers (83), promoted clubs without a stadium (46), the Champions League quarter-final draw (103), Olympics and Asian Champions League dates (84, 86).

### 2. Market and AI
- **Loans don't work:** "the loan offer is rejected, no matter what wages or period or the player has no intention of going on loan" (JLa, t=1593). In holiday mode, loans get stuck (xeno 88). The human can't sign on-loan players through Bosmans (58.2).
- **Rich clubs sitting still**, with hundreds of millions unused (13.1). The AI doesn't invest in facilities or the stadium (30).
- **Expired contracts that don't release** the player or coach, who stays on £0 a week (95). Squad status greyed out after a rejection (t=1593).
- **Managers moving oddly:** by league reputation rather than club reputation (36).

### 3. Youth, regens and development
- "For 5 years no home-grown player turned up" (t=5879, 100 replies). It's Here2Win79's question, which we answered.
- **Regen names:** no common names (t=2669, 82 replies). After some years, regens and referees get names from other nations (xeno 64; t=4121: a "Flemming Beckford-Quailey" at Skonto Riga).
- **Regen attributes:** low determination and high positioning (t=4085).
- **Development is frustrating:** "random players get worse for no apparent reason"; the game takes CA from some to give to the young (t=3528, 119 replies).
- **Concentrated regens:** Tapani's change that moves regens to big nations fills the big clubs with youngsters (xeno 70). It's what we saw in the CLZ nationality patch.
- **More than 50 players or optional B teams** (xeno 21). See phase 6 of the roadmap.

### 4. Club economy
- Stadium and attendance should grow with success; capacity should grow at 90% occupancy (xeno 8).
- Balanced income and spending, with ticket prices, sponsorship and merchandising by reputation (12). Sponsorships almost never appear (92).
- Attendance at cup and Champions League matches is lower than at ordinary league matches (93). The stadium shrinks on bankruptcy (57).

### 5. Scouts and information
- "All the scouts I use always seem to come back with the same players recommended" (leovii, t=1593; xeno 51.1). Scouting takes too long (50.1), and there's no permanent scouting (50.2).
- Scouts and coaches with 20/20 potential judging should know that a PA 125 player is not a great signing for Manchester United (Telcontar, t=1593).
- Revealing hidden attributes through a longer scout research (Here2Win79, t=1593). A medical before signing (51.2).

### 6. Quality of life (screens)
- All manager vacancies on one screen (Jack Stein); a pause in holiday mode when a job offer arrives (18.2); delaying the answer to a job offer (62).
- Skipping World Cup and Euro matches; continuing with unread news (102); seeing the board's expectations (96).
- Squad numbers up to 99 (3.3); telling full-back from wing-back and defensive from central midfielder on screen (Telcontar); congratulating players, not just disciplining them (Here2Win79); a national team manager screen with the eligible players (Flu309).
- More holiday options: sell only transfer-listed players, return at the end of the season (18.1).

### 7. Stability and performance
- **The 25–28 December crash:** 205 replies (t=3077). It's actually a hang (an infinite loop), not a crash.
  - **A workaround exists:** Alan_v2 found the spot in October 2025 and published a one-byte patch (0x92EA55 `JNZ` → `JMP`, skipping the loop). He wrote himself that he doesn't know why it happens, and the topic keeps getting new cases up to October 2026.
  - **The spot:** `world_cup.cpp` (the routine at 0x92e940). A loop draws a slot among positions 2 to 7 of a list of 8 five-byte records (byte 0xFF marks a slot as taken) and repeats while the drawn slot is taken. If all six are taken, it never ends. Alan's skip puts the national team in a slot that's already taken.
  - **Hypothesis (to prove):** database updates changed nations and confederations, and in some careers, through the 2002 World Cup qualifying results, there are more national teams than slots. Hence only some careers, and only with new databases.
- "Game Crashing" (829 replies), errors like "virtual staff cpp 195" (t=3423), slow career creation (t=1582).

### 8. Platforms
- **Mac** (t=1569, 360 replies) and **Android** (t=1579, 296 replies): today only through Wine and emulators, slow and fragile.
- Resolution and screen (t=1583, 256 replies); higher resolutions don't work in network games (xeno 52).

### 9. Staff, national teams and nationality
- Chairmen and directors don't regenerate; a club that starts without a chairman never gets one (xeno 20).
- National team managers coaching small clubs at the same time (65); staff develop little in long careers (petermf23's item 18).
- Second nationality and foreigner restrictions (t=3668, 100 replies); the non-EU loan rule cancels the work permit (xeno 69).

## Where we have an edge

| Front | Why it's ours | Size |
|---|---|---|
| ~~Competition structures~~ | **out of scope:** davdav already does it, and does it well | (the biggest demand, but already served) |
| **The December crash and other crashes** | Alan_v2's workaround skips the loop; with the routine understood, we can fix the cause (the missing slot) and log what triggers it | small to medium; much requested |
| **Regens: intake, names, nationality, attributes** | we're already inside these routines | medium to large (the intake is phase 6) |
| **Loans and market AI** | the transfer and contract routines are in Rust | medium |
| **Player development** | the rule that redistributes CA can be replaced | medium; touches game balance |
| **Economy: stadium, attendance, sponsorship** | finances and income are in Rust | medium |
| **Scouts and information** | a rule (accuracy by judging attribute) and, later, screens | medium |
| **Quality of life (screens)** | needs new screens: after the identical interface, per the project's rule | large; later |
| **Mac and Android** | only with the whole game in Rust; a DLL won't do it | very large; long term |

## Compared with the hypotheses from before the reading

| Hypothesis | What the reading showed |
|---|---|
| 1. Long careers degrade | **Confirmed:** names from other nations, staff, regens, chairmen |
| 2. Performance | **Partial:** complaints about career creation and match speed, not about advancing days |
| 3. Modern Windows, video | **Confirmed**, and bigger than I thought: **Mac and Android** are big, and I hadn't foreseen them |
| 4. Market AI | **Confirmed**, with **loans** standing out, which I hadn't foreseen |
| 5. Match engine | **Different:** tactics are a hobby (benchmark leagues), not a complaint; the positioning penalty has been fixed |
| 6. Competition structures | **Confirmed, and it's the biggest demand** |
| 7. Youth | **Confirmed**, with **regen names** and **development** as themes of their own |
| 8. Staff | **Confirmed** |
| 9. Outdated rules | **Partial:** second nationality and foreigners, yes; Brexit barely shows up |
| 10. Morale and contracts | **Confirmed:** expired contracts that don't release, greyed-out squad status |
| 11. Tools | **Confirmed:** the save editor, and above all the database ecosystem |
| 12. Network play | **Weak:** few people, but it exists (resolution in network games) |
| 13. National teams | **Confirmed** (call-ups, qualifiers, FIFA ranking) |

**What I hadn't foreseen:** Mac and Android; loans; the stadium and attendance economy; scouts who always return the same names; the December crash; frustration with development; and the size of the **database update** culture, the forum's biggest activity.
