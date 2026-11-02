# Testes e resultados

*English version: [TESTES.en.md](TESTES.en.md)*

Tudo o que foi rodado para provar o CM0102-Rust-Tweaks, de 01/11 a 02/11/2026, com os números de cada execução. É a base para a postagem no fórum. **[medido]** = saiu do `tweaks.log` da execução; **[inferência]** = conclusão a partir dos números, a confirmar.

## 1. Resumo

| Frente | Situação |
|---|---|
| A DLL abre, liga as partes e respeita o exe | ✅ no 3.9.68 original, no exe com o Regen Tweak e no CLZ (NickPatcher + NickDLL) |
| Save acima de 2 GB | ✅ ligado carrega; desligado para no erro do jogo, nos dois exes |
| CLZ com `override_exe_changes = 1` | ✅ estável: seis execuções seguidas, duas delas de quase duas temporadas |
| O retrato do exe e o aviso de dependência | ✅ reconhece o original, o Regen Tweak e o CLZ; acha os desvios da NickDLL e da LaMOHatesCD em tempo de execução |
| As caixas de mensagem no log | ✅ três erros reais apanhados |
| A distribuição dos regens | ✅ numa temporada e meia; ⏳ falta o efeito ao longo de vários anos, com mais ligas |

## 2. O método

- **A carreira automática de férias** que o próprio jogo tem (`auto_setup.cfg` com `ON_HOLIDAY=YES`): o jogo cria a carreira e o técnico humano sai de férias, sem clique nenhum. Banco `MINIMUM`, ligas jogáveis Inglaterra, Brasil e Itália, motor de partidas desligado, lances da IA ligados.
- O jogo roda **oculto**, em janela de 960×540, com a nossa `ddraw.dll` de trabalho encadeada atrás (`ddraw_chain.dll`) só pelo vídeo, sem nenhuma rotina dela trocada.
- **`log = 1`** no `tweaks.cfg`: uma linha por evento (`regen`, `birth`, `idle`, `tick` a cada mês, `crash`, `hang`, `msgbox`, `exe`, `runtime`).
- **Um jogo de cada vez** (o jogo só aceita uma cópia aberta), com a vigia de memória do laboratório (não roda com menos de 4 GB livres).
- **Os exes:**
  - `cm0102.exe` 3.9.68 intacto (só para o retrato: ele pede o CD);
  - `cm0102-1080.exe`, o 3.9.68 com a resolução 1920×1080 do projeto CM0102: as carreiras "originais";
  - `cm0102-1080-regentweak.exe`: o mesmo com o Regen Tweak de xeno/Saturn aplicado;
  - o exe do **CLZ** (NickPatcher, NickDLL e LaMOHatesCD, banco de 2025), um nome de exe novo por execução (a marca `RUNASADMIN` que a NickDLL grava no registro).
- **Os bancos:** o de fábrica do 3.9.68; o oficial de outubro de 2026 da equipe de atualização (champman0102.co.uk), sem patch; o do CLZ.

## 3. Estabilidade

### 3.1 Exe já modificado: o Regen Tweak

O exe com o Regen Tweak muda o corpo da disputa (o elenco 45 em 0x7ad1bd). **[medido]**
- `override_exe_changes = 0`: a parte da disputa fica com a rotina do exe e o log diz "keeping the exe's own routine"; as outras ligam.
- `override_exe_changes = 1`: a disputa passa à regra nova ("overriding it").
- Nos dois casos o jogo segue normalmente.

### 3.2 CLZ

| Execução | Ajustes | Até | Resultado |
|---|---|---|---|
| clz0, clz0b, clz0c | override 0 | 2025-338 | ✅ sem falha (versões anteriores da DLL) |
| clz1 | override 1 | 2025-260 | ❌ violação de acesso; DLL antiga, sem o registro de quedas, e o Windows não guardou o local |
| clz1b | override 1 | 2026-001 | ✅ |
| clzA | override 1 | 2026-033 | ❌ data parada até o limite de 15 min; antes do vigia de travamento, sem pilha |
| clzB, clzC | override 1 | 2026-082, 2026-077 | ✅ |
| **clzLonga** | override 1 | **2027-185** | ✅ quase duas temporadas; 287 regens, 746 nascimentos |
| clzRef1, clzRef2 | nenhum ajuste (referência) | 2026-093 | ✅ |
| clzD, clzE, clzF | override 1 | 2026-092, 2026-092, 2026-077 | ✅ |
| **clzLonga2** | override 1, com o vigia, o registro de quedas e as caixas | **2027-187** | ✅ 302 regens, 750 nascimentos, nenhuma caixa de erro |

**Conclusão:** depois das duas falhas, seis execuções seguidas sem falha, duas de quase duas temporadas. As falhas foram em versões anteriores da DLL; se voltarem, o log de hoje diz onde. [medido / inferência]

