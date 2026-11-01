# CM0102-Rust-Tweaks

*English version: [README.md](README.md)*

Ajustes para o **Championship Manager 01/02 (3.9.68)** numa `ddraw.dll`, sem alterar o exe. Feitos com IA, a partir da reescrita do jogo em Rust (o projeto CM0102). Roda com o exe original ou com os do NickPatcher; cada ajuste confere o código do exe antes de ligar.

Cada ajuste tem a sua chave no `tweaks.cfg`, ao lado do exe. **Todos começam desligados:** sem o `.cfg`, ou sem a linha, a DLL não muda nada no jogo. Um configurador (o seletor de opções) vai ler e gravar esse mesmo arquivo.

| Chave | O que faz |
|---|---|
| `save_over_2gb = 1` | O **save acima de 2 GB** (até 4 GB), o mesmo do save2gb: as três rotinas do contêiner (`zopen` 0x9326e0, `zseek` 0x933080, `ztell` 0x933170) vão para as de `src/conteiner.rs` |
| `ages = custom` | As **idades dos regens** por uma tabela (§ As idades) |
| `rule = new` | A **disputa dos clubes** pelo jovem regen (`PLAYER_REGEN::pick_best_club_for_player`, 0x7ad0e0) vira um sorteio proporcional (§ A regra nova). A regra original está em [docs/DISTRIBUICAO_DOS_REGENS.md](docs/DISTRIBUICAO_DOS_REGENS.md) |

Quando este repositório for aberto, ele substitui o `CM0102-Rust/cm0102-save2gb`, que passa a apontar para cá: as evoluções ficam todas aqui. Por isso o save está nesta DLL; com uma DLL só, ninguém precisa encadear duas `ddraw.dll`.

**Estado: protótipo para discussão.** Compila e confere as assinaturas, mas ainda não rodou no jogo. A curva dos regens é um ponto de partida, não uma proposta fechada.

## Arquivos

| Arquivo | O quê |
|---|---|
| `src/lib.rs` | A DLL: repassa o `DirectDrawCreate` (a `ddraw_chain.dll` ou a do Windows), lê `tweaks.cfg`, confere as assinaturas e liga cada ajuste |
| `src/conteiner.rs` | O contêiner do save acima de 2 GB (cópia do `rust/src/conteiner.rs` do projeto CM0102; estável desde o save2gb) |
| `tweaks.cfg` | As opções, em inglês e comentadas (é o arquivo da comunidade); vai ao lado do exe |
| `tools/calibrar.py` | Simulador fora do jogo: os clubes reais de um país, lidos de um save, disputam várias temporadas de jovens pela regra original e pela nova (com a tabela de idades do `.cfg`) |
| `tools/simula_nota.py` | O exemplo de clubes inventados do documento (§7) |
| `tools/regens_por_clube.py` | Os regens por clube num save, por faixa de reputação e de Training |
| `tools/sav.py` | Leitura dos itens de um `.sav` |

Compilar: `cargo build --release` (sai em `target/i686-pc-windows-msvc/release/ddraw.dll`).

Cada ajuste confere duas assinaturas (FNV-1a) antes de ligar:
- **as bordas**, o que o desvio usa (a entrada, a volta, as rotinas chamadas). Se não batem, o ajuste nunca liga;
- **o corpo**, a rotina original que o ajuste troca. Se só ele difere, o exe já tem outra modificação ali e, **como no resto do projeto, fica a rotina do exe**, com aviso no `tweaks.log`. `override_exe_changes = 1` manda sobrescrever.

Nos exes que temos [observado]:

| Exe | Save | Idade | Idade (fora do jogo) | Disputa |
|---|---|---|---|---|
| `cm0102.exe`, `cm0102-1080.exe`, portátil | liga | liga | liga | liga |
| os três do CLZ | liga | do exe | do exe | do exe (Regen Tweak) |
| `cm0102_GDI.exe` | nunca | nunca | nunca | nunca |

"Do exe" vira "liga" com `override_exe_changes = 1`. O log nomeia o Regen Tweak de xeno/Saturn quando o acha.

## As idades

A idade do jovem é sorteada num bloco da `regenerate_as_youth_player` (0x7ac597..0x7ac6f6), que também limita o CA pela idade. Com `ages = custom`, a DLL troca esse bloco por um pequeno trecho de máquina que chama `sortear_idade`: a idade sai da tabela `age_NN = chance, teto do CA` do `.cfg` (14 a 35 anos), e o teto do CA dessa idade é aplicado, menos em `ca_cap_skip`% das vezes. A idade segue para o resto da rotina como na original (o ano de nascimento, os ajustes de atributos pela idade). E `contest_max_age` (padrão 18) troca o corte "menos de 19 anos entra na disputa" (o byte de 0x7ace69).

