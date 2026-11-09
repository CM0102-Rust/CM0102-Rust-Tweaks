# Pessoas novas: o estudo de viabilidade (04/11/2026)

*[English](PESSOAS_NOVAS.en.md)*

Pedido do Lucas: "a solução é criar pessoas; é entender se é viável, não importa o esforço". Estudo só de leitura (desmontagem do `.text` inteiro, pseudocódigo e saves; nada mudou, o jogo não rodou). Continua [FORNADA_DE_REGENS.md](FORNADA_DE_REGENS.md) §5: numa carreira longa, quem limita os regens é a vaga (a pessoa), não a ficha.

Marcas: **[observado]** no código ou nos saves; **[inferência]** pelo fluxo, a provar na bancada.

## 1. A resposta

**É viável, de dois jeitos.** Nenhuma tabela, item do save ou editor prende a contagem de pessoas a um número fixo [observado]. O que dá trabalho são os 16 técnicos humanos, que o jogo identifica como as 16 últimas pessoas da tabela.

| | Caminho 1: estoque na criação | Caminho 2: crescer em qualquer save |
|---|---|---|
| Como | o próprio jogo já cria pessoas a mais ao montar a carreira; a DLL pede K a mais e as deixa adormecidas, soltando-as pela fila dos regens | a tabela de pessoas cresce no meio da carreira, depois dos humanos |
| Patches | 1 desvio no corte do banco, mais a marcação e a soltura | ~276 nos humanos, ~15 nos cinzentos, ~8 vetores que crescem junto |
| Vale para | carreira nova | carreira nova e save existente |
| Save | formato igual, abre sem a DLL | formato igual, mas só abre certo com a DLL; o CMRTE erra os humanos |
| Limite | K fixado na criação (~20 mil sem mexer na alocação; mais, aumentando o `malloc`) | sem limite prático |

**Recomendação:** o caminho 1 primeiro. Ele cabe nas duas semanas e já é a fornada de verdade para carreira nova. O caminho 2 vem depois, para os saves existentes, e reaproveita tudo do 1.

## 2. O que o jogo já faz na criação da carreira

O corte do banco (`database_trim_staff_tables` 0x524d70) mantém as pessoas das nações escolhidas e **cria pessoas a mais**: 1%, ou 2% numa opção, das que mantém. A conta está em 0x525f84..0x525f8c (`fild`/`fmul`/`ftol`) [observado]:

- cada pessoa criada vem com ficha de jogador e preferências, nasce em 1982 e fica sem clube;
- elas entram **antes** dos 16 humanos, e o corte move os humanos para as 16 últimas linhas;
- tudo o que vem depois (a tabela de 79 bytes, os subsistemas, PlayerStats, treino) nasce com a contagem final.

**Um desvio nesse `ftol` dá K pessoas a mais**, com ficha, sem patch nenhum nos humanos, nos cinzentos ou na gravação, porque a contagem nunca muda depois [observado o trecho; o desvio é proposta].

Limites [observado]:

- o `realloc` do corte aborta se o bloco mudar de lugar, então a folga natural é de uns 20 mil (das 23.000 de reserva). Acima disso, aumentar o `malloc` em 0x529e02 (pessoas) e 0x529e11 (fichas);
- o teto em 0x525fa4 tem um defeito: se K passar do limite, ele soma em vez de subtrair (`lea ebx,[esi+eax+0x52d]`), e o extra fica enorme. O desvio corrige isso.

## 3. O caminho 1 em detalhe

1. **Criar:** o desvio no `ftol` devolve K (por exemplo, pela soma das cotas das temporadas planejadas).
2. **Adormecer:** logo depois de `create_game` 0x828110, antes de `game_run_to_start_date` 0x5cc850, a DLL marca as K pessoas como aposentadas: sem clube, com os bits 0x30 do registro de 79 bytes ≠ 0. É o mesmo estado das 1.260 pessoas que ficam na fila esperando ficha nos saves de teste, ou seja, um estado que o jogo já conhece [observado]. As fichas delas vão para a reserva no momento certo.
3. **Soltar:** no dia da promoção, a DLL põe as vagas na fila (`queue_staff_for_regen` 0x7a87d0) e as fichas na reserva; o jogo faz o regen (0x7abac0) e a assinatura com o clube da cota (FORNADA_DE_REGENS §4.3).
4. **Gravar e carregar:** nada a fazer; a contagem é a de sempre.

A conferir:

- se uma pessoa adormecida passa inerte pelos 104 laços sobre todas as pessoas (o estado é o da fila, mas por anos, não por semanas);
- se `game_run_to_start_date` já cria cinzentos (não importa no caminho 1; importa no 2);
- o custo por dia com K grande: os laços ficam mais longos na proporção das pessoas.

## 4. O caminho 2 em detalhe

### 4.1 Os humanos

1.155 usos das contagens no exe; 960 deles são da contagem de pessoas `[0xae2364]` [observado, varredura do `.text` inteiro conferida por busca de bytes]:

| Uso | No jogo | Na criação e carga |
|---|---|---|
| laço sobre todas as pessoas | 104 | 11 |
| limite/validação de id | 423 | 1 |
| humano | 275 | 77 |
| alocação | 19 | 1 |
| cinzentos | 8 | 0 |
| gravação/carga | 20 | 6 |
| outro (sorteio, proporções) | 15 | 0 |

