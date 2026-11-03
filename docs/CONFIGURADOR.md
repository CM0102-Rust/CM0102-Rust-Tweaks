# O configurador: projeto da interface

*English version: [CONFIGURADOR.en.md](CONFIGURADOR.en.md)*

Projeto da Fase 3 do [ROADMAP](../ROADMAP.pt.md): o programa que edita o `tweaks.cfg` com barras, gráficos e o estado do exe. Isto é projeto e maquete, não o programa. A maquete interativa, que abre no navegador, está em [configurador_maquete.html](configurador_maquete.html). Escrito em 02/11/2026.

Fontes: `tweaks.cfg`, `src/lib.rs` (`Regras`, `PADRAO`, `ler_regras`, as `Parte` e `pode_ligar`), `src/retrato.rs`, `src/pasta.rs`, [TESTES.md](TESTES.md). As faixas abaixo são as do código; onde o código não limita, a faixa é **proposta** (ou "a definir") e está marcada assim.

## 1. Os princípios

- **Um `.exe` só** (`tweaks-config.exe`), separado da DLL, sem instalador e sem nada instalado antes. Abre ao lado do exe do jogo, ou pede a pasta.
- **O `tweaks.cfg` é a única fonte.** O configurador lê o arquivo, mostra e grava de volta trocando só o valor das linhas mudadas. Comentários, ordem, linhas em branco e chaves que ele não conhece ficam como estão (§5).
- **Barra e campo andam juntos.** Toda opção numérica tem uma barra de arrastar e um campo de digitação ligados: mexer num move o outro.
- **Ajuda ao lado.** A ajuda de cada opção é o comentário dela no próprio `.cfg`. Aparece no painel da direita ao passar o mouse ou ao focar o controle.
- **Ver antes de gravar.** Gráficos que mudam enquanto o usuário arrasta: as idades, a curva do nível, a chance de cada clube e a chance de aposentadoria.
- **O exe primeiro.** Para o exe escolhido, cada ajuste diz o que vai acontecer: liga, fica com a do exe ou não serve (§6).
- **Visual próprio.** Nada da interface do NickPatcher (formulário cinza com abas de caixas de seleção). Fundo escuro de gramado à noite, um acento verde, cartões por ajuste e gráficos grandes.

## 2. A janela

```
┌──────────────────────────────────────────────────────────────────────────────────────┐
│ CM0102 Rust Tweaks   Exe: [cm0102.exe ▾ CLZ (NickPatcher)]  Predefinição: [Equilibrado ▾]  [Gravar] │
├────────────┬─────────────────────────────────────────────────────┬───────────────────┤
│ Início     │                                                     │ Ajuda             │
│ Jogo       │   cartões da aba escolhida                          │ (o comentário do  │
│ Idades     │   (interruptores, barras+campo, gráficos)           │  .cfg da opção    │
│ Inativos   │                                                     │  sob o mouse)     │
│ Disputa    │                                                     │                   │
│ Registro   │                                                     │ Neste exe:        │
│ tweaks.cfg │                                                     │ ● liga            │
├────────────┴─────────────────────────────────────────────────────┴───────────────────┤
│ 3 mudanças não gravadas · tweaks.cfg em D:\Jogos\CM0102 · última leitura do tweaks.log: 02/11 21:40 │
└──────────────────────────────────────────────────────────────────────────────────────┘
```

- **Barra de cima:** o exe escolhido (lista com os `.exe` da pasta), a predefinição (§4) e Gravar. A predefinição mostra "Personalizado" quando os valores não batem com nenhuma.
- **Esquerda:** as abas, cada uma com um ponto colorido que resume o estado do ajuste naquele exe (verde liga, amarelo fica com o exe, vermelho não serve, cinza desligado).
- **Direita:** a ajuda e o estado do ajuste no exe (§6).
- **Rodapé:** mudanças pendentes, caminho do `.cfg`, e se há um `tweaks.log` recente (§6.3).

## 3. As abas e os controles

