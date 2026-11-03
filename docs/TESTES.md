# Testes e resultados

*English version: [TESTES.en.md](TESTES.en.md)*

Tudo o que foi rodado para provar o CM0102-Rust-Tweaks, de 01/11 a 02/11/2026 (as carreiras longas de cinco temporadas em 02/11), com os números de cada execução. É a base para a postagem no fórum. **[medido]** = saiu do `tweaks.log` da execução; **[inferência]** = conclusão a partir dos números, a confirmar.

## 1. Resumo

| Frente | Situação |
|---|---|
| A DLL abre, liga as partes e respeita o exe | ✅ no 3.9.68 original, no exe com o Regen Tweak e no CLZ (NickPatcher + NickDLL) |
| Save acima de 2 GB | ✅ ligado carrega; desligado para no erro do jogo, nos dois exes |
| CLZ com `override_exe_changes = 1` | ✅ estável: seis execuções seguidas, duas delas de quase duas temporadas |
| O retrato do exe e o aviso de dependência | ✅ reconhece o original, o Regen Tweak e o CLZ; acha os desvios da NickDLL e da LaMOHatesCD em tempo de execução |
| As caixas de mensagem no log | ✅ três erros reais apanhados |
| A distribuição dos regens | ✅ em cinco temporadas, com 10 ligas (6 jogáveis e 4 de fundo), no banco de fábrica e no CLZ (§4.3) |
| Estabilidade nas carreiras longas | ✅ cinco carreiras de cinco temporadas, sem nenhuma queda, travamento ou caixa de erro |

## 2. O método

- **A carreira automática de férias** que o próprio jogo tem (`auto_setup.cfg` com `ON_HOLIDAY=YES`): o jogo cria a carreira e o técnico humano sai de férias, sem clique nenhum. Banco `MINIMUM`, ligas jogáveis Inglaterra, Brasil e Itália, motor de partidas desligado, lances da IA ligados.
- O jogo roda **oculto**, em janela de 960×540, com a nossa `ddraw.dll` de trabalho encadeada atrás (`ddraw_chain.dll`) só pelo vídeo, sem nenhuma rotina dela trocada.
- **`log = 1`** no `tweaks.cfg`: uma linha por evento (`regen`, `birth`, `idle`, `tick` a cada mês, `crash`, `hang`, `msgbox`, `exe`, `runtime`).
- **Um jogo por pasta**, com a vigia de memória do laboratório (não roda com menos de 4 GB livres). O jogo só aceita uma cópia aberta (o mutex "Championship Manager 2001/02 Process"); nas carreiras longas, cada pasta roda com `TWEAKS_MUTEX` = um caractere próprio, uma variável de ambiente só dos testes que troca a última letra do nome do mutex, e até três carreiras rodaram ao mesmo tempo. Sem a variável, nada muda.
- **A linha `season`** (desde 02/11): em julho de cada ano, o retrato da população, só lendo a memória: jogadores e quantos estão sem clube, os elencos (mínimo, média e máximo nos clubes das competições que rodam), os regens (quantos, quantos sem clube, o máximo num clube só, e qual), os de PA ≥ 150 (quantos, sem clube, em clube grande e a reputação média do clube). `tools/temporadas.py` monta a tabela por temporada.
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
| `Cup..cpp 1187` | numa carreira de 10 temporadas com os ajustes, em 2001-331 (banco de fábrica): a `Cup::process_fixture` (0x51a150) não achou o mandante do jogo na chave da copa (`find_tree_slot_for_fixture` devolveu -1). Código em que a DLL não mexe; a mesma configuração, repetida, passou e chegou a 2011 |
| `match_eng..cpp 612` e `652` | no banco oficial de outubro de 2026: "Only %d home players" (e o visitante), um time sem jogadores suficientes para a partida. 2 de 8 carreiras curtas até janeiro de 2002; numa delas, em 2001-248, os ajustes ainda não tinham feito nada no jogo (nenhuma aposentadoria, nenhum jovem na disputa). O banco tem clube de competição que começa com 0 jogadores (a linha `season`): é do banco, não da DLL |