Com uma global nova `CH` ("fim dos humanos", fixa) no lugar da contagem nos usos de humano:

- **191 lugares** só trocam os 4 bytes do endereço lido: piso e teto, ids gerados a partir do índice do humano, sem teste;
- **76 lugares** testam só `id ≥ contagem − 16`, contando com o teto implícito (acima da contagem é cinzento). Uma pessoa nova depois dos humanos passaria por humano: cada um vira um desvio;
- **8 laços** "todos menos os humanos" param em contagem − 16 e são reescritos: 0x5d6ef0, 0x69bcd1, 0x69c13a, 0x69c549, 0x7a5936, 0x7a6eab, 0x7a6ee9, 0x83a10d (e talvez o laço de depuração em 0x52d20a).

As tabelas de 16 entradas indexadas pelo índice do humano (HUMAN_MANAGER, prêmios, olheiros, treino, notas, cinzentos, névoa, Staff_Records e as globais 0xb6b7d0, 0xb73390, 0xadab50) continuam certas com `CH` [observado].

**Mover os humanos para o fim (renumerar)** foi descartado: os ids deles ficam em notícias, empregos, buscas, prêmios, contratos, históricos e no `club.dat`, e a lista não fecha. Um esquecimento corrompe a carreira em silêncio.

**`CH` na carga:** os humanos têm a classificação +0x5f = 0 (o Tweaks já usa isso), então dá para recalculá-lo sem arquivo novo [inferência].

### 4.2 Os cinzentos

- **Vida:** são criados por partida (0x6b43f8, 0x6b4a43, 0x6b5bc1, 0x84c339) e por tela de elenco (0x484215), e removidos depois da partida (0x90d6b0), quando o clube contrata (0x90d860) e na carga (o resolve 0x5315a0 zera os elencos) [observado].
- **Espaço:** 53 blocos de 25 = 1.325 linhas a partir da contagem; num jogo de um humano, uns 7 blocos vivem ao mesmo tempo [observado].
- **Os ~400 testes `id ≥ contagem`** só querem dizer "não é pessoa real" e continuam certos com qualquer base virtual acima da contagem. **Só `get_id_block` 0x90e760 depende de onde fica a área virtual** [observado].
- **Base fixa:** 14 trocas de endereço em 0x90e791..0x90e984 e uma em 0x897a4f (Tactics). Outra saída é crescer só quando não há cinzento vivo.
- **Defeito do original achado de passagem:** `relocate_virtual_players` 0x90e9f6 compara com sinal e nunca sobe os cinzentos de partida para o bloco do clube visto; duas vagas podem apontar a mesma linha. É a origem dos ids ≥ N nos elencos do `TesteTelas` [observado].

### 4.3 As tabelas por pessoa

Têm uma entrada por pessoa, crescem junto (`realloc` no começo do dia; ninguém guarda ponteiro para dentro delas) e, na carga, exigem exatamente N registros [observado]:

| Estrutura | Por pessoa | Entrada nova |
|---|---|---|
| a tabela de 79 B `[0xae2d0c]`, contagem `[0xae2d10]` (`general.dat`) | 79 B | o molde de `create_game` (0x82854b), com os bits 0x30 = 0x20 |
| Discipline +0x14 (`discipline.dat`, 0x5559d1) | 29 B | 28 bytes 0xff e +0x1c = 0 |
| Injury +0 (`injury.dat`, 0x62ac35) | 31 B | como na criação (0x62d031) |
| névoa de guerra (`fog.dat`, 0x5b009e), por humano | 9 B | nível pela 0x5b03e0, data de hoje |
| TRANSFER_MANAGER +0x86e (`transfer.dat`, 0x8bcb16) | 8 B | zeros |
| Staff_Records +0xc (`staff records.dat`) | 8 B | −1, −1 |
| AWOL +0xc, DISPUTE +4 | 1 B | zero |

O resto são listas com ids dentro (contratos, notícias, olheiros, seleções, ofertas, prêmios, históricos) e não precisa crescer. O SearchEngine é alocado quando a tela de busca abre: não crescer com ela aberta.

### 4.4 A tabela de pessoas e a de fichas

Não podem mudar de lugar: há ponteiros para pessoas nos elencos, nas preferências, no HUMAN_MANAGER (0xb74c26), nas pessoas (+0x61) e nas partidas [observado]. A folga é reservada no `malloc` da carga (0x529e02, 0x529e11), antes do resolve, e no `realloc` do corte (0x526003). Na carga, tudo o que é por pessoa só fica pronto na volta de `load_game_data` 0x82a9c0; é ali, sem cinzento vivo, que a contagem pode subir.

## 5. O save e os editores

- **Gravação:** cada item grava a contagem da memória; o `index.dat` leva a quantidade de cada tabela; o cabeçalho e a cauda não dependem dela. O formato não muda [observado].
- **Editores:** o SGE (original e a reescrita), o SaveChanger e o Scouter tiram a quantidade do tamanho do item; o CMExplorer, do `index.dat`; o CMRTE lê a contagem ao vivo, mas assume que as 16 últimas pessoas são os humanos [observado nos fontes disponíveis]. **No caminho 1, todos funcionam. No caminho 2, o CMRTE erra os humanos.**

## 6. O tamanho do trabalho