Colunas: **Controle**; **Aceita** (o que `ler_regras` aceita, com os limites do código); **Barra** (a faixa da barra); **Sem a linha** (o padrão da DLL, `PADRAO`); **No .cfg** (o valor do `tweaks.cfg` distribuído).

Regra das faixas: o **campo** aceita a faixa válida inteira. A **barra** cobre a parte útil e fica encostada no fim quando o valor passa dela. Nas chaves sem limite no código, a faixa válida é a proposta (decisão do Lucas, §9).

Os interruptores gravam sempre `1` ou `0`, porque a DLL só liga com `v == "1"` exato (`log` liga com qualquer número diferente de 0). Os seletores gravam a palavra exata que a DLL testa (`custom`, `new`, `table`, `only_if_no_room`); qualquer outra palavra vale como o original.

### 3.1 Início

Sem controles de valor. Mostra:
- o **retrato do exe**: nome, tamanho, se é o 3.9.68 intacto ou quantas funções e blocos mudaram, seções novas (a `.nick` do CLZ), os casos conhecidos pelo nome (a tabela `CONHECIDOS` de `retrato.rs`);
- a **tabela "o que cada ajuste vai fazer"** (§6), uma linha por ajuste, com o botão "ir para a aba";
- a predefinição em uso e as diferenças para ela.

### 3.2 Jogo (save, pasta, administrador, sobrescrever)

| Chave | Controle | Aceita | Barra | Sem a linha | No .cfg |
|---|---|---|---|---|---|
| `save_over_2gb` | interruptor | `1` liga, o resto desliga | — | 0 | 0 |
| `portable_folder` | interruptor | idem | — | 0 | 0 |
| `clear_admin_flag` | interruptor (com aviso "só serve com a NickDLL" quando não há `NickDLL.dll` na pasta) | idem | — | 0 | 0 |
| `override_exe_changes` | interruptor com confirmação ("substituir as mudanças do exe pelas regras desta DLL") | idem | — | 0 | 0 |
| `video` | seletor (original / janela) | `window` liga, o resto desliga | — | original | original |
| `resolution` | seletor (original / 1920×1080); com 1080, o `video` aparece ligado e travado | `1080` liga, o resto desliga | — | original | original |
| `no_cd` | interruptor | `1` liga, o resto desliga | — | 0 | 0 |

`video`, `resolution` e `no_cd` já existem na DLL (02/11; a maquete ainda os mostra como "em breve").

### 3.3 Idades

Um cartão "**Nação no jogo**" no topo (§7) e depois:

| Chave | Controle | Aceita | Barra | Sem a linha | No .cfg |
|---|---|---|---|---|---|
| `ages` | seletor de dois: "Tabela" (`custom`) / "Original" (`original`) | `custom` liga | — | original | custom |
| `age_14` … `age_35` | **gráfico de barras arrastáveis** (a chance) + campo da barra escolhida + campo do teto do CA | chance: número, negativo vira 0, sem máximo; teto: inteiro (o `.cfg` diz 1 a 200), sem teto = omitido | chance 0–40 (proposta) | a tabela original: 15 = 0,0025; 16 = 16,67; 17 e 18 = 29,11; 19 e 20 = 12,44; 21 a 26 = 0,04; o resto 0 | igual |
| `younger_from_age` | barra + campo | inteiro, sem limite no código | 14–35 (proposta) | 22 | 22 |
| `younger_by` | barra + campo ("0 = desligado") | inteiro ≥ 0 | 0–15 (proposta) | **0** | 6 |
| `ages_out_of_game` | seletor: "Sorteio original" / "A tabela também" | `table` | — | original | original |
| `ca_cap_skip` | barra + campo, em % | número, sem limite (é %) | 0–100 (proposta: 0–5 na barra) | 0,5 | 0,5 |
| `contest_max_age` | barra + campo; cinza quando `unattached = only_if_no_room` (ali não conta) | inteiro 0–126 | 14–25 (proposta) | 18 | 18 |
| `unattached` | seletor: "Original" / "Sem clube só sem vaga" | `only_if_no_room` | — | original | original |