Desde 03/11, o registro também grava o detalhe que o jogo monta antes de algumas caixas (a linha `msgbox-detail`, o texto em 0xde1f64), como os times da partida no `match_eng`.

O desvio encadeia no da LaMOHatesCD no CLZ e não muda nada no jogo (as cargas e carreiras com ele ligado seguiram normais).

### 3.6 Sem SetupReg e sem administrador (CLZ v6.1)

Uma cópia do CLZ 2026 V6.1 (exe do V5.3, NickDLL nova do grupo, intocada), com o `Location` do registro apontando para outra instalação (a do V5.4) [medido, 02/11/2026]:
- `portable_folder = 1`: o log diz `portable folder: on (... Location is this folder: <a cópia>)`, e o jogo abre e grava na cópia;
- `clear_admin_flag = 1`: a NickDLL grava `WIN95 RUNASADMIN` para o exe da cópia e a DLL tira o `RUNASADMIN` na primeira `DirectDrawCreate`; a abertura seguinte não pede elevação. O `WIN95` fica: sem o modo Windows 95, a NickDLL do v6.1 cai ao criar a carreira (falha de heap no `ntdll`, logo depois de `bra_setup Brasileiro Sub-21 B`, quatro vezes, com e sem esta DLL); com `__COMPAT_LAYER=WIN95 HIGHDPIAWARE`, sem elevação, a carreira é criada e salva (sugestão da equipe do CLZ);
- em janela, com a `ddraw.dll` do Windows o CLZ para em "Unable to initialise the graphics in windowed mode ... 16 bit colour" (caixa no log): o teste usa a `ddraw.dll` de trabalho como `ddraw_chain.dll`, só pelo vídeo (`CM_SEM_DESVIOS=1`).

### 3.7 Vídeo, resolução e CD

Carreiras automáticas curtas na pasta do banco de outubro de 2026 (inglês), sem a `ddraw_chain.dll` (a camada de vídeo é da própria DLL) [medido, 02/11/2026]:

| Exe | Chaves | Resultado |
|---|---|---|
| `cm0102.exe` 3.9.68 intacto (pede o CD) | `no_cd = 1`, `resolution = 1080` | "no CD: on", "resolution 1080: on", "video: window"; quadro de 1920×1080; a carreira anda até 2001-244 sem caixa; o retrato da memória: "no other DLL changed the exe's code" |
| `cm0102-portatil.exe` | `video = window` | quadro de 800×600 na janela; até 2001-232 |
| `cm0102-1080.exe` (já tem o CD e a resolução) | `no_cd = 1`, `resolution = 1080` | "this exe already has it" nos dois; só a trava das 30 linhas entra; até 2001-245 |
| CLZ (só os bytes do exe, sem rodar) | — | CD: 38 de 66 trechos como no 3.9.68 e 28 já trocados (14 das 33 checagens); resolução: 196 como no 3.9.68 e 5 já trocados (o mesmo código do NickPatcher); a entrada do `Area::init` igual. Com a regra de fazer o possível, a DLL completa o que falta; o CD fica com a `LaMOHatesCD.dll`, e a resolução com a NickDLL do 6.1 quando ela tem uma resolução escolhida ([COMPATIBILIDADE.md](COMPATIBILIDADE.md)) |

Depois da regra de fazer o possível (a lista da resolução completa o que o exe já tem; o CD checagem por checagem) [medido, 02/11 à noite]:

| Exe | Resultado |
|---|---|
| `cm0102.exe` intacto | CD: 66 trechos trocados agora; resolução: 201; a trava das 30 linhas; o vídeo em janela; até 2001-216 |
| `cm0102-1080.exe` | CD: 0 agora, 66 já no exe; resolução: 0 agora, 201 já no exe; até 2001-215 |
| cópia "misturada", como o CLZ (14 checagens do CD e 5 trechos da resolução já trocados) | CD: 38 agora, 28 já no exe; resolução: 196 agora, 5 já no exe; o jogo em 1080; até 2001-215 |
| cópia "estranha" (um trecho da resolução com bytes de outro patch) | a resolução recusa ("1 of 201 places ... another patch there; nothing was changed"); o CD e a trava entram; o jogo segue em 800×600 até 2001-215 |