- **Caminho 1:** um desvio no corte, a marcação das adormecidas, a soltura pela fila (com o dia fixo e a cota da FORNADA_DE_REGENS) e as provas na bancada (carreira com K = 20 mil: estável, tempo por dia, save, editores). Cabe em duas semanas [inferência].
- **Caminho 2:** ~276 patches nos humanos (191 mecânicos, geráveis por ferramenta a partir da lista; 76 desvios; 8 laços), ~15 nos cinzentos, 8 vetores que crescem junto, e a prova de cada um. Mais de duas semanas, mas sem bloqueio conhecido [inferência].

## 7. A conferir antes do código

- A pessoa adormecida por anos nos 104 laços (caminho 1).
- O laço de depuração em 0x52d20a; ler um a um os 76 desvios; o `CONTRACT_MANAGER` +4; ponteiros de pessoa guardados nos subsistemas (caminho 2).
- O significado dos valores do "tamanho do banco" no `general.dat`.

Os relatórios completos e as listas (os 1.155 usos, um por linha) ficaram no rascunho da sessão; entram no repositório quando o código começar.

## 8. O caminho 1 na bancada (04/11/2026, experimental)

**O nome:** "fornada" vem da tradução do Football Manager em português de Portugal. No FM em inglês é *Youth Intake* (o dia, *Youth Intake Day*), e as chaves seguem esse nome (`intake_*`); no FM em espanhol, *hornada de juveniles*.

O código está em `src/regens/fornada.rs`. Saiu na versão 1.2 (05/11/2026, com o ok do Lucas), ainda experimental. Fica desligado sem as chaves; nada muda no jogo de quem não as usa.

**O desenho final (05/11, noite; decidido com o Lucas ponto a ponto). Substitui o de 05/11 à tarde.** O princípio: **regen é regen**. De onde ele vem não importa, e o programa não trata regens de origens diferentes de jeitos diferentes. A fornada muda duas coisas: a data (os regens só chegam no dia da fornada) e a quantidade (todo clube com vaga recebe `intake_per_club`, 3 por padrão). O resto é a aba Regens.

1. **A data.** Com a fornada ligada, quem se aposenta espera na fila do jogo, e os regens só nascem no dia da fornada. O padrão é o fim da temporada de cada país (`season_end`); o dia fixo é opção. As aposentadorias podem ir para um dia só (`retire_day`).
2. **A cota.** No dia, cada clube das ligas jogáveis (as de fundo, com `intake_clubs = selected`) tem a cota de `intake_per_club`, limitada às vagas do elenco (50): com 2 vagas, 2; sem vaga, nenhum, e o técnico humano recebe a mensagem de falta de vaga. A vaga é contada no dia: quem libera jogadores até a véspera recebe. O clube humano recebe pela mesma regra.
3. **Os regens do dia.** Os aposentados das nações do dia nascem todos pela rotina do jogo (`regenerate_as_youth_player` 0x7abac0: o PA, as posições, os pés e o perfil técnico e mental da ficha herdada; o CA, o físico, a idade, o nome e as línguas sorteados), sem ir à disputa ainda. Entram também os regens livres de até 21 anos da nação (os que sobraram de fornadas anteriores).
4. **Os que faltam.** Os aposentados não bastam para 3 por clube (na carreira de 10 anos faltaram de 136 a 419 por ano com 278 clubes; com todas as ligas, uns 4.200 por ano). Os que faltam nascem do mesmo jeito: a pessoa vem da reserva criada com a carreira (oculta, 50 mil pedidas; o jogo dá o que couber na folga do corte do banco, 120 mil com Brasil e Inglaterra); a ficha é a mistura de duas fichas de aposentados da nação **da mesma posição** (goleiro com goleiro, zagueiro com zagueiro, meia com meia, atacante com atacante): as posições e os pés de uma, cada atributo de uma das duas, e o PA sorteado entre os PAs das duas. Os pais são os regens que acabaram de nascer de aposentados (a ficha deles é a do aposentado), então os criados seguem a distribuição de PA e de posições de quem se aposenta: os da própria nação; com menos de 50 no dia, também os das outras nações do dia (a nação do regen continua a do clube); com menos ainda, os jogadores de 30 anos ou mais da nação. Depois, a mesma 0x7abac0. Misturar evita os clones: copiar uma ficha repetiria o PA, as posições, os pés e o perfil técnico e mental. (Nos testes, jogadores em atividade como pais deram criados fortes demais, PA mediano 112 a 118 contra 70 a 80 dos aposentados; com os pais de agora, 71.)
5. **A distribuição.** O grupo inteiro da nação (os do dia, os livres e os criados) é distribuído de uma vez, do maior PA para o menor, cada um sorteado pela curva da aba Regens (nível, instalações, comissão, surpresa, temperatura) entre os clubes que ainda têm cota. Cada clube termina com a sua cota; a curva decide **quais** regens vão para cada um. Com a fornada ligada, o teto por temporada (`season_cap`) e o espaço no elenco (`squad_limit`) não entram: a cota já é o limite.
6. **As sobras.** Regens do dia que sobram depois das cotas cheias vão para a disputa normal da nação, entre os clubes com vaga; sem clube com vaga, ficam sem clube, como no original.
7. **Fora da garantia.** As nações não escolhidas (e as de fundo, no padrão) seguem o caminho original do jogo.
8. **A mensagem.** O clube do técnico humano recebe a lista dos que chegaram, com o parecer; sem vaga, a mensagem diz que não chegou ninguém por falta de vaga. As frases estão no `intake_news.txt`.

