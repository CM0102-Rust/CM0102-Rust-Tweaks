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