As cópias de teste são geradas por `resultados/misto.py` a partir do `cm0102.exe`, numa pasta de teste, e apagadas depois.

Falta o teste com a tela: a janela, o Print Screen e o relatório de uma partida antiga em 1080.

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

### 4.3 Carreiras longas: cinco temporadas, 10 ligas

Ligas jogáveis Inglaterra, Brasil, Itália, Espanha, Alemanha e França; de fundo Argentina, Portugal, Holanda e Escócia. Banco de fábrica de 2001 a 2006 e CLZ de 2025 a 2030. Os ajustes ligados com `unattached = only_if_no_room`; na fábrica com `protect_young_unemployed = 23` (de um `.cfg` anterior), no CLZ com o padrão de hoje, 0. **[medido]**

Na primeira temporada da fábrica, a carreira ligada teve bem menos regens que a original (318 contra 1.119): a proteção dos menores de 23 corta a reciclagem dos jovens sem clube em regens. Em cinco anos a diferença some (7.573 contra 7.476), e o padrão do `.cfg` passou a 0.

**Defeito achado e corrigido no caminho:** o teto de 2 jovens por clube a cada 365 dias contava só os regens que ainda estavam no elenco. O clube que emprestava, vendia ou mandava o jovem ao time B voltava a receber: na primeira carreira ligada, o Real Valladolid e o Real Madrid receberam 15 cada pela disputa em quatro anos, quase todos pela surpresa, porque na Espanha só a primeira divisão roda e a surpresa sorteia entre uns 20 clubes. Agora a DLL guarda cada jovem que entregou, com a data, e o teto conta por essa lista (ou pelo elenco, o que for maior: a lista não vai para o save). A "ligada v1" é a carreira com o defeito; a "v2", a corrigida.

**A disputa, somada nas cinco temporadas:**

| Carreira | Jovens na disputa | Clubes que receberam | Máx. num clube numa temporada | Quem mais recebeu (5 anos) | Para os grandes | PA ≥ 150 na disputa | Aposentadorias de inativos |
|---|---|---|---|---|---|---|---|
| original (fábrica) | 4.518 | 1.488 | 12 | Real Madrid, 26 | 15% | 93 | — |
| ligada v1 (teto vazando) | 6.909 | 2.162 | 8 | Real Valladolid, 20 | 8% | 165 | 775 |
| **ligada v2** | 6.955 | 2.188 | **2** | San Martín de Tucumán, 9 | 6% | 177 | 772 |
| CLZ, regras do Nick | 1.947 | 613 | 11 | River Plate, 23 | 14% | 14 | — |
| **CLZ, as nossas** (override 1) | 3.454 | 1.239 | **2** | ABC (RN), 9 | 5% | 37 | 9 |

(Um 3 numa temporada da v2 é de dois clubes diferentes com o mesmo nome, "US Montagnarde", de reputações 1000 e 540; cada um recebeu 2.)

**A população em julho do último ano** (a linha `season`):

| Carreira | Jogadores | Sem clube | Elenco médio (máx.) | Regens | Regens sem clube | Máx. de regens num clube | PA ≥ 150: total / sem clube / em clube grande |
|---|---|---|---|---|---|---|---|
| original, 2006 | 36.518 | 3.220 | 30,9 (50) | 7.476 | 899 | 47 | 230 / 12 / 42 |
| ligada v1, 2006 | 36.531 | 2.484 | 30,7 (48) | 7.490 | 372 | 18 | 223 / 1 / 66 |
| **ligada v2, 2006** | 36.574 | 2.509 | 30,6 (48) | 7.573 | 353 | 17 | 244 / 2 / 72 |
| CLZ do Nick, 2030 | 38.982 | 2.855 | 26,2 (46) | 5.258 | 233 | 27 | 165 / 2 / 18 |
| **CLZ, as nossas, 2030** | 38.989 | 2.687 | 26,0 (47) | 5.263 | 77 | 24 | 175 / 1 / 28 |

