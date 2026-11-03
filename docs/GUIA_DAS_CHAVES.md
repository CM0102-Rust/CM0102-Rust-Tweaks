# Guia das chaves do tweaks.cfg

*English version: [GUIA_DAS_CHAVES.en.md](GUIA_DAS_CHAVES.en.md). Gerado por `tweaks-config.exe --guia pt` dos mesmos textos da ajuda do configurador; não editar à mão.*

Cada opção: o que faz, quando usar e o que acontece no jogo. Sem a linha no `tweaks.cfg`, vale o padrão indicado (o da DLL). O comentário de cada linha do `tweaks.cfg` traz o detalhe técnico.

## Jogo

### Save acima de 2 GB — `save_over_2gb`

Permite gravar e carregar saves acima de 2 GB (até 4 GB). Ligue se o seu save é grande (muitas ligas, carreira longa) ou se o jogo dá erro ao carregar. Desligado, um save acima de 2 GB não carrega. Não muda nada na carreira.

Valores: `1` liga, `0` desliga. Padrão: `0`.

### Pasta portátil (sem SetupReg) — `portable_folder`

O jogo usa a pasta onde está o exe, sem precisar do SetupReg nem do registro do Windows. Útil para ter várias instalações lado a lado ou uma cópia num pendrive. Nada é gravado no registro.

Valores: `1` liga, `0` desliga. Padrão: `0`.

### Tirar a marca de administrador da NickDLL — `clear_admin_flag`

Só para quem usa a NickDLL (CLZ e outros pacotes do NickPatcher): ela marca o exe para rodar como administrador a cada abertura. Esta opção tira essa marca (mantém o modo Windows 95). Vale a partir da abertura seguinte. Deixe desligado se você marcou isso de propósito.

Valores: `1` liga, `0` desliga. Padrão: `0`.

### Sobrescrever as mudanças do exe — `override_exe_changes`

Quando o seu exe já tem uma modificação no mesmo ponto de um ajuste (o Regen Tweak, as idades do NickPatcher), o normal é ficar com a do exe. Ligado, os ajustes desta DLL substituem essas modificações. Use só se você quer as regras desta DLL no lugar das do seu pacote.

Valores: `1` liga, `0` desliga. Padrão: `0`.

### Vídeo — `video`

Janela: o jogo desenha numa janela comum. O Print Screen e os gravadores de tela funcionam, a janela muda de tamanho com escala nítida, e o Windows nunca troca de resolução (sem piscar no Alt+Tab). Original: o DirectDraw do Windows, como sempre.

Valores: `original` (DirectDraw do Windows), `window` (Janela do projeto (Print Screen, escala)). Padrão: `original`.

### Resolução — `resolution`

1920×1080: as telas do jogo em 1920×1080, aplicado em memória (o exe não é alterado). Liga o vídeo em janela junto e evita o erro do relatório de partidas antigas. Se o exe ou a NickDLL já têm resolução própria, fica com a deles.

Valores: `original` (A do exe), `1080` (1920×1080). Padrão: `original`.

### Sem o CD — `no_cd`

O jogo roda sem o CD. Se você já usa um NoCD (a LaMOHatesCD do CLZ, um exe sem CD), fica com ele.

Valores: `1` liga, `0` desliga. Padrão: `0`.

## Idades

### Idades dos regens — `ages`

Tabela: a idade com que os regens nascem segue a tabela de idades (as barras). Original: o sorteio do jogo (quase todos de 16 a 20). Use a tabela para regens mais novos ou mais velhos. Vale para os regens que nascerem dali em diante, também num save em andamento.

Valores: `original` (Original), `custom` (Tabela). Padrão: `original`.

### Tabela de idades — `age_14` … `age_35`

A chance de um regen nascer com cada idade (as chances não precisam somar 100) e o teto do CA naquela idade: o jovem não começa com CA acima do teto. Arraste a barra para mudar a chance; zero tira a idade.

Formato: `age_NN = chance, teto do CA`.

### Mais novos: a partir de — `younger_from_age`

"Regens mais novos" (ideia do petermf23): os que sairiam com esta idade ou mais nascem mais novos. Funciona junto com "anos a menos".