**O gráfico das idades** (o centro da aba):
- 22 barras, de 14 a 35 anos. A altura é a chance relativa; o rótulo, a porcentagem normalizada (as chances não precisam somar 100). Escala de raiz quadrada, para que 0,04 apareça ao lado de 29,11.
- Arrastar a barra para cima ou para baixo muda a chance. Clicar escolhe a idade: abaixo do gráfico, barra + campo da chance e campo do teto do CA (vazio = sem teto).
- Abaixo de cada barra, o teto do CA (80, 100, 110...) num selo.
- **Sobreposição "depois de regens mais novos":** contornos que mostram a distribuição final quando `younger_by > 0` (quem sai com `younger_from_age` anos ou mais desce `younger_by` anos). Muda ao arrastar as duas barras.
- `ages = original` deixa o gráfico cinza com o aviso "a tabela não vale: o jogo usa as idades do 3.9.68".

### 3.4 Inativos

| Chave | Controle | Aceita | Barra | Sem a linha | No .cfg |
|---|---|---|---|---|---|
| `retire_idle_players` | interruptor | `1` liga | — | 0 | 1 |
| `idle_days` | barra + campo, em dias | inteiro ≥ 0 | 0–730 (proposta) | 180 | 180 |
| `idle_min_age` | barra + campo | inteiro, sem limite no código | 16–40 (proposta) | 28 | 28 |
| `idle_rep` | barra + campo | número ≥ 0 | 0–10000 (proposta) | 1500 | 1500 |
| `idle_rep_per_year` | barra + campo | número ≥ 0 | 0–2000 (proposta) | 250 | 250 |
| `protect_young_unemployed` | barra + campo ("0 = sem proteção") | inteiro ≥ 0 | 0–30 (proposta) | 0 | 0 |

**Gráfico:** chance de aposentar por verificação (o jogo verifica nos dias 0, 80, 173 e 250 do ano), por idade de 18 a 40, para uma reputação de exemplo (barra própria, 500 a 10000, começa em 3000):
- a partir de `idle_min_age`: `mín(1, (idle_rep + idle_rep_per_year × (idade − idle_min_age)) / reputação)`, depois de `idle_days` sem clube (linha cheia);
- abaixo de `idle_min_age`: a regra original, `mín(1, 750 / reputação)` depois de 365 dias (tracejada);
- abaixo de `protect_young_unemployed`: zero;
- desligado: só a tracejada, em todas as idades.

Com reputação 3000 e o `.cfg` distribuído, o gráfico tem de dar 50% aos 28, 75% aos 31 e 100% aos 34 (o exemplo do próprio `.cfg`). Uma nota lembra a outra regra do original (passou do limite de idade da posição), que não muda.

### 3.5 Disputa (regens)

| Chave | Controle | Aceita | Barra | Sem a linha | No .cfg |
|---|---|---|---|---|---|
| `rule` | seletor: "Sorteio proporcional" (`new`) / "Original" | `new` liga | — | original | new |
| `target_k` | barra + campo | número, sem limite | 0–100 (proposta) | 50 | 50 |
| `target_ca` | barra + campo ("0 = PA, 1 = CA") | número, sem limite (sentido 0–1) | 0–1 | 0 | 0 |
| `sigma` | barra + campo | número ≥ 1 | 100–10000 (proposta) | 2500 | 2500 |
| `sigma_below` | barra + campo ("0 = igual a sigma") | número ≥ 0 | 0–10000 (proposta) | 1200 | 1200 |
| `level_floor` | barra + campo | número, sem limite (o `.cfg` diz 0 a 1) | 0–0,05 na barra, 0–1 no campo (proposta) | 0,002 | 0,002 |
| `facilities_base` | barra + campo | número, sem limite | 0–2 (proposta) | 0,4 | 0,4 |
| `facilities_k` | barra + campo | número, sem limite | 0–0,2 (proposta) | 0,06 | 0,06 |
| `staff_min` | barra + campo | número, sem limite | 0–2 (proposta) | 0,8 | 0,8 |
| `staff_max` | barra + campo (aviso se menor que `staff_min`) | número, sem limite | 0–2 (proposta) | 1,2 | 1,2 |
| `squad_limit` | barra + campo | 1–50 (o código limita) | 20–50 | 48 | 48 |
| `squad_exp` | barra + campo | número, sem limite | 0–3 (proposta) | 0,5 | 0,5 |
| `season_cap` | barra + campo ("0 = sem teto") | número, sem limite | 0–10 (proposta) | 2 | 2 |
| `surprise` | barra + campo, em % | 0–100 (o código limita) | 0–50 | 15 | 15 |
| `temperature` | barra + campo | número, sem limite | 0,25–3 (proposta) | 1 | 1 |