O que os números mostram:
- **O teto é exato.** Com a correção, nenhum clube recebe mais de 2 jovens por temporada pela disputa; em cinco anos, o que mais recebeu ficou com 9 (contra 26 do Real Madrid na original e 23 do River Plate na regra do Nick). Os grandes ficam com 5% a 6% (15% na original, 14% no Nick).
- **A população fica estável.** O número de jogadores e de regens é o mesmo nas duas regras, e o elenco médio também; os ajustes não fazem o mundo encolher nem inchar.
- **Menos jovens sem clube.** Com `unattached`, quase todo regen de nação no jogo vai à disputa: a disputa recebe 50% (fábrica) a 80% (CLZ) mais jovens, espalhados por 50% (fábrica) a 100% (CLZ) mais clubes, e os regens sem clube caem de 899 para 353 (fábrica) e de 233 para 77 (CLZ).
- **Os craques continuam chegando aos grandes**, agora pelo mercado: os regens de PA ≥ 150 em clubes grandes sobem de 42 para 72 (fábrica) e de 18 para 28 (CLZ), mesmo com os grandes recebendo menos na disputa.
- **O acúmulo que sobra é de nação fora do jogo.** Mesmo com o teto, um clube junta 17 regens em cinco anos na fábrica e 24 no CLZ (47 e 27 sem os ajustes). A segunda carreira do CLZ com as nossas regras, já com o nome do clube na linha `season`, mostrou quem é: o AS Douanes, de Dakar (Senegal, nação fora do jogo), com 8, 13, 16 e 17 regens de 2027 a 2030, e nenhum deles pela disputa (o clube não aparece em nenhuma linha `regen`). Nas nações fora do jogo, o regen segue o caminho original e não passa pelo teto; como esses clubes não jogam competição que roda, ninguém os contrata e os regens se juntam. Para quem joga, o efeito é invisível; um teto também ali é uma opção futura. Na primeira temporada o líder era o MC Algiers (Argélia, também fora do jogo).
- **A repetição confirma.** A segunda carreira do CLZ com as nossas regras deu o mesmo quadro da primeira: máximo de 2 por clube em cada temporada, grandes com 4% a 6%, 5.259 regens (69 sem clube) e 171 de PA ≥ 150 em 2030.
- **Inativos:** 772 aposentadorias em cinco anos na fábrica; 9 no CLZ, onde o RetirePatchV4 barra quase todas (o log avisa).
- **Estabilidade:** as cinco carreiras chegaram à data, de 1 h 23 min a 2 h 11 min cada, sem nenhuma queda, travamento ou caixa de erro.

### 4.4 Carreiras de 10 temporadas (2001 a 2011)

As mesmas 10 ligas, banco de fábrica, a original contra os ajustes como vão para o jogador (o `tweaks.cfg` de hoje, com `protect_young_unemployed = 0`, mais `unattached = only_if_no_room`). As duas chegaram a 2011-202, em 2 h 48 min e 2 h 42 min, sem queda, travamento ou caixa de erro. **[medido]**

**A disputa, somada em 10 temporadas:**

| Carreira | Jovens na disputa | Clubes que receberam | Máx. num clube numa temporada | Quem mais recebeu | Para os grandes | Sem clube | PA ≥ 150 na disputa | Aposentadorias de inativos |
|---|---|---|---|---|---|---|---|---|
| original | 12.257 | 1.894 | 14 | Real Madrid e Milan, 57 cada | 15% | 1 | 331 | — |
| ajustes | 18.932 | 2.745 | **2** | Burgos, 19 | **4%** | 84 | 587 | 2.278 |

**A população em julho, ano a ano (linha `season`):**

| Julho | Original: sem clube / elenco médio / regens sem clube / máx. num clube | Ajustes: sem clube / elenco médio / regens sem clube / máx. num clube |
|---|---|---|
| 2002 | 1.008 / 32,5 / — / 7 | 675 / 32,7 / — / 2 |
| 2004 | 1.954 / 32,9 / — / 15 | 1.512 / 32,9 / — / 8 |
| 2006 | 3.168 / 30,9 / 913 / 23 (Tigre) | 2.418 / 30,9 / 367 / 15 (U. Las Palmas) |
| 2008 | 3.544 / 29,9 / — / 34 (Defensores de Belgrano) | 2.278 / 28,9 / 607 / 23 (U. Las Palmas) |
| 2011 | 3.282 / 30,3 / 1.785 / 40 (Tigre) | 2.079 / **27,6** / 869 / 30 (U. Las Palmas) |

