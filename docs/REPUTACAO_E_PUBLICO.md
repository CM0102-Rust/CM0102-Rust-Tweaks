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

## 4. O que a carreira mediu

*(a preencher com a carreira de 07/11)*

## 5. Chaves propostas

*(a preencher)*
