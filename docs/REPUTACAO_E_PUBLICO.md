# Reputação e público (estudo)

*English version: [REPUTACAO_E_PUBLICO.en.md](REPUTACAO_E_PUBLICO.en.md)*

Estudo de 07/11/2026. Pedido do Lucas:
- **o público** dos times pequenos é baixo demais e não cresce com o sucesso: não lota nem o estádio deles;
- **o jogador** que se destaca não ganha reputação suficiente para virar cobiçado pelo mercado, e **o clube** que faz sucesso também demora a ficar interessante.

Método, o mesmo dos regens: mapear no código (todas as rotinas abaixo já estão reescritas em Rust no projeto CM0102 e comparadas com a original), medir numa carreira automática com todas as ligas do jogo e propor chaves com o padrão igual ao original. O mercado em si está em [MERCADO.md](MERCADO.md).

**[observado]** = lido no código; **[medido]** = carreira automática; **[inferência]** = conclusão a provar.

## 1. O público

**Em cada jogo** (0x85cba0) [observado]: o público do mandante é a **média do clube** (+0x73) × um fator (a reputação da competição sobre a da liga, a posição na tabela, clássico, distância do visitante), e fica **entre o mínimo (+0x77) e o recorde (+0x7b) do clube**. A capacidade do estádio só limita depois.

**No fim da temporada de cada liga** (0x5d6280) [observado]: a média anda pelo resultado (campeão, vice, acesso, meio da tabela, queda), com somas pequenas e tetos de 2.000, 3.000 ou 5.000 × a reputação da liga, e com um peso por nação (0,1 em Gales, nas Irlandas e na Austrália; 0,25 nos nórdicos e nos EUA; 0,5 na Escócia). No fim fica, de novo, entre o mínimo e o recorde.

**O recorde (+0x7b) só sobe com título ou acesso** [observado: as únicas três escritas]: +20 × a reputação da liga no título, +10 × no acesso (só se o recorde estiver a menos de 500 da média) e o piso de quem sobe. **Nada o sobe por estádio cheio, por boa campanha ou por reputação.** O clube que faz uma ótima temporada sem ganhar nada bate a média no recorde e fica preso nele.

**O estádio** (0x59cfa0, fim da temporada) [observado]: cresce se a média real passa de 92% da capacidade, o que o recorde quase sempre impede, ou se a capacidade é menor que 2 × a reputação acima de 4.250, e com saldo de 5,25 milhões ou mais.

## 2. A reputação do clube

Por partida (0x6d3de0) [observado]: uma média móvel que anda para `reputação + (nível / 2 + 5) × (nota do time − média) × 50`, com peso **202 − 10 × nível** (o nível é a reputação da competição, até 20). Numa liga de nível 10 o peso é 102: um time 0,5 acima da média ganha **+2,4 por jogo**, uns 90 pontos numa temporada. O passo máximo é 5 × nível por jogo. Título: no máximo +750, até a reputação da competição × 500. Acesso: +500. Queda: −500.

## 3. A reputação do jogador

Por partida (0x6d26b0) [observado]: as três reputações (casa, atual, mundial) andam pela **nota** do jogo contra a média do campeonato. **Gols e jogos não contam**, só a nota. Os limites:
- **teto pela liga:** 4.000 + 300 × nível por partida (liga de nível 10: 7.000). A reputação do jogador de liga de reputação baixa não passa disso jogando;
- **teto pela habilidade:** CA × 50 + 2.500 (a mundial, CA × 50 + 1.250);
- **a mundial** quase não anda em jogo de clube: peso 202 − 10 × nível;
- passo máximo por jogo: 250 a 375 × (2 − reputação / 10.000).

## 4. O que a carreira e o levantamento mostraram

**Carreira-base** (todas as ligas, 2001 a 2005) [medido]: ocupação média de 49% nos clubes de reputação até 2.000 e de 85% nos de 8.000 a 10.000; 153 dos 1.122 clubes terminam com a média presa no recorde. Uma carreira de teste que puxava a média pela reputação no fim da temporada soltou o recorde (60 presos), mas foi descartada: cresce sem causa em campo, e os pequenos quase não mudam.

**O banco de dados** [medido nas tabelas de 2001-052]:
- o **recorde** (+0x7b) é o recorde histórico, não a torcida de hoje (Albion Rovers: média 250, recorde 18.321, estádio de 2.496); passa da capacidade em 415 dos 1.111 clubes;
- a **média** é o retrato de hoje; em 150 clubes foi preenchida pela reputação (reputação² ÷ 2.000; quase todos esses, 128, também com o mínimo = média ÷ 5), como os grandes do Brasil;
- **o teto de expansão do estádio** (+0x44) é 0 em 69% dos clubes das ligas de nível 16 a 20.

**Crescimento artificial que já existe no jogo** [observado, e medido na base]: quando não amplia, o clube converte lugares em pé em sentados e soma os lugares novos também na capacidade total, então o estádio com espaço em pé cresce todo ano, por sorteio e saldo, sem olhar público nem teto. Na base, 34 dos 66 estádios que cresceram tinham teto 0 (Bahia: 115.000 → 134.000 com média de 21.125 → 4.225). O gatilho "reputação × 2 acima da capacidade" também amplia sem público.

**O que o jogo já tem e que serve** [observado]:
- a posição prevista de cada clube (+0x72 da ficha da competição) e a atual (+0x73);
- no jogo, um bônus por ir melhor que o previsto; no fim da temporada, a média anda por (prevista − final + 1) e mistura o público real da temporada;
- o público real por competição (+5 jogos em casa, +6 público somado [inferência]) e por temporada nos recordes do clube [inferência];
- o código da rodada de cada jogo (120 quartas, 130 semifinal, 150 final) e o tipo da competição.

