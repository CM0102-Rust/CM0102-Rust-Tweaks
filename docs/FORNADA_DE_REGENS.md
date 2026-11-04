# A fornada de regens: o estudo (04/11/2026)

*[English](FORNADA_DE_REGENS.en.md)*

Estudo só de leitura (desmontagem, pseudocódigo e saves; nenhum código mudou, o jogo não rodou), pedido pelo Lucas antes de qualquer código. A pergunta: dá para redesenhar o ciclo dos regens, com as aposentadorias todas num dia, os regens todos num dia de promoção, um número garantido de regens por clube e fichas criadas pela DLL? Complementa [DISTRIBUICAO_DOS_REGENS.md](DISTRIBUICAO_DOS_REGENS.md) e a fase 6 do [ROADMAP](../ROADMAP.pt.md).

Marcas: **[observado]** no código ou nos saves; **[inferência]** pelo fluxo, a provar na bancada.

## 1. A resposta curta

| Peça | Dá? | Como |
|---|---|---|
| Aposentadorias todas num dia | **sim, simples** | trocar a data na porta única das aposentadorias (0x7a8510) |
| Regens todos num dia de promoção | **sim, simples** | segurar a fila (0x5cda7a) fora do dia |
| N regens garantidos por clube | **sim, médio** | o clube vem da cota no desvio da disputa que o Tweaks já tem (0x7ad0e0) |
| Fichas novas (mais regens que aposentadorias) | **dá, mas rende pouco** | linhas a mais na tabela de fichas na carga, mais as duas tabelas paralelas; só viram regens enquanto houver vaga parada (§5) |
| Mais regens por temporada | **sim** | mais aposentadorias, a partir dos ~13.000 sem clube: vaga e ficha juntas, com a população igual (§5) |
| Pessoas novas (vagas) | **não** | os 16 técnicos humanos são as últimas pessoas da tabela; e, numa carreira longa, a vaga é o limite (§5) |

## 2. Vaga e ficha

Todo regen jogador precisa de duas peças:

- **Vaga** é a **pessoa** aposentada (nome, nascimento, id, personalidade). O regen passa a ser essa pessoa, com nome e idade novos.
- **Ficha** é a **ficha de jogador** (`player.dat`, 70 bytes): PA, posições, pés e atributos. Ela decide se o regen é bom.

No começo da carreira **sobra vaga e falta ficha**: no `TesteCMRTE` (dez/2001), 845 vagas de não jogadores já vencidas esperavam ficha, com a reserva vazia [observado]. **Isso não dura:** no save de todas as ligas em 2063 não há vaga parada; a fila só tem vagas com data futura, quase todas de jogadores (§5).

## 3. O ciclo hoje

Tudo mora num objeto em `[0xb7434c]`, com quatro tabelas (ponteiro, quantidade, capacidade; crescem por `realloc` de 250 em 250, sem limite) gravadas no item `retire.dat` do save (0x7ad7c0 grava, 0x7adcd0 lê) [observado]:

| Tabela | Registro | O que é |
|---|---|---|
| +4 | 26 B | aposentadorias marcadas: id, clube, data do anúncio, data da aposentadoria, motivo, anunciada |
| +0x10 | 12 B | a fila de vagas: id, data |
| +0x1c | 16 B | a reserva de fichas de não jogador |
| +0x3c | 16 B | a reserva de fichas de jogador: id da ficha, nação, segunda nação, dono |

O laço do dia (0x5cda73..0x5cdaa9) chama, nesta ordem [observado]:

1. `find_new_retirements` 0x7a5790, só nos dias 0, 80, 173 e 250: marca quem vai se aposentar. Data: hoje (desempregado) ou o fim da temporada da nação (idade).
2. `process_retirements` 0x7a59e0, todo dia: na data, a ficha vai para a reserva e a vaga para a fila (hoje, ou hoje + 1 a 65 dias para quem tem 30 anos ou mais). Não jogadores: a fila (sem clube, técnico principal, clube humano, ou 1 em 4) ou um não jogador novo na mesma vaga (0x7ab340).
3. `process_regen_queue` 0x7a65d0, todo dia: enquanto houver vaga vencida e ficha na reserva, `regenerate_as_youth_player` 0x7abac0 junta as duas. Depois, os clubes da nação disputam o jovem (0x7ad0e0) e `player_regen_sign_for_club` 0x7a6f10 assina.

Números dos saves: 2.481 a 7.780 aposentadorias marcadas; da ordem de 2.000 a 4.000 aposentadorias por temporada [observado nos saves; a taxa por temporada é inferência].

**Achado à parte:** o aviso "vai se aposentar no fim da temporada" quase nunca sai no original. A lista é ordenada de um jeito que deixa para trás quem já passou da data do anúncio, e o laço para antes. Nos saves foram 0 anunciadas de 7.501 [observado + inferência].

## 4. As peças do redesenho

### 4.1 Aposentadorias num dia fixo

- **Onde:** `add_retirement` 0x7a8510 é a porta única das quatro origens. A DLL troca ali a data da aposentadoria pelo dia escolhido, e passa a lista uma vez na carga do save (para as que já estavam marcadas). A rotina do dia reordena sozinha [observado].
- **Detalhe:** a comparação é "antes de hoje", estrita: para aposentar no dia 31/07, gravar 30/07 [observado].
- **Até a data, a pessoa continua jogando normalmente** [observado].
- **Cancelamentos:** na data, o jogo desiste de quem está emprestado, tem proposta aberta ou mudou de clube há menos de 209 dias [observado]. Quanto mais longa a espera, mais desistências; um dia fixo dentro da janela de transferências aumenta isso [inferência]. **Sugestão:** o dia fixo ser o fim da temporada de cada nação (o original já usa essa data para a idade), não uma data do calendário igual para todos.
- **Custo:** milhares num dia. O `regen_origins.csv` da pasta de teste da 1.1 mostra 262 regens no mesmo segundo; o dia pesado deve levar poucos segundos [inferência].

### 4.2 Regens só no dia da promoção

- **Onde:** a chamada de `process_regen_queue` em 0x5cda7a. Fora do dia, a DLL zera a quantidade da reserva de fichas (+0x40) em volta da chamada original, e a fila não anda; a distribuição das fichas de técnico continua [inferência pelo código].
- **A data da fila:** gravar hoje nas duas chamadas de `queue_staff_for_regen` 0x7a87d0 (0x7a64c9 e 0x7a64e2), que o Tweaks já intercepta.
- **A ficha dividida** (o aposentado na fila ainda aponta para a ficha) só aumenta em número; as rotinas lidas pulam o aposentado (0x7a7eb0) [observado]. O risco que sobra são editores e rotinas não lidas [inferência].
- **Cuidado:** se a vaga tiver clube quando chegar a vez, a 0x7abac0 descarta a vaga sem gastar a ficha [observado].

### 4.3 N regens por clube

- **Onde:** a disputa 0x7ad0e0, que o Tweaks já desvia (com os nops em 0x7ace6a/0x7ace89). A DLL devolve o clube da cota; a assinatura (0x7a6f10), a notícia de juvenil promovido (0x7a8ba0, só para o humano) e o resto vêm do próprio jogo [inferência].
- **Elenco cheio:** a 0x7a6f10 procura vaga nas 50 do elenco (+0xd7) e devolve 0 se não houver. A DLL confere antes e passa ao próximo clube da cota [observado; a regra é proposta].
- **Contrato:** fim em +2 anos, ou +1 a 6 acima dos 18; na IA, no máximo 5 anos e número de camisa; no clube humano, vai para os reservas (0x4cc0f0) [observado].
- **O limite:** a oferta é de fichas da nação. Sem fichas novas, "3 por clube" só fecha nas nações que liberam fichas suficientes.

