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

O código está em `src/fornada.rs`. Saiu na versão 1.2 (05/11/2026, com o ok do Lucas), ainda experimental. Fica desligado sem as chaves; nada muda no jogo de quem não as usa.

**O desenho de agora (05/11, pedido do Lucas: "o que muda é só a data de chegada; a ideia é não perder nada").** Na 1.2, os jovens da fornada saíam todos do estoque, com a ficha trocada pela cópia de um jogador real, e não passavam pela disputa: a curva da aba Regens (nível, instalações, comissão, espaço, teto, surpresa, temperatura), o PA da ficha do aposentado e o `regen_origins` deixavam de valer. Agora, no dia da fornada:
1. **Os regens do jogo** (`intake_only = 1`): quem se aposentou desde a última fornada esperava na fila do jogo; a fila nasce inteira, cada jovem pela mesma chamada da fila (0x7a664f, a do `regen_origins`) e pela disputa, com toda a curva. O PA é o da ficha do aposentado, como sempre. Só a data muda.
2. **A garantia:** todo clube com espaço no elenco que ficou com menos de `intake_per_club` é completado primeiro com os regens do jogo que a disputa deixou sem clube (até 21 anos, da nação do clube, assinados pela 0x7a6f10; na carreira curta, 180 de 184), e só o que faltar com o estoque (a ficha copiada de um jogador real da nação do clube, PA pela reputação, a variação e a surpresa `intake_surprise`: de vez em quando o PA sorteado entre o do clube e o teto).
3. **A mensagem** lista os jovens das duas origens.
4. **As frases da mensagem** ficam no `intake_news.txt`, ao lado do exe (pedido do Lucas, 05/11): uma seção `[CÓDIGO]` para cada uma das 19 línguas do jogo, com os códigos do próprio jogo (a tabela em 0x9eb244; a língua atual em [0x9eb2dc]), e linhas `chave = frase` (título, abertura, o parágrafo de cada jovem com `{name}`, `{age}`, `{position}`, `{now}`, `{potential}`, as posições e as 9 faixas). Vêm traduzidas inglês, português (POR e POR_BRA, em português do Brasil), espanhol (ESP e ESP_MEX), francês, alemão, italiano, holandês, dinamarquês, norueguês e sueco; polonês, russo, japonês, coreano e chinês ficam em inglês para a comunidade traduzir. Sem o arquivo, ou sem a frase, valem as embutidas (inglês, português, espanhol).
5. **O dia das aposentadorias** (`retire_day`): a porta única das aposentadorias (`add_retirement` 0x7a8510) é desviada na entrada e grava a data da aposentadoria (+0x10 dia, +0x12 ano do registro de 26 bytes) no dia escolhido; o jogo aposenta quando hoje passa da data (0x7a5a57), então grava o dia anterior. Vale com ou sem a fornada. Com a fornada em 1º de julho e as aposentadorias em 24 de junho, quem se aposenta nasce uma semana depois.

No modo `season_end`, a fila nasce por país: as fichas da reserva das nações daquele dia (+4 da entrada de 16 bytes, o id da nação, como a 0x7a8ab0 grava) vão para a frente, e nascem só essas. Com `intake_only = 0`, nada disso: os regens do jogo nascem ao longo da temporada, e o estoque dá `intake_per_club` a cada clube por cima, como na 1.2.

### 8.1 As chaves