Em 2011: 18.875 regens na original e 18.982 com os ajustes; PA ≥ 150: 868 (42 sem clube, 245 em clube grande) contra 867 (9 sem clube, 238 em clube grande).

O que os números mostram:
- **O teto se mantém por 10 anos:** nenhum clube passa de 2 jovens por temporada pela disputa. Na original, o Real Madrid e o Milan recebem 57 cada, e um clube chega a 14 numa temporada.
- **Os grandes recebem 4% dos jovens da disputa (15% na original) e terminam com o mesmo número de craques** (238 contra 245 de PA ≥ 150 em clube grande): chegam a eles pelo mercado.
- **Muito menos talento perdido:** os regens sem clube caem pela metade (869 contra 1.785), e os de PA ≥ 150 sem clube de 42 para 9.
- **O acúmulo que sobra vem do mercado:** o Universidad de Las Palmas junta 30 regens, mas só 2 vieram pela disputa (o log mostra cada jovem da disputa e o clube); na original, o Tigre junta 40, quase todos pela disputa (30). Desde 03/11 a linha `season` separa os regens do clube líder que a disputa deu a ele (`given by the contest`) dos que passaram pela disputa rumo a outro clube e chegaram depois por transferência ou empréstimo (`via the contest to another club`); antes, o "from the contest" juntava os dois (na carreira com `outside_competitions = 0,25`, o Getafe tinha 15 "from the contest", mas a disputa só lhe deu 3).
- **Ponto a acertar: o elenco médio dos clubes das competições cai** de 32,9 (2004) para 27,6 (2011) com os ajustes; na original fica em torno de 30. O total de jogadores é igual nas duas; com os ajustes, menos jogadores livres (2.079 contra 3.282) e os jovens espalhados por 2.745 clubes, muitos fora das competições. As variantes do §4.5 descartam os inativos como causa; o teto de 3 ajuda pouco.
- **Sem clube na disputa (84):** a partir de 2007, 16 a 26 jovens por temporada não acham clube elegível no país (elencos no teto ou teto da temporada): é o `unattached` funcionando como pedido ("sem clube só sem vaga").

### 4.5 Por que o elenco cai, e o clube "depósito"

**As variantes (10 temporadas cada, o resto igual ao §4.4)** [medido]:

| Julho | Elenco médio: ajustes (§4.4) | sem os inativos | teto de 3 por temporada |
|---|---|---|---|
| 2004 | 32,9 | 32,8 | 32,9 |
| 2006 | 30,9 | 30,8 | 31,3 |
| 2008 | 28,9 | 28,9 | 29,7 |
| 2010 | 27,9 | 28,1 | 29,0 |
| 2011 | 27,6 | 27,8 | 28,7 |

- **Os inativos não são a causa:** sem eles, o elenco cai do mesmo jeito.
- **O teto de 3 segura cerca de um jogador por elenco** (28,7 contra 27,6 em 2011), com os grandes ainda em 4% a 10% da disputa. Não resolve sozinho. A causa mais provável é o espalhamento dos jovens por clubes fora das competições (2.745 clubes recebem na carreira de 10 anos). Decisão do Lucas (03/11): uma chave nova, `outside_competitions`, o peso dos clubes fora das competições (1 = como hoje; 0 = nunca); uma carreira de 10 temporadas com 0,25 está rodando.

**Um clube "depósito" de regens (em aberto):** em várias carreiras com os ajustes, um clube fora das competições junta muitos regens que não vieram da disputa (a linha `season` mostra quantos vieram dela): o Universidad de Las Palmas com 30 (§4.4) e, em outras carreiras, o Guiseley (inglês, fora das ligas) com 13 a 49. Nas carreiras da original, o clube com mais regens recebeu a maior parte pela disputa (o Tigre, 30 de 40).