### 4.4 Fichas novas

#### Por que pessoas não

- Os 16 técnicos humanos são as últimas pessoas contadas: uns 100 lugares testam `id >= contagem − 16` e uns 60 calculam `contagem − id − 1` [observado].
- A tabela de 79 bytes por pessoa (0xae2d0c) é criada com o tamanho exato (0x82849a); o Staff_Records segue a contagem de não jogadores [observado].
- As 23.000 pessoas de reserva só existem durante a criação da carreira (consumidas por 0x527240 e pelo corte do banco); nos saves sobram 0 [observado].

A tabela de pessoas não cresce sem mexer nesses lugares. Isso limita as fichas novas (§5).

#### Por que fichas sim

- **Contagem:** `[0xae236c]`, tabela `[0xae23c4]`, apontada por endereço absoluto em `pessoa+0x61`. Nenhuma rotina da carreira muda a contagem; o save grava a que está em memória (0x531044) [observado].
- **A alocação** é um `malloc` só das fichas (0x529e0c na carga) ou um `realloc` no corte do banco (0x526010, jogo novo). Ali a DLL pode somar K linhas antes, sem mover ponteiro nenhum [observado; o desvio é proposta].
- **As 1.325 linhas depois da contagem são dos cinzentos:** `get_id_block` 0x90e760 dá blocos de 25 a partir da contagem. Subir a contagem enquanto há cinzentos vivos (partida, clube aberto pelo humano) sobrescreve as fichas deles [observado]. Por isso a reserva da DLL fica **depois** das 1.325, ou a contagem só sobe quando não há cinzentos.
- **Duas tabelas paralelas por ficha** têm de crescer junto:
  - PlayerStats `[0xb74350]`: 39 B por ficha em +0x30, quantidade própria em +0x34. A 0x7c6650, chamada pela 0x7abac0, escreve ali sem conferir o limite [observado].
  - O treino `[0xde1f44]`: 6 B por ficha em +0x651d. A carga do `training.dat` exige exatamente `[0xae236c]` registros (0x8b11c2..0x8b11d2), senão dá erro [observado].
- **Ordem na carga:** a contagem só sobe depois do `training.dat` e do PlayerStats lidos [observado o limite; a ordem é proposta].
- **Varredura:** só 43 referências ao vetor de fichas no exe; as outras (diário, partida, virtual_staff) são dinâmicas [observado]. O risco é uma contagem própria fora delas.

#### A ficha nova

O jogo já faz ficha por molde: `create_virtual_player` 0x90e0a0 copia os 70 bytes de um jogador real e troca só o id, o número, o CA e as reputações. Prova que uma cópia crua é aceita [observado]. A ficha não tem ponteiros; os vínculos (dono, PlayerStats, treino) são todos pelo índice da linha [observado].

Receita mínima:

1. Uma linha `i` contada, sem dono e fora da reserva.
2. Copiar os 70 bytes de um molde vivo da mesma nação e posição.
3. Ajustar `+0 = i`, `+4 = 0` (número), `+0x45 = 10` (moral), `+7 = PA` sorteado (1..200).
4. Inicializar a entrada de treino (como em 0x8b3c50); a 0x7abac0 zera o PlayerStats.
5. Pôr `{i, nação, 0, −1}` na reserva (+0x3c), crescendo com o `realloc` do jogo (0x945667).

Coerência: atributos absolutos de 1 a 20, uma posição = 20, um lado = 20 (fora goleiro), um pé = 20. Os códigos de PA da base (−1, −2, −9) só valem na criação. Em memória, os 18 atributos técnicos ficam relativos ao CA (`(10 × nota − CA) / 2`): um molde tirado do arquivo da base precisa de conversão [observado].

### 4.5 O que o PA da ficha dá

A 0x7abac0 sorteia o CA como fração do PA, pela posição (nação no jogo ou com 350 pessoas ou mais) [observado]:

| Posição | CA |
|---|---|
| atacante | 0,3·PA + sorteio(0,6·PA) |
| zagueiro | 0,25·PA + sorteio(0,55·PA) |
| goleiro | 0,25·PA + sorteio(0,5·PA) |
| outras | 0,25·PA + sorteio(0,6·PA) |

Com teto pela idade: 80, 90, 100 e 110 entre 15 e 20 anos; cerca de 75% dos regens têm menos de 19. Na prática, PA 100 a 130 dá CA médio de 55 a 70, e PA 180 a 200 trava em 80 a 110 [inferência pelas fórmulas]. As nações fora do jogo com menos de 350 pessoas usam outras fórmulas e idades de 17 a 27.

## 5. A conta (correção de 04/11, pergunta do Lucas)

A primeira versão deste estudo dizia que "a fila já tem vagas sobrando" e que, por isso, bastavam fichas novas. Era uma leitura de saves do começo da carreira. A conta com o save de todas as ligas (`All Leagues - 3.9.68 - Compress.sav`, 2001 a 2063, 62 temporadas) mostra outra coisa [observado, `conta.py`, `hoje.py` e `np.py`, só leitura]:

| | Todas as ligas, 2063 |
|---|---|
| Clubes no banco | 10.892 |
| Clubes de nação com liga escolhida | 7.283 (3.889 de liga jogável) |
| Clubes com alguém no elenco | 6.752 (média de 13 no elenco; 9 cheios) |
| Pessoas (tabela fixa) | 158.790: 114.708 com ficha de jogador, 44.801 com ficha de não jogador |
| Regens jogadores vivos por ano de nascimento (2036 a 2043) | 6.624 a 7.833 por ano |
| Fila e reserva no save | 1.504 vagas, todas com data futura (1.428 de jogadores), e 1.913 fichas esperando essas datas |
| Sem clube | 9.303 regens jogadores e 4.054 não jogadores |

**O que a conta diz:**

- **O ritmo natural é de uns 7.500 regens jogadores por temporada** com todas as ligas, um por aposentadoria de jogador.
- **3 por clube** dá 11.667 por temporada só nos 3.889 clubes de liga jogável, e 20.256 nos 6.752 com elenco. Faltariam de 4.200 a 12.800 por temporada.
- **A vaga é o limite numa carreira longa, não a ficha.** Cada regen precisa de uma pessoa, e a tabela de pessoas é fixa. As 845 a 1.260 vagas paradas dos saves do começo são um estoque que acaba (os não jogadores da base que se aposentam cedo). Em 2063 não há vaga parada.
- **Logo, fichas novas sozinhas não dão regens a mais por muito tempo:** sem pessoas novas, elas só gastariam o estoque inicial de vagas e depois ficariam esperando na reserva.
- **Com a população fixa, regens a mais por temporada só vêm de aposentadorias a mais.** Hoje, uns 7.500 de ~115.000 jogadores se renovam por temporada (carreira média de umas 15 temporadas). Para 11.700 por temporada, a carreira média cairia para umas 10.
- **A reserva para isso já existe no mundo:** 9.303 regens jogadores e 4.054 não jogadores sem clube. Aposentar mais cedo quem está parado dá vaga e ficha juntas, sem tabela nova; é o `retire_idle_players`, numa dose maior.

### O que sobra para crescer a população

Pessoas novas exigem mudar os ~160 lugares que tratam as 16 últimas pessoas como técnicos humanos (ou mover os humanos), a tabela de 79 bytes (0x82849a) e o Staff_Records. Fica fora do alcance agora; o caminho viável é renovar mais rápido a população que existe.

### O efeito do dia fixo e das fichas novas

- **Só com o dia fixo** (4.1 a 4.3): ninguém novo. O número de regens é o mesmo, só concentrado.
- **Com fichas novas** (4.4): regens a mais só enquanto houver vaga parada (o começo da carreira).
- **Com aposentadorias a mais:** regens a mais em qualquer fase, com a população igual.

