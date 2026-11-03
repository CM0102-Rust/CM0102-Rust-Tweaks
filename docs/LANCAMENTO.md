# CM0102 Rust Tweaks 1.0

*English version: [LANCAMENTO.en.md](LANCAMENTO.en.md)*

Uma `ddraw.dll` de ajustes para o Championship Manager 01/02 (3.9.68) e o configurador `tweaks-config.exe`. Nada muda no exe: a DLL troca rotinas em memória quando o jogo abre, e tudo é ligado pelo `tweaks.cfg`. **Sem o `.cfg`, ou com tudo desligado, o jogo é o original.** Vale para jogos novos e para saves em andamento (a partir da carga).

## O que vem

| Grupo | O que faz | Chaves |
|---|---|---|
| **Save** | saves acima de 2 GB (até 4 GB), o mesmo do save2gb | `save_over_2gb` |
| **Instalação** | o jogo usa a pasta do exe (sem SetupReg); tira a marca de administrador da NickDLL | `portable_folder`, `clear_admin_flag` |
| **Tela e CD** | 1920×1080 em memória; o vídeo numa janela comum (Print Screen, escala nítida, sem trocar a resolução do Windows); sem o CD | `resolution`, `video`, `no_cd` |
| **Idades dos regens** | a idade com que os regens nascem, por tabela; "regens mais novos" (a ideia do petermf23) | `ages`, `age_14`…`age_35`, `younger_from_age`, `younger_by` |
| **Quem vai à disputa** | todo jovem de nação no jogo vai à disputa dos clubes, e só fica sem clube se não houver vaga | `unattached` |
| **A disputa dos clubes** | o jovem vai para um clube do tamanho dele, por sorteio proporcional (nível, instalações, comissão, vaga), com no máximo 2 por clube a cada 365 dias; o peso dos clubes fora das competições | `rule`, `season_cap`, `outside_competitions` e os pesos |
| **Inativos** | o jogador sem clube que ninguém quer se aposenta antes, e a ficha dele vira regen | `retire_idle_players` e os limites |
| **De quem é cada regen** | o `regen_origins.csv`: cada regen, para qual clube foi, e de quem herdou a vaga e a ficha (com nome, idade, CA e PA na aposentadoria), com a data do jogo, a do computador e a carreira | `regen_origins` |
| **Registro** | o `tweaks.log` para testes e para mandar quando algo der errado | `log` |

O guia de cada chave, para quem joga: [GUIA_DAS_CHAVES.md](GUIA_DAS_CHAVES.md).

## O configurador

`tweaks-config.exe`, um exe só, em português e inglês ([CONFIGURADOR.md](CONFIGURADOR.md)):
- o estado de cada ajuste no exe escolhido: compatível, sobrescreve (com o nome do patch), fica com o outro, não compatível;
- as opções com barra e campo, a ajuda prática de cada uma e o detalhe técnico do `.cfg`;
- gráficos ao vivo: as idades (barras arrastáveis), a chance de aposentar por idade, a curva do nível e as chances por clube;
- predefinições (Original, Só o save, Equilibrado, Equilibrado com todos com clube), com a lista do que muda;
- o histórico de regens com busca por nome e limpeza;
- grava só o valor das linhas mudadas e guarda o `.bak`.

## Compatibilidade

A regra é fazer o possível: cada ajuste confere o próprio trecho do exe e, se não servir, só ele fica de fora, com o motivo no log; os outros ligam ([COMPATIBILIDADE.md](COMPATIBILIDADE.md)).

| Exe | Situação |
|---|---|
| 3.9.68 original e o portátil/1080 do projeto | tudo liga |
| exe com o Regen Tweak (xeno/Saturn) | a disputa fica com a do exe (ou a nossa, com `override_exe_changes = 1`); o resto liga |
| CLZ 6.1 (NickPatcher, NickDLL, LaMOHatesCD) | as idades e a disputa ficam com as do exe (ou as nossas, com override); a resolução fica com a NickDLL quando ela escolhe uma; o CD com a LaMOHatesCD; o resto liga |

## O que foi testado

Os números estão em [TESTES.md](TESTES.md).
- Carreiras automáticas de 5 e 10 temporadas, com 10 ligas, no 3.9.68 e no CLZ, sem queda: o teto de 2 jovens por clube se mantém por 10 anos; os grandes recebem 4% dos jovens da disputa (15% no original) e terminam com os mesmos craques; os regens sem clube caem pela metade.
- O save acima de 2 GB carrega nos dois exes.
- Tela e CD: no 3.9.68 intacto, no de 1080, numa cópia "misturada" como a do CLZ e numa com outro patch.
- A origem dos regens: 44 regens numa carreira, todos com a vaga e a ficha identificadas.

Ainda sem teste: a janela, o Print Screen e o relatório em 1080 olhando a tela; o `video = window` e o `regen_origins` no CLZ; a ligação da carreira com o nome do save (a carreira automática não grava).

## Instalar e tirar

1. Copiar `ddraw.dll`, `tweaks.cfg` e `tweaks-config.exe` para a pasta do jogo (a do `cm0102.exe`).
2. Abrir o `tweaks-config.exe`, escolher as opções e gravar.
3. Para tirar: apagar a `ddraw.dll` (o jogo volta ao original; os saves continuam valendo, menos os acima de 2 GB).