O que saiu (eram regras paralelas, só para os jovens do estoque, e não fazem sentido com regen é regen): `intake_pa_min`, `intake_pa_max`, `intake_pa_spread`, `intake_surprise`, `intake_squad_max`, `intake_only` e `intake_stock` (a reserva é fixa e oculta). A chave que liga é `intake = 1`.

**Limite conhecido:** a reserva cabe na folga do corte do banco (as pessoas e as fichas das nações que ficam de fora da carreira). Com todas as ligas escolhidas, sobra pouca folga de fichas; aumentar a alocação (0x529bfe, fichas; 0x529b39, pessoas) fica para quando uma carreira com todas as ligas mostrar a falta.

**O resultado do desenho final (05/11, noite).** Carreira automática de férias, Brasil, Inglaterra e Itália (278 clubes), fornada no fim da temporada de cada país (o padrão), de 2001 a 29/03/2008 (parou numa caixa de fechar janela, não da DLL; seis anos e meio bastaram) [observado, `resultados/final5longa.log`]:

| Fornada | Clubes | Nascidos da fila | Criados | Entregues | Sem vaga | Sobras assinadas | Reserva restante |
|---|---|---|---|---|---|---|---|
| 2002 (BRA, dia 10) | 112 | 1 | 335 | 336 | 0 | 0 | 49.665 |
| 2002 (ING/ITA, dia 169) | 166 | 413 | 117 | 436 | 11 | 42 | 49.548 |
| 2003 BRA | 112 | 168 | 168 | 336 | 0 | 0 | 49.379 |
| 2003 ING/ITA | 166 | 749 | 0 | 404 | 18 | 298 | 49.379 |
| 2004 BRA | 112 | 135 | 201 | 336 | 0 | 0 | 49.178 |
| 2004 ING/ITA | 166 | 442 | 105 | 403 | 13 | 91 | 49.073 |
| 2005 BRA | 112 | 130 | 206 | 336 | 0 | 0 | 48.867 |
| 2005 ING/ITA | 166 | 534 | 93 | 446 | 9 | 97 | 48.774 |
| 2006 BRA | 112 | 153 | 180 | 336 | 0 | 0 | 48.593 |
| 2006 ING/ITA | 166 | 697 | 0 | 492 | 1 | 148 | 48.593 |
| 2007 BRA | 112 | 193 | 140 | 336 | 0 | 0 | 48.452 |
| 2007 ING/ITA | 166 | 840 | 33 | 498 | 0 | 244 | 48.419 |
| 2008 BRA | 112 | 216 | 114 | 336 | 0 | 0 | 48.305 |

- Todo clube com vaga recebeu a cota em todas as fornadas: nenhuma cota faltando, nenhuma nação sem pais. A reserva de 50.000 foi criada inteira (folga de 120.467); em seis anos e meio gastou 1.695, uns 200 anos no mesmo ritmo.
- Os criados só entram quando os aposentados não bastam: no Brasil, metade ou mais da fornada; na Inglaterra e na Itália, de nenhum a 117.
- As sobras são da Inglaterra e da Itália, onde se aposentam mais do que as cotas somam; vão para a disputa normal, como combinado. Por isso um clube passa de 3 por ano: o máximo de regens num clube foi de 5 (2002) a 27 (Varese, 2007).
- Elenco médio de 35 a 38 (mínimo 5 a 10, máximo 50 ou 51); regens com PA 150 ou mais no fim de cada temporada: 25, 42, 96, 165, 236, 339.

**Todas as ligas, 10 anos, antes das correções (06/11).** A mesma configuração, agora a do `tweaks.cfg` distribuído, com todas as nações do jogo original selecionadas (1.134 clubes), de 2001 a 2011-199, sem quedas e sem caixas de erro [observado, `resultados/todas10anos.log`]. A fornada do dia 169 (o fim da temporada da maior parte da Europa):

| Ano | Nascidos da fila | Criados | Entregues | Faltando | Sem vaga | Sobras assinadas | Reserva restante |
|---|---|---|---|---|---|---|---|
| 2002 | 1.176 | 1.068 | 2.121 | 66 | 36 | 91 | 47.919 |
| 2004 | 1.439 | 948 | 2.128 | 66 | 25 | 282 | 45.658 |
| 2006 | 2.411 | 445 | 2.313 | 35 | 0 | 656 | 44.041 |
| 2008 | 3.249 | 241 | 2.316 | 36 | 1 | 1.157 | 43.172 |
| 2011 | 3.055 | 118 | 2.294 | 43 | 3 | 876 | 42.372 |