| Chave | Padrão | O que faz |
|---|---|---|
| `intake_stock` | 0 (desligado) | pessoas criadas com a carreira (o estoque) |
| `intake_per_club` | 3 | o mínimo garantido por clube em cada fornada: os regens da disputa contam, e o estoque completa o que faltar |
| `intake_day` | 181 | dia do ano da fornada (181 = 30/06 ou 01/07), ou `season_end`: cada nação no fim da própria temporada (as 35 da tabela de nações-chave 0xb63d60; as outras no dia 181) |
| `intake_clubs` | `playable` | `playable`: clubes das ligas jogáveis; `selected`: também as ligas de fundo |
| `intake_pa_min`, `intake_pa_max` | 60, 170 | o PA vai do mínimo (clube de reputação 0) ao máximo (reputação 10.000) |
| `intake_pa_spread` | 30 | sorteio em volta do PA do clube; o resultado fica entre o mínimo e o máximo |
| `intake_squad_max` | 50 | clube com esse tanto no elenco fica de fora da fornada daquele ano |
| `intake_news` | 1 | a mensagem na caixa de entrada do clube de técnico humano (ver o passo 6) |
| `intake_news_numbers` | 0 | 1: a mensagem mostra também o CA e o PA |
| `intake_only` | 1 | 1: os regens do jogo ficam guardados na fila e nascem todos no dia da fornada, pela disputa e pela curva; 0: nascem ao longo da temporada, e o estoque vem por cima |
| `intake_surprise` | 10 | % dos jovens do estoque com o PA sorteado entre o do clube e o teto (o craque do clube pequeno) |
| `retire_day` | `original` | o dia do ano (0 a 365) de todas as aposentadorias de jogadores, ou `original` (as datas do jogo) |

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
- **A fornada com a curva** (05/11, carreira curta com o `tweaks.cfg` do repositório: `rule = new`, `ages = custom`, `season_cap = 2`, `surprise = 15`, `regen_origins = 1`; aposentadorias no dia 240 e fornada no 250 de 2001): no dia da fornada nasceram 508 regens do jogo pela disputa (198 sem clube, o caminho normal para os de países fora da carreira), todos os 508 no `regen_origins.csv`; o estoque completou 640 (49 surpresas); 237 dos 278 clubes ficaram com 3 ou mais (31 de elenco cheio); a mensagem chegou. Sem quedas. A carreira de 10 anos está em [8.3, a preencher].

### 8.4 A fazer

- ~~**Mensagem na caixa de entrada**~~: feita e vista no jogo (05/11), passo 6 acima. O pedido: no dia da fornada, cada clube de técnico humano recebe uma notícia com os jovens que chegaram (nome, idade, posição) o nível de agora (CA) e o potencial (PA), os dois **em faixas**, como o parecer de um treinador da base ("já pode treinar com o time principal", "pode virar um jogador de nível internacional"); a faixa do CA comparada com o elenco do próprio clube (o mesmo jovem pode estar pronto num clube pequeno e longe disso num grande); os números exatos só com uma chave. Pelo sistema de notícias do próprio exe (`g_news` 0xb7431c, o texto em `NewsItem::write_text` 0x77bc80), sem a NickDLL. O texto vem da DLL: inglês, português e espanhol embutidos, pela língua do jogo (sem tradução, inglês), e um arquivo de frases ao lado da DLL para outras línguas.
- Medir o tempo com a máquina livre (a carreira longa chegou a 2011 sem quedas).
- Gravar e carregar com o Lucas: o save tem de abrir com e sem a DLL; conferir nos editores.
- ~~A data da fornada por nação~~: feita (05/11), `intake_day = season_end`. O fim da temporada de cada nação-chave passa pela `end_season_for_nation` 0x59cdd0 (chamada em 0x5d5ea2, no laço das 35 nações-chave); a DLL anota a nação e a solta na fila do dia.
- ~~A aba do configurador~~: feita (05/11), aba **Fornada (experimental)** em Ajustes da carreira, com as 8 chaves e a ajuda nas duas línguas.
- Posições: os regens saem com as posições do jogador real copiado (G 8%, D 31%, M 37%, A 23%), o retrato da nação; sem ajuste por ora.
- Elencos cheios: os clubes não dispensam na mesma proporção, e o elenco médio sobe de 33 para 40. O `intake_squad_max` resolve (40 → ~37 médio); falta escolher o padrão.