Três desenhos, ao vivo:
1. **A curva do nível.** Eixo x: reputação do clube, 0 a 10000. Eixo y: peso do nível, 0 a 1. Uma linha vertical no alvo (`target_k × ((1 − target_ca) × PA + target_ca × CA)`), a curva assimétrica (`sigma_below` à esquerda, `sigma` à direita) e o piso (`level_floor`) como faixa clara no fundo. Os clubes do exemplo aparecem como pontos sobre a curva. O jovem de exemplo tem PA e CA em barras próprias (PA 150, CA 60).
2. **Instalações e comissão** em uma linha de texto que muda: "Training 1 = 0,46 · 10 = 1,00 · 20 = 1,60" e "pior treinador 0,80 · melhor 1,20".
3. **Chances por clube.** Barras horizontais com a chance de cada clube de exemplo receber o jovem, com a mesma conta de `peso()` em `lib.rs` (nível × instalações × comissão × vaga, elevado a `temperature`; fora quem tem elenco ≥ `squad_limit` ou já recebeu `season_cap`). A surpresa é misturada: `(1 − s) × normal + s × surpresa`, a surpresa sem o nível e só entre os clubes de competição. A regra original (o maior sorteio(nota)) não é desenhada: é outra conta, e o exemplo mostra só a nova.

Os clubes do exemplo são fictícios e marcados assim ("exemplo"): um grande, um médio, um pequeno e "120 clubes pequenos somados", para mostrar por que `sigma_below` e `level_floor` existem (as centenas de clubes pequenos, somadas, levam os jovens). Melhoria futura: carregar os clubes reais de um save, como o `tools/calibrar.py` (§9).

`rule = original` deixa os três desenhos cinza com o aviso "o jogo usa a disputa do 3.9.68".

### 3.6 Registro

| Chave | Controle | Aceita | Sem a linha | No .cfg |
|---|---|---|---|---|
| `log` | interruptor | qualquer número ≠ 0 liga; grava 1 ou 0 | 0 | 0 |

A aba mostra a ajuda das linhas do log (o próprio comentário do `.cfg`) e, se houver um `tweaks.log` na pasta, as linhas `exe`, `runtime` e de cada ajuste da última abertura (§6.3).

### 3.7 tweaks.cfg

O arquivo como vai ficar, com as linhas mudadas destacadas e, no topo, "N linhas mudadas, M acrescentadas, nenhuma apagada". Somente leitura. Botões: Gravar e Desfazer tudo (volta ao lido do disco).

## 4. As predefinições

As predefinições mexem nos grupos de jogo (idades, inativos, disputa) e, só a "Só o save", no `save_over_2gb`. Não mexem em `portable_folder`, `clear_admin_flag`, `override_exe_changes` e `log`, que dependem da instalação e não do gosto.