- A reserva de 50.000 coube com todas as ligas e gastou 7.600 em 10 anos.
- Elenco médio de 29,8 (2001) a 36,3, estável de 2005 em diante; máximo 50 a 52.
- Regens com PA 150 ou mais no fim de cada temporada: de 20 (2002) a 1.008 (2011).
- Duas paradas longas (mais de 60 s numa data, em 2001-194 e 2010-169), que o jogo venceu sozinho.
- **Gravar e carregar:** o save de 2011 (no dia da fornada) carregou com a reserva intacta (42.372), e a fornada seguinte bateu com a da carreira contínua [observado, `resultados/todas10anos_carga.log`].
- **Faltando** eram clubes com vaga que não receberam a cota inteira: 35 a 80 de 1.134 por ano. A DLL passou a anotar o motivo de cada um (linha `intake-why`) e, numa carreira curta com todas as ligas (fornada no dia 200 de 2001), eram 119 vagas, por quatro causas [medido, `resultados/porque_falta*.log`]:
  1. **Os times B** (o parceiro pela 0x540a50): a curva não manda regen para eles. Saíram da cota.
  2. **A criação da reserva perdia regens:** ~80 por fornada saíam da criação já com um clube (o jogo os colocou) e a vaga ficava sem ninguém. Agora a DLL tenta de novo, até 3 vezes por vaga. A linha `intake-diag` conta os casos.
  3. **O elenco somado ao do time B:** a rotina do jogo que conta o elenco (0x53c590) soma o time A com o B. Na Espanha, em Portugal e na Alemanha, A + B passa de 50, e o Real Madrid, o Porto e o Bayern pareciam cheios. Na fornada, o limite agora são as 50 vagas do próprio clube.
  4. **Os clubes bascos:** o Athletic Bilbao e o Eibar só contratam bascos (0x53d7f0, motivo 0x19 da 0x8ce990); a Real Sociedad não contrata espanhol que não seja basco (motivo 0x1a). Quando só eles ainda têm cota, o jovem espanhol ganha a segunda nacionalidade basca (a nação 0x9cf424) e vai para um deles. O Eibar precisou de mais um ajuste: o veto do regen basco da distribuição do jogo (0x7ad0e0) só o deixa ir para 5 clubes, e o Eibar não é um deles. Desde a 1.3, o veto da DLL (na fornada e na disputa `rule = new`) aceita também os clubes que o próprio jogo marca como só bascos (0x53d7f0: o Athletic e mais 21, o Eibar entre eles) e confere a primeira nacionalidade além da segunda.

  Com as quatro correções: os 1.134 clubes com a cota inteira, 0 faltando, 9 jovens feitos bascos. No fim da temporada (a fornada de verdade, até julho de 2002, `resultados/fim_temporada.log`): as 9 fornadas com 0 faltando; a do dia 169, 788 clubes, 9 feitos bascos. A história confere: o Athletic só escala bascos desde 1912; a Real Sociedad largou a regra em 1989 (Aldridge) mas só contratou espanhol não basco em 2001 (Boris); do Eibar não há fonte firme, e a DLL segue o que o jogo diz dele. Com o repositório reorganizado (07/11), a mesma prova até julho de 2002: 0 faltando nas 9 fornadas, 9 feitos bascos, sem queda nem caixa de erro [observado, `resultados/final13.log`; na prova dos bascos, `resultados/bascos.log`, o C.F. Arenas, um dos clubes só bascos fora dos 5 do jogo, recebeu regen]; e o save de 2011 desta carreira, carregado com `intake = 0`, seguiu sem erro, com a fornada desligada [observado, `resultados/carga_sem_fornada.log`].
- **O clube depósito** (o Guiseley com 41 regens em 2003) vinha da criação dos regens da fornada: a lista da nação (0x7ad590) mandava 1 em 10 para clube de reputação ≤ 2. Desde 06/11 a fornada segura esse caminho, e o maior acumulado em 2003 é o Prato com 12: ver `TESTES.md` §4.5.

**Todas as ligas, 10 anos, com as correções (07/11).** A mesma carreira (1.134 clubes, 2001 a 2011-199), com a cota inteira, a regra basca e a lista da nação segurada; sem quedas e sem caixas de erro, 36.580 s [observado, `resultados/final10anos.log`]. A fornada do dia 169:

| Ano | Nascidos da fila | Criados | Entregues | Faltando | Sem vaga | Sobras assinadas | Reserva restante | Feitos bascos |
|---|---|---|---|---|---|---|---|---|
| 2002 | 1.202 | 1.077 | 2.205 | 0 | 28 | 74 | 47.937 | 9 |
| 2004 | 1.473 | 970 | 2.177 | 2 | 19 | 320 | 45.724 | 5 |
| 2006 | 2.394 | 400 | 2.328 | 2 | 1 | 643 | 44.220 | 6 |
| 2008 | 3.223 | 247 | 2.324 | 0 | 2 | 1.229 | 43.389 | 3 |
| 2011 | 3.042 | 137 | 2.311 | 0 | 1 | 929 | 42.691 | 4 |

- **Faltando:** das 112 fornadas, 95 com 0; as outras com 1 a 6 vagas, quase todas de clubes alemães (o Werder Bremen em 5 anos; o Dortmund e o Friburgo com 2 em 2007), contra 35 a 80 por ano antes. O motivo anotado é a recusa 33 (0x21) da 0x8ce990 em 1 a 6 jovens, com uns 106 aceitáveis no dia. **Resolvido (07/11):** a recusa 33 é o contrato pendente (0x4cca30 no gerente de contratos [0xae1050], registro do tipo 2): o jovem livre já acertado com um clube, que ainda não assinou, e nenhum outro pode contratá-lo. A fornada passou a deixá-lo fora do grupo dos regens livres; a mesma carreira até 2007 deu 0 faltando nas 60 fornadas [observado, `resultados/pendente.log`].
- Elenco médio de 33,5 (2002) a 36,2, estável de 2005 em diante; máximo 50 a 52. Regens com PA 150 ou mais no fim de cada temporada: de 27 (2002) a 1.012 (2011).
- **Sem clube depósito:** o maior acumulado foi de 5 (2002) a 42 (o Stabæk em 2011: 19 pela cota e 16 pelas sobras, em 10 anos); os clubes de reputação negativa com 0 a 29 regens, nenhum pela cota. Em 2009 e 2010 o maior foi o Atlético de Madrid B (40 e 43, nenhum pela fornada nem pela disputa: contratados depois pelo próprio jogo).
- **As paradas longas são do jogo:** a fornada mais lenta levou 14,7 s (a do dia 169, com uns 780 clubes; as outras, de 0,02 s a 4,5 s, campo `ms` da linha `intake`). As paradas de mais de 60 s caíram em 2001-194, antes da primeira fornada, na lista de observação dos treinadores (0x83fa5d e 0x83fb57, `shortlist_manager`), e no fim da temporada de 2006 e 2010 (dia 169), nas estatísticas dos jogadores (0x7c6d40, `player_stats`) e em 0x9453d7, rotinas do próprio jogo que rodam no mesmo dia.