**Inativos no CLZ:** zero aposentadorias pela regra nova em todas as execuções. Não é defeito: o NickPatcher do CLZ pula, na busca de aposentadorias (0x7a5790), quem tem menos de 34 anos (RetirePatchV4, salto em 0x7a5806), e quase ninguém chega à regra. O log avisa. [medido]

### 3.3 Save acima de 2 GB

Saves de teste com os itens além dos 2 GB: o do banco de fábrica com 2,55 GB (187 itens além de 2 GB) e o do CLZ (76 itens além). **[medido]**

| Exe | `save_over_2gb = 1` | `save_over_2gb = 0` |
|---|---|---|
| 3.9.68 (1080) | carrega: 187 itens, 66 MB descompactados | caixa do jogo: `friendly..cpp 4695` |
| CLZ | carrega: 76 itens, 77 MB | caixa do jogo: `friendly..cpp 4634` |

Uma lição de método: as primeiras cargas com a chave desligada pareciam passar, porque a data do save vem do começo do arquivo e a caixa de erro é desenhada pelo jogo, invisível para um teste oculto. Só o registro das caixas (§3.5) separou os dois casos.

### 3.4 O retrato do exe

| Exe | No disco | Na memória, depois das DLLs do exe |
|---|---|---|
| 3.9.68 intacto | "the same code and data as the original 3.9.68" | nada mudado |
| Regen Tweak | 83 de 11.314 funções e 3 de 373 blocos de dados; o Regen Tweak nomeado | nada mudado |
| CLZ | ponto de entrada na seção nova `.nick` (2 MB); 519 funções e 28 blocos; seis casos nomeados (Regen Tweak, RetirePatchV4, AlwaysRegenToNewGoodCountry, o ramo das nações fora do jogo pulado, as faixas raras de idade) | a NickDLL e a LaMOHatesCD mudam mais 1.205 trechos (10.090 bytes) |

Pelo catálogo do NickPatcher (`tools/retrato_exe.py`, fora do jogo), o CLZ tem 85 patches reconhecidos. Na memória, dois desvios caem em trechos que os ajustes conferem; os dois foram lidos e são inofensivos [medido]:
- **0x540a50 → NickDLL**: a rotina do time B com a lista de pares do banco do CLZ; a disputa a usa para deixar os times B de fora, como a do exe faria.
- **0x5e8290 → LaMOHatesCD**: um filtro da caixa de mensagem (o aviso do CD); os outros erros continuam aparecendo.

### 3.5 As caixas de mensagem do jogo

Com `log = 1`, toda caixa vai para o log antes de aparecer. Casos reais apanhados [medido]:

| Caixa | Causa |
|---|---|
| `Database..cpp 7784` | carreira automática em português no banco oficial de outubro de 2026: a troca de idioma falha (0x5339a0). Em inglês (`ENG_UK`) anda |
| `friendly..cpp 4695` / `4634` | save acima de 2 GB sem `save_over_2gb` |
| "Coloque o CD" | o `cm0102.exe` 3.9.68 intacto pede o CD |

O desvio encadeia no da LaMOHatesCD no CLZ e não muda nada no jogo (as cargas e carreiras com ele ligado seguiram normais).

### 3.6 Sem SetupReg e sem administrador (CLZ v6.1)

Uma cópia do CLZ 2026 V6.1 (exe do V5.3, NickDLL nova do grupo, intocada), com o `Location` do registro apontando para outra instalação (a do V5.4) [medido, 02/11/2026]:
- `portable_folder = 1`: o log diz `portable folder: on (... Location is this folder: <a cópia>)`, e o jogo abre e grava na cópia;
- `clear_admin_flag = 1`: a NickDLL grava `WIN95 RUNASADMIN` para o exe da cópia e a DLL tira na primeira `DirectDrawCreate` (`removed "WIN95 RUNASADMIN"`); a entrada fica vazia e a abertura seguinte não pede elevação;
- em janela, com a `ddraw.dll` do Windows o CLZ para em "Unable to initialise the graphics in windowed mode ... 16 bit colour" (caixa no log): o teste usa a `ddraw.dll` de trabalho como `ddraw_chain.dll`, só pelo vídeo (`CM_SEM_DESVIOS=1`).

## 4. Os regens

### 4.1 Fora do jogo: o simulador