- **Só com o dia fixo** (4.1 a 4.3): ninguém novo. O número de regens é o mesmo, só concentrado; a primeira temporada não tem mais aposentadorias, elas só se juntam.
- **Com fichas novas** (4.4): cada ficha é um jogador a mais, para sempre. 3 por clube em 300 clubes = 900 por temporada. Mais jogadores dão mais aposentadorias anos depois, o que freia o crescimento; o equilíbrio tem de ser medido em carreiras longas.
- **Sem fichas novas, mais regens:** aposentar mais cedo quem ninguém quer (`retire_idle_players`, já no Tweaks).

## 6. Riscos e o que ainda falta conferir

- Os cinzentos vivos na hora de subir a contagem.
- Esquecer o PlayerStats ou o treino, ou subir a contagem antes da carga do `training.dat` (erro na carga).
- Uma contagem própria fora das 43 referências.
- O significado dos 6 bytes de treino por ficha.
- Fichas órfãs (contadas, sem dono, fora da reserva): o `Teste.sav` da pasta de teste da 1.1 tem 130; o `TesteTelas` e o `Lucas.sav`, nenhuma [observado]. Se o jogo já convive com órfãs, uma ficha criada e ainda não entregue é um estado tolerado [inferência]; falta saber de onde vêm as 130.
- Editores (CMRTE, CM3SaveEditor) com contagens maiores e com a ficha dividida por mais tempo.
- Os exes do NickPatcher e do CLZ, que mexem nas aposentadorias (RetirePatchV4).

## 7. A ordem proposta

1. **O dia fixo** (4.1 e 4.2), sem nada novo no mundo: mede as desistências, o custo do dia e o efeito nos clubes.
2. **A cota por clube** (4.3) com os regens que já nascem: mede quantos clubes fecham a cota em cada nação.
3. **O ritmo das aposentadorias**, para chegar ao número de regens por temporada que se quer, a partir de quem está sem clube.
4. **As fichas novas** (4.4) só se sobrar vaga parada; pessoas novas, só como projeto à parte.

Cada passo com a sua chave no `tweaks.cfg`, desligado por padrão, e provado na bancada antes do seguinte.

## 8. Endereços

| Endereço | O que é |
|---|---|
| 0x5cda73..0x5cdaa9 | o laço do dia: aposentadorias, fila, novas aposentadorias |
| 0x7a5790 | `find_new_retirements` (dias 0, 80, 173, 250) |
| 0x7a59e0 | `process_retirements` |
| 0x7a8510 | `add_retirement`, a porta única |
| 0x7a87d0 | `queue_staff_for_regen` (chamadas 0x7a64c9, 0x7a64e2) |
| 0x7a65d0 | `process_regen_queue` (chamada 0x5cda7a; a reserva vazia em 0x7a6618) |
| 0x7abac0 | `regenerate_as_youth_player` |
| 0x7ab340 | `regenerate_as_non_player` |
| 0x7ad0e0 | a disputa pelo jovem (desviada pelo Tweaks) |
| 0x7a6f10 | `player_regen_sign_for_club` |
| 0x7ad7c0 / 0x7adcd0 | grava e lê o `retire.dat` |
| 0x529bfe, 0x52a04b..0x52a05a | as 1.325 linhas dos cinzentos na carga |
| 0x529e0c / 0x526010 | a alocação das fichas: carga / corte do banco |
| 0x8b11c2..0x8b11d2 | a carga do `training.dat` exige a contagem exata |
| 0x90e760 / 0x90e0a0 | os blocos dos cinzentos / a ficha por molde |
| `[0xae236c]`, `[0xae23c4]` | contagem e tabela das fichas |
| `[0xb7434c]` | o objeto dos regens |
| `[0xb74350]`, `[0xde1f44]` | PlayerStats e o treino |