- **Regens mais novos, a ideia de petermf23** (item 14 do mod dele das divisões polonesas para o 3.9.60): o jovem sorteado com `younger_from_age` (22) anos ou mais fica `younger_by` (6) anos mais novo (22 → 16, 26 → 20), para ter um período de desenvolvimento. A regra vale depois do sorteio, na tabela e nas nações fora do jogo. Nas jogáveis, 22 anos ou mais são raríssimos (uns 0,2%); nas nações fora do jogo, a original sorteia quase sempre 23 a 27, e é ali que a regra pesa (e a idade menor leva também à fórmula do CA dos mais novos). Lá, com `ages_out_of_game = original`, a DLL reproduz o sorteio da original e aplica a regra. Ligada no `.cfg` distribuído; `younger_by = 0` desliga.
- A tabela padrão do `.cfg` é a distribuição original: 16 a 20 anos quase sempre (17 e 18 com 29% cada), 15 e 21 a 26 raríssimos.
- Diferença assumida: o teto do CA vira um valor por idade. Na original, o de 17 e o de 18 dependem da faixa sorteada (80 ou 90, 100 ou 110).
- As nações fora do jogo de reputação < 350 têm um sorteio próprio na original: quase sempre 23 a 27 anos, às vezes 17 a 23, e ali a idade escolhe a fórmula do CA (mais novo, CA menor), sem teto. Com `ages_out_of_game = table` elas também usam a tabela (0x7abeb1..0x7abeea); a fórmula do CA pela idade continua a da original, e os tetos da tabela não valem ali. O padrão é `original`.
- **Os exes do CLZ já mexem na idade** [observado nos bytes]: a faixa raríssima de 15 a 18 vira 14 a 17 (0x7ac5b1), a rara de 21 a 26 vira 20 a 25 (0x7ac699), e as nações fora do jogo passam a usar o sorteio das jogáveis (`jmp` em 0x7abeab). Também trazem o Regen Tweak completo (0x7ad1bf `0x2d` e 0x7a6f48 `jl`) e outra troca de nacionalidade dos jovens (0x7abcd7..0x7abd1c). Por padrão a DLL deixa essas rotinas com o exe.

## A regra nova

Os clubes elegíveis são os mesmos da original:
- o jovem não é estrangeiro no clube;
- não é time B;
- não é vetado pela regra basca;
- o clube pode chegar ao jogador;
- elenco abaixo do teto.

Cada clube recebe um **peso**, e o clube é sorteado **na proporção do peso**, pelo gerador do próprio jogo. Peso nenhum: o jovem fica sem clube, como na original.

**A sequência aleatória muda:** a regra nova gasta 2 sorteios do gerador por jovem; a original gasta 1 por clube elegível (e às vezes mais, nos bônus da nota). A tabela de idades também gasta outros sorteios. Para um mod é o esperado, mas uma carreira com a mesma semente (`-seed`) não sai igual à do jogo sem a DLL.

    peso = (nível × instalações × comissão × vaga) ^ temperatura

| Fator | Conta | Ajustes |
|---|---|---|
| Nível | curva em sino da reputação do clube em torno do alvo = target_k × PA (ou a mistura com o CA) | `target_k`, `target_ca`, `sigma`, `level_floor` |
| Instalações | facilities_base + facilities_k × Training | `facilities_base`, `facilities_k` |
| Comissão | de staff_min a staff_max pelo melhor "Working with Youngsters" entre o técnico e os 5 treinadores. Vale para a IA e para o humano igual | `staff_min`, `staff_max` |
| Vaga | ((squad_limit − elenco) / squad_limit) ^ squad_exp; com o elenco no teto, peso 0 | `squad_limit`, `squad_exp` |
| Teto por clube | peso 0 se o clube já recebeu `season_cap` regens nos últimos 365 dias (contados no próprio save: o bit do regen e a data de entrada no clube; sem dado novo no save) | `season_cap` |
| Temperatura | expoente: 1 proporcional, maior mais previsível, menor mais aleatório | `temperature` |

Com `log = 1`, cada escolha vai para `tweaks.log`: PA, clube, reputação, Training, chance e quantos disputaram.