### 8.1 As chaves

Desde o desenho final (05/11, noite). As chaves do estoque (`intake_stock`, `intake_pa_min`, `intake_pa_max`, `intake_pa_spread`, `intake_surprise`, `intake_squad_max`) e o `intake_only` saíram; um cfg da 1.2 com `intake_stock` > 0 e sem a linha `intake` liga a fornada.

| Chave | Padrão | O que faz |
|---|---|---|
| `intake` | 0 (desligada) | 1: a fornada ligada (só numa carreira nova) |
| `intake_per_club` | 3 | regens por clube em cada fornada, limitados às vagas do elenco (50) |
| `intake_day` | `season_end` | cada nação no fim da própria temporada (as 35 da tabela de nações-chave 0xb63d60; as outras no dia 181), ou um dia do ano para todos (0 a 365; 181 = 30/06 ou 01/07) |
| `intake_clubs` | `playable` | `playable`: clubes das ligas jogáveis; `selected`: também as ligas de fundo; as nações de fora seguem o caminho original |
| `intake_news` | 1 | a mensagem na caixa de entrada do clube de técnico humano, ou o aviso de falta de vaga |
| `intake_news_numbers` | 0 | 1: a mensagem mostra também o CA e o PA |
| `retire_day` | `original` | o dia do ano (0 a 365) de todas as aposentadorias de jogadores, ou `original` (as datas do jogo) |

Com a fornada ligada, o `season_cap` e o `squad_limit` da aba Regens não valem: a cota é o limite.

### 8.2 Como funciona

1. **Criar:** um desvio no `ftol` do corte do banco (0x525f8c) pede K pessoas a mais ao próprio jogo, dentro da folga (pessoas, fichas e preferências), e corrige o teto com defeito (0x525f9b).
2. **Adormecer:** antes de `game_run_to_start_date` (chamada em 0x829fd9), as K pessoas ficam aposentadas (bits 0x30 = 0x20 do registro de 79 bytes), sem clube e fora da fila dos regens.
3. **Soltar a garantia** (desde 05/11 é só o complemento, depois dos regens do jogo): no `intake_day`, depois da fila do dia (chamada em 0x5cda7a), cada clube abaixo do mínimo recebe adormecidas até `intake_per_club` (clube com `intake_squad_max` no elenco fica de fora):
   - a ficha vira a cópia de um jogador real da nação do clube, com o PA da fornada;
   - ela vai para a frente da reserva de fichas, e `regenerate_as_youth_player` 0x7abac0 faz o regen;
   - a disputa dos clubes (0x7acea2) é forçada para o clube da fornada; se ele ainda ficar sem clube, entra a assinatura 0x7a6f10.