| Chave | Original | Só o save | Equilibrado | Equilibrado, todos com clube |
|---|---|---|---|---|
| `save_over_2gb` | (não mexe) | 1 | (não mexe) | (não mexe) |
| `ages` | original | original | custom | custom |
| tabela `age_NN` | (não mexe) | (não mexe) | a original | a original |
| `younger_from_age`, `younger_by` | (não mexe) | (não mexe) | 22, 6 | 22, 6 |
| `ages_out_of_game` | original | original | original | original |
| `ca_cap_skip`, `contest_max_age` | (não mexe) | (não mexe) | 0,5, 18 | 0,5, 18 |
| `unattached` | original | original | original | **only_if_no_room** |
| `retire_idle_players` | 0 | 0 | 1 | 1 |
| inativos (dias, idade, rep, por ano, proteção) | (não mexe) | (não mexe) | 180, 28, 1500, 250, 0 | idem |
| `rule` | original | original | new | new |
| disputa (`target_k` … `temperature`) | (não mexe) | (não mexe) | os da Fase 1 (o `.cfg` distribuído) | idem |

- **Original:** a DLL não muda nada no jogo; os números ficam como estão porque não valem com os interruptores desligados.
- **Só o save:** para quem vinha do `cm0102-save2gb`.
- **Equilibrado:** o `tweaks.cfg` distribuído, decidido e calibrado na Fase 1.
- **Equilibrado, todos com clube:** o das carreiras longas de [TESTES.md](TESTES.md) §4.3 (os regens sem clube caem de 899 para 353 na fábrica e de 233 para 77 no CLZ).

Ao escolher uma predefinição, a interface mostra a lista de mudanças antes de aplicar ("12 valores vão mudar") e permite desfazer.

## 5. Ler e gravar sem apagar os comentários

O `.cfg` é lido como lista de linhas, nunca reconstruído do zero. O algoritmo:

**Ler**
1. Ler o arquivo como bytes; guardar o fim de linha usado (`\r\n` ou `\n`) e se o arquivo termina com quebra.
2. Para cada linha `i`: `codigo = parte antes do primeiro '#'`. Se `codigo` tem `=`, `chave = trim(antes)`, `valor = trim(depois)`. Guardar `posicao[chave] = i`. **A última ocorrência vence**, como na DLL (o laço de `ler_regras` sobrescreve).
3. Interpretar cada valor exatamente como `ler_regras` (`1` exato nos interruptores, `custom`/`new`/`table`/`only_if_no_room` nos seletores, número com ponto). Valor que a DLL não entenderia (`sigma = abc`) aparece com aviso "a DLL ignora esta linha e usa o padrão".
4. A ajuda de cada chave: as linhas de comentário logo acima dela, até a primeira linha em branco ou de chave; se não houver nenhuma (como `idle_days`), a ajuda da chave anterior do mesmo bloco.

**Gravar**
1. Para cada chave mudada que já tem linha: trocar só o trecho do valor. Expressão: `^(\s*CHAVE\s*=\s*)([^#]*?)(\s*(#.*)?)$` → `$1 + novo + $3`. Espaços e comentário no fim da linha ficam.
2. Para cada chave mudada **sem** linha: acrescentar só se o novo valor for diferente do padrão da DLL sem a linha (o arquivo não cresce à toa). Inserir logo depois da linha da chave anterior na ordem do `.cfg` distribuído que exista no arquivo (as idades, em ordem de idade); se nenhuma existir, no fim, sob `# added by the configurator`.
3. Nunca apagar linha. Tirar uma idade = `age_NN = 0` (o que o próprio `.cfg` manda). Chaves desconhecidas e linhas comentadas ficam intactas.
4. Formato dos números: ponto decimal, sem zeros sobrando (até 4 casas; `0.0025` fica `0.0025`). Idade: `chance, teto` ou só `chance` sem teto.
5. Gravar em `tweaks.cfg.tmp`, copiar o anterior para `tweaks.cfg.bak` e renomear (troca atômica). Se o arquivo mudou no disco desde a leitura (data ou tamanho), perguntar antes.
6. Sem `tweaks.cfg` na pasta: criar a partir do `.cfg` distribuído, embutido no exe (`include_str!`), com os valores aplicados. Esse mesmo texto embutido dá a ajuda quando o arquivo do usuário não tem comentários.