## A calibração (Inglaterra, `TesteTelas.sav`, 200 temporadas simuladas)

Parâmetros:
- 687 clubes;
- 52 jovens por temporada na disputa (no `Valencia.sav`, a Inglaterra teve ~88 regens por temporada, e ~56% entram na disputa);
- PA sorteado entre os jogadores ingleses de 31 anos ou mais;
- o elenco volta ao do save a cada temporada [simplificação].

| Reputação | Clubes | Original: jovens por clube por temporada | Nova (padrão) | Original: PA ≥ 140 | Nova: PA ≥ 140 |
|---|---|---|---|---|---|
| 0–999 | 499 | 0,00 | 0,03 | 0,000 | 0,000 |
| 1000–1999 | 83 | 0,09 | 0,05 | 0,000 | 0,000 |
| 2000–2999 | 24 | 0,40 | 0,14 | 0,000 | 0,001 |
| 3000–3999 | 21 | 0,28 | 0,28 | 0,002 | 0,004 |
| 4000–4999 | 27 | 0,22 | 0,42 | 0,020 | 0,019 |
| 5000–5999 | 13 | 0,27 | 0,40 | 0,038 | 0,039 |
| 6000–6999 | 13 | 0,54 | 0,37 | 0,083 | 0,068 |
| 7000–7999 | 3 | 1,26 | 0,31 | 0,110 | 0,140 |
| 8000–8999 | 2 | 1,45 | 0,21 | 0,083 | 0,102 |
| 9000+ | 2 | 1,96 | 0,09 | 0,138 | 0,062 |

Na original, Liverpool, Arsenal e Manchester United recebem ~2 por temporada cada, a maioria jovens comuns. Na nova, os grandes recebem menos jovens no total, mas a parte deles nos de PA ≥ 140 é parecida (menos no 9000+, porque esses estão com 47 a 48 no elenco). Os clubes médios, de 4000 a 6999, passam a receber mais.

Duas tentativas descartadas [observado no simulador]:
- **Piso de nível de 5% e teto de elenco de 45:** os grandes zeraram (estavam acima de 45) e uns 30 dos 52 jovens foram para os 499 clubes amadores. Com centenas de clubes pequenos, um piso pequeno somado vira a maior fatia do sorteio.
- **Temperatura 2:** concentra mais nos clubes do tamanho certo; os de 7000+ ficam com o dobro dos PA ≥ 140.

Rodar: `python tools/calibrar.py TesteTelas.sav Inglaterra --humano Fulham --temporadas 200 [--cfg outro.cfg]`.

## Para discutir

1. **O alvo.** Onde o jovem "deveria" ir: pelo nível que vai ter (PA, o padrão) ou pelo que tem hoje (CA, `target_ca`)? O alvo de 50 × PA põe um jovem de PA 180 num clube de 9000.
2. **A largura (`sigma`).** Com 1500, um clube a 3000 do alvo fica com ~14% do peso máximo. Mais estreita: menos surpresa.
3. **Os grandes e os craques.** Quanto dos jovens de PA alto os grandes devem levar? Hoje a nova regra mantém a parte deles parecida com a original.
4. **O peso das instalações.** Hoje Training 20 vale 2,7 vezes o Training 1 (1,5 contra 0,55). Mais forte?
5. **A comissão.** Hoje vai de 0,8 a 1,2. O humano entra igual à IA, pelos treinadores. Mais forte?
6. **O teto por clube** (3 por 365 dias) e o teto de elenco (50).
7. **A aleatoriedade (`temperature`).**
8. **Fora do protótipo:**
   - os ~44% de jovens que nascem sem clube poderiam passar pelo mesmo sorteio com peso menor (um ajuste depois da fila diária, 0x7a65d0);
   - o PA do jovem poderia variar alguns pontos pelo Training do clube que o recebe (ao assinar, 0x7a6f10).

## Próximos passos

- Fechar a curva na discussão.
- O configurador: um `.exe` que lê e grava o `tweaks.cfg` (sem apagar os comentários), valida as faixas, oferece predefinições e mostra, para o exe escolhido, o que cada ajuste vai fazer (liga, fica a do exe ou não serve).
- Na abertura: o `cm0102-save2gb` passa a apontar para cá.
- Prova no jogo: carreira automática de férias na bancada, com o exe original e esta DLL, comparando a distribuição por faixa (`tools/regens_por_clube.py`) com a original.
- Teste com o NickPatcher e o CLZ.