4. **Gravar:** nada a fazer. A adormecida é uma aposentada que não está na fila, um estado que o save já guarda.
5. **Só a fornada** (`intake_only = 1`, 05/11; desde a tarde de 05/11, a fila guardada nasce pela disputa no dia da fornada, em vez de virar estoque): a `process_regen_queue` 0x7a65d0 tem duas metades: o laço dos nascimentos (0x7a65f0..0x7a66b7, a fila em +0x10/+0x14 e a reserva de fichas em +0x3c/+0x40) e a reposição dos não jogadores (+0x1c/+0x20, de 0x7a66bd em diante), que não toca na fila. A DLL chama a rotina com a contagem da fila zerada (e a devolve depois): o laço não anda e a outra metade segue. Na fornada, os jogadores aposentados que esperam na fila, com a ficha ainda na reserva, entram como estoque, antes dos criados; cada um sai da fila (como a 0x7a65d0 tira a cabeça: as de trás sobem, a última vaga fica −1) e a ficha dele sai da reserva, para nenhum outro regen herdar a mesma ficha depois. Assim a fornada se renova com as aposentadorias. Os não jogadores da fila ficam aposentados nela. Com isso, a disputa dos clubes (`rule = new`, `season_cap`...) e o `regen_origins` não entram mais: a fornada já escolhe o clube, e o `regen_origins` fica na chamada de dentro da fila (0x7a664f). As idades (`ages`) e os nomes (`regen_names`) continuam valendo, porque ficam dentro da 0x7abac0.
6. **A mensagem** (`intake_news = 1`, 05/11): o clube de técnico humano recebe uma notícia do próprio jogo com os jovens da fornada e o parecer do treinador da base sobre cada um. O caminho é o mesmo que a NickDLL do CLZ usa nas mensagens dela: montar um `NewsItem` (0x77bbe0), entregar pelo `News` (`send_to_club_manager` 0x7861e0, que só entrega a técnico humano) e escrever o texto na hora de mostrar. Ela usa o tipo 6009 (as finanças da diretoria) com um código que o jogo não conhece, e sem a DLL o escritor original mostraria a memória da pilha sem inicializar. Aqui o tipo é o 0x1388, a mensagem entre técnicos do jogo em rede: o texto dele fica nas próprias vagas da notícia (a partir da vaga 3), então **sem a DLL a notícia aparece com uma linha curta** guardada na língua do dia ("Fornada: 3 jovens chegaram da base."). Com a DLL, `NewsItem::write_text` 0x77bc80 é desviado na entrada e, para a notícia com as marcas (vaga 2 "FORN", vaga 20 "INTK"), escreve o título e o corpo na língua do jogo na hora (`langlib`, [0x9eb2dc], códigos em 0x9eb244: POR e POR_BRA → português, ESP e ESP_MEX → espanhol, o resto inglês), com os nomes clicáveis (link 0x7d8, o de pessoa). A mensagem entre técnicos vem com o botão "Responder"; `NewsItem::get_action_button` 0x782b70 também é desviado na entrada e devolve "sem botão" para a nossa notícia. Visto no jogo (05/11): aparece na aba Mensagens, sem botão, e ao trocar a língua nas opções a mesma notícia passa a sair em inglês. As vagas 22.. guardam, para até 14 jovens, o id e o parecer (CA, PA e as duas faixas), calculados no dia: o save leva tudo no news.dat.
   - **Nível de agora:** o CA comparado com a média dos 11 melhores do elenco: até 5 abaixo, "já está pronto para o time principal"; até 20, "pode ganhar chances em breve"; até 40, "precisa de um ou dois anos na base"; abaixo, "ainda está muito cru".
   - **Potencial:** pelo PA: 170+, "pode virar um dos melhores do mundo"; 150+, "tem potencial para a seleção"; 130+, "pode ser titular de um clube grande"; 100+, "deve fazer carreira na primeira divisão"; abaixo, "dificilmente passará das divisões de baixo".

### 8.3 Os resultados (carreira automática de férias, 278 clubes, 2001 → 2006-200)

Estoque de 3.000, 3 por clube, as duas carreiras em paralelo, sob a mesma carga [observado]:

| | Controle (sem fornada) | Fornada |
|---|---|---|
| Tempo até 2006-200 | 3.523 s | 3.423 s |
| Jogadores (temporada 2002 → 2006) | 19.346 → 19.293 | 22.255 → 22.191 |
| Elenco médio (2003 / 2005 / 2006) | 33,9 / 33,3 / 31,8 | 38,4 / 40,4 / 37,5 |
| Regens acumulados até 2006 | 3.554 | 6.730 |
| Regens com PA ≥ 150 (2006) | 197 | 318 |
| Quedas, caixas de erro | nenhuma | nenhuma |

- **A fornada de cada ano:** ~740 regens nos 278 clubes (de 40 a 50 clubes cheios, de 8 a 13 regens que ficaram sem o clube). O estoque de 3.000 acabou na 4ª fornada (2005), como previsto.
- **As adormecidas ficaram inertes:** um ano antes da 1ª fornada e quatro antes da última. Nenhuma entrou na fila nem num clube antes da hora.
- **Os regens da fornada:**
  - 15 a 20 anos (moda 17);
  - PA mediano 102, CA mediano 54;
  - posições: M 37%, D 31%, A 23%, G 8%.
- **Sem custo de tempo mensurável:** 3.000 pessoas a mais não mudaram o tempo da carreira. Uma 1ª comparação mostrou a fornada 75% mais lenta, mas o controle tinha rodado depois, com a máquina livre; em paralelo, o tempo é o mesmo.
- **Estoque de 20.000** (duas carreiras até 2006, 05/11):
  - criado inteiro (folga de 120.400 pessoas) e adormecido;
  - 5 fornadas de ~750, sem quedas, e sobraram 16.200;
  - 39.300 jogadores contra 19.300 do controle.
