# Os olheiros: a fama no julgamento

*English version: [OLHEIROS.en.md](OLHEIROS.en.md)*

Relato do fórum (25/11/2026): depois de algumas temporadas, quase nenhum relatório de olheiro com 5 estrelas, embora o CM Scout mostre regens de PA perto de 200. No começo do jogo, muitos jogadores saíam com 5 estrelas e "excelente contratação".

## 1. Como o jogo julga

`match_man_judge_player_ability` (0x6e86d0, cdecl, 5 argumentos: quem julga, o jogador, o clube, o modo e a reputação, ou −1 para o jogo buscá-la) estima o CA (modo 2) ou o PA de um jogador:

`estimativa = ((valor real + ruído) × p + r × (20 − p)) / 20`

- `p` = julgamento do olheiro / 2 + 10, de 10 a 20;
- `r` = a reputação do jogador vista por quem julga (`database_get_staff_reputation_for_viewer` 0x5407e0) / 50; no PA, menos (idade − 23)², ou (idade − 26)² no goleiro;
- o ruído encolhe conforme o julgamento do olheiro.

Um olheiro de julgamento 10 vê 75% de habilidade e 25% de fama; só o de 20 vê a habilidade pura. No começo do jogo, os bons jogadores são as estrelas do banco, de reputação alta: a fama ajuda. Anos depois, os bons são regens jovens e desconhecidos: a fama atrapalha. Um regen de PA 190 e reputação 300 sai, para um olheiro de julgamento 10, perto de 144.

## 2. As estrelas

O relatório (`PLAYER_SEARCH::report_on_player` 0x7b62a0) dá as estrelas pela nota de `match_man_score_player_for_role` (0x6e2f20), que chama o julgamento acima para o CA e para o PA. A nota é comparada com uma meta do clube do técnico (`player_search_get_club_target_rating` 0x7bca90: a reputação do clube, a da liga, a situação profissional, o elenco e as finanças, de 1.750 a 35.000). Cinco estrelas: acima da meta e acima de reputação do clube × 2 + 10.750 (no potencial, + 10.500). Conforme o clube cresce, a régua sobe; isso é desenho do jogo e fica como está.

## 3. A opção (`scout_reputation`, 0 a 100; 100 = o original)

Só para os olheiros dos clubes de técnico humano (`0x600a50`): quando o jogo pede a reputação (o 5º argumento −1), a DLL passa no lugar dela a mistura `k × reputação + (1 − k) × valor × 50`. Com 0, o termo da fama vira o próprio valor, e sobra só o ruído, que já depende do julgamento do olheiro. A IA (o mercado, as listas de procura) continua julgando como no original. Desligada (100) no pacote. Código em `src/jogo/olheiros.rs`.

**Não medido em carreira:** a carreira automática não manda olheiros. A prova é a leitura do código e uma carreira escondida com `scout_reputation = 0` sem queda ([TESTES.md](TESTES.md)).