Teste de aceitação: ler o `tweaks.cfg` distribuído e gravar sem mudar nada dá o mesmo arquivo, byte a byte. A maquete faz o mesmo processo em JavaScript na aba tweaks.cfg.

## 6. O estado do exe

### 6.1 O que o configurador confere

As mesmas conferências da DLL, lidas do arquivo do exe em vez da memória: para cada `Parte` de `lib.rs` (`SAVE`, `IDADE`, `IDADE_FORA`, `DISPUTA`, `INATIVOS`, `SEM_CLUBE`), o FNV-1a de 64 bits das **bordas** e do **corpo**, com os endereços (VA) convertidos em deslocamento no arquivo pelas seções do PE. Mais o retrato do disco (`retrato.rs`: funções e blocos contra `referencia.rs`, seções novas, `CONHECIDOS`).

Para não duplicar as tabelas, as `Parte` e a `referencia.rs` passam a um módulo comum, usado pela DLL e pelo configurador (decisão do Lucas, §9).

### 6.2 Os estados de cada ajuste

| Estado | Quando | Cor |
|---|---|---|
| **Desligado** | a chave do ajuste está desligada no `.cfg` | cinza |
| **Liga** | bordas e corpo batem | verde |
| **Liga, sobrescrevendo o exe** | corpo diferente e `override_exe_changes = 1` (com o nome da mudança conhecida) | azul |
| **Fica com o exe** | corpo diferente e `override_exe_changes = 0`; mostra o nome achado ("Regen Tweak by xeno/Saturn") e o botão "usar as regras desta DLL" (liga o `override_exe_changes`) | amarelo |
| **Não serve neste exe** | bordas diferentes: a DLL nunca liga | vermelho |

Notas que acompanham o estado (as mesmas do log):
- **Inativos** com o salto em 0x7a5806 (RetirePatchV4): "liga, mas quase não acha ninguém: este exe pula aposentadorias abaixo de 34".
- **Idades fora do jogo** no CLZ, sobrescrevendo: "o desvio fica, mas o exe pula o ramo; continua sem rodar".
- **`clear_admin_flag`**: "útil: há uma NickDLL nesta pasta" ou "sem NickDLL, não faz nada".
- **`portable_folder`**: mostra a pasta que o registro aponta hoje (`Location`) se for outra.
- **Futuros (`video`, `resolution`, `no_cd`)**: no CLZ, "não serve: o exe já tem resolução e NoCD próprios" (a Fase 4 manda recusar).

Exemplos, pelos números de [TESTES.md](TESTES.md) §3:

| Ajuste | 3.9.68 | 3.9.68 + Regen Tweak | CLZ (NickPatcher) |
|---|---|---|---|
| Save acima de 2 GB | liga | liga | liga |
| Idades | liga | liga | fica com o exe (faixas raras mudadas) |
| Idades fora do jogo | liga | liga | fica com o exe (ramo pulado) |
| Disputa | liga | fica com o exe (Regen Tweak) | fica com o exe (Regen Tweak) |
| Inativos | liga | liga | liga, com a nota do RetirePatchV4 |
| Sem clube só sem vaga | liga | liga | liga |

### 6.3 O que o disco não mostra

A NickDLL e a LaMOHatesCD mudam o exe **ao rodar** (1.205 trechos no CLZ). O configurador não roda o jogo. Ele lê o último `tweaks.log` da pasta, se existir, e mostra as linhas `runtime` (avisos e notas) e a linha de cada ajuste, com a data. Assim o painel diz o que a DLL de fato fez na última abertura, ao lado do que o configurador prevê. Se os dois discordam, o painel mostra a diferença.

## 7. "Nação no jogo"

Um cartão fixo no topo das abas Idades e Disputa, com um desenho de três fichas:

```
 Ao criar a carreira, você escolhe ligas:
 [ Jogável ]  byte 2  ─┐
 [ De fundo ]  byte 1  ─┴─▶  NAÇÃO NO JOGO   (para os regens, as duas contam igual)
 [ Não escolhida ] 0   ───▶  FORA DO JOGO
```

Texto: "Nação no jogo é aquela cuja liga você escolheu ao criar a carreira, jogável ou de fundo. Para os regens as duas contam igual: o jogo só olha se o byte `NationLeagueSelected` não é 0. Fora do jogo é a nação cuja liga não foi escolhida." E onde importa:
- **idades:** as nações fora do jogo com menos de 350 pessoas no banco têm o sorteio próprio (`ages_out_of_game`);
- **quem vai à disputa:** só os jovens de nação no jogo (`contest_max_age`, `unattached`);
- **disputa:** os jovens de nação fora do jogo seguem o caminho original, sem o teto por clube (o acúmulo de [TESTES.md](TESTES.md) §4.3, o AS Douanes).

## 8. A tecnologia

| Opção | Exe único sem dependências | Gráficos arrastáveis | Contra |
|---|---|---|---|
| **Rust + eframe/egui** | sim: CRT estática (`+crt-static`), uns 4 a 8 MB, nada instalado | fácil: desenho imediato, `Plot`/`Painter`, arrastar é um `Response` | visual próprio (não nativo do Windows), mas aqui isso é o que se quer |
| Rust + Win32 nativo (`native-windows-gui` ou `windows`) | sim, e menor (menos de 1 MB) | difícil: gráficos e arrastar desenhados à mão em GDI | trabalho grande na parte que mais importa (os gráficos); aparência de formulário do Windows, perto da do NickPatcher |
| Tauri (HTML/JS) | não: depende do WebView2 (vem no Windows 10/11 atualizado, mas não em todo lugar) | fácil (a maquete viraria o programa) | o Lucas já disse que não precisa ser Tauri; dependência de sistema |

**Recomendação: Rust + eframe/egui.** É a mesma linguagem do projeto, pode reaproveitar as `Parte`, a `referencia.rs` e a lógica do `peso()` (os exemplos ficam idênticos à DLL), sai num exe só e desenha gráficos interativos sem esforço. Detalhes a confirmar num protótipo:
- o renderizador: o `wgpu` (DirectX 12, com o adaptador de software WARP como reserva) evita depender do driver OpenGL das máquinas antigas; o `glow` (OpenGL) é menor. Testar numa máquina virtual sem placa de vídeo antes de decidir;
- o alvo: `x86_64-pc-windows-msvc` cobre quase todos; i686 só se alguém pedir;
- fontes: as padrão do egui cobrem português, turco e o resto do Latin; embutir uma fonte própria só se o visual pedir.

## 9. Decisões que dependem do Lucas

1. **Tecnologia:** egui (recomendada) ou outra.
2. **Faixas propostas** das chaves sem limite no código (marcadas "proposta" no §3) e a regra "a barra cobre a parte útil, o campo aceita tudo".
3. **Predefinições:** os nomes, se "Equilibrado" já deve vir com `unattached = only_if_no_room` (as carreiras longas usaram), e se uma predefinição pode mexer no `save_over_2gb`.
4. **Idioma da interface:** inglês e português com troca, ou só inglês na abertura. A ajuda é o comentário do `.cfg`, que está em inglês; traduzir a ajuda exige o texto em português embutido.
5. **Módulo comum** entre DLL e configurador (`Parte`, `referencia.rs`, `peso()`), num workspace Cargo no mesmo repositório, ou cópia.
6. **Clubes do exemplo:** fictícios (como na maquete) ou lidos de um save do usuário (como o `calibrar.py`), numa versão seguinte.
7. **Chaves da Fase 4** (`video`, `resolution`, `no_cd`): os valores aceitos além de `window`, `1080` e `1` estão a definir.
8. **Onde fica o configurador:** no mesmo zip da DLL (`tweaks-config.exe` ao lado de `ddraw.dll` e `tweaks.cfg`), sugerido.