Uma suspeita, descartada [medido]: a lista `PELA_DISPUTA` da DLL (um `Vec` que crescia a cada jovem na linha do jogo) pareceu a causa, porque o Guiseley apareceu nas três primeiras carreiras com ela e sumiu sem ela. Mas a carreira final, com um mapa de bits fixo no lugar da lista, também teve o Guiseley (13 em julho de 2002). Contagem: o depósito do Guiseley apareceu em 4 de 8 carreiras com a DLL nova (com e sem a lista) e em 0 de 2 com a anterior; é a trajetória da carreira, que muda com qualquer diferença (o jogo sorteia pelo relógio e lê memória não inicializada), e não um defeito da lista. O mapa de bits ficou: não alocar na linha do jogo é a prática certa.

O que falta saber: se esses regens chegam por contratação ou por empréstimo (no empréstimo, o +0x39 do jogador passa a ser o clube que recebe), e se o mesmo acontece na original sem ser notado (a disputa original já concentra tanto que o depósito não aparece no topo). Para isso, um save no meio de uma carreira com o depósito, lido com `tools/regens_por_clube.py`.

Uma leitura da memória do jogo em execução (de fora, só leitura; `resultados/deposito.py`), na carreira final do §4.5 em 2003: o Guiseley tinha 16 jogadores, **todos regens**, de 17 a 21 anos. O clube de fora das ligas trocou o elenco inteiro por jovens. A leitura do dono do contrato não deu valores coerentes (o leiaute do contrato ainda não está certo), então contratação contra empréstimo segue em aberto.

**E na original?** Uma temporada da original, lida da mesma forma em julho de 2002: o clube com mais regens era o SC Kleve (Alemanha), com 6 jogadores, **todos regens**, nenhum pela disputa. É o mesmo fenômeno: um clube pequeno, fora das competições, perde o elenco do banco e o refaz com jovens livres. Na original ele fica escondido atrás dos clubes que a disputa enche (o Tigre com 30 pela disputa); com os ajustes, a disputa tem teto e o depósito passa a ser o clube com mais regens. **É do jogo, não dos ajustes** [medido nesta carreira; uma só]. O Guiseley nem existe com esse nome nessa carreira da original: o conjunto de clubes muda de uma carreira para outra.

## 5. Limites e próximos testes

- **Cinco temporadas, uma carreira de cada configuração.** Falta repetir para ver a variação entre carreiras, e ir a 10 temporadas. As seleções ainda não entram na linha `season`.
- **Uma carreira de cada:** o jogo é aleatório; os números grandes (o teto, a concentração) vêm da regra, os pequenos (o PA máximo) variam de uma carreira para outra.
- **O acúmulo nas nações fora do jogo** (§4.3): decidir se vale um teto também ali.

## 6. Como reproduzir

- O `tweaks.cfg` com `log = 1`; o `tweaks.log` sai ao lado do exe.
- A carreira automática: `auto_setup.cfg` na pasta do jogo com `LANGUAGE=ENG_UK`, `DATABASE=MINIMUM`, `FOREGROUND_LEAGUES=ENG BRA ITA` (nas longas: `ENG BRA ITA ESP GER FRA` e `BACKGROUND_LEAGUES=ARG POR NED SCO`), `ENABLE_MATCH_ENGINE=NO`, `ENABLE_COMPUTER_BIDS=YES`, `ON_HOLIDAY=YES` (guarde o `game.cfg` antes e apague o `auto_setup.cfg` depois).
- Os logs brutos de cada execução (com o nome da tabela: `clzLonga2.log`, `oficialSemVaga.log`, `longaOriginal.log`, `longaLigadoV1.log`, `longaLigado2.log`, `longaClzNick.log`, `longaClzNosso.log`, `longaClzNosso2.log`, `dezOriginal.log`, `dezAjustes.log`, `dezSemInativos.log`, `dezTeto3.log`, `controleDllNova.log`, `bisect*.log`...), os `.cfg` usados e o executor dos testes (`prova_tweaks.py`) ficam em `resultados/`, fora do git.
- `tools/comparar_logs.py` compara dois logs; `tools/temporadas.py` monta a tabela por temporada das carreiras longas; `tools/ler_retrato.py` dá nome às linhas do retrato; `tools/regens_por_clube.py` lê os regens de um save.