**O que estraga** [observado]: o recorde fixo (corta o jogo e a média), os tetos da média pela liga (2.000, 3.000 ou 5.000 × nível) e, no jogo, o corte das ligas de nível até 4 (acima de 1.500 × nível, só 20% do excesso).

**O estádio** (0x59cfa0) [observado]: dono, liga e saldo acima de 5,25 milhões; gatilho de uma temporada da liga acima de 92%; obra de 21 mil lugares com saldo acima de 20 milhões, até o teto e um terço dele; nenhum estádio acima de 125 mil amplia; custo de uns 100 por lugar (875 no profissional, todo sentado). Uma obra real de 2001 custava, em ordem de grandeza, de 1.000 a 3.500 libras por lugar [inferência, de memória].

## 5. As chaves (08/11; `src/publico.rs`)

Princípio: o público é **a torcida × o momento × o jogo**. Cresce pelo que acontece em campo e esfria quando o desempenho volta ao esperado; nada cresce sem causa. **O padrão é o nosso**: com um `tweaks.cfg`, cada chave ausente vale a nossa regra; sem `tweaks.cfg` nenhum, nada muda. Os números estão em porcentagem ou em pessoas, e o configurador mostra a conta de cada um (aba Público e estádio).

| Chave | Nosso padrão | Original | O que faz |
|---|---|---|---|
| `attendance` | `on` | `off` | o interruptor do público |
| `attendance_toward_real` | 50 | 50 | quanto (%) a média anda até o público real da temporada (os clássicos e jogos grandes entram sem o acréscimo deles) |
| `attendance_per_place` | 3 | — | quanto (%) vale cada posição acima ou abaixo do previsto |
| `tradition_per_place` | 2.5 | 0 | quanto (%) da distância até o recorde histórico volta por posição, com 2 ou mais acima do previsto |
| `attendance_floor` | `on` | — | o piso do original: quem cai não fica abaixo do que o jogo original daria (`.publico` ao lado do save) |
| `attendance_cap_weak` / `_mid` / `_strong` | 0 | 2000 / 3000 / 5000 | o teto da média em pessoas por ponto de nível da divisão (até 7, 8 a 12, acima); 0 = sem teto |
| `weak_league_limit` / `weak_league_keep` | 0 / 20 | 1500 / 20 | o corte do público de cada jogo nas divisões de nível até 4 (pessoas por nível) e quanto (%) fica acima dele |
| `attendance_record` | `real` | `fixed` | o recorde do clube é o maior público que de fato veio |
| `pent_up_demand` | 10 | 0 | quanto (%) de quem ficou de fora vira recorde novo |
| `knockout_ceiling` | `stadium` | `record` | no mata-mata, só o estádio limita |
| `title_race`, `title_race_max`, `title_race_top2` | `on`, 50, 25 | `off` | o jogo grande pela tabela: a mais (%) na última rodada e num 1º × 2º; o clássico (+50% do jogo) por cima |
| `stadium` | `on` | `off` | o interruptor do estádio |
| `stadium_seating_growth` | `off` | `on` | a conversão em sentados não soma mais na capacidade |
| `stadium_reputation_trigger` | `off` | `on` | sem o gatilho "reputação × 2 acima da capacidade" |
| `stadium_occupancy_min` | 92 | 92 | a ocupação (%) para a temporada contar como cheia |
| `stadium_full_seasons` | 2 | 1 | temporadas cheias seguidas (1 a 10) para poder ampliar; a contagem vai no `.publico` |
| `stadium_occupancy` | `all` | `league` | a ocupação conta todos os jogos em casa |
| `stadium_reserve` | 5250000 | 5250000 | o dinheiro que a obra não toca |
| `stadium_size` | `demand` | `original` | a obra do tamanho da demanda medida |
| `stadium_seat_cost` | `auto` | `original` | o custo por lugar: `auto` = 1.500 no banco de 2001, na proporção do caixa dos clubes grandes do banco carregado; ou um número |
| `stadium_ceiling_zero` | `demand` | `original` | no estádio sem teto no banco, o teto pela demanda medida |
| `stadium_max` | 125000 | 125000 | a maior capacidade que ainda amplia |

Com `log = 1`: `publico_jogos.tsv`, `publico_temporada.tsv` e `publico_estadio.tsv` ao lado do exe.

**Compatibilidade** [observado, 08/11]: nenhuma opção do NickPatcher nem patch do MiscPatches toca as três chamadas trocadas (0x6b86bc, 0x5d5cfb, 0x59cee4); o `protect_against_virtual_staff_195_error` muda 13 bytes no fim da rotina do público do jogo, que continua valendo (o ajuste chama a rotina inteira e protege também o visitante sem nação). Nos exes do CLZ 6.1, do CLZ 5.4 e da demo do NickPatcher, as chamadas estão como no 3.9.68.

**As carreiras de teste** (todas as ligas, 2001 a 2011, comparadas ao jogo original só registrando): com o padrão, o clube que termina no esperado fica estável; quem vai bem cresce de 30% a 55% numa temporada, diminuindo com os anos; quem vai mal não fica abaixo do original; a ocupação da liga vai de 60% para 76% em 9 temporadas; 85 obras em 10 temporadas (o original, 184, várias em estádio vazio), todas com duas temporadas cheias.