Valores: de 0 a 126 (inteiro). Padrão: `22`.

### Mais novos: anos a menos (0 = desligado) — `younger_by`

Quantos anos a menos para os regens de "a partir de" para cima. 0 desliga. Mais anos a menos = mais jovens de 16 a 20 e menos veteranos recém-gerados.

Valores: de 0 a 126 (inteiro). Padrão: `0`.

### Nações fora do jogo — `ages_out_of_game`

As nações fora do jogo com poucas pessoas no banco sorteiam idades de um jeito próprio (quase sempre 23 a 27). "A tabela também" faz essas nações usarem a sua tabela. Só muda nações cuja liga você não escolheu.

Valores: `original` (Sorteio original), `table` (A tabela também). Padrão: `original`.

### Pular o teto do CA (%) — `ca_cap_skip`

Chance (%) de um jovem ignorar o teto do CA da idade dele: alguns nascem já prontos, como no jogo original (cerca de 1 em 200). Mais alto = mais jovens prontos cedo.

Valores: de 0 a 100. Padrão: `0.5`.

### Idade máxima da disputa — `contest_max_age`

Até que idade o jovem vai à disputa dos clubes no jogo original (os mais velhos começam sem clube). Sem efeito com "Sem clube só sem vaga", que manda todos à disputa.

Valores: de 0 a 126 (inteiro). Padrão: `18`.

### Quem vai à disputa — `unattached`

Sem clube só sem vaga: todo jovem de nação no jogo vai à disputa e só fica sem clube se nenhum clube do país tiver vaga. Original: só os de até 18 anos vão, e 1 em 4 é descartado (muitos começam sem clube e se perdem). Recomendado: menos talentos desperdiçados.

Valores: `original` (Original), `only_if_no_room` (Sem clube só sem vaga). Padrão: `original`.

## Inativos

### Aposentar os inativos — `retire_idle_players`

O jogador sem clube que ninguém quer se aposenta mais cedo, e a ficha dele vira um regen semanas depois. Resultado: mais jovens e menos veteranos parados no mercado. Num pacote que barra aposentadorias (o RetirePatchV4 do NickPatcher), quase não age.

Valores: `1` liga, `0` desliga. Padrão: `0`.

### Dias sem clube — `idle_days`

Quantos dias sem clube antes de a regra dos inativos valer. Menos dias = aposentadorias mais rápidas.

Valores: de 0 a … (inteiro). Padrão: `180`.

### Idade mínima — `idle_min_age`

A partir de que idade a regra dos inativos vale. Abaixo dela, fica a regra original do jogo.

Valores: de 0 a 126 (inteiro). Padrão: `28`.

### Reputação-limite — `idle_rep`

A reputação até a qual o inativo se aposenta: quem tem reputação menor sai quase sempre; quem tem mais, raramente. Mais alto = mais aposentadorias.

Valores: de 0 a …. Padrão: `1500`.

### A mais por ano de idade — `idle_rep_per_year`

Quanto a reputação-limite sobe por ano acima da idade mínima: os mais velhos se aposentam mais fácil.

Valores: de 0 a …. Padrão: `250`.

### Proteger os menores de (0 = não) — `protect_young_unemployed`

Abaixo desta idade, nenhum jogador sem clube se aposenta pela regra dos inativos. 0 = sem proteção (o padrão: o jogo já dá 365 dias ao jovem sem clube). Proteger os jovens diminui a reciclagem em regens.

Valores: de 0 a 126 (inteiro). Padrão: `0`.

## Disputa

### A disputa dos clubes — `rule`

Sorteio proporcional: o jovem vai para um clube do tamanho dele, sorteado pelo nível, instalações, comissão e vaga, com no máximo 2 jovens por clube por ano. Original: os grandes recolhem quase todos (até 14 numa temporada). Recomendado para carreiras equilibradas.

Valores: `original` (Original), `new` (Sorteio proporcional). Padrão: `original`.

### Alvo: reputação por ponto de PA — `target_k`

Que reputação de clube combina com o jovem: o alvo é este valor vezes o PA (50 × PA 150 = clube de 7500). Mais alto = jovens bons indo para clubes maiores.