200 temporadas com os clubes reais de um save, pela regra original e pela nova. Os números estão no [README](../README.pt.md#a-calibração-simulador-testetelassav-200-temporadas): os grandes deixam de recolher os jovens comuns (Inglaterra 8000+: de 1,58 para 0,26 por temporada) e ficam com a parte deles dos craques (PA ≥ 140: 0,10 nos dois).

### 4.2 No jogo

Uma carreira de cada, da data inicial até a data da tabela. "Grandes" = clubes de reputação 7000 ou mais. **[medido]**

| Carreira | Banco | Até | Regens na disputa | Clubes | Máx. num clube | 10 clubes que mais receberam | Para os grandes | Reputação média do clube: PA < 100 / 100–129 / 130+ |
|---|---|---|---|---|---|---|---|---|
| original | fábrica | 2003-015 | 679 | 345 | 13 | 11% | 16% | 2861 / 5626 / 6440 |
| ajustes ligados | fábrica | 2003-022 | 480 | 343 | 3 | 4% | 4% | 2772 / 4507 / 5188 |
| ajustes ligados (2ª) | fábrica | 2003-027 | 466 | 334 | 3 | 5% | 5% | 2595 / 4568 / 6091 |
| original | outubro de 2026 | 2002-180 | 293 | 198 | 8 | 18% | 9% | 2968 / 5413 / 5428 |
| ajustes ligados | outubro de 2026 | 2002-180 | 152 | 122 | 2 | 13% | 3% | 3313 / 4528 / 5608 |
| ligados + `unattached = only_if_no_room` | outubro de 2026 | 2002-180 | 266 | 192 | 2 | 8% | 2% | 2929 / 4093 / 5009 |
| CLZ, regra do exe (Regen Tweak) | CLZ | 2026-093 | 59 e 68 | 40 e 46 | 6 e 4 | 46% e 41% | 2% e 1% | 4139 e 4030 / 5262 e 4929 / nenhum |
| CLZ, override 1 | CLZ | 2027-187 | 302 | 228 | 3 | 9% | 4% | 3228 / 4535 / 7130 |

O que os números mostram:
- **A concentração some.** Na regra original, um clube chega a receber 13 regens em uma temporada e meia e os grandes ficam com 16%; com os ajustes, ninguém passa de 3 (o teto de 2 por 365 dias) e os grandes ficam com 4% a 5%.
- **O jovem vai para um clube do tamanho dele.** A reputação média do clube sobe com o PA nas duas regras; com os ajustes, os jovens comuns e os médios se espalham pelos clubes médios, e os de PA alto continuam indo para clubes fortes.
- **Os jovens de 19 e 20 anos.** Com as idades da tabela, parte dos jovens nasce com mais de 18 anos e, pela regra original de quem vai à disputa, começa sem clube: daí a queda de 293 para 152 na disputa no banco de 2026. Com `unattached = only_if_no_room`, todos os 266 nascidos das nações no jogo foram à disputa (os outros 71 são de nações fora do jogo), espalhados por 192 clubes, e nenhum ficou sem clube.
- **Os inativos** (`retire_idle_players`): 35 a 37 aposentadorias nas carreiras de fábrica até 2003 e 16 nas de 2026 até meados de 2002; cada ficha liberada vira um regen semanas depois.

## 5. Limites e próximos testes

- **Uma temporada e meia não mostra o equilíbrio.** Falta a carreira longa (5 a 10 temporadas) com saves no caminho: o tamanho dos elencos, os jogadores sem clube, a população, os regens de PA alto por ano e onde estão três anos depois, as seleções.
- **Uma carreira de cada:** o jogo é aleatório; os números grandes (o teto, a concentração) vêm da regra, os pequenos (o PA máximo) variam de uma carreira para outra.
- **Mais ligas:** os testes tiveram três ligas jogáveis e nenhuma de fundo; falta uma carreira com ligas de fundo (que para os regens contam como no jogo) e com mais países.
- **`unattached` no CLZ:** o retrato mostra que o CLZ não muda o trecho; falta rodar.
- **O tamanho dos elencos** não está no log; virá da leitura dos saves.

## 6. Como reproduzir

- O `tweaks.cfg` com `log = 1`; o `tweaks.log` sai ao lado do exe.
- A carreira automática: `auto_setup.cfg` na pasta do jogo com `LANGUAGE=ENG_UK`, `DATABASE=MINIMUM`, `FOREGROUND_LEAGUES=ENG BRA ITA`, `ENABLE_MATCH_ENGINE=NO`, `ENABLE_COMPUTER_BIDS=YES`, `ON_HOLIDAY=YES` (guarde o `game.cfg` antes e apague o `auto_setup.cfg` depois).
- Os logs brutos de cada execução (com o nome da tabela: `clzLonga2.log`, `oficialSemVaga.log`...), os `.cfg` usados e o executor dos testes (`prova_tweaks.py`) ficam em `resultados/`, fora do git.
- `tools/comparar_logs.py` compara dois logs; `tools/ler_retrato.py` dá nome às linhas do retrato; `tools/regens_por_clube.py` lê os regens de um save.
