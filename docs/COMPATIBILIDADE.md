# Compatibilidade com outros patches

*English version: [COMPATIBILIDADE.en.md](COMPATIBILIDADE.en.md)*

O que cada ajuste do Tweaks faz quando o exe, ou uma DLL ao lado dele, já mudou o mesmo trecho. **A regra é fazer o possível:** cada ajuste confere o seu trecho sozinho, e o que não serve fica de fora com o motivo no `tweaks.log`. Os outros ajustes ligam normalmente, e o jogo nunca é interrompido por isso. **[observado]** = visto no código, nos bytes ou no log; **[inferência]** = a confirmar no jogo.

Para quem usa o NickPatcher, o aviso opção por opção está em [NICKPATCHER.md](NICKPATCHER.md).

## 1. Os estados

Os mesmos que o configurador vai mostrar ([CONFIGURADOR.md](CONFIGURADOR.md)):

| Estado | Quando | Exemplo |
|---|---|---|
| ✅ compatível | o trecho está como no 3.9.68, ou o exe já tem esta mesma mudança | o save acima de 2 GB em qualquer exe |
| ⚠️ sobrescreve | o exe tem uma mudança **conhecida** no trecho, e `override_exe_changes = 1` manda trocá-la pela nossa | a disputa sobre o Regen Tweak |
| 🔁 fica com o outro | uma mudança **conhecida** faz o mesmo papel; a nossa fica de fora, e a do exe ou da outra DLL vale | a resolução da NickDLL do CLZ 6.1; o CD da LaMOHatesCD |
| ❓ desconhecido | o trecho difere do 3.9.68 de um jeito que o catálogo não reconhece; só este ajuste fica de fora | um patch novo de alguém |

"Conhecido" quer dizer nomeado pelo retrato do exe (`src/retrato.rs`, os casos conhecidos), pelo catálogo do NickPatcher (`tools/retrato_exe.py`) ou por este documento.

## 2. A matriz

| Ajuste | 3.9.68 (e os exes do projeto CM0102) | Regen Tweak (xeno/Saturn) | CLZ 6.1: exe (NickPatcher) | CLZ 6.1: NickDLL e LaMOHatesCD |
|---|---|---|---|---|
| `save_over_2gb` | ✅ | ✅ | ✅ | ✅ a NickDLL chama a 0x933610 (outra rotina do contêiner), que o ajuste não troca |
| `portable_folder`, `clear_admin_flag` | ✅ | ✅ | ✅ | ✅ feitos para o CLZ |
| `ages` | ✅ | ✅ | 🔁 o NickPatcher mudou as faixas raras de idade; ⚠️ com override 1 | ✅ |
| `ages_out_of_game` | ✅ | ✅ | 🔁 o ramo foi pulado no exe; ⚠️ com override 1 (o ramo continua sem rodar) | ✅ |
| `rule = new` (a disputa) | ✅ | 🔁 o Regen Tweak (elenco 45); ⚠️ com override 1 | 🔁 / ⚠️ igual | ✅ a NickDLL troca a rotina do time B (0x540a50) pela dela, com a lista do banco; a disputa a usa como usaria a do exe |
| `retire_idle_players` | ✅ | ✅ | ✅ liga, mas o RetirePatchV4 não deixa ninguém com menos de 34 chegar à regra (o log avisa) | ✅ |
| `unattached` | ✅ | ✅ | ✅ | ✅ |
| `no_cd` | ✅ | ✅ | ✅ por checagem: o exe já tem 14 das 33; a DLL completa as outras | 🔁 com a `LaMOHatesCD.dll` ao lado, fica com ela |
| `resolution = 1080` | ✅ (no exe de 1080, "já tem") | ✅ | ✅ o exe tem 5 dos 201 trechos já trocados (o mesmo código do NickPatcher); a DLL completa os outros | 🔁 com a NickDLL do 6.1 e `"resolucao"` diferente de 800x600 no `NickDLL_config.json`, fica com ela |
| a trava das 30 linhas (com `resolution`, `video` ou a resolução da NickDLL) | ✅ | ✅ | ✅ | ✅ nenhuma correção da NickDLL toca o `Area::init` |
| `video = window` | ✅ | ✅ | ❓ não testado: o CLZ em janela com a `ddraw.dll` do Windows para em "16 bit colour"; a camada de vídeo deve resolver [inferência] | ❓ |
| `log = 1` (o registro das caixas, 0x5e8290) | ✅ | ✅ | ✅ | ⚠️ a LaMOHatesCD e a NickDLL do 6.1 (`hook_msgbox`) também desviam a 0x5e8290; o registro encadeia no desvio que já estiver lá; se a NickDLL instalar o dela depois e conferir os bytes originais, o dela pode se recusar [inferência, a confirmar no jogo] |

## 3. O inventário da NickDLL do CLZ 6.1

`tools/inventario_dll.py` procura, nas seções de uma DLL, os endereços que caem nos trechos de cada ajuste. Não roda nada e não copia código: aponta onde olhar, e a confirmação é o retrato em memória (as linhas `runtime` do `tweaks.log`). A leitura da NickDLL tem a autorização do autor (dmcarmo), e a versão do CLZ é editada pela equipe do CLZ.

| Versão (`D:\Games\CLZ Leagues Patch v6.1`) | O que toca nos nossos trechos |
|---|---|
| `NickDLL.dll` de 02/11 22:08 (a do 6.1) | **a resolução:** 208 dos nossos endereços; a "resolucao ampliada" (1280×720, 1280×800, 1280×1024, 1920×1080, pelo `NickDLL_config.json`) usa a mesma tabela de trocas e recusa se achar os bytes já mudados. **A caixa de mensagem** (0x5e8290). **O time B** (0x540a50). Chamadas à 0x933610 (contêiner do save) e a dois pontos da criação do regen (0x7abd90/96), fora dos trechos que trocamos |
| `NickDLL.dllold` de 02/11 14:40 | o mesmo |
| a de 18/10 (a das carreiras longas, em `copias\bancada4\clz_tweaks`) | sem a resolução; o resto igual |
| `NickDLL.dlloriginal` de 08/10 | sem a resolução (só 3 endereços do Display) e com a 0x7a8060 (dentro da aposentadoria do desempregado) |
| `LaMOHatesCD.dll` | só a caixa de mensagem |

As correções da NickDLL do 6.1 (0x6847C0, 0x51F890, 0x768310, 0x522E00, 0x4A2540, 0x48F150, 0x4A4850, 0x4ACEB0, 0x6F46B0, 0x6E1090, 0x6CD35C, 0x6CD3FE, 0x6F3A67, pelas mensagens do log dela) não caem em nenhum trecho do Tweaks [observado].

## 4. Uma DLL nova (a do Davdav e as próximas)

1. `python tools/inventario_dll.py NovaDLL.dll`: os ajustes cujos trechos ela toca.
2. Para cada ajuste tocado, ler o que ela faz ali (sem copiar) e pôr na matriz: ✅, ⚠️, 🔁 ou ❓.
3. Uma carreira curta com `log = 1`: as linhas `runtime` mostram o que ela mudou de fato na memória, e o `check` diz se algum desvio nosso foi sobrescrito.
4. Se ela faz o mesmo papel de um ajuste nosso (como a resolução da NickDLL), a DLL a reconhece e fica com ela, como em `resolucao_da_nickdll` (`src/lib.rs`).
