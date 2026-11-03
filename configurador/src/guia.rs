//! A ajuda prática de cada chave (português, inglês): o que faz, quando usar e o que acontece no jogo. É o texto da dica
//! e do painel da direita do configurador e o do guia `docs/GUIA_DAS_CHAVES(.en).md` (`tweaks-config.exe --guia pt|en`).
//! O comentário do `tweaks.cfg` continua como o detalhe técnico.

pub fn ajuda(chave: &str) -> (&'static str, &'static str) {
    match chave {
        "save_over_2gb" => (
            "Permite gravar e carregar saves acima de 2 GB (até 4 GB). Ligue se o seu save é grande (muitas ligas, carreira longa) ou se o jogo dá erro ao carregar. Desligado, um save acima de 2 GB não carrega. Não muda nada na carreira.",
            "Lets the game save and load saves above 2 GB (up to 4 GB). Turn it on if your save is large (many leagues, a long career) or the game errors when loading. Off, a save above 2 GB won't load. It changes nothing in the career.",
        ),
        "portable_folder" => (
            "O jogo usa a pasta onde está o exe, sem precisar do SetupReg nem do registro do Windows. Útil para ter várias instalações lado a lado ou uma cópia num pendrive. Nada é gravado no registro.",
            "The game uses the folder its exe is in, with no SetupReg and no Windows registry. Useful for several installs side by side or a copy on a USB stick. Nothing is written to the registry.",
        ),
        "clear_admin_flag" => (
            "Só para quem usa a NickDLL (CLZ e outros pacotes do NickPatcher): ela marca o exe para rodar como administrador a cada abertura. Esta opção tira essa marca (mantém o modo Windows 95). Vale a partir da abertura seguinte. Deixe desligado se você marcou isso de propósito.",
            "Only for NickDLL users (CLZ and other NickPatcher packs): it marks the exe to run as administrator every time. This removes that mark (keeps Windows 95 mode). It takes effect from the next start. Leave it off if you set that on purpose.",
        ),
        "override_exe_changes" => (
            "Quando o seu exe já tem uma modificação no mesmo ponto de um ajuste (o Regen Tweak, as idades do NickPatcher), o normal é ficar com a do exe. Ligado, os ajustes desta DLL substituem essas modificações. Use só se você quer as regras desta DLL no lugar das do seu pacote.",
            "When your exe already has a change where a tweak hooks in (the Regen Tweak, NickPatcher's ages), the exe's stays by default. On, this DLL's tweaks replace those changes. Use it only if you want this DLL's rules instead of your pack's.",
        ),
        "video" => (
            "Janela: o jogo desenha numa janela comum. O Print Screen e os gravadores de tela funcionam, a janela muda de tamanho com escala nítida, e o Windows nunca troca de resolução (sem piscar no Alt+Tab). Original: o DirectDraw do Windows, como sempre.",
            "Window: the game draws to an ordinary window. Print Screen and screen recorders work, the window resizes with sharp scaling, and Windows never changes resolution (no flicker on Alt+Tab). Original: Windows' DirectDraw, as always.",
        ),
        "resolution" => (
            "1920×1080: as telas do jogo em 1920×1080, aplicado em memória (o exe não é alterado). Liga o vídeo em janela junto e evita o erro do relatório de partidas antigas. Se o exe ou a NickDLL já têm resolução própria, fica com a deles.",
            "1920×1080: the game's screens at 1920×1080, applied in memory (the exe file isn't changed). It turns on the windowed video too and avoids the old match report error. If the exe or NickDLL already has its own resolution, theirs stays.",
        ),
        "no_cd" => (
            "O jogo roda sem o CD. Se você já usa um NoCD (a LaMOHatesCD do CLZ, um exe sem CD), fica com ele.",
            "The game runs without the CD. If you already use a NoCD (CLZ's LaMOHatesCD, a no-CD exe), that one stays.",
        ),
        "ages" => (
            "Tabela: a idade com que os regens nascem segue a tabela de idades (as barras). Original: o sorteio do jogo (quase todos de 16 a 20). Use a tabela para regens mais novos ou mais velhos. Vale para os regens que nascerem dali em diante, também num save em andamento.",
            "Table: the age regens are born with follows the age table (the bars). Original: the game's draw (almost all 16 to 20). Use the table for younger or older regens. It applies to regens born from then on, in a save in progress too.",
        ),
        "age_NN" => (
            "A chance de um regen nascer com cada idade (as chances não precisam somar 100) e o teto do CA naquela idade: o jovem não começa com CA acima do teto. Arraste a barra para mudar a chance; zero tira a idade.",
            "The chance of a regen being born at each age (the chances don't need to add up to 100) and the CA cap at that age: the youngster doesn't start with CA above the cap. Drag the bar to change the chance; zero removes the age.",
        ),
        "younger_from_age" => (
            "\"Regens mais novos\" (ideia do petermf23): os que sairiam com esta idade ou mais nascem mais novos. Funciona junto com \"anos a menos\".",
            "\"Younger regens\" (petermf23's idea): those who would come out at this age or older are born younger. Works with \"years less\".",
        ),
        "younger_by" => (
            "Quantos anos a menos para os regens de \"a partir de\" para cima. 0 desliga. Mais anos a menos = mais jovens de 16 a 20 e menos veteranos recém-gerados.",
            "How many years younger for regens from \"from age\" up. 0 turns it off. More years = more 16-to-20 youngsters and fewer newly generated veterans.",
        ),
        "ages_out_of_game" => (
            "As nações fora do jogo com poucas pessoas no banco sorteiam idades de um jeito próprio (quase sempre 23 a 27). \"A tabela também\" faz essas nações usarem a sua tabela. Só muda nações cuja liga você não escolheu.",
            "Nations not in the game with few people in the database draw ages their own way (mostly 23 to 27). \"The table too\" makes those nations use your table. It only changes nations whose league you didn't choose.",
        ),
        "ca_cap_skip" => (
            "Chance (%) de um jovem ignorar o teto do CA da idade dele: alguns nascem já prontos, como no jogo original (cerca de 1 em 200). Mais alto = mais jovens prontos cedo.",
            "Chance (%) that a youngster ignores the CA cap of his age: a few are born ready, as in the original game (about 1 in 200). Higher = more youngsters ready early.",
        ),
        "contest_max_age" => (
            "Até que idade o jovem vai à disputa dos clubes no jogo original (os mais velhos começam sem clube). Sem efeito com \"Sem clube só sem vaga\", que manda todos à disputa.",
            "Up to what age a youngster goes to the clubs' contest in the original game (older ones start without a club). No effect with \"Clubless only without room\", which sends everyone.",
        ),
        "unattached" => (
            "Sem clube só sem vaga: todo jovem de nação no jogo vai à disputa e só fica sem clube se nenhum clube do país tiver vaga. Original: só os de até 18 anos vão, e 1 em 4 é descartado (muitos começam sem clube e se perdem). Recomendado: menos talentos desperdiçados.",
            "Clubless only without room: every youngster from a nation in the game goes to the contest and is only left without a club if no club in his country has room. Original: only those up to 18 go, and 1 in 4 is dropped (many start clubless and get lost). Recommended: less wasted talent.",
        ),
        "retire_idle_players" => (
            "O jogador sem clube que ninguém quer se aposenta mais cedo, e a ficha dele vira um regen semanas depois. Resultado: mais jovens e menos veteranos parados no mercado. Num pacote que barra aposentadorias (o RetirePatchV4 do NickPatcher), quase não age.",
            "A clubless player nobody wants retires sooner, and his record becomes a regen a few weeks later. Result: more youngsters and fewer idle veterans on the market. In a pack that blocks retirements (NickPatcher's RetirePatchV4) it barely acts.",
        ),
        "idle_days" => (
            "Quantos dias sem clube antes de a regra dos inativos valer. Menos dias = aposentadorias mais rápidas.",
            "How many days without a club before the idle rule applies. Fewer days = faster retirements.",
        ),
        "idle_min_age" => (
            "A partir de que idade a regra dos inativos vale. Abaixo dela, fica a regra original do jogo.",
            "From what age the idle rule applies. Below it, the game's original rule stays.",
        ),
        "idle_rep" => (
            "A reputação até a qual o inativo se aposenta: quem tem reputação menor sai quase sempre; quem tem mais, raramente. Mais alto = mais aposentadorias.",
            "The reputation up to which an idle player retires: lower-reputation players almost always go; higher ones rarely. Higher = more retirements.",
        ),
        "idle_rep_per_year" => (
            "Quanto a reputação-limite sobe por ano acima da idade mínima: os mais velhos se aposentam mais fácil.",
            "How much the reputation threshold rises per year above the minimum age: older players retire more easily.",
        ),
        "protect_young_unemployed" => (
            "Abaixo desta idade, nenhum jogador sem clube se aposenta pela regra dos inativos. 0 = sem proteção (o padrão: o jogo já dá 365 dias ao jovem sem clube). Proteger os jovens diminui a reciclagem em regens.",
            "Below this age no clubless player retires by the idle rule. 0 = no protection (the default: the game already gives a clubless youngster 365 days). Protecting the young reduces recycling into regens.",
        ),
        "rule" => (
            "Sorteio proporcional: o jovem vai para um clube do tamanho dele, sorteado pelo nível, instalações, comissão e vaga, com no máximo 2 jovens por clube por ano. Original: os grandes recolhem quase todos (até 14 numa temporada). Recomendado para carreiras equilibradas.",
            "Proportional draw: the youngster goes to a club his size, drawn by level, facilities, staff and room, with at most 2 youngsters per club a year. Original: big clubs collect almost all of them (up to 14 in a season). Recommended for balanced careers.",
        ),
        "target_k" => (
            "Que reputação de clube combina com o jovem: o alvo é este valor vezes o PA (50 × PA 150 = clube de 7500). Mais alto = jovens bons indo para clubes maiores.",
            "What club reputation suits the youngster: the target is this times the PA (50 × PA 150 = a 7500 club). Higher = good youngsters going to bigger clubs.",
        ),
        "target_ca" => (
            "Quanto o alvo olha o CA (o que o jovem já sabe) em vez do PA (o que pode vir a ser). 0 = só o PA; 1 = só o CA.",
            "How much the target looks at the CA (what the youngster can do now) instead of the PA (what he may become). 0 = PA only; 1 = CA only.",
        ),
        "sigma" => (
            "Quanto um clube acima do alvo ainda tem chance. Maior = os jovens se espalham por clubes de níveis mais variados; menor = vão para clubes bem do tamanho deles.",
            "How much a club above the target still has a chance. Larger = youngsters spread over clubs of more varied levels; smaller = they go to clubs right their size.",
        ),
        "sigma_below" => (
            "O mesmo, para os clubes abaixo do alvo (0 = igual à largura de cima). Mantido menor porque as centenas de clubes pequenos de um país, somadas, levariam os jovens bons.",
            "The same for clubs below the target (0 = the same as above). Kept smaller because the hundreds of small clubs in a country, added up, would take the good youngsters.",
        ),
        "level_floor" => (
            "A chance mínima de um clube longe do alvo (0 a 1). Acima de 0, um clube pequeno ainda pode, raramente, receber um craque.",
            "The minimum chance of a club far from the target (0 to 1). Above 0, a small club can still, rarely, get a star.",
        ),
        "facilities_base" => (
            "Quanto as instalações de base (Training) pesam: o peso é esta base mais o valor por ponto. Clubes com boa base atraem mais jovens.",
            "How much youth facilities (Training) weigh: the weight is this base plus the per-point value. Clubs with good facilities attract more youngsters.",
        ),
        "facilities_k" => (
            "Quanto cada ponto de Training soma ao peso (com 0,4 e 0,06: Training 1 = 0,46; 10 = 1,00; 20 = 1,60).",
            "How much each Training point adds to the weight (with 0.4 and 0.06: Training 1 = 0.46; 10 = 1.00; 20 = 1.60).",
        ),
        "staff_min" => (
            "O peso do clube cuja comissão trabalha pior com jovens (\"Working with Youngsters\" 1).",
            "The weight of the club whose staff is worst with youngsters (\"Working with Youngsters\" 1).",
        ),
        "staff_max" => (
            "O peso do clube cuja comissão trabalha melhor com jovens (\"Working with Youngsters\" 20). Deve ser maior que o do pior.",
            "The weight of the club whose staff is best with youngsters (\"Working with Youngsters\" 20). It should be above the worst's.",
        ),
        "squad_limit" => (
            "Clube com este número de jogadores ou mais não recebe jovem. Mais baixo = os jovens vão para clubes com elenco menor.",
            "A club with this many players or more gets no youngster. Lower = youngsters go to clubs with smaller squads.",
        ),
        "squad_exp" => (
            "O quanto a vaga no elenco pesa: maior = clubes com elenco curto têm muito mais chance que os de elenco quase cheio.",
            "How much squad room weighs: larger = clubs with short squads get much more chance than nearly full ones.",
        ),
        "season_cap" => (
            "Quantos jovens um clube recebe no máximo a cada 365 dias pela disputa (0 = sem teto). É o que impede um clube de recolher dezenas de regens. Com 3, os elencos das ligas jogadas ficam um pouco mais cheios.",
            "How many youngsters a club gets at most every 365 days from the contest (0 = no cap). It's what stops a club collecting dozens of regens. With 3, the squads of the leagues you play stay a bit fuller.",
        ),
        "surprise" => (
            "Percentagem dos jovens sorteados sem olhar o nível do clube, só entre clubes das competições: o craque que surge num clube pequeno, ou o comum num grande. 0 = nunca.",
            "Percentage of youngsters drawn without looking at the club's level, only among competition clubs: the star who turns up at a small club, or the ordinary one at a big club. 0 = never.",
        ),
        "outside_competitions" => (
            "O peso dos clubes cuja divisão não roda na sua carreira (nem como liga de fundo). 1 = iguais aos outros; 0,25 = um quarto da chance; 0 = nunca (sem clube de competição com vaga, o jovem fica sem clube). Valores baixos mantêm cheios os elencos das ligas que você joga; com poucas ligas carregadas, 1 pode servir melhor.",
            "The weight of clubs whose division doesn't run in your career (not even as a background league). 1 = the same as the others; 0.25 = a quarter of the chance; 0 = never (with no competition club with room, the youngster stays clubless). Low values keep the squads of the leagues you play fuller; with few leagues loaded, 1 may suit better.",
        ),
        "temperature" => (
            "Acentua (maior que 1) ou suaviza (menor que 1) as diferenças de peso entre os clubes. 1 = o peso como calculado.",
            "Sharpens (above 1) or softens (below 1) the weight differences between clubs. 1 = the weight as calculated.",
        ),
        "regen_origins" => (
            "Grava no regen_origins.csv, ao lado do exe, cada regen que nasce: o nome, a idade, a posição, o PA, o CA e o clube que ganhou a disputa (ou \"sem clube\"), e de quem ele herdou: a vaga (o id e a personalidade) e a ficha de jogador (o PA, as posições e os atributos), com o nome, a idade, o CA e o PA do dono no momento da aposentadoria. Cada linha leva a data do jogo, a data do computador e a carreira (o save). Assim você acha \"o regen do Ronaldo\". O jogo não guarda isso em lugar nenhum: só os regens criados com esta opção ligada ficam registrados.",
            "Writes to regen_origins.csv, next to the exe, every regen born: his name, age, position, PA, CA and the club that won the contest (or \"no club\"), and whom he inherited from: the slot (the id and personality) and the player record (the PA, positions and attributes), with the owner's name, age, CA and PA at retirement. Each line has the game date, the computer's date and the career (the save). So you can find \"Ronaldo's regen\". The game keeps this nowhere: only regens created with this option on are recorded.",
        ),
        "log" => (
            "Grava o tweaks.log ao lado do exe: o que cada ajuste fez, cada jovem e o clube, as caixas de erro e, todo julho, o retrato da população. Para testar e para mandar quando algo der errado. Desligado no uso normal.",
            "Writes tweaks.log next to the exe: what each tweak did, each youngster and his club, error boxes and, every July, a picture of the population. For testing and to send when something goes wrong. Off in normal use.",
        ),
        _ => ("", ""),
    }
}