- **O tempo na bancada é ruído:** com as levas das outras sessões rodando ao lado, a mesma configuração variou de 2.692 a 3.523 s (controle) e de 3.043 a 4.484 s (20.000). Não há custo que se destaque desse ruído.
- **Tirar a ficha da adormecida até a soltura** (`+0x61 = 0`, experimento para tirá-la das varreduras de jogadores livres) não ganhou tempo e foi descartado: exigiria refazer o par pessoa ↔ ficha a cada carga.
- **Corrigido:** na 1ª versão o espalhamento passava do `intake_pa_max` (chegava a 185 com o máximo em 170); agora o máximo é o teto.
- **`intake_squad_max = 40`** (05/11): elenco médio ~37 em vez de 40; de 140 a 160 clubes pulados por ano, ~400 regens dados por fornada. A chave faz o que promete.
- **`intake_clubs = selected` com liga de fundo** (ARG, até 2004): a fornada chegou aos 323 clubes (os 278 das jogáveis e os 45 da Argentina), ~850 por ano, sem quedas; o estoque de 3.000 acabou na 3ª fornada. Os clubes de fundo recebem como os outros.
- **Carreira longa com 20.000** (2001 → 2011-201, 8.489 s, 05/11): sem quedas, 10 fornadas de 730 a 800, sobraram 12.400. Com `intake_squad_max = 50` o elenco médio sobe todo ano (41,2 em 2006, 44,7 em 2011), e os regens com PA ≥ 150 vão de 346 a 1.008: por isso o padrão sugerido é 40. As adormecidas contam como livres na estatística (14.000 a 17.000 "livres").
- **`intake_day = season_end`** (05/11, até 2003): o Brasil recebe no dia 10 (112 clubes, ~335 regens) e a Inglaterra e a Itália no dia 169 (166 clubes, ~330 a 430), cada uma no fim da própria temporada. A 1ª versão dava uma fornada a mais no 1º dia da carreira: na preparação, o laço passa por todas as nações; a DLL agora faz a mesma conferência de data do começo da 0x59cdd0 (0x456770) e ignora essas passagens.
- **`intake_only = 1`** (05/11, até 2004-216, 2.322 s, sem quedas): nenhum nascimento nem disputa ao longo das temporadas (`births 0`, `contest 0` em todos os registros); três fornadas de ~750 (278 clubes), 767, 677 e 516 delas vindas da fila de aposentados, e o estoque criado caiu só de 3.000 para ~2.650. Jogadores de 22.287 para 22.105 em três anos (−0,8%); elenco médio 35,3 → 37,8; 16 a 21 jovens por fornada ficam sem o clube (saem como livres). O "stock left" do registro conta junto os aposentados da fila que sobraram (por isso passa de 3.000 na 1ª).
- **A fornada com a curva** (05/11, carreira curta com o `tweaks.cfg` do repositório: `rule = new`, `ages = custom`, `season_cap = 2`, `surprise = 15`, `regen_origins = 1`; aposentadorias no dia 240 e fornada no 250 de 2001): no dia da fornada nasceram 508 regens do jogo pela disputa (198 sem clube, o caminho normal para os de países fora da carreira), todos os 508 no `regen_origins.csv`; o estoque completou 640 (49 surpresas); 237 dos 278 clubes ficaram com 3 ou mais (31 de elenco cheio); a mensagem chegou. Sem quedas. 
- **As duas carreiras de 10 anos com a curva** (05/11, 2001 → 2011-185, `tweaks.cfg` do repositório com estoque de 20.000, fornada no dia 181 e aposentadorias no 174; sem quedas, save gravado no dia da última fornada e carregado de novo): A = a garantia só com o estoque (7.019 s); B = a garantia primeiro com os regens do jogo sem clube (até 21 anos, da nação do clube), depois o estoque (5.737 s).

| Em 2011 | A | B |
|---|---|---|
| estoque gasto em 10 fornadas | 6.490 | 2.946 |
| regens sem clube que completaram clubes (por fornada) | — | 186 a 464 |
| clubes com 3 ou mais na fornada (de 278) | 264 | 278 |
| elenco médio | 40,3 (subindo) | 37,2 (estável de 36,4 a 38,0 desde 2004) |
| regens livres (sem clube) no mundo | 1.168 | 463 |
| regens com PA ≥ 150 acumulados | 1.373 | 1.081 |
| o maior acumulado de regens num elenco ("clube depósito") | 44 (Bangu) | 39 (Corinthians de Caicó) |

O B fica: gasta menos da metade do estoque, completa todos os clubes, segura o elenco e deixa menos regens perdidos como livres. A pendência dos dois é o clube depósito, que cresce ano a ano (de 5 a 39 no B), concentrado em clubes pequenos do Brasil (resolvido em 06/11: a lista da nação, ver `TESTES.md` §4.5).

### 8.4 A fazer

- ~~**Mensagem na caixa de entrada**~~: feita e vista no jogo (05/11), passo 6 acima. O pedido: no dia da fornada, cada clube de técnico humano recebe uma notícia com os jovens que chegaram (nome, idade, posição) o nível de agora (CA) e o potencial (PA), os dois **em faixas**, como o parecer de um treinador da base ("já pode treinar com o time principal", "pode virar um jogador de nível internacional"); a faixa do CA comparada com o elenco do próprio clube (o mesmo jovem pode estar pronto num clube pequeno e longe disso num grande); os números exatos só com uma chave. Pelo sistema de notícias do próprio exe (`g_news` 0xb7431c, o texto em `NewsItem::write_text` 0x77bc80), sem a NickDLL. O texto vem da DLL: inglês, português e espanhol embutidos, pela língua do jogo (sem tradução, inglês), e um arquivo de frases ao lado da DLL para outras línguas.
- Medir o tempo com a máquina livre (a carreira longa chegou a 2011 sem quedas).
- Gravar e carregar com o Lucas: o save tem de abrir com e sem a DLL; conferir nos editores.
- ~~A data da fornada por nação~~: feita (05/11), `intake_day = season_end`. O fim da temporada de cada nação-chave passa pela `end_season_for_nation` 0x59cdd0 (chamada em 0x5d5ea2, no laço das 35 nações-chave); a DLL anota a nação e a solta na fila do dia.
- ~~A aba do configurador~~: feita (05/11), aba **Fornada (experimental)** em Ajustes da carreira, com as 8 chaves e a ajuda nas duas línguas.
- Posições: os regens saem com as posições do jogador real copiado (G 8%, D 31%, M 37%, A 23%), o retrato da nação; sem ajuste por ora.
- ~~Elencos cheios~~: o `intake_squad_max` saiu no desenho final; o limite é a cota até 50 vagas (elenco médio de 35 a 38 na carreira de 05/11, noite).