Valores: de … a …. Padrão: `50`.

### Alvo: peso do CA (0 = PA, 1 = CA) — `target_ca`

Quanto o alvo olha o CA (o que o jovem já sabe) em vez do PA (o que pode vir a ser). 0 = só o PA; 1 = só o CA.

Valores: de … a …. Padrão: `0`.

### Largura acima do alvo — `sigma`

Quanto um clube acima do alvo ainda tem chance. Maior = os jovens se espalham por clubes de níveis mais variados; menor = vão para clubes bem do tamanho deles.

Valores: de 1 a …. Padrão: `2500`.

### Largura abaixo do alvo (0 = igual) — `sigma_below`

O mesmo, para os clubes abaixo do alvo (0 = igual à largura de cima). Mantido menor porque as centenas de clubes pequenos de um país, somadas, levariam os jovens bons.

Valores: de 0 a …. Padrão: `1200`.

### Piso do nível — `level_floor`

A chance mínima de um clube longe do alvo (0 a 1). Acima de 0, um clube pequeno ainda pode, raramente, receber um craque.

Valores: de … a …. Padrão: `0.002`.

### Instalações: base — `facilities_base`

Quanto as instalações de base (Training) pesam: o peso é esta base mais o valor por ponto. Clubes com boa base atraem mais jovens.

Valores: de … a …. Padrão: `0.4`.

### Instalações: por ponto de Training — `facilities_k`

Quanto cada ponto de Training soma ao peso (com 0,4 e 0,06: Training 1 = 0,46; 10 = 1,00; 20 = 1,60).

Valores: de … a …. Padrão: `0.06`.

### Comissão: o pior — `staff_min`

O peso do clube cuja comissão trabalha pior com jovens ("Working with Youngsters" 1).

Valores: de … a …. Padrão: `0.8`.

### Comissão: o melhor — `staff_max`

O peso do clube cuja comissão trabalha melhor com jovens ("Working with Youngsters" 20). Deve ser maior que o do pior.

Valores: de … a …. Padrão: `1.2`.

### Elenco: teto — `squad_limit`

Clube com este número de jogadores ou mais não recebe jovem. Mais baixo = os jovens vão para clubes com elenco menor.

Valores: de 1 a 50. Padrão: `48`.

### Elenco: expoente da vaga — `squad_exp`

O quanto a vaga no elenco pesa: maior = clubes com elenco curto têm muito mais chance que os de elenco quase cheio.

Valores: de … a …. Padrão: `0.5`.

### Teto por clube a cada 365 dias (0 = sem) — `season_cap`

Quantos jovens um clube recebe no máximo a cada 365 dias pela disputa (0 = sem teto). É o que impede um clube de recolher dezenas de regens. Com 3, os elencos das ligas jogadas ficam um pouco mais cheios.

Valores: de … a …. Padrão: `2`.

### Surpresa (%) — `surprise`

Percentagem dos jovens sorteados sem olhar o nível do clube, só entre clubes das competições: o craque que surge num clube pequeno, ou o comum num grande. 0 = nunca.

Valores: de 0 a 100. Padrão: `15`.

### Clubes fora das competições (peso) — `outside_competitions`

O peso dos clubes cuja divisão não roda na sua carreira (nem como liga de fundo). 1 = iguais aos outros; 0,25 = um quarto da chance; 0 = nunca (sem clube de competição com vaga, o jovem fica sem clube). Valores baixos mantêm cheios os elencos das ligas que você joga; com poucas ligas carregadas, 1 pode servir melhor.

Valores: de 0 a 1. Padrão: `1`.

### Temperatura — `temperature`

Acentua (maior que 1) ou suaviza (menor que 1) as diferenças de peso entre os clubes. 1 = o peso como calculado.

Valores: de … a …. Padrão: `1`.

## Registro

### Registro (tweaks.log) — `log`

Grava o tweaks.log ao lado do exe: o que cada ajuste fez, cada jovem e o clube, as caixas de erro e, todo julho, o retrato da população. Para testar e para mandar quando algo der errado. Desligado no uso normal.

Valores: `1` liga, `0` desliga. Padrão: `0`.

