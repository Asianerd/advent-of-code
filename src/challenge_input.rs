pub fn input(day: i32) -> String {
    let n: Vec<&str> = vec![
"5228833336355848549915459366737982598312959583817455621545976784792489468198365998232722734876612332352376192813552949814275947575774339529811976644361517795586998319242241614813622734255797569571577699238592667287428166398221572885869416419682687759743978434571821267146514338394624525648338739929479912368172669885577319718389278168766844487948761697438722556857882433224393723131298876252626643517236883999115665656935521675772866516185899317132494716723615493476397115627687887665194781746377341468995954554518252916859227397693885254329628812355612487594445522395853551734567498838382248616137969637971369615443599973588326388792893969924855316437952313492551671545714262784738343517166544197194547173515155927244175447296474282154114951181648317875827525814453758846194548872789943372281952995222779173812444186491115426476188672253249744478946863317915136832199132868917891243591195719354721129116229164688256853628339233919671468781913167415624214152793864585332944468428849171876873433621524242289488135675313544498245498637424139153782925723745249728743885493877792648576673196889949568317234125863369187953788611841388353999875519172896329524346527265231767868839696693328273381772726782949166112932954356923757485139367298699922984925977724972944277991686823219295939734313874834861796179591659174726432357533113896212781566659154939419866797488347448551719481632572231632463575591599696388223344219228325134233238538854289437756331848887242423387542214691157226725179683638967415678697625138177633444765126223885478348951332634398291612134852858683942466178329922655822225426534359191696177633167962839847985826676955417426617126288255366123169174674348417932158291334646767637764323226842771523598562429399935789788215958367362467652444854123951972118358417629679454978687337137675495295768451719631999398617828287671937584998697959425845883145736323818225129311845997214987663433375689621746665629187252511643969315283316269222835744532431378945137649959158495714472963839397214332815241141327714672141875129895",
"790	99	345	1080	32	143	1085	984	553	98	123	97	197	886	125	947
302	463	59	58	55	87	508	54	472	63	469	419	424	331	337	72
899	962	77	1127	62	530	78	880	129	1014	93	148	239	288	357	424
2417	2755	254	3886	5336	3655	5798	3273	5016	178	270	6511	223	5391	1342	2377
68	3002	3307	166	275	1989	1611	364	157	144	3771	1267	3188	3149	156	3454
1088	1261	21	1063	1173	278	1164	207	237	1230	1185	431	232	660	195	1246
49	1100	136	1491	647	1486	112	1278	53	1564	1147	1068	809	1638	138	117
158	3216	1972	2646	3181	785	2937	365	611	1977	1199	2972	201	2432	186	160
244	86	61	38	58	71	243	52	245	264	209	265	308	80	126	129
1317	792	74	111	1721	252	1082	1881	1349	94	891	1458	331	1691	89	1724
3798	202	3140	3468	1486	2073	3872	3190	3481	3760	2876	182	2772	226	3753	188
2272	6876	6759	218	272	4095	4712	6244	4889	2037	234	223	6858	3499	2358	439
792	230	886	824	762	895	99	799	94	110	747	635	91	406	89	157
2074	237	1668	1961	170	2292	2079	1371	1909	221	2039	1022	193	2195	1395	2123
8447	203	1806	6777	278	2850	1232	6369	398	235	212	992	7520	7304	7852	520
3928	107	3406	123	2111	2749	223	125	134	146	3875	1357	508	1534	4002	4417",
"312051",
"vxjtwn vjnxtw sxibvv mmws wjvtxn icawnd rprh
fhaa qwy vqbq gsswej lxr yzl wakcige mwjrl
bhnlow huqa gtbjc gvj wrkyr jgvmhj bgs umo ikbpdto
drczdf bglmf gsx flcf ojpj kzrwrho owbkl dgrnv bggjevc
ndncqdl lncaugj mfa lncaugj skt pkssyen rsb npjzf
kdd itdyhe pvljizn cgi
jgy pyhuq eecb phwkyl oeftyu pyhuq hecxgti tpadffm jgy
zvc qdk mlmyj kybbh lgbb fvfzcer frmaxa yzgw podt dbycoii afj
zfr msn mns leqem frz
golnm ltizhd dvwv xrizqhd omegnez nan yqajse lgef
gbej rvek aehiz bgje
yej cphl jtp swe axhljo ddwk obwsq mnewiwu klddd
ipiev henods rpn qfpg gjfdgs zcpt sswab eosdhn teeil
gzje ydu oiu jzge udy sqjeoo olxej
mgn gox tcifta vzc lxry gox gox mvila qdl jipjnw dvu
hxk xhk unhdmdz yomze povrt nbww bxu qqsqc rvuk tgffy twddm
fyx fyx nzkm fyx
ymnoc zogudq yncom tqrob sidvy dfuu ccjpiej yidvs
bxebny akknwxw jeyxqvj syl cedps akknwxw akknwxw zpvnf kuoon pnkejn wqjgc
kcebrkj zmuf ueewxhi mgyepbr nleviqc dez
argavx fqguii gebohvw klnrq rkqnl goevhbw
ywqi abwi eswph nlplbl pswhe lnqx fpgk lllnpb
abpb mpkw ampey yapme xnuyj
tmuaq asd bhbs sqmbsnw wsbnqsm ydwdncn rpa vrllkh
dnltf cck djy ydj
wywwl scezo clowuz dkgqaj dohyzcp
diimshr vlmsnlj whqb dkicau ckdaiu terp kgcii npqc vvzrqzv nol
wfpxe sqf tbb ruqpcq zfgb
kajykuz tsxgtys vuz kglmgg ihnnn plyjxj rcrvo mij plyjxj jqiur
pxs hmet dwgvd mvhkvn cjxg yvid vmhnkv kwxz rfemsua wdgvd okixk
lzwxas ddtyeh ivyama crrhxdt hedytd jfw
vjfv oyd fvjv kfwlj mradbx mckseee xradmb
llga yytxyvj lstspek lstspek lstspek
fabgf wgop fabgf bvsfoaw
grqnbvo tntomdw hizg tmotdwn
mau ufkw cxfi rhehj ebe xyv rhehj acxngo arl qtl rhehj
kbkto stqjtm tpcwshj saerkrt pffj dthp pfjf axc gwmmfdw glnqtdy xmskw
veff zqm hzhxap lgwnwq twsdk mqz xbbarbv cdx fhnwt qjcji bbvbrxa
fjw eds hofskl nkbsv des hvx xyn
qzort qzort qesz rtq oonk vwzlw wapoj ifr cta
pja hvy nhjg paj smtfe fmtse
xvi tcjj xvkjtab nqftt aumijl xkd cmilegf hvsmodx uuo igmcelf mslkq
mdhezgv lelzy kzfvsqu hvmvaxw pxiqjc hvmvaxw kzfvsqu
hsicsav csshrhx znojm eapi lhmzq bbwnz seao gfk azk
pup xtgjyzy wqt ijeektl
ktwh qdegzs btj pfwzzho
xdkmdm izqtjrr iqbke vtp
fmrbpdr zpccv tmtwx tmtwx tmtwx bys
ehphfgq idd ehphfgq ehphfgq uphe hvrc jcscne nbnslqy
xzqucgj fcih fljk barz lvln hcfi azrb
cmfmclv mfgvifw rnxgn jpg bsnq wnduzj ymsdx smdxy pqomf
rlqsm qrsml emts qsmcowv scmvwqo
tshzkpa zwtpda ftsiwo nil tpawdz kjpa ptzashk
mnep sfc swjawtd vnwud gyulluw zpa kmwyvln evd btnmoi dnwe
jwq scepq redoxmw rbdzsa wlkzso kxpm bttg vxuc moxwdre ijtdd rzsabd
wpvo dsjox amuwjm pls lgwksva ctakgpl rmsjj lzwwpr zzm udg
bji obbn tmwyc afpmkxr glvrd kahhgpq rna qkxyntp vmd mloshc
ymq rtnr nxjzm pqiddrn qmy vgxw ull
mmzk ikge zhtzhs xyo qwe lll gjjm icetq qgrr mzwqa knec
kxomfck idlh xrbowo nyetbnl qskh xuwkkxe upmmmf zhvuyp
srcwyhl czgr xmhuws jueyh xcuib xhsuwm bxuic
crkueh beyxopz xpyozbe dxgadw qktmce rjropjg
lbktun imdpcp fkssp fhcpt fehho jqdnt aoewa
jmun pynzjo trs ijwcc pelf oft pcsqdxg zvql
mneaaq vjrg jidlrzz phd mvxpivd ldkhu
sao xqw nrukn gatkz quscpsx vmz oscoeb
goi wzxhb rrk aylqqcd mlcbvvf ororn heptid kdu byevr
qsj lsbieef deez vzwdx hez iwd
lmgfb keqt mqbsuis ogrr errbi xiqe xsszacp
ato hmk zfjaj kmh plxup cida dqd pfwh nkbxvpr buajw pxkrvnb
cli bdwu vrwott vowtrt grle
zisgks ciuaqr zvk tcb kvz ugmtv
oegrojm wofpwp gnaocx rweyull ellhwow dtefylf dqsz oiw varr bcirpf oxusz
oydkmib oydkmib yefts gbl gbl
sruwjk pgkrp kea gppkr zdcky cfljh
obpxbax jhpcrj slcsa lgd fborz vvpaus wsrpsws ifijuzo
rixz jwh uhdaf hoacv hdfua
kntk qprmfow kntk tbmcjx
vnqe ooyxtb ixl hdmnpn orpz ykspl xromvj kowtq wmho gquos
ynk xjjqw sut lmtub bmtlu zdo dztlk bpkuul smhpx rbczg
zals csdbe sbj dibicq kdfwwt
coyy pjddlfc lwvhyms ldjdcfp ryubz kfwst dqjrjja jtv jjjaqrd
jaexhms iqoiln ewgyr exmnrr fsr lgmyy fdofhn
pjgyn hfoz zbcnz nczbz
ovntivq vcey vdrkse giu ohyaxy ionyy fvpn yvwrgrv qta
yelpz htbk njgeyub tggh mdthzp fwyux rduqli twlhfp pnh gywif ttn
yxhsbil vplsmmx rgtq grsf lyibxhs hctnkfr awg lmloz jroy lpgb wga
kzytass szyksat tyskasz ehmhhu
jkus hwjv ymnnkk yffugg cvtnits gbl lywkn szihcn dbrbalf rxqpbqh
koyfcef wkom mwok qgjrytl
slmhry lcr slmhry lcr
mvoxbt cfkz purnsui xar ouhtc thbx
xcdifw kvvxyrj knac qmypw bou tmukqy eusgaoo bktiu
ablgnhb axumg bwpxnjp zqpc vtw ghhoxu zqpc znfpvl ghhoxu jlg ntdk
vmvc cdkhrx cvz rvxk mmcuo udpcayd lsmm gufduzt linj
mgyeqkv hqionh rgnqgz kkc qrgnzg egkmqyv topdp
koa dimwx gjxa atlfdy
uuez ueuz zeuu ezo daq
ofpaw bgomvmt mqa dexpy mbipd epyzcoa nuwrh vwly xppz qkjrleo rwhnu
wok grxk lchvtg plrzr lwaax cfeu ijapws dmkdwc cfeu
zkd hysxxip hlydw wicsvy gbwoaw dapre ktjn dzg uri
blzh hblz qgmjceg fyf
vkhpn xnc ogva pjrh cxn hkpnv
aja cldzta tdcazl lorr fwmxxh knilf ges tdhp gnlo vihrl
ucpr peair nlbmc msfg
trv ppq bmo xqd vbui yegsr xqxawu fvuz aclhspo wnan
loiq fvg kare rmgq hir rzo ossd ziw renh ygtkjys vda
xmans kio alexs ujekfl vvf ddghn
fcxvsf bjuytet zrzsobo uhn mlfzhlq bjefs
zys htlqvky plno pbcqfuf fjwc vshkxrl lonp lyzmy dqmui vyyc glad
tlc krhcter krhcter bolk tlc opryl
idcii dverl uswb wusb zgax zhbt gjsnlso yhs
cti npri rcbxjdw ollj nirp ghfvxzh
blyhug aflnrrz zudyw ccnstq cyoju jxtqoj ntuknjq gunjiwy ycuoj igac cqctns
bul yehpnw jifjrhc ifetu ufrodp hqzpeqf hdvpc qtvgxg ibb wcxsitx xztshb
xzct scetn eoaufyo jtudgkx xrpgxip lpubtq juezstc nuc hokswh obkf ipbu
nfq lwpmn qltal xnphsqs zlrgf iewtrtd mqzsob duokpy kfbqs icg
vil zjz xkqrvni uay ystq
terrrnt lnfg clm lbs ptpiy ybcuup ayzjm pqugx lmc yppit mbf
dtajh vqivg vnblt fmn qxkw stiwna pclrrr fro khu wbslnqp tjyosu
uqlehn tjuiy obt uedct bbwiq uxndqn
hiqfovy xiimca zwne ivunvjk cmctzi mxnnrx dclib xzaoq ieztkg
shpr xuorihj chuwq poadbo mhtvex gymsp iltgl sypjfua fmyh sgiv
alv nxjt txnj bhact
vjvtrex obmrxk fgigs meixbc fggsi awi rxdjpeg
ypwo oicmbdw xbpeeyj uabzj cjvutvc oicmbdw immtmks
exijri hogl epr gzdqyur xiiejr pre ihzlgzu
rlh qfhx lrh qmvrx
kogq okhd mivmivb mivmivb okhd
taekt nhjaa znbaahn iaospxy jawwf
ytdvq ghtqwud jkiig mre kzmmjxu jba nwpykc
ktyzr aczd exgadhb uinrgac izazxky yyfe
yrifb qgc lsiuapg teyelxn ugezu
wdzkc ltx fkhncb hwrecp kfbchn sfcpc hjvq
rjdjyt ahwxh nvggsmx lmz oshd xbcik powse ahhxw yhiq gxmgsnv
qdr qjnam gag qjamn kooek mqnaj
pza gml opf ilfbblu kjp luilbfb rhfrzgp ixagj ofp
yphz runy dhull bozcsgk wfxekrd akgkbz urcphc
tfyxwol lhcl npik beug
szatel yfkve yfkve lzqhs
yjzqon pcjibu bdncmcl kczuymm pbmg nyn
rerqvs aoxucwi pmstl sstawu joqu abvcchg mvgjn mslpt vhmfkr utusuh
gqbec jjpqdh yeaiavi nledfi jhzwc vyxjpf momnm vnknjs nvgjzik ipm
psirt rispt lrkgma irtsp
jbbaph xvunete gsvnr mjd ifxhpry cpsx hmuokkx vhcm yth shrrl zbhd
gfa bcmlxtf sqyanrp cugg qxfvftz pbl ujsgc jajxltm gugc oil
xjuhyg aht vmyvzhh oby oyb ybo xbybgmx
atfk qjudfzz mky tfy
nxk yzy jqgg qxgjt bevvvv efi xcbw bohc zaqlqjq
hdc qpnx ygmtqw acvoa udboxw dhc klh mwgpk xfpuri
cycgbkq skwhyf skwhyf veaqss skwhyf
jnezf jowjt vsdu uck scgxd fvopomz vfajslp
djvi epgkyqn apzd cpm owm kpwih fsr adlhqu jicp pmc
erxlmhj wqxvofi ugj ttrmtsb
omku vmrgoy tdicbje ewml dfnwbap
gpih pyt ptsmzc gmdbu rqxkqmz objm nurxjz oozbere ztxug koth
jpnl jpnl dmeh qed
intdwv ksgw qwlzhq zpd lrl mwjl dozrjwq aujbet bsnf vhqyg
eqs uot qyz xor aem kmrh mrhk jqx tsbrf
irytjab mdzm qbb kkjt gofiwo xgbovg kyeyxqn tcks tljhx
zgejy qodgah nqavvx xnigdvt
eqve bizrxq lkhz yzwxgt nwe zfe sxypkz xnssept
bxqn lkfg yfxbszo sphwifz wnj crhbq dvokzw
vzn afatwye ogzvnu vnz rfjba xtugnj kpbgly ocsjd
xrc cxr rahv yvhk khyv bed ctgbuq cmqwpqa jlbg hpj vmesvw
jbshkya dgqw lfl mzcch jxsg czcmh ifruvlw ufwrlvi xcczlol cqqchmr
rbk mhn tnmqdc sxnnn kvoa mhn sxnnn mgemob ieiyajs
cqi ghxg ghxg ghxg
uqwdxn qli gdtkngp gnptdgk udxqwn
dmcczr dnjaqc qwdta rhrbi hkdwe qdjcan peic iulaz xns
tcmppb nzq ecy sitdud nft ecy afrbf wvnc vmfpzx tcmppb cgb
plitv efnpq mjqav nrxxo izg lpitv rwbzdo rdbzwo
day dntga adtng agndt hhvtd
yrg iudsh gyr ryg
qttyeew tco flq bszw jkzftc wdh efcwnp mja rfmju
moch prkze uslzyv plhjuy kxczyq qlmm hgq
xtg ypz izy ixg bvs xlqgj xcy sepza abiylsg
wxvsxn bqag jnlzgxq ikxwa dfd plqxl xlgqnjz nuqvoyb emhodso gaqb
bzjdsm xmxkj fhuqn gauyw ntl kjxmx zcxdr vrds
ofjcc uxyzlk ofjcc ofjcc
zwosex kkvwobl cpudsmb kes zklf bayuojr otqnyr udbbs
iqpvzh ybds piovrh oivprh voprih pov sfl
upns cpeelht xboyk itb hsxdmt dnwgfbw upns fygf kwdpxzm mli dyy
djwutl sikh shki ikhs gecd jqkon trqyw
prbbdf vdp bvvfjcg ydqb muxygg
vhpurzn psemqe xwqfk hrvonxu nxkxacq
xicmhss tnpja qiad woipfy uvadcq usljh hzgs jntvfv wzikk
mmupc twntp upcmm pumcm
qnisuzy lppnfd uiqr eyqbain uxlp eyrfwjo olgkrps sbikam zin vckr
nmokl skfni jcdfot njzqeaj nqzjjea
slmaxx offfzqp wudicrf nfn rwfcdui cwirufd
paffi murnjd oyj lbtjdqe babuas dtqh qkt stapzl yrqlp
eedc rig zmnfmn edec ecde
bcfdf edovdj lacx nzvze sordvxj ybs ujh zvvvp rzstejg ueosuq
xrrfsd okuvem znzlvmb jwzcb bfg bmuxbc qzwfry
pqgxybd cvgra acgn ocd ancg fvfcx fbb bfb zfzv
tmmv mpywyg fwl bnvcv lcnv flw
xxnfbro papc ianru beuzx apcp rnt
wuyhycj nrnc cka ebg rncn rvo wcyhjuy
thh cmoog hwf imqfp okzpxd
rzxiqt rtaiy ytria tyria
cjkmro myif myif xyirn aqxlol wlhwibi dhzsen pzwgm bfbz bufjs qwffg
mxhiui umiihx zomyll vfieccs
yyntf rjk iivgj mwh rjk
dsshx wsmaxhc xcwuelh rdsgtr wsmaxhc rgtsfj
rdh nwlxiwu xsjzbpr bsgps
ufyo vqtzkg kpeohu mxzt fyuo gawgaq youf
hzbhut bxsnjwb zuhhbt zhhtbu
pdz sgntypg ragev hrrji goitft yphnebs xjzoo sqf jsuzijq dsocb hcxg
pptsq woomypc woomypc woomypc
axcg wfbnpql ejqb cmnn nncm csvlc wraludb pkmp whtht tfpicer
moom oomm ommo vfqeii
xvrgpp rofl yxyrkb oage nypzau pwfnkn jxnhkw cyxsi clzb adwpuh
mfbz vdtt muzhm wvwwfl ttdv
cpqgvbu byc pgfrlkr aftl tqm zcqxi juu gnf ppovxh huoa
konpcp lzordid jqng lwxs nqgj gghkxmf kyn ngqj
iorhccj xfygc cnfr tysqc xpcyf vmjpitf nut zmrk mgbrtb tcblxwf dkadwrm
kov jtmp xoatesx qxkilp rmggpfx ltpxzwf vko reqms mqq nps
hjigmk fyqy wpuwe mwmso thsimfs okcmeyh mzqkez duzaq vzhyrm uyvpkox cwivpls
ukoerf korufe zhs ntwfz hugem vriyk enfaib hrrcdgf zllsk vkiyr
shkx khxs wntpjv qdevaw noqyht nwpvjt egh hgok mukdjfi law bzbvjz
dquk kczxsq tdu trnkjs wqtdc ybvcb
hlrotxn cumcjkm qwufgle ylm nejh hnje pvaigrx myl sfvsd
szmvisn aywic vsnimsz iufmybr
zjozr zojzr qmn ffrggdh wam dafvok
nxkvlhr posmf posmf posmf zhlzb
ywis kpqpyb qae zqxpuz pcj hbsfz ejlwa lajew znuom
qxsl ussivur dstd avojo
yoeagao egpaqm ymzf kkauy ivm illir wsvchne skmamvn nqxc
cldo ixzzy vhk nra zhypgab
qjdd ecxud tbuqq mpotbdk tjdpczn knncm tyy
rbfc fhhjf innia tsjbbbv fmtcuup rapvhqz ebpzt whdbms gvjoy lykl fquvcby
bihhfwi lhal udxz uwjwp dmb
fekxamy uophet yzvv rqj zawlp ldrv mdymkzy taauf
rcwxvmh edueui ltdyo xfghz dgjig senm ifj
qcu fii axmgijj ifi oixjfsg jxagijm
sdtyr rbdh yvnvq czzuig wro
lot xkto cmpiena nht ozcg aotcw xiegl cyaouj und lsclep cexn
pgihljk cmgmv sajhi zfvbqij ogwoc ajsih zmppe
jexwkdp dwpexjk mzjydfu bff rubgdb
yshfhx emkl hshxyf mkle
dxgti jdo tkwprv pbxbrqd oiz gsbdphd qotu utfdnq tzvve bqc
ovdf bshfxyl xspjpd vljdsm mgkd djlsvm mlsjdv
etyia eytai sfq qafj xzgp ewhsn snwhe lhqp
zjz mwh dorxm ges gexo rckwsa dltoq mmntha
hqkuj ypsjcxo dixbe rmvnhjh ovnr
edc iffaxc lolu xwrvpb gva vti vit
ceuxq xbwejr lzyvm rozseit cwe mham fivpwj qtv omaktaw
alzdrk tsxbuld mdbq pgbdtoo xwf vzalric nqe jqwlxsy cbtylu dtubxsl lqm
rqjmjcs exjpn kpilcgu ihcm lfadjm mlri hpd vqs cxqwqhu twxrtk
aeuvlcp aubvnw riedvz arypagp uuvg kliehx cokt ogh xsdw cdsyywv
ddwrgvp bscaq bbfv qrbutp
jpdg uey eyu uyarl zgbk qyhqq fdvlql zmwkp
kbt bkt lebhpfu smrzt xalw mmwa zmtzfry tkb
fcvcv oewfzu fvvcc mldww lwdmw
ejrltsu sqoyx wfvsdbp bfdspvw bfir jqhgrmt ofdmrjg ysq
jzwucwn erqjd eikq knpf cvk xvqnscy eei wvfjzmj xujq cqaim boev
jqhkmr ipjpj zwno ybu krhqjm zqfyyzb dyciy
ugwsw rpwteje qtvwi pwyhrzt hfcdfmc qbovk ibws
ffy kdder qjookz bfvmvvq yjzuaj fvxllfb pjyz jcezhye fimyydt qjookz qjookz
loupd nwsc yytvuqo ltcqxnf
iho ulvxguz fxbf iqu ofjtmvq xhs ybbusd kxg mebdnah ucttcf zufb
wzdb wumuhtv kef aavv buu xmjtlur faaccl wospwff bjasr eapfsi
jau qzszci ciu inagax
kui tqig fyovsp fvwol fyovsp mzth tcp nhoq
ajdla wtpj amylu jly tvq wjqef
ofqc einz bdze tows bdze eew
avwavzt aesrsjv lbmpi hllv chdbul ezelxn
imcprs cafb clfg rsjo iylqu nvk vkrq izezlnu vkqr tyhnv
rwj zboui reh buzio wuhpvid cpzy jrw tsbuiby hmxwqr ute
ixq luwbi uoiwsjh souz ysoubw uilbw ffwjvw ewzswoh hci zmfdaov whowzse
xrhgqf xrhgqf giyv giyv
toiqgzv gakg udgdlb wvi carrn pjyha muqclu
wuxthi srtszr ourab hpds bakvy fnk yefe yfee doowxcx
ijdc ujhvls xmy hwg yetsda qelbe nang xgywo wgh
bhm icq cnam dec enksf qfctz pwxoo bdf cnma xoowp rbls
jguzh fextz yax kesaunn waljo jltcza tfzxe dezs syi ebwxnks
flvq bzgd clvqw ahtyvu xbdyv wssxx boscm grgl nqcg
caskpli hqctxxc nwpyo wjlqfqf ebti dva
wmsz fzpd ikgeq gti ejftoou ezs cqef mybojc rgwz
mdaay yfppa pavl fuuvfkh hpod tpb dhxmia emdecm rbqcwbk vecyt
neha rmvl ndp vlrm dpn debghi vyhvc
bnp zkxdu iqqkesd abtlx hmjasdq kyvekr krt srrjyd oxmfev oot
dumlcqd ccm hyir oritdz madjjw
oakqrs advfmu verrc zkfdcn btndsp
onlkinl rdtm bscfxre bnu oumyrvv kgc zkj
tfxfsgm uwmic agswclg uofezgc
wpfdyjn kjlihk etbot fbu scm gwccce xgownte wig cuaijbo
bzbdk etozk qracb oftfoo lkooe
xupzw vmxwu sis wzpxu
gbz oqbgh jwgrru bzg kwmxcfc jrurgw
agyjnyc tuec imxlult omwiyjg fiwnoqx nhmnro qtg kbr agyjnyc
koiq llreotu elrtoul dubfvgy whq
htm lll crzppb gdjaae nsmxzh gnfvn obiuy ymspzbo iuboy
thm xlfrr pbxdfo mht tygi sapxgbv mmngzf dej
eus seu qmstw ues
yvfsw esut biblze kbjcpk estu xih qzki ezlbbi blzv
ohq ugc tqqeo jygvpwm vfs ldnfibp ycbpa sml rmime
kuuow gbg nzwdaf wiimtg lam oqmm
wsbwkdd hda nqk ticz mvt
gqbljyh zqugqs cjod sxwlqy qkfs wwvwvt dsojb qbhjlgy riusoa uosari
jkphfx dbt les jsvoij rnuw mxmmchu dol vto swn
qqxe vwvephr twdqlyg cvdu xjiych clooq vkwavl whvverp yuz vkwval
txtbudi tiutdbx wqhx tws utgbf amh hmf izsez ooz
egdube nhsxjs nxjshs xoy sjsxnh
egdziod diodegz ejxn zogedid uhhkr rnm cyvvuc uqbl
rbn pinwag sidwdwv jqdbe jlbemk blkeaqq ipfqbtn zkrbp
bdryz sbh wxvn mhot wemsfm oemkff
vxyn xvdwwo xhd vyca zxjaw vfkz xhg ofsphks dyq mmzzd
yjrqsjf iiesdh envwyx rmtbmiv ggzsg ukx bprfym qmyqc vag ymho hjtoh
fuxxrd wbweptd vkoffr wbweptd
gfwcez smetli yjyh pslpz qyokpsm qsy cxjymg wqfkf obuq awz
eqhm ceest kayf heqm
rdi dti vntcf ewkmpvf jjwoihc
sfq qlb xrm ocy vtnj zdznbal zvon stln zwnj wsgalvq vhphap
pya jay mgnyo pya xmapdn
hrwbj xhr gvwl ktq ktq gvwl
rzgqi hjwtthl kxhggbl wepc hgavj ctmqug
tzfwkc xeqfath iiuwq iiuwq dhwuvy
gibagy smq getjofc lum msq ulm xuxu bilrus ily
xlv ndrkch hdcknr nqltoze xvl
wmc vuzlrj mwc atp cvpx atv ujatz
hxpafgl ymjltv nvvpy ahycdk jhpdcks ettm lvqyw ertpivm dnezwxx usi kdhcay
vrh hqyomv mcq ilwjbkz yprjxad
ugv szfitxg zeluib pfj ijm zmiigxx gltxzz jzljhgh otskue
mxp bilj jlbi tce yfted zxsqas ftyed
ykasqv ehye kirmnl upmi dojwmw wzj ykasqv ifixn vreoypz
kerbgub nnroqk onkqnr gbebkur tjhl knjo ccsem yozvrcg
ygq evkoj wkn ffljhds scxeibh egsybeg mwvi vgjblj qda ywqpp
hocvpl ozgkxp xgmj ejzyxm
gernu kks lxe nxzv sypg xle goz
xoatis fjp wzlbo dzkonz jtutyj vdonj swro tqclemv xhomap ymeqkua vaxcw
mxcyjs ywyxndk wng vpftv nsuvu
jmiyyhh gwser shgcu jmyg cjzegc hmhe eopg kmkan
smdd dmds mgqhtkh qtamih haqmit skkcy
dnj rmggy rgymg uburbao rymgg
klcpjgq ons ajyv sqryt son pjlcgkq xlobdt
piw shonk tzi mcdumz noskh tebolw yaypn
ozm mvmjgtg nxj weommiq asnmhzq xjn uobztuo cqgjh utfb oydt ommiewq
qlwgsc vvpe xgft ahpjc zjtx iyof scwqlg dxgcokx ltrefj xyzq rwto
ggqdd dqgdg ggdqd kjkmmfp
htzjam fjbg iagc xls iagc iydtf ihxl boa iydtf
vhe nqj bwgdoi hhaoa qtulz
axvyja hpdkwee hnryj prou rgadv oubjdqg knjbc
caz xibj wqkzwe peioeya vmz hesy ftb
dudwcr gupj sjrtzc xsqbb hiet nujv bebcvsj eks uuzlcx gex
kywozi tfzuc mflssw hnxxxqt zzc tzfuc hkokuv mnjg lwkavjp lvpwjak xez
izgh zfv cingjt dkf cknite qox vfz zvf
ojpu dzk tehpgnt gntpteh
glxfxa uxq ajtles ahgzn ajlste zwgc mrpu adz wuunwhc zda
hdgdtn hnoyz aromkb qujfv yjgmn tbf atw
uyvsv oaopjv uyvemxk ldpp tthe iisjk txr hebmd yxevukm rkziao znt
ypdr mnwuzvw acpg kzwz ywbn wcrr umrnlbe lkult ljify azyhu mgqoo
abmpl omsd xmyl mxyl mgoq kracrf ufm ppwi zpggh
uxfdpv jnm vvc vchunhl ubv ktj mxolsxz
fcja eci edzrb nlvksaw lhf ycohh tfztt xso ceub tyv
rkwtp tcmmvv kufg cxui hdamg suuaej fgku cvjlv
oldbgy riadoyo djsi wca zxoeq pmemqap aijxa
nyy ruxcosx xisqoz yny jvzfpbe tlfdiaj ybd jifatdl zuzv
kxwdz qvrvx svllp ergmme
swjfuv eronk favcxfm acptbh pnbjn ciqcrlt rgvdnlt icgahb
ddza xxfn use obqka bfzwjp gmf bld fyvde mxdfdl
ame bmxbyf ame bmxbyf
rdgby pyfog dybrg gdryb lpztd
sntg impd uxgxai naoalb ntnk xgix
oadpmqj oso criln izih oos
ouzjq gtl ito xefqt phnv ouzjq hoyjjj
mlp rboq lpm roqb whvp
tghcw ggshevw dzsgj ggshevw kec ggshevw
kmwhb kfcb mbhkw gemz fdh
euve veue kplrq evue
hikfiw bcdktj hcnawja gjasvwc vcht igrzly rkxijxe ikfwhi dvmp
hvksis kafs ktcs sfyqzyt etctrgt vodwr wff tskc juobnm
dpcsodn ehwc pglywfl yhdp mdiyzx
ibog umftejh cfm pnxhna wqwx yabnk ygws dqw
dezz tqw qism rarfe fpmlab xvbau irwtfs wwmoyss yvn xetqp xtqep
pchqwk npsmd jefec qok uuc ucnpz rlkakn
kudh rjysb xrdbx bkbmjfo xrdbx
rogu ssdwsus voa ncw obkxsr
tflf hlevus scq rrbpat tau wxsq wxoblt
rzr lex kqdy whtj ffnys xlgkkff msjhy dimaq hrc wyde qkwf
ghtwd wernjpn tdgwh olrfvmr edq gxvp
rjirvf skhdgln aauit bipu mubjiwp kowz gyjfbjx cmgdqs
aftfpbv agajyy aqjll vsf twh robpys lebt eav yribup
sby ymkla sxkbfwl awmd nhb vlp
kizvjj ycjswr jkzjiv vuy jijzkv jcs
cwvch xzqfal tephz lqfzax cnkbdcr mql zflaxq
jjxzwl himpra ssjf bibfiui seeaq pzse
jogrn jogrn sqew jogrn oixgwr
khonpyw iiyxir vybhc ndnxxv kzlt ipmncn
okqkqu svbemi nfn ovd xgwy edd ujet nrrbv dde vdo
jobvf dus asvio vaosi sovia
knmz qbz nkmz zmkn
isbmopr unduey impobrs hea zswciev sopbmri duuj
ocs ntgnrdu kbvtzp cvyieu fiyn znmh lhrz ixtnzrj vktbpz lbpqx vzkpbt
muduhc sabc dlyoisz kuaz ogpyepw yuog ictiiqt
xjflsf nfklvml thfh uajnmby cichyj xxoqi lpime bxpyx
riahifn bohbgd obhdgb jni qzvkf ybp hjkkwq ytutd cakcsh smfdoe tuytd
iddku nccp zgtl yne ppzpqcx lwm
refpcz uqt uqt uqt
mtn czxkagb nmt caqacrg bcakxgz
itxjii uethxbj vpds bsqod diqax inv zrwt doepe
bfyaj nbvhg zmi buf
dtre dkwdr nrapm qtfth odvt bbcnae vxuk gqm enlg
ybt qcfozrk yzrh bfp euuozuz pzsdkxx mhi nbkzprb
vpuhqn gyx caint antci vfep incat kqdakdx
ddhi chgnjk ibg xbemitr mjtdph eovw
ngbtuvq qdttlsg dbqhhwk bkrqze qdttlsg qdttlsg
evn smvhi dgcmn xjo ascc ahbpj uvzc pwn tung
ksu thr omg onvsqzz rllakar ysfjtfj grxwyx oawix gpk suk
qvb iouav yhtndkd vuoia ouaiv
kud kofcip hcczrgc cvvxxlk rvyamwe duthdzr dftun
rgv ynw gph tmxwfup nwy
dnc trawj kwzbx trawj zvp
ogqxijy tbqtsg tbo vqinnlq jbvgl sfafh rve mcxqs ubh
qccr lpv puuvdyb tydaflf uxic
tlon tbfwkxg tlon tlon
iytiz qjlqaqw uixb lnt zwro uzgxqfi gklgnqs zwgoidw iifk wkwdo
tmvhxw tmvhxw tmvhxw fhiqpjy ejk kvysd
cmphg xjjz groiccd dvetuk xbwa zhm lyi ohhd neg bxaw yil
kdmzopy lxx bvhach goxmxu qbqvzcm qbbrhvb nrfom aixmio grpxz hbrqbbv lkucih
bnqn phqr uycuxc mopyyfh bbpesqm stgigq stggqi cwtjm asqhpl imvlxj lbmloo
pws iuvbvjr cwccm qbr srqnstz cjebq
bfh jobkcy gtbroe lpagq icmax jobyck fbh
ounqdo qrrr pwi alho rrqr beao rsioepe
vrccqge qvcgrce cbslkjs qnclw rvmjkw
aaxjns deupjs wtgxtp penad depbho tbrdt depbho qxg zhjxpgd
drqfo kbp jfa jaf
izn oczcitj cpae quvzqo iwwk jck idjdpm
ecort zgcvxx bvh vrprsf
fhubfvy ndcfjo kol hyufbfv hvpka
kpt zgajpc rjvsxa gayznjd
xeoixk peq kfu lqa mjnv mzvh bicl hlfk
wyt imdx lksy twy
xeptp ilxs qbsqzwn rsy slxi xtpep dsdkekl
rotvbt fuirp elos ciu nhx bxej trmtx ixn xbpc vrxtma
skcprn yns sao ghlq vftezvc aaryahy telt
fkaov gexa xijv yiksa xega dhgw okfva gxxs edkecag mqbqvrm nrzcqub
ljc jujxeof fdj gdzjzr mabbktu pmyrfv uspven zxry snt hrah
nhujhdr jdhrnuh midm bbavhpp cpjk zmpbasz eptrpou znq zqn
ywzfq wuu lfflon uuw rke qzwyf hjbms gakx
yqrq zsk jzn uuuzrml kzs lseupsg waynfh blech
gwyqej weyjqg uwuje uujwe
lxud rnwkc bgygkh csq rfvtos ystqp keb gkakodj uthcce eqxifl
elvj evj rfwo vvgkosh aarcgjs utsbh orwf jxcqvmh uowmktl qtgf
bqszre oxntty ombwiz mbiwzo
ccp iilcc tacf czk giwv erqi jgdfah wip xtrzhv wosvbyb
gymyw rwsxeg gvydr izyk spsonkg knospsg
djj tbr tbr tbr ice
yyzh zkykapw puydtik ysxc hjumhsd cuhhw dnnhida yyzh lnklymg
nhbcxsu ccrbbyw scbxunh ghxrkqh brcwcyb
latdaav sexa ipzuzjl ayusb etb fshh
giz akqd vjmabii arfuzgv efrww jxkvolg efrww vrnzgbx
jmcc vqy adkzj fqrkdo tjrczp ccmj cfponk rptzjc
jsviu sraw imsj fujm cdf xwqhl lhz ojejzuy trtqblg
ibz dulm muoq quom etvjzxn tuhrpp jfukac jqctqn qhgbae msgmcit ludm
zgx bpfa elhp rnyqtq wyceube nkeuxz
lzxfo vygpecv jszacku zfxlo
cpmv ysaaj xnp wbvqg hrsiuj venjxna yeqvwmk ftaga dcqxc jgapb rqdixp
xpbbe tyn hfdlu fto wrgzkou sxylv cqto wdv xqc pnu rapk
pkrxypl wnu oipq tzbhnc gpug tgzf ofjb
mvaz bwcv gll itgcye dessw szt gzimgeu bvmohh wbywyhc kzerxbr anjsive
lhvnrzs qkmjwy pnyciwp mgp jfdz ghvtf yusfzg upab
xbscukx aubulj snbcmc uscxkbx ddpucyg
hgv ollh yzpjmpy fcicyae vhg gvh
prd onyd iux oik xui
zipadig nvewx cir lbpcusx dljqy
ifyxzsc btmy lsu tmyb lus ldyzx
egmyxbe ieasvek dylmj qahtatr uyqgbk
mejjczw spj vaekp kdud
vwan mgenld mnlged vpfuil euoxlr rclkpi dfknyoa rhthij kcyxl qaxab crlpik
pqm eihogk iwml nuauxi ngilkoh jmu mbdi cqxz nblb rmuj zczdgp
pswbe mtzch wbeps fxtnc psa aioff pas
prwrpvz oadpqvz tgzrt giom pjyihh rxdir dmya xjolzxv
khdybe obqkjn kdq jkvmgwo enpat wyw qjbnko waid msest wwkoyts
yep liv ofmtpod imdd qyw
afnrx jgn gxarpb myltj ggrsajy mdaobjo vbtn vbtn zlziz eds
hqr kqu oub skoeqk icnfm cqvld aay bto
rga odaf exoosh pwevx zpbd plaa xoseoh
mbr gqu oxvchrt nqa larxmjx pfozej
ozuo ywubjbg xcua eblwqp nfdvw hmhen zkjfu gmhgp bsyi ktprtf
src vrysby srybvy znwjm hmypwdl gdmau pqe
cldr crhi lbaq fbuduyn hygbz uhida
qrxukq dygkp oaks soka oask
vpido ajgfq pwlv hezt fmg epwrxo rqvjke iovpd hhkjm
anxf ydl xnfa hqph olorp
exydcg onxjm psqlbv ehz boar hze qsblpv
mnzrvc ipj swg ijp sgw gdkntsd fzz grqwly
erpq qghpj fay gci uglm afy
jwbq hbxaub jpdilyt yvalrlk topl qup
eczonk ftcc paltirb owz tihhe dglxory wthvqcb qdnxm lirejh alyxsr
ooruaby gboyeu lkv arrz jcqyzl uxlfk fhmeony fcmh
wzr xjb pwmf okqj adwcedy lkidve uwekxf asbdzr biub
dikhur pxgh urdinjh wednf ulzdxs
iplf byt tyt qnnlba pzt bednml ljjtkvo tjovlkj uwms xat
htzk ltmfha xikeze atfmhl fchxhyz
lqala bqwgcul vetaa xuxjau zcb wtdmomu wfqmpq sief uyblyz ahv
aytvvo awm ojaaigg awm dbfaokz
abq npcyld fzbfku oia qss jkxldm wgtmki pasgxi dieix rpqnuac tecnfy
nmr qzfj qjfz lsz vnahex
djxoo jzlkh svy xige
tjlkkg glcuvmh fwzlhi ecun qlgulj hrfhyql qgdlf ofakqdf zokkvm gelxkq oowgs
upfpk gfstjlv lxc rjd nhj sbq jpzsz zsjzp
favd nzqfdid nekfjsf mtjndu
sgdqx uvpuefv vhwrgd aivav gsqxd jdhfoq
llaf cthbgy njrpw fqgkx jzf xqkgf lnrfrm gkxqf
wzdwlc wisst alw kyjeur sjsqfcr tta bijnyn whfyoxl
dtjr baxkj lmnyrlg nrmyllg
mtgky xmwf zdko nnocxye gytkm ygp hixk xwmf
maudjy okgjga uadjmy dzfrk omd
azz ajdcqkd bcafn zaz dcjaqdk gylyzo
xzvfbf fopmfxu mvftgr mfupoxf coyhof talcc vpkslo",
"2
1
2
-1
2
-3
1
-2
-7
-3
-1
-5
-2
-6
-1
-7
-11
-10
0
-5
-13
0
-5
-13
-19
-19
-6
-16
1
-18
-28
-8
-15
-15
-32
-25
-33
-7
-13
-20
-33
-33
-3
-10
-32
-9
-12
-35
2
-31
2
1
-6
1
-15
-51
-20
-28
-38
-3
-36
-29
-19
-48
-39
-56
1
-32
-14
-57
-20
-8
-59
-56
-30
-68
-62
-28
1
-62
-14
-61
-19
-24
-17
-70
-39
-6
-12
-69
-39
-63
-75
-38
-40
-62
-71
-65
-84
-59
-92
-21
-3
-2
-53
-42
-2
-64
-64
-23
-33
-40
-17
-21
-38
-17
-70
-78
-62
-95
-97
-115
-101
-31
-13
-121
0
-21
-69
-94
-70
-9
-122
-30
-38
-28
-14
-91
-28
-110
-114
-96
-104
-78
-30
-125
-41
-26
-94
-142
-74
-86
-12
-103
-126
-16
-118
-133
-72
-147
-149
-119
-91
-44
-145
-42
-97
-66
-38
-33
-6
-79
-5
-151
-145
-23
-118
-103
-20
-66
-18
-140
-99
-176
-143
-60
-3
-51
-65
-37
-178
-27
-1
-181
-115
-122
-162
-12
-140
-34
-146
-133
-115
-144
-152
-85
-173
-44
-29
-98
-159
-88
-173
-117
-172
-210
-41
-125
-181
-149
-54
-71
-57
-188
-160
-85
-85
-215
-108
-130
-150
-36
-225
-1
-207
-233
-171
-2
-176
-16
-83
-148
-200
-59
-108
-62
-118
-94
-31
-211
-89
-243
-7
-106
-36
-239
-116
-135
-98
-136
-159
-242
-233
-212
-29
-128
-234
-208
-153
-200
-122
-38
-159
-211
-264
-41
-74
-219
-274
-114
-116
-24
-243
-74
-198
-250
-35
-95
-146
-110
-179
-213
-279
-160
-34
-90
-282
-174
-17
-287
-146
-196
-140
-139
-238
-148
-22
-43
-50
-300
-104
-67
-243
-129
-264
-192
-8
-171
-263
-38
-147
-315
-65
-141
-49
-164
-104
-43
-203
-217
-320
-167
-21
-42
-180
1
-89
-67
-27
-319
-131
-240
-211
-283
-148
-341
-207
-97
-242
-183
-339
-182
-97
-157
-53
-346
-112
-282
-127
-226
-341
-196
-230
-362
-191
-180
-347
-223
-5
-278
-217
-50
-268
-288
-258
-300
-353
-296
-221
-303
-167
-378
-259
-17
-199
-140
-323
-352
-226
-380
-299
-88
-265
-315
-285
-139
-331
-146
-282
-331
-279
-347
-13
-26
-156
-299
-364
-155
-309
-280
-34
-362
-16
-193
-240
-194
-55
-315
-292
-121
-110
-334
-95
-341
-118
0
-282
-194
-128
-137
-161
-111
-46
-31
-375
-339
-131
-224
-62
-183
-50
-156
-338
-440
-186
-86
-54
-401
-197
-135
-304
-369
-31
-20
-62
-272
-321
-289
-205
-448
-40
-114
-393
-403
-244
-285
-19
-334
-45
-19
-422
-308
-271
-224
-252
-433
-198
-451
-353
-333
-310
-130
-396
-182
-414
-221
-425
-121
-311
2
-236
-24
-81
-21
-340
-393
-173
-360
-339
-282
-245
-330
-273
-448
-425
-398
-137
-434
-400
-439
-110
-295
-287
-321
-338
-288
-217
-444
-87
-237
-405
-406
-6
-126
-94
-2
-257
-62
-365
-499
-527
-409
-230
-275
-376
-270
-191
-252
-476
-340
-458
-465
-511
-232
-488
-445
-402
-299
-515
-164
-113
-397
-323
-339
-388
-146
-435
-292
-433
-86
-158
-459
-172
-107
-334
-411
-517
-459
-62
-186
-102
-198
-189
-549
-176
-216
-393
-475
-528
-551
-196
-319
-457
-404
-22
-158
-414
-15
-509
-307
-531
-389
-171
-353
-306
-281
-374
-348
-125
-461
-121
-192
-75
-95
-555
-113
-603
-361
-195
-516
-509
-23
-211
-271
-119
-401
-544
-557
-584
-294
-155
-92
-306
-351
-292
-202
-291
-334
-485
-280
-98
-374
-411
-572
-577
-350
-590
-312
-297
-200
-489
-141
-99
-589
-326
-420
-152
-365
-560
-548
-562
-456
-206
-175
-617
-454
-141
-621
-560
-503
-156
-498
-469
-433
-629
-459
-234
-454
-490
-72
-188
-395
-185
-21
-645
-47
-549
-225
-587
-174
-334
-358
-244
-592
-15
-109
-258
-639
-676
-550
-29
-255
-633
-469
-367
-190
-601
-667
-120
-170
-236
-16
-464
-194
-263
-648
-613
-642
-263
-111
-8
-142
-543
-670
-178
-293
-660
-14
-502
-279
-391
-74
-298
-286
-640
-658
-144
-648
2
-328
-418
-294
-461
-646
-612
-563
-250
-420
-318
-464
-478
-172
-47
-284
-293
-121
-642
-41
-406
-538
-100
-44
-505
-663
-437
-680
-181
-57
-619
-17
-434
-27
-509
-182
-635
-710
-514
-693
-678
-351
-129
-63
-556
-312
-359
-627
-44
-391
-138
-79
-607
-226
-637
-340
-508
-632
-124
-637
-575
-263
-392
-521
-760
-526
-213
-209
-501
-381
-76
-555
-321
-172
-685
-14
-135
-360
-799
-70
-85
-534
-100
-414
-519
-596
-714
-493
-136
-442
-379
-778
-555
-429
-245
-360
-399
-117
-410
-383
-406
-316
-633
-163
-328
-224
-421
-196
-801
-549
-156
-541
-245
-424
-324
-242
-834
-444
-513
-300
-86
-4
-564
-617
-631
-454
-22
-201
-819
-241
-621
-332
-563
-855
-436
-144
-403
-101
-527
-157
-778
-256
-378
-6
-860
-805
-307
-581
-595
-380
-83
-641
-566
-71
-809
-866
-856
-234
-402
-511
-382
-716
-331
-701
-279
-142
-90
-828
-415
-768
-25
-409
-158
-423
-161
-670
-349
-188
-590
-504
-599
-57
-849
-521
-275
-257
-883
-476
-602
-683
-393
-374
-695
-573
-52
-263
-446
-690
-708
-881
-218
-334
-149
-674
-419
-793
-764
-413
-230
-541
-705
-777
-440
-651
-387
-532
-290
-824
-659
-772
-511
-817
-384
-703
-309
-880
-518
-579
-38
-114
-902
-693
-496
-682
-744
-543
-929
-774
-699
-364
-542
-198
-170
-822
-681
-811
-769
-219
-824
-904
-161
-334
-736
-246
-846
-615
-597
-565
-61
-813
-460
-194
-882
-272
-416
-270
-79
-719
-256
-405
-939
-200
-776
-375
-798
-829
-894
-96
-41
-405
-684
-851
-588
-132
-127
-839
-123
-8
-916
-370
-486
-178
-761
-481
-532
-1015
-64
-13
-551
-295
-964
-879
-16
-304
-1022
-191
-734
-486
-337
-819
-701
-769
-687
-854
-800
-999
-317
-435
-97
-966
-887
-589
-793
-962
-1032
-595
-19
-119
-857
-520",
"vxjtwn vjnxtw sxibvv mmws wjvtxn icawnd rprh
fhaa qwy vqbq gsswej lxr yzl wakcige mwjrl
bhnlow huqa gtbjc gvj wrkyr jgvmhj bgs umo ikbpdto
drczdf bglmf gsx flcf ojpj kzrwrho owbkl dgrnv bggjevc
ndncqdl lncaugj mfa lncaugj skt pkssyen rsb npjzf
kdd itdyhe pvljizn cgi
jgy pyhuq eecb phwkyl oeftyu pyhuq hecxgti tpadffm jgy
zvc qdk mlmyj kybbh lgbb fvfzcer frmaxa yzgw podt dbycoii afj
zfr msn mns leqem frz
golnm ltizhd dvwv xrizqhd omegnez nan yqajse lgef
gbej rvek aehiz bgje
yej cphl jtp swe axhljo ddwk obwsq mnewiwu klddd
ipiev henods rpn qfpg gjfdgs zcpt sswab eosdhn teeil
gzje ydu oiu jzge udy sqjeoo olxej
mgn gox tcifta vzc lxry gox gox mvila qdl jipjnw dvu
hxk xhk unhdmdz yomze povrt nbww bxu qqsqc rvuk tgffy twddm
fyx fyx nzkm fyx
ymnoc zogudq yncom tqrob sidvy dfuu ccjpiej yidvs
bxebny akknwxw jeyxqvj syl cedps akknwxw akknwxw zpvnf kuoon pnkejn wqjgc
kcebrkj zmuf ueewxhi mgyepbr nleviqc dez
argavx fqguii gebohvw klnrq rkqnl goevhbw
ywqi abwi eswph nlplbl pswhe lnqx fpgk lllnpb
abpb mpkw ampey yapme xnuyj
tmuaq asd bhbs sqmbsnw wsbnqsm ydwdncn rpa vrllkh
dnltf cck djy ydj
wywwl scezo clowuz dkgqaj dohyzcp
diimshr vlmsnlj whqb dkicau ckdaiu terp kgcii npqc vvzrqzv nol
wfpxe sqf tbb ruqpcq zfgb
kajykuz tsxgtys vuz kglmgg ihnnn plyjxj rcrvo mij plyjxj jqiur
pxs hmet dwgvd mvhkvn cjxg yvid vmhnkv kwxz rfemsua wdgvd okixk
lzwxas ddtyeh ivyama crrhxdt hedytd jfw
vjfv oyd fvjv kfwlj mradbx mckseee xradmb
llga yytxyvj lstspek lstspek lstspek
fabgf wgop fabgf bvsfoaw
grqnbvo tntomdw hizg tmotdwn
mau ufkw cxfi rhehj ebe xyv rhehj acxngo arl qtl rhehj
kbkto stqjtm tpcwshj saerkrt pffj dthp pfjf axc gwmmfdw glnqtdy xmskw
veff zqm hzhxap lgwnwq twsdk mqz xbbarbv cdx fhnwt qjcji bbvbrxa
fjw eds hofskl nkbsv des hvx xyn
qzort qzort qesz rtq oonk vwzlw wapoj ifr cta
pja hvy nhjg paj smtfe fmtse
xvi tcjj xvkjtab nqftt aumijl xkd cmilegf hvsmodx uuo igmcelf mslkq
mdhezgv lelzy kzfvsqu hvmvaxw pxiqjc hvmvaxw kzfvsqu
hsicsav csshrhx znojm eapi lhmzq bbwnz seao gfk azk
pup xtgjyzy wqt ijeektl
ktwh qdegzs btj pfwzzho
xdkmdm izqtjrr iqbke vtp
fmrbpdr zpccv tmtwx tmtwx tmtwx bys
ehphfgq idd ehphfgq ehphfgq uphe hvrc jcscne nbnslqy
xzqucgj fcih fljk barz lvln hcfi azrb
cmfmclv mfgvifw rnxgn jpg bsnq wnduzj ymsdx smdxy pqomf
rlqsm qrsml emts qsmcowv scmvwqo
tshzkpa zwtpda ftsiwo nil tpawdz kjpa ptzashk
mnep sfc swjawtd vnwud gyulluw zpa kmwyvln evd btnmoi dnwe
jwq scepq redoxmw rbdzsa wlkzso kxpm bttg vxuc moxwdre ijtdd rzsabd
wpvo dsjox amuwjm pls lgwksva ctakgpl rmsjj lzwwpr zzm udg
bji obbn tmwyc afpmkxr glvrd kahhgpq rna qkxyntp vmd mloshc
ymq rtnr nxjzm pqiddrn qmy vgxw ull
mmzk ikge zhtzhs xyo qwe lll gjjm icetq qgrr mzwqa knec
kxomfck idlh xrbowo nyetbnl qskh xuwkkxe upmmmf zhvuyp
srcwyhl czgr xmhuws jueyh xcuib xhsuwm bxuic
crkueh beyxopz xpyozbe dxgadw qktmce rjropjg
lbktun imdpcp fkssp fhcpt fehho jqdnt aoewa
jmun pynzjo trs ijwcc pelf oft pcsqdxg zvql
mneaaq vjrg jidlrzz phd mvxpivd ldkhu
sao xqw nrukn gatkz quscpsx vmz oscoeb
goi wzxhb rrk aylqqcd mlcbvvf ororn heptid kdu byevr
qsj lsbieef deez vzwdx hez iwd
lmgfb keqt mqbsuis ogrr errbi xiqe xsszacp
ato hmk zfjaj kmh plxup cida dqd pfwh nkbxvpr buajw pxkrvnb
cli bdwu vrwott vowtrt grle
zisgks ciuaqr zvk tcb kvz ugmtv
oegrojm wofpwp gnaocx rweyull ellhwow dtefylf dqsz oiw varr bcirpf oxusz
oydkmib oydkmib yefts gbl gbl
sruwjk pgkrp kea gppkr zdcky cfljh
obpxbax jhpcrj slcsa lgd fborz vvpaus wsrpsws ifijuzo
rixz jwh uhdaf hoacv hdfua
kntk qprmfow kntk tbmcjx
vnqe ooyxtb ixl hdmnpn orpz ykspl xromvj kowtq wmho gquos
ynk xjjqw sut lmtub bmtlu zdo dztlk bpkuul smhpx rbczg
zals csdbe sbj dibicq kdfwwt
coyy pjddlfc lwvhyms ldjdcfp ryubz kfwst dqjrjja jtv jjjaqrd
jaexhms iqoiln ewgyr exmnrr fsr lgmyy fdofhn
pjgyn hfoz zbcnz nczbz
ovntivq vcey vdrkse giu ohyaxy ionyy fvpn yvwrgrv qta
yelpz htbk njgeyub tggh mdthzp fwyux rduqli twlhfp pnh gywif ttn
yxhsbil vplsmmx rgtq grsf lyibxhs hctnkfr awg lmloz jroy lpgb wga
kzytass szyksat tyskasz ehmhhu
jkus hwjv ymnnkk yffugg cvtnits gbl lywkn szihcn dbrbalf rxqpbqh
koyfcef wkom mwok qgjrytl
slmhry lcr slmhry lcr
mvoxbt cfkz purnsui xar ouhtc thbx
xcdifw kvvxyrj knac qmypw bou tmukqy eusgaoo bktiu
ablgnhb axumg bwpxnjp zqpc vtw ghhoxu zqpc znfpvl ghhoxu jlg ntdk
vmvc cdkhrx cvz rvxk mmcuo udpcayd lsmm gufduzt linj
mgyeqkv hqionh rgnqgz kkc qrgnzg egkmqyv topdp
koa dimwx gjxa atlfdy
uuez ueuz zeuu ezo daq
ofpaw bgomvmt mqa dexpy mbipd epyzcoa nuwrh vwly xppz qkjrleo rwhnu
wok grxk lchvtg plrzr lwaax cfeu ijapws dmkdwc cfeu
zkd hysxxip hlydw wicsvy gbwoaw dapre ktjn dzg uri
blzh hblz qgmjceg fyf
vkhpn xnc ogva pjrh cxn hkpnv
aja cldzta tdcazl lorr fwmxxh knilf ges tdhp gnlo vihrl
ucpr peair nlbmc msfg
trv ppq bmo xqd vbui yegsr xqxawu fvuz aclhspo wnan
loiq fvg kare rmgq hir rzo ossd ziw renh ygtkjys vda
xmans kio alexs ujekfl vvf ddghn
fcxvsf bjuytet zrzsobo uhn mlfzhlq bjefs
zys htlqvky plno pbcqfuf fjwc vshkxrl lonp lyzmy dqmui vyyc glad
tlc krhcter krhcter bolk tlc opryl
idcii dverl uswb wusb zgax zhbt gjsnlso yhs
cti npri rcbxjdw ollj nirp ghfvxzh
blyhug aflnrrz zudyw ccnstq cyoju jxtqoj ntuknjq gunjiwy ycuoj igac cqctns
bul yehpnw jifjrhc ifetu ufrodp hqzpeqf hdvpc qtvgxg ibb wcxsitx xztshb
xzct scetn eoaufyo jtudgkx xrpgxip lpubtq juezstc nuc hokswh obkf ipbu
nfq lwpmn qltal xnphsqs zlrgf iewtrtd mqzsob duokpy kfbqs icg
vil zjz xkqrvni uay ystq
terrrnt lnfg clm lbs ptpiy ybcuup ayzjm pqugx lmc yppit mbf
dtajh vqivg vnblt fmn qxkw stiwna pclrrr fro khu wbslnqp tjyosu
uqlehn tjuiy obt uedct bbwiq uxndqn
hiqfovy xiimca zwne ivunvjk cmctzi mxnnrx dclib xzaoq ieztkg
shpr xuorihj chuwq poadbo mhtvex gymsp iltgl sypjfua fmyh sgiv
alv nxjt txnj bhact
vjvtrex obmrxk fgigs meixbc fggsi awi rxdjpeg
ypwo oicmbdw xbpeeyj uabzj cjvutvc oicmbdw immtmks
exijri hogl epr gzdqyur xiiejr pre ihzlgzu
rlh qfhx lrh qmvrx
kogq okhd mivmivb mivmivb okhd
taekt nhjaa znbaahn iaospxy jawwf
ytdvq ghtqwud jkiig mre kzmmjxu jba nwpykc
ktyzr aczd exgadhb uinrgac izazxky yyfe
yrifb qgc lsiuapg teyelxn ugezu
wdzkc ltx fkhncb hwrecp kfbchn sfcpc hjvq
rjdjyt ahwxh nvggsmx lmz oshd xbcik powse ahhxw yhiq gxmgsnv
qdr qjnam gag qjamn kooek mqnaj
pza gml opf ilfbblu kjp luilbfb rhfrzgp ixagj ofp
yphz runy dhull bozcsgk wfxekrd akgkbz urcphc
tfyxwol lhcl npik beug
szatel yfkve yfkve lzqhs
yjzqon pcjibu bdncmcl kczuymm pbmg nyn
rerqvs aoxucwi pmstl sstawu joqu abvcchg mvgjn mslpt vhmfkr utusuh
gqbec jjpqdh yeaiavi nledfi jhzwc vyxjpf momnm vnknjs nvgjzik ipm
psirt rispt lrkgma irtsp
jbbaph xvunete gsvnr mjd ifxhpry cpsx hmuokkx vhcm yth shrrl zbhd
gfa bcmlxtf sqyanrp cugg qxfvftz pbl ujsgc jajxltm gugc oil
xjuhyg aht vmyvzhh oby oyb ybo xbybgmx
atfk qjudfzz mky tfy
nxk yzy jqgg qxgjt bevvvv efi xcbw bohc zaqlqjq
hdc qpnx ygmtqw acvoa udboxw dhc klh mwgpk xfpuri
cycgbkq skwhyf skwhyf veaqss skwhyf
jnezf jowjt vsdu uck scgxd fvopomz vfajslp
djvi epgkyqn apzd cpm owm kpwih fsr adlhqu jicp pmc
erxlmhj wqxvofi ugj ttrmtsb
omku vmrgoy tdicbje ewml dfnwbap
gpih pyt ptsmzc gmdbu rqxkqmz objm nurxjz oozbere ztxug koth
jpnl jpnl dmeh qed
intdwv ksgw qwlzhq zpd lrl mwjl dozrjwq aujbet bsnf vhqyg
eqs uot qyz xor aem kmrh mrhk jqx tsbrf
irytjab mdzm qbb kkjt gofiwo xgbovg kyeyxqn tcks tljhx
zgejy qodgah nqavvx xnigdvt
eqve bizrxq lkhz yzwxgt nwe zfe sxypkz xnssept
bxqn lkfg yfxbszo sphwifz wnj crhbq dvokzw
vzn afatwye ogzvnu vnz rfjba xtugnj kpbgly ocsjd
xrc cxr rahv yvhk khyv bed ctgbuq cmqwpqa jlbg hpj vmesvw
jbshkya dgqw lfl mzcch jxsg czcmh ifruvlw ufwrlvi xcczlol cqqchmr
rbk mhn tnmqdc sxnnn kvoa mhn sxnnn mgemob ieiyajs
cqi ghxg ghxg ghxg
uqwdxn qli gdtkngp gnptdgk udxqwn
dmcczr dnjaqc qwdta rhrbi hkdwe qdjcan peic iulaz xns
tcmppb nzq ecy sitdud nft ecy afrbf wvnc vmfpzx tcmppb cgb
plitv efnpq mjqav nrxxo izg lpitv rwbzdo rdbzwo
day dntga adtng agndt hhvtd
yrg iudsh gyr ryg
qttyeew tco flq bszw jkzftc wdh efcwnp mja rfmju
moch prkze uslzyv plhjuy kxczyq qlmm hgq
xtg ypz izy ixg bvs xlqgj xcy sepza abiylsg
wxvsxn bqag jnlzgxq ikxwa dfd plqxl xlgqnjz nuqvoyb emhodso gaqb
bzjdsm xmxkj fhuqn gauyw ntl kjxmx zcxdr vrds
ofjcc uxyzlk ofjcc ofjcc
zwosex kkvwobl cpudsmb kes zklf bayuojr otqnyr udbbs
iqpvzh ybds piovrh oivprh voprih pov sfl
upns cpeelht xboyk itb hsxdmt dnwgfbw upns fygf kwdpxzm mli dyy
djwutl sikh shki ikhs gecd jqkon trqyw
prbbdf vdp bvvfjcg ydqb muxygg
vhpurzn psemqe xwqfk hrvonxu nxkxacq
xicmhss tnpja qiad woipfy uvadcq usljh hzgs jntvfv wzikk
mmupc twntp upcmm pumcm
qnisuzy lppnfd uiqr eyqbain uxlp eyrfwjo olgkrps sbikam zin vckr
nmokl skfni jcdfot njzqeaj nqzjjea
slmaxx offfzqp wudicrf nfn rwfcdui cwirufd
paffi murnjd oyj lbtjdqe babuas dtqh qkt stapzl yrqlp
eedc rig zmnfmn edec ecde
bcfdf edovdj lacx nzvze sordvxj ybs ujh zvvvp rzstejg ueosuq
xrrfsd okuvem znzlvmb jwzcb bfg bmuxbc qzwfry
pqgxybd cvgra acgn ocd ancg fvfcx fbb bfb zfzv
tmmv mpywyg fwl bnvcv lcnv flw
xxnfbro papc ianru beuzx apcp rnt
wuyhycj nrnc cka ebg rncn rvo wcyhjuy
thh cmoog hwf imqfp okzpxd
rzxiqt rtaiy ytria tyria
cjkmro myif myif xyirn aqxlol wlhwibi dhzsen pzwgm bfbz bufjs qwffg
mxhiui umiihx zomyll vfieccs
yyntf rjk iivgj mwh rjk
dsshx wsmaxhc xcwuelh rdsgtr wsmaxhc rgtsfj
rdh nwlxiwu xsjzbpr bsgps
ufyo vqtzkg kpeohu mxzt fyuo gawgaq youf
hzbhut bxsnjwb zuhhbt zhhtbu
pdz sgntypg ragev hrrji goitft yphnebs xjzoo sqf jsuzijq dsocb hcxg
pptsq woomypc woomypc woomypc
axcg wfbnpql ejqb cmnn nncm csvlc wraludb pkmp whtht tfpicer
moom oomm ommo vfqeii
xvrgpp rofl yxyrkb oage nypzau pwfnkn jxnhkw cyxsi clzb adwpuh
mfbz vdtt muzhm wvwwfl ttdv
cpqgvbu byc pgfrlkr aftl tqm zcqxi juu gnf ppovxh huoa
konpcp lzordid jqng lwxs nqgj gghkxmf kyn ngqj
iorhccj xfygc cnfr tysqc xpcyf vmjpitf nut zmrk mgbrtb tcblxwf dkadwrm
kov jtmp xoatesx qxkilp rmggpfx ltpxzwf vko reqms mqq nps
hjigmk fyqy wpuwe mwmso thsimfs okcmeyh mzqkez duzaq vzhyrm uyvpkox cwivpls
ukoerf korufe zhs ntwfz hugem vriyk enfaib hrrcdgf zllsk vkiyr
shkx khxs wntpjv qdevaw noqyht nwpvjt egh hgok mukdjfi law bzbvjz
dquk kczxsq tdu trnkjs wqtdc ybvcb
hlrotxn cumcjkm qwufgle ylm nejh hnje pvaigrx myl sfvsd
szmvisn aywic vsnimsz iufmybr
zjozr zojzr qmn ffrggdh wam dafvok
nxkvlhr posmf posmf posmf zhlzb
ywis kpqpyb qae zqxpuz pcj hbsfz ejlwa lajew znuom
qxsl ussivur dstd avojo
yoeagao egpaqm ymzf kkauy ivm illir wsvchne skmamvn nqxc
cldo ixzzy vhk nra zhypgab
qjdd ecxud tbuqq mpotbdk tjdpczn knncm tyy
rbfc fhhjf innia tsjbbbv fmtcuup rapvhqz ebpzt whdbms gvjoy lykl fquvcby
bihhfwi lhal udxz uwjwp dmb
fekxamy uophet yzvv rqj zawlp ldrv mdymkzy taauf
rcwxvmh edueui ltdyo xfghz dgjig senm ifj
qcu fii axmgijj ifi oixjfsg jxagijm
sdtyr rbdh yvnvq czzuig wro
lot xkto cmpiena nht ozcg aotcw xiegl cyaouj und lsclep cexn
pgihljk cmgmv sajhi zfvbqij ogwoc ajsih zmppe
jexwkdp dwpexjk mzjydfu bff rubgdb
yshfhx emkl hshxyf mkle
dxgti jdo tkwprv pbxbrqd oiz gsbdphd qotu utfdnq tzvve bqc
ovdf bshfxyl xspjpd vljdsm mgkd djlsvm mlsjdv
etyia eytai sfq qafj xzgp ewhsn snwhe lhqp
zjz mwh dorxm ges gexo rckwsa dltoq mmntha
hqkuj ypsjcxo dixbe rmvnhjh ovnr
edc iffaxc lolu xwrvpb gva vti vit
ceuxq xbwejr lzyvm rozseit cwe mham fivpwj qtv omaktaw
alzdrk tsxbuld mdbq pgbdtoo xwf vzalric nqe jqwlxsy cbtylu dtubxsl lqm
rqjmjcs exjpn kpilcgu ihcm lfadjm mlri hpd vqs cxqwqhu twxrtk
aeuvlcp aubvnw riedvz arypagp uuvg kliehx cokt ogh xsdw cdsyywv
ddwrgvp bscaq bbfv qrbutp
jpdg uey eyu uyarl zgbk qyhqq fdvlql zmwkp
kbt bkt lebhpfu smrzt xalw mmwa zmtzfry tkb
fcvcv oewfzu fvvcc mldww lwdmw
ejrltsu sqoyx wfvsdbp bfdspvw bfir jqhgrmt ofdmrjg ysq
jzwucwn erqjd eikq knpf cvk xvqnscy eei wvfjzmj xujq cqaim boev
jqhkmr ipjpj zwno ybu krhqjm zqfyyzb dyciy
ugwsw rpwteje qtvwi pwyhrzt hfcdfmc qbovk ibws
ffy kdder qjookz bfvmvvq yjzuaj fvxllfb pjyz jcezhye fimyydt qjookz qjookz
loupd nwsc yytvuqo ltcqxnf
iho ulvxguz fxbf iqu ofjtmvq xhs ybbusd kxg mebdnah ucttcf zufb
wzdb wumuhtv kef aavv buu xmjtlur faaccl wospwff bjasr eapfsi
jau qzszci ciu inagax
kui tqig fyovsp fvwol fyovsp mzth tcp nhoq
ajdla wtpj amylu jly tvq wjqef
ofqc einz bdze tows bdze eew
avwavzt aesrsjv lbmpi hllv chdbul ezelxn
imcprs cafb clfg rsjo iylqu nvk vkrq izezlnu vkqr tyhnv
rwj zboui reh buzio wuhpvid cpzy jrw tsbuiby hmxwqr ute
ixq luwbi uoiwsjh souz ysoubw uilbw ffwjvw ewzswoh hci zmfdaov whowzse
xrhgqf xrhgqf giyv giyv
toiqgzv gakg udgdlb wvi carrn pjyha muqclu
wuxthi srtszr ourab hpds bakvy fnk yefe yfee doowxcx
ijdc ujhvls xmy hwg yetsda qelbe nang xgywo wgh
bhm icq cnam dec enksf qfctz pwxoo bdf cnma xoowp rbls
jguzh fextz yax kesaunn waljo jltcza tfzxe dezs syi ebwxnks
flvq bzgd clvqw ahtyvu xbdyv wssxx boscm grgl nqcg
caskpli hqctxxc nwpyo wjlqfqf ebti dva
wmsz fzpd ikgeq gti ejftoou ezs cqef mybojc rgwz
mdaay yfppa pavl fuuvfkh hpod tpb dhxmia emdecm rbqcwbk vecyt
neha rmvl ndp vlrm dpn debghi vyhvc
bnp zkxdu iqqkesd abtlx hmjasdq kyvekr krt srrjyd oxmfev oot
dumlcqd ccm hyir oritdz madjjw
oakqrs advfmu verrc zkfdcn btndsp
onlkinl rdtm bscfxre bnu oumyrvv kgc zkj
tfxfsgm uwmic agswclg uofezgc
wpfdyjn kjlihk etbot fbu scm gwccce xgownte wig cuaijbo
bzbdk etozk qracb oftfoo lkooe
xupzw vmxwu sis wzpxu
gbz oqbgh jwgrru bzg kwmxcfc jrurgw
agyjnyc tuec imxlult omwiyjg fiwnoqx nhmnro qtg kbr agyjnyc
koiq llreotu elrtoul dubfvgy whq
htm lll crzppb gdjaae nsmxzh gnfvn obiuy ymspzbo iuboy
thm xlfrr pbxdfo mht tygi sapxgbv mmngzf dej
eus seu qmstw ues
yvfsw esut biblze kbjcpk estu xih qzki ezlbbi blzv
ohq ugc tqqeo jygvpwm vfs ldnfibp ycbpa sml rmime
kuuow gbg nzwdaf wiimtg lam oqmm
wsbwkdd hda nqk ticz mvt
gqbljyh zqugqs cjod sxwlqy qkfs wwvwvt dsojb qbhjlgy riusoa uosari
jkphfx dbt les jsvoij rnuw mxmmchu dol vto swn
qqxe vwvephr twdqlyg cvdu xjiych clooq vkwavl whvverp yuz vkwval
txtbudi tiutdbx wqhx tws utgbf amh hmf izsez ooz
egdube nhsxjs nxjshs xoy sjsxnh
egdziod diodegz ejxn zogedid uhhkr rnm cyvvuc uqbl
rbn pinwag sidwdwv jqdbe jlbemk blkeaqq ipfqbtn zkrbp
bdryz sbh wxvn mhot wemsfm oemkff
vxyn xvdwwo xhd vyca zxjaw vfkz xhg ofsphks dyq mmzzd
yjrqsjf iiesdh envwyx rmtbmiv ggzsg ukx bprfym qmyqc vag ymho hjtoh
fuxxrd wbweptd vkoffr wbweptd
gfwcez smetli yjyh pslpz qyokpsm qsy cxjymg wqfkf obuq awz
eqhm ceest kayf heqm
rdi dti vntcf ewkmpvf jjwoihc
sfq qlb xrm ocy vtnj zdznbal zvon stln zwnj wsgalvq vhphap
pya jay mgnyo pya xmapdn
hrwbj xhr gvwl ktq ktq gvwl
rzgqi hjwtthl kxhggbl wepc hgavj ctmqug
tzfwkc xeqfath iiuwq iiuwq dhwuvy
gibagy smq getjofc lum msq ulm xuxu bilrus ily
xlv ndrkch hdcknr nqltoze xvl
wmc vuzlrj mwc atp cvpx atv ujatz
hxpafgl ymjltv nvvpy ahycdk jhpdcks ettm lvqyw ertpivm dnezwxx usi kdhcay
vrh hqyomv mcq ilwjbkz yprjxad
ugv szfitxg zeluib pfj ijm zmiigxx gltxzz jzljhgh otskue
mxp bilj jlbi tce yfted zxsqas ftyed
ykasqv ehye kirmnl upmi dojwmw wzj ykasqv ifixn vreoypz
kerbgub nnroqk onkqnr gbebkur tjhl knjo ccsem yozvrcg
ygq evkoj wkn ffljhds scxeibh egsybeg mwvi vgjblj qda ywqpp
hocvpl ozgkxp xgmj ejzyxm
gernu kks lxe nxzv sypg xle goz
xoatis fjp wzlbo dzkonz jtutyj vdonj swro tqclemv xhomap ymeqkua vaxcw
mxcyjs ywyxndk wng vpftv nsuvu
jmiyyhh gwser shgcu jmyg cjzegc hmhe eopg kmkan
smdd dmds mgqhtkh qtamih haqmit skkcy
dnj rmggy rgymg uburbao rymgg
klcpjgq ons ajyv sqryt son pjlcgkq xlobdt
piw shonk tzi mcdumz noskh tebolw yaypn
ozm mvmjgtg nxj weommiq asnmhzq xjn uobztuo cqgjh utfb oydt ommiewq
qlwgsc vvpe xgft ahpjc zjtx iyof scwqlg dxgcokx ltrefj xyzq rwto
ggqdd dqgdg ggdqd kjkmmfp
htzjam fjbg iagc xls iagc iydtf ihxl boa iydtf
vhe nqj bwgdoi hhaoa qtulz
axvyja hpdkwee hnryj prou rgadv oubjdqg knjbc
caz xibj wqkzwe peioeya vmz hesy ftb
dudwcr gupj sjrtzc xsqbb hiet nujv bebcvsj eks uuzlcx gex
kywozi tfzuc mflssw hnxxxqt zzc tzfuc hkokuv mnjg lwkavjp lvpwjak xez
izgh zfv cingjt dkf cknite qox vfz zvf
ojpu dzk tehpgnt gntpteh
glxfxa uxq ajtles ahgzn ajlste zwgc mrpu adz wuunwhc zda
hdgdtn hnoyz aromkb qujfv yjgmn tbf atw
uyvsv oaopjv uyvemxk ldpp tthe iisjk txr hebmd yxevukm rkziao znt
ypdr mnwuzvw acpg kzwz ywbn wcrr umrnlbe lkult ljify azyhu mgqoo
abmpl omsd xmyl mxyl mgoq kracrf ufm ppwi zpggh
uxfdpv jnm vvc vchunhl ubv ktj mxolsxz
fcja eci edzrb nlvksaw lhf ycohh tfztt xso ceub tyv
rkwtp tcmmvv kufg cxui hdamg suuaej fgku cvjlv
oldbgy riadoyo djsi wca zxoeq pmemqap aijxa
nyy ruxcosx xisqoz yny jvzfpbe tlfdiaj ybd jifatdl zuzv
kxwdz qvrvx svllp ergmme
swjfuv eronk favcxfm acptbh pnbjn ciqcrlt rgvdnlt icgahb
ddza xxfn use obqka bfzwjp gmf bld fyvde mxdfdl
ame bmxbyf ame bmxbyf
rdgby pyfog dybrg gdryb lpztd
sntg impd uxgxai naoalb ntnk xgix
oadpmqj oso criln izih oos
ouzjq gtl ito xefqt phnv ouzjq hoyjjj
mlp rboq lpm roqb whvp
tghcw ggshevw dzsgj ggshevw kec ggshevw
kmwhb kfcb mbhkw gemz fdh
euve veue kplrq evue
hikfiw bcdktj hcnawja gjasvwc vcht igrzly rkxijxe ikfwhi dvmp
hvksis kafs ktcs sfyqzyt etctrgt vodwr wff tskc juobnm
dpcsodn ehwc pglywfl yhdp mdiyzx
ibog umftejh cfm pnxhna wqwx yabnk ygws dqw
dezz tqw qism rarfe fpmlab xvbau irwtfs wwmoyss yvn xetqp xtqep
pchqwk npsmd jefec qok uuc ucnpz rlkakn
kudh rjysb xrdbx bkbmjfo xrdbx
rogu ssdwsus voa ncw obkxsr
tflf hlevus scq rrbpat tau wxsq wxoblt
rzr lex kqdy whtj ffnys xlgkkff msjhy dimaq hrc wyde qkwf
ghtwd wernjpn tdgwh olrfvmr edq gxvp
rjirvf skhdgln aauit bipu mubjiwp kowz gyjfbjx cmgdqs
aftfpbv agajyy aqjll vsf twh robpys lebt eav yribup
sby ymkla sxkbfwl awmd nhb vlp
kizvjj ycjswr jkzjiv vuy jijzkv jcs
cwvch xzqfal tephz lqfzax cnkbdcr mql zflaxq
jjxzwl himpra ssjf bibfiui seeaq pzse
jogrn jogrn sqew jogrn oixgwr
khonpyw iiyxir vybhc ndnxxv kzlt ipmncn
okqkqu svbemi nfn ovd xgwy edd ujet nrrbv dde vdo
jobvf dus asvio vaosi sovia
knmz qbz nkmz zmkn
isbmopr unduey impobrs hea zswciev sopbmri duuj
ocs ntgnrdu kbvtzp cvyieu fiyn znmh lhrz ixtnzrj vktbpz lbpqx vzkpbt
muduhc sabc dlyoisz kuaz ogpyepw yuog ictiiqt
xjflsf nfklvml thfh uajnmby cichyj xxoqi lpime bxpyx
riahifn bohbgd obhdgb jni qzvkf ybp hjkkwq ytutd cakcsh smfdoe tuytd
iddku nccp zgtl yne ppzpqcx lwm
refpcz uqt uqt uqt
mtn czxkagb nmt caqacrg bcakxgz
itxjii uethxbj vpds bsqod diqax inv zrwt doepe
bfyaj nbvhg zmi buf
dtre dkwdr nrapm qtfth odvt bbcnae vxuk gqm enlg
ybt qcfozrk yzrh bfp euuozuz pzsdkxx mhi nbkzprb
vpuhqn gyx caint antci vfep incat kqdakdx
ddhi chgnjk ibg xbemitr mjtdph eovw
ngbtuvq qdttlsg dbqhhwk bkrqze qdttlsg qdttlsg
evn smvhi dgcmn xjo ascc ahbpj uvzc pwn tung
ksu thr omg onvsqzz rllakar ysfjtfj grxwyx oawix gpk suk
qvb iouav yhtndkd vuoia ouaiv
kud kofcip hcczrgc cvvxxlk rvyamwe duthdzr dftun
rgv ynw gph tmxwfup nwy
dnc trawj kwzbx trawj zvp
ogqxijy tbqtsg tbo vqinnlq jbvgl sfafh rve mcxqs ubh
qccr lpv puuvdyb tydaflf uxic
tlon tbfwkxg tlon tlon
iytiz qjlqaqw uixb lnt zwro uzgxqfi gklgnqs zwgoidw iifk wkwdo
tmvhxw tmvhxw tmvhxw fhiqpjy ejk kvysd
cmphg xjjz groiccd dvetuk xbwa zhm lyi ohhd neg bxaw yil
kdmzopy lxx bvhach goxmxu qbqvzcm qbbrhvb nrfom aixmio grpxz hbrqbbv lkucih
bnqn phqr uycuxc mopyyfh bbpesqm stgigq stggqi cwtjm asqhpl imvlxj lbmloo
pws iuvbvjr cwccm qbr srqnstz cjebq
bfh jobkcy gtbroe lpagq icmax jobyck fbh
ounqdo qrrr pwi alho rrqr beao rsioepe
vrccqge qvcgrce cbslkjs qnclw rvmjkw
aaxjns deupjs wtgxtp penad depbho tbrdt depbho qxg zhjxpgd
drqfo kbp jfa jaf
izn oczcitj cpae quvzqo iwwk jck idjdpm
ecort zgcvxx bvh vrprsf
fhubfvy ndcfjo kol hyufbfv hvpka
kpt zgajpc rjvsxa gayznjd
xeoixk peq kfu lqa mjnv mzvh bicl hlfk
wyt imdx lksy twy
xeptp ilxs qbsqzwn rsy slxi xtpep dsdkekl
rotvbt fuirp elos ciu nhx bxej trmtx ixn xbpc vrxtma
skcprn yns sao ghlq vftezvc aaryahy telt
fkaov gexa xijv yiksa xega dhgw okfva gxxs edkecag mqbqvrm nrzcqub
ljc jujxeof fdj gdzjzr mabbktu pmyrfv uspven zxry snt hrah
nhujhdr jdhrnuh midm bbavhpp cpjk zmpbasz eptrpou znq zqn
ywzfq wuu lfflon uuw rke qzwyf hjbms gakx
yqrq zsk jzn uuuzrml kzs lseupsg waynfh blech
gwyqej weyjqg uwuje uujwe
lxud rnwkc bgygkh csq rfvtos ystqp keb gkakodj uthcce eqxifl
elvj evj rfwo vvgkosh aarcgjs utsbh orwf jxcqvmh uowmktl qtgf
bqszre oxntty ombwiz mbiwzo
ccp iilcc tacf czk giwv erqi jgdfah wip xtrzhv wosvbyb
gymyw rwsxeg gvydr izyk spsonkg knospsg
djj tbr tbr tbr ice
yyzh zkykapw puydtik ysxc hjumhsd cuhhw dnnhida yyzh lnklymg
nhbcxsu ccrbbyw scbxunh ghxrkqh brcwcyb
latdaav sexa ipzuzjl ayusb etb fshh
giz akqd vjmabii arfuzgv efrww jxkvolg efrww vrnzgbx
jmcc vqy adkzj fqrkdo tjrczp ccmj cfponk rptzjc
jsviu sraw imsj fujm cdf xwqhl lhz ojejzuy trtqblg
ibz dulm muoq quom etvjzxn tuhrpp jfukac jqctqn qhgbae msgmcit ludm
zgx bpfa elhp rnyqtq wyceube nkeuxz
lzxfo vygpecv jszacku zfxlo
cpmv ysaaj xnp wbvqg hrsiuj venjxna yeqvwmk ftaga dcqxc jgapb rqdixp
xpbbe tyn hfdlu fto wrgzkou sxylv cqto wdv xqc pnu rapk
pkrxypl wnu oipq tzbhnc gpug tgzf ofjb
mvaz bwcv gll itgcye dessw szt gzimgeu bvmohh wbywyhc kzerxbr anjsive
lhvnrzs qkmjwy pnyciwp mgp jfdz ghvtf yusfzg upab
xbscukx aubulj snbcmc uscxkbx ddpucyg
hgv ollh yzpjmpy fcicyae vhg gvh
prd onyd iux oik xui
zipadig nvewx cir lbpcusx dljqy
ifyxzsc btmy lsu tmyb lus ldyzx
egmyxbe ieasvek dylmj qahtatr uyqgbk
mejjczw spj vaekp kdud
vwan mgenld mnlged vpfuil euoxlr rclkpi dfknyoa rhthij kcyxl qaxab crlpik
pqm eihogk iwml nuauxi ngilkoh jmu mbdi cqxz nblb rmuj zczdgp
pswbe mtzch wbeps fxtnc psa aioff pas
prwrpvz oadpqvz tgzrt giom pjyihh rxdir dmya xjolzxv
khdybe obqkjn kdq jkvmgwo enpat wyw qjbnko waid msest wwkoyts
yep liv ofmtpod imdd qyw
afnrx jgn gxarpb myltj ggrsajy mdaobjo vbtn vbtn zlziz eds
hqr kqu oub skoeqk icnfm cqvld aay bto
rga odaf exoosh pwevx zpbd plaa xoseoh
mbr gqu oxvchrt nqa larxmjx pfozej
ozuo ywubjbg xcua eblwqp nfdvw hmhen zkjfu gmhgp bsyi ktprtf
src vrysby srybvy znwjm hmypwdl gdmau pqe
cldr crhi lbaq fbuduyn hygbz uhida
qrxukq dygkp oaks soka oask
vpido ajgfq pwlv hezt fmg epwrxo rqvjke iovpd hhkjm
anxf ydl xnfa hqph olorp
exydcg onxjm psqlbv ehz boar hze qsblpv
mnzrvc ipj swg ijp sgw gdkntsd fzz grqwly
erpq qghpj fay gci uglm afy
jwbq hbxaub jpdilyt yvalrlk topl qup
eczonk ftcc paltirb owz tihhe dglxory wthvqcb qdnxm lirejh alyxsr
ooruaby gboyeu lkv arrz jcqyzl uxlfk fhmeony fcmh
wzr xjb pwmf okqj adwcedy lkidve uwekxf asbdzr biub
dikhur pxgh urdinjh wednf ulzdxs
iplf byt tyt qnnlba pzt bednml ljjtkvo tjovlkj uwms xat
htzk ltmfha xikeze atfmhl fchxhyz
lqala bqwgcul vetaa xuxjau zcb wtdmomu wfqmpq sief uyblyz ahv
aytvvo awm ojaaigg awm dbfaokz
abq npcyld fzbfku oia qss jkxldm wgtmki pasgxi dieix rpqnuac tecnfy
nmr qzfj qjfz lsz vnahex
djxoo jzlkh svy xige
tjlkkg glcuvmh fwzlhi ecun qlgulj hrfhyql qgdlf ofakqdf zokkvm gelxkq oowgs
upfpk gfstjlv lxc rjd nhj sbq jpzsz zsjzp
favd nzqfdid nekfjsf mtjndu
sgdqx uvpuefv vhwrgd aivav gsqxd jdhfoq
llaf cthbgy njrpw fqgkx jzf xqkgf lnrfrm gkxqf
wzdwlc wisst alw kyjeur sjsqfcr tta bijnyn whfyoxl
dtjr baxkj lmnyrlg nrmyllg
mtgky xmwf zdko nnocxye gytkm ygp hixk xwmf
maudjy okgjga uadjmy dzfrk omd
azz ajdcqkd bcafn zaz dcjaqdk gylyzo
xzvfbf fopmfxu mvftgr mfupoxf coyhof talcc vpkslo",
"cfkcj (74)
kmwhbm (32)
uuvdgc (58)
siyms (98)
jgtvhkv (1885) -> ykqcpiv, gvupyd, vuyxvq
ppdypq (86)
lovxjut (90) -> fmvna, ddneaes, sakwdmk, lqmoz
ijnvdm (98)
ibzlhq (86)
pabuo (81)
brexb (75) -> tbmiv, iwrph
uqiwwa (705) -> inriw, rnelci
psyroai (498) -> porhb, qinqu, esxiq
ldkxt (69)
mzfbeo (68)
spjtt (299) -> xfcxpkh, sizgye
dnjao (243)
rqwnb (9)
jfnfq (94)
giuonbb (68)
gbywwf (103) -> ugskjw, gelpv
fldjrj (52)
vuxzudl (31)
ewnboyh (38)
bolsi (11)
gbziz (33)
ytkppl (95)
rahuwnt (86)
toprb (282) -> dzyvcxt, xlyngh, tkhbr, avufn, uhhiz, tmtqgn
jadnvpr (13)
rnsotgi (47)
ovcgnvp (309) -> zlffm, iqwjlgh
xpbro (69)
tjulok (12)
pvylfqc (199) -> pzybx, nbdewmj
smwdyt (22)
oozzs (55) -> vrqgow, iynmj
qmzcp (89)
ssneqxd (22)
fqhdfwi (1041) -> ytjhh, rqpbxi, uvprg
bgsbjcq (81)
ilmvjcn (7)
adnrrap (69)
tspxgun (48)
tbmiv (77)
tceqj (58)
nlptf (157) -> vpvgnt, memsb, mcsoi, ctnckd
yzaveth (320) -> frfkpn, viockjp, cqzvhh
dgwely (74)
dpjpifo (47)
lncbnc (238) -> bmiqc, uvnmx
ppgvqkj (48)
fdbsgs (31)
usjpp (55) -> cxhms, niixavl, bspsg, htwyil, pyavtyz
ovhxqx (53)
vzyut (17)
pvxvvv (52)
gkouyvq (73)
ugjmo (86)
mblsyx (51158) -> vqpzey, bavhyuz, fqwyfso
vyrwv (99)
gdjawl (10)
vxcjbsi (56)
tiedml (44)
ihxixk (61) -> pduyk, betaiu
pvthg (41)
gehqj (77)
luxlkxu (1170) -> bzlttq, rowhly, nlptf
irckle (65) -> kuvzyp, nkotu, uqfiwz
fjwfhq (73)
drzukuh (55)
pzyxbdy (81)
brxrm (51)
dirdh (56)
ovujcr (13)
usfiw (7750) -> dtvjn, jektxhw, sfmvh
ytjhh (219) -> jcooqy, gvuvmfz
umnhed (1311) -> lefse, brexb, bexmjvo
vzvwks (52)
ydzdome (10)
iekhet (68)
jcooqy (5)
pcvok (38)
gqmqt (675) -> udebyn, agifkpl, gogsic
gkyvjtj (69)
uhhiz (88) -> bojcjr, krhtjha
jenzlcn (96)
mjntxj (48) -> mdhduoi, aztaw, tfkse
xgelkov (12)
rpadzt (24)
uoewbgw (54)
hmroizo (1564) -> hyjba, qetylot
qinqu (78) -> pabuo, bgsbjcq
cuikvge (150) -> vuxzudl, fdbsgs
jqjyspz (91)
dcaot (27) -> dnjao, bwbufij, mlzxc, auawgd, laxqy, awzuv, pvylfqc
yheomy (69)
kvspfv (81)
uoxqwap (161)
nfefd (82)
wfgammo (97)
mifrxb (158)
ihvae (5)
xfcxpkh (12)
pjegzrx (63) -> pgigk, zijedtv, ghdgob
xrxabjj (45)
fqmjnw (58)
pjgwz (99)
uxzazjg (64)
seeqovh (25)
eyxbeon (35)
umdhwh (27)
sesui (38)
nhnwqgh (59)
bnnfxqe (12)
blivyc (55)
ilrkhhd (84)
kzdxtck (37)
gfsmrr (172) -> xkuubv, eyxbeon
atwwiy (67)
rnelci (99)
hxlve (68)
pkwfv (32)
jnack (61)
pqqxhy (235) -> xsiklw, hmpcc
wcehmi (167) -> ygukv, upllk, kkdvdz, dqaum, synets
onwpfb (77)
zledwkk (7)
mjaqsj (78)
qsoybe (67)
eidhz (91)
blnsa (68) -> hftkbw, snllz, ddfnw, gefhbss
cswrj (176) -> laydujd, hpdegx
jtwckf (71)
lleyk (243) -> nyfnss, veujpah
synets (89) -> uuvdgc, vxqvj
swqrnfi (500) -> ivohs, zzqwyx, sxuafke
oyxlg (11)
zuruj (5)
loaex (63) -> jtwckf, enxsaag
mviybt (27)
qrxvlk (369) -> bpbfd, gdbapcx
yagjnwy (78)
ftlrjo (91)
zklwp (56555) -> kaursj, lahahn, ciwjgw
dehbyec (51)
udoabw (123) -> ewgjf, icdyrl
bojcjr (34)
dwmkyes (119) -> schbl, finmnfn, iiropvx
plhno (14)
bsfpjtc (42) -> zklwp, zoibnsq, mblsyx, vkkwtq, cwluofv
thfdorc (48) -> ieuus, resill, agiybx, lleyk, mqruxqs
hupbun (52)
nyfnss (14)
nkhiz (51)
gicaag (50)
kgsyd (92) -> pzyxbdy, uozna, bcxexao, kvspfv
fuopub (65) -> vqijdaj, mzfbeo
wekwg (51)
vuyxvq (55) -> rsaruxl, blukof
awzuv (133) -> kxbjrca, drzukuh
onrbqjg (314) -> qulqh, wekwg
gefhbss (171) -> rhfpsr, oziwk
gkcpihl (88)
nwbomv (87)
betaiu (82)
wuhui (66)
ocopf (53)
jjceeu (65)
gurmk (64)
ymrkr (77)
grupgc (24)
ucqgh (75)
ujdishy (60)
mrwgq (85)
buvuq (3294) -> usjpp, swqrnfi, oqdntf
cqzvhh (9)
ikhbgx (54)
ddfnw (77) -> blhurw, qdeybxh, dehbyec, hkhvjq
xnfjw (17)
nfrbj (25)
zdrbia (250) -> dpjpifo, rnsotgi
wkennss (43) -> xcobqnc, lpogx, jjceeu
njlyt (65)
thnxnda (66)
xkdua (12)
frfkpn (9)
uwlpeon (30)
gwuxmq (89) -> rladyxh, mjaqsj, ozkrca
lnxtx (91)
djpkm (75)
gbnmi (21) -> mviybt, umdhwh, nswhn
nswhn (27)
kkktvk (32)
rgzkfeq (275)
hcywt (26)
hbjiz (129) -> ldkxt, tvshylj
fkzwy (16)
aztaw (10) -> yfrcezm, kqglah, ytkppl, puxzdr
ocztn (95)
jbsdgwa (34)
fqwyfso (34) -> vuqlmx, hmroizo, fqhdfwi, ksvwst, hihvghh, dcaot
urnycch (8)
uyhqy (23)
resill (127) -> uvptuv, luzuu
udebyn (181)
qetylot (82)
hdjgig (51)
jntmriy (90)
rfvtvw (46)
lsfjnsb (186) -> azmknz, qrriz
vnantj (71) -> avxjqab, oxjor, dattu, nwbomv
epxtul (12) -> aiyvkt, yhmavn
ysqwf (161) -> mxumuj, zbjhd
jieqymv (77)
rqqirx (91)
xoqqhw (346) -> uigna, qfequa
qdrbdcf (59)
uufted (217) -> megix, vewew, ovujcr
frkzri (15)
ddjyphd (66)
modlepu (59)
ynbhq (17)
jsdqcw (7)
dpevfm (7)
tqmkur (63)
bzhnu (150) -> qcpdn, ymjhqjw
sbaeoqo (58)
vtprv (39)
cshts (7)
tlitf (70)
ynodbxy (58)
arfft (259) -> eebqhug, jggld
rxrayit (22)
vdymnhw (37)
uvptuv (72)
fjueeps (17)
tollmm (100) -> qjipfjv, glbhpwz, pkfyhk
emwqaz (5)
unehvbu (147) -> tjulok, fwapzg
wlqagku (34)
unikoz (47)
vizehnk (97)
lnotuxj (198) -> feyiw, vrcfi
tkneufh (51)
usfag (80)
lgwut (308) -> ohdwce, uoewbgw
hfpcxco (85)
qddkwld (233) -> fnzboe, lbrknfc, plhno
ksvwst (1053) -> gzwkbw, ihxixk, vxzowjy
camosg (85)
onbizml (64)
rrlifdx (21)
pzybx (22)
mhbhg (50)
upksvnf (8)
utxkvw (52)
mmhjhh (20)
gjqcj (64)
xljdooj (37)
aejrdpp (201) -> zopkd, beoww
fiutgke (17)
qnmuc (202) -> mmhjhh, sfyps
aqjrxbd (40) -> mrwgq, hfpcxco, kozlucm
imtxkez (94) -> cwthmnc, mzvyddo, rmvgczw, jgtvhkv, hejlvq, ihxivjd
daaexz (199)
ghdgob (86)
owqupte (43) -> vkuvc, ucwwc
eztix (16)
pudsuxc (20)
vlwnw (90)
nlotooe (89) -> azzfba, ghbfbn, ppdypq
xqdpgzt (66)
hzain (46)
tfjzhz (88) -> mhpsnlv, kpjzc
gihhi (20)
gvuvmfz (5)
hmpcc (38)
lotngsa (12)
rupou (1090) -> tcgqf, onbizml
xxjcew (40)
yxfyvw (26) -> nlotooe, kqcmyr, yzaveth, fsqkr, kryid
xlyngh (132) -> qznfyv, lotngsa
rjbaqwb (25)
ibhzmtf (10)
kqglah (95)
hwmegb (845) -> xmlzuy, nehvflm, rnpwyut
sfhrl (240)
fbxcht (284)
xmlzuy (90) -> tspxgun, ppgvqkj
npsgry (85) -> pdttnl, aqzkz, negvvn, blivyc
kxbjrca (55)
jrtqzng (12)
tcgqf (64)
rcmogxy (169) -> xbxnqfz, lkipow
gvfgvtd (124) -> vdymnhw, twyuae, fjhgrk
brzgxi (588) -> untbk, nawhukx, pelersw
cxfgjp (57)
xxqaj (250) -> irsxut, hmdmy
tldocc (42)
icdcn (73)
jtmhpm (44) -> cynginx, pjgwz
tdtiqm (135) -> hymwone, oejnh
qybpba (125) -> elknfln, ctmtk
sozke (24)
uvnmx (9)
hcovonm (56)
yzpxkx (62)
jyfkl (89)
kdhmnh (27) -> uljmqrb, ynxvst
dajvk (245) -> abjhq, qkdikgs
rwvvlbx (69) -> zligfrr, ttmotv
skrlqr (217) -> ibhzmtf, xodpco
ugpaod (27)
gxtabbr (85)
ipqome (26)
dattu (87)
zxozizh (96)
hgtwnxu (12) -> mtkquru, qdzefux, wccvu, qrnztia, etblz
zlkaugz (7) -> uyxisy, faifx, njiof, prkspdn, lovxjut, fatahc, kxxqxm
auawgd (153) -> vnuwn, hugzzwo
ehynzhf (99)
kvrbtlv (1024) -> wkennss, lnotuxj, wbzccs, hlijr, gkvdu
xyvst (196) -> upksvnf, rvdntii
okzaj (208) -> rgyofuq, bdzqyu
rsvtgxk (32)
thzgmn (33)
fxjooi (12)
extfuy (291) -> tnvblm, ljxeb
hazdl (58)
zisnz (66)
ukxuw (66)
mllmqhl (90)
hihvghh (48) -> kbyhru, xkyjdv, okzaj, dsqvto, sfhrl, trjerm, ocatfcc
xodpco (10)
fkqlr (94)
anhrt (14) -> jntmriy, mllmqhl, rmdui, tuorl
mytpi (432) -> inthrl, ffcivin, auijtpl
zvctpo (45)
ttmotv (84)
hftkbw (281)
zijedtv (86)
sksjqz (277) -> zghku, skykdl
vpvgnt (10)
odqxcfl (93)
qfequa (14)
tnvblm (7)
gkfxnzo (17)
pkfyhk (58)
kuypvuy (67)
uhavg (1034) -> aqjrxbd, aafpdpe, eoluf, opbjmvf
acxrgal (287) -> jholt, rqwnb
scengkj (97)
taausp (47)
rxfnzf (18)
uljmqrb (67)
zbvvekj (67)
zwzhv (270) -> yxpzp, pzwxz
gelpv (17)
kunxx (67)
etblz (273) -> dpevfm, kojrycd, lhbuca, jsdqcw
xqfwvbf (20)
ghiqkm (192) -> onwpfb, amqvafo
mfdhdz (99)
fdztdbr (1063) -> sqzdg, hlcre, uyiaqu, usttmag
opnqtf (10)
fsqkr (165) -> jggsgh, xwtls
qrriz (35)
dtvjn (1287) -> gvfgvtd, khajtz, hkyiv
xsyepmg (35) -> camosg, gxtabbr
khykj (37) -> jfnfq, fkqlr
pduyk (82)
aqzkz (55)
merswry (32)
wevnd (64)
blukof (37)
fzhncm (327) -> qybpba, pwtlh, gbywwf, ymbtxzb, dwmkyes
nbdewmj (22)
hmdmy (13)
slnbq (78)
ozkrca (78)
vnoztu (96)
eksshr (914) -> chcsjda, udoabw, fuopub
fnzboe (14)
irsxut (13)
qvyqnuk (132) -> wfmql, rxrayit
ewgjf (39)
kdcgzbw (75)
bbytzn (1597) -> lqwwuqk, shypdye, javnv
ljxeb (7)
vkuvc (78)
kbyhru (58) -> eidhz, lnxtx
cigbnvp (37)
nubyqxj (56)
mtlapd (73) -> veeinqe, nlbnj, jadnvpr
feiowiw (1956) -> zcibgd, chpsgen, slimx
pdfjuw (71) -> fllyd, ghdqy, vzcvxt, keesuec
vrcfi (20)
jggld (83)
wrsdsa (7)
kmzeahn (178) -> gkfxnzo, fiutgke
elknfln (6)
kxbclr (30)
wrrwam (180) -> scengkj, vizehnk
hlfgoa (111) -> wfgammo, rolegtf
uyxisy (566) -> vgicm, mnpow, urnnskw, rcmogxy
brtvs (188) -> ftumhcp, xkdua
jvyscs (126) -> smwdyt, ttegw
jwenagh (25)
hlijr (238)
zghku (22)
ykqcpiv (129)
dpmsnm (971) -> mifrxb, fdpsrru, mqvstxt, rxarqvt, vbypq
vgicm (113) -> rvwtrl, mhbhg
izjxyb (37)
bfanbgh (128) -> arfft, xfvfo, nitrl
hkykap (26)
cpbxfaf (11)
wrbqnx (39)
pwtlh (63) -> izjxyb, kzdxtck
tfpqwb (48)
htkhsaa (74)
fvdsi (40)
iwexk (50)
mtkquru (70) -> mdcnazp, ymrkr, gehqj
pxbjhvu (96) -> nubyqxj, bvxxoux, hcovonm, wxjuvo
jeopi (71)
oqdntf (335) -> ocwyg, afdox, axcgjhj
bveca (138)
bexmjvo (99) -> njlyt, uxupys
xteuk (74)
upllk (131) -> cigbnvp, pewlflc
lahahn (2750) -> bbytzn, fzvctf, utnrb
ycpff (564) -> qgyngw, hwhleil, obumrhe, rgzkfeq, qddkwld, nykzm
usttmag (266)
axcgjhj (291) -> ddroz, ilmvjcn
tdmpe (77) -> buxlsok, ufense, klkyckz
kchcxe (96)
fkksmck (11)
fzvctf (85) -> xgydg, hgyhzc, pqqxhy, qadkf, nsxfciq, gbuhi
mqwdr (2656) -> kvrbtlv, uhavg, tsndvs, ycpff, yadfos
pulsq (84)
giugg (141) -> fieeic, uocco, msjyqjc, xrvdagj
dgdeleb (35)
qyixlaf (25)
wrbrkm (193) -> hdjgig, brxrm, nkhiz
ddpua (95) -> fspnagx, tqmkur, xwfnl
eoluf (39) -> gjqcj, uxzazjg, wevnd, laepzgh
kgfeyyg (45)
ovqpta (602) -> hbjiz, wbrop, ihyawsa
kqkyaey (25)
uigna (14)
qulqh (51)
kdwsl (96)
fkurfvl (15)
auijtpl (44) -> kdtiuq, ijnvdm
wzdgidb (10270) -> xqqgyu, osydb, mytpi
ihyawsa (77) -> ocztn, eiacrfj
bmiqc (9)
urnnskw (5) -> zdsag, pntxt, hupbun, utxkvw
gbhssic (63) -> smguzlg, mfkfr
enxsaag (71)
ysuhs (82)
lxnqzpf (63)
mjwsaam (311) -> rxfnzf, qxgwwc, flxoy
ghbfbn (86)
iyhde (95)
clqri (96)
eoyji (17)
ciwjgw (95) -> fdztdbr, miymv, qwmvf, mzzmpgt
motddh (44) -> ilmqyc, nmfdw
glbhpwz (58)
lpuutn (161) -> mruemdv, chlofj
pcvfnve (58) -> ssneqxd, arizdt
ucefyhu (53)
hymwone (55)
peoneex (89)
mzzmpgt (1159) -> qnmuc, jtmhpm, gxrmso, gfsmrr
mpesve (308) -> ipbhoko, ddivge, ihvae
zwfoos (8)
pewlflc (37)
qxgwwc (18)
kjibzax (32) -> aejrdpp, qgaohbj, oozzs, tdtiqm
agifkpl (75) -> ucefyhu, jfccr
tkhbr (104) -> ipqome, ohwwaa
wcqwomp (34)
vvxmua (54)
zbcwm (124) -> dlsayg, wuhui
avrjyy (25)
hwhleil (86) -> lxnqzpf, nhgdduv, tpxrpfd
eiacrfj (95)
vdoqb (94)
paokema (52)
mhoofw (95)
diovwj (29) -> hwmegb, bfanbgh, tawdmir, ovqpta, thfdorc
inuxln (11)
snllz (103) -> ykztsh, qmzcp
kuvzyp (29)
ocdsfd (59)
qxwsk (320) -> kmwhbm, rsvtgxk, jkqnf
rmvgczw (1744) -> nuqlt, qvyqnuk, ttdeu
fdpsrru (24) -> ntzgern, kuypvuy
kdgcv (96)
iiropvx (6)
uocco (45)
prkspdn (380) -> lnpng, ghiqkm, wrbrkm
txmua (78) -> zisnz, xqdpgzt, ddjyphd
yswzfg (73)
biydd (170) -> modlepu, ocdsfd
clkduv (60)
zligfrr (84)
mdcnazp (77)
cwluofv (52565) -> feiowiw, zlkaugz, smamg
bavhyuz (7693) -> ijibo, ncpzhu, uqiwwa
zscii (59)
wtfjqnx (24)
ucorqrv (122) -> didkxzb, gozty, ezimk
yxpzp (73)
lqmoz (232) -> qyixlaf, jwenagh, nfrbj, twsvvzh
wqcpszg (99)
yyqvf (220) -> irjtkb, mzsoo, rjbaqwb, mxigb
dbtzr (34)
lshcpil (52)
bcxexao (81)
osydb (930) -> htkhsaa, cfkcj, xteuk
zskvah (84)
rvwaido (283) -> oyxlg, uhbka
wfoili (18)
ptzanxy (113) -> uymxxd, vxcjbsi
arizdt (22)
bbupp (71)
kkdvdz (97) -> baqqex, ikhbgx
jniuewa (39)
mdhduoi (262) -> zfnxo, mxaoa, zfrzbb, kkktvk
tmtqgn (108) -> eztix, gufoi, fkzwy
rsnptie (198) -> lausf, pcvok
htwyil (133) -> ocopf, eddcns
ttegw (22)
ayloijg (188) -> pkwfv, merswry, klrrpn
bylpn (93)
jektxhw (456) -> lsfjnsb, cryxpp, ffvtymw, lncbnc, zbcwm, uufted
hlcre (59) -> efszdn, adnrrap, koacic
rvwtrl (50)
lhbuca (7)
xgydg (163) -> jgvzkp, dgwely
ddivge (5)
xkqsn (29) -> umdspul, japznn
irjtkb (25)
bfhoskf (69)
ttdeu (88) -> hpovffo, ulzbyy
utnrb (538) -> jmrywkk, skrlqr, gbhssic, bxbaua, rwvvlbx, fftxfs
hkyiv (45) -> iyhde, mhoofw
itfjcv (85)
xfvfo (41) -> clqri, fdvpljm, kdgcv, kdwsl
jkqnf (32)
esxiq (174) -> gbziz, thzgmn
nazoxxl (61)
gkvdu (138) -> gicaag, iwexk
hejlvq (346) -> sksjqz, lnrnk, pjegzrx, ovcgnvp, zjiaam, giugg
azzfba (86)
kosdbim (21)
bjfmsgp (81)
cyizx (90) -> kiflapm, fjbvj
viockjp (9)
nsxfciq (215) -> wfekn, cdnagg
cxhms (103) -> iekhet, ljcnfds
nmfdw (47)
mxumuj (12)
tawdmir (32) -> gnamu, ppptkjy, qnsnqy
uhbka (11)
vxzowjy (9) -> gfrqgpm, vfsdg, ltfanj, vvxmua
rvdntii (8)
xoseld (44)
zjiaam (185) -> dbtzr, ortdyy, jbsdgwa, dtqejc
qffvjvy (20)
nhgdduv (63)
ddroz (7)
rghhbxs (58)
mhzvara (240) -> rrizdbi, ebjqfgp, zvdii, vrfkca
vnuwn (45)
cdnagg (48)
xbypzyz (354) -> wdvmq, bcqcl, biydd
flxoy (18)
miymv (47) -> qxwsk, onrbqjg, lgwut, zwzhv, kgsyd
onbize (62) -> djpkm, ucqgh
ymjhqjw (31)
zcibgd (41) -> wrrwam, xoqqhw, izcxju, vqtknqo, anhrt, cswrj, blfzhbb
rgyofuq (16)
zbuar (48)
amsxxe (220) -> plzvkk, ugpaod
blfzhbb (254) -> clkduv, ujdishy
vilcpj (17)
smguzlg (87)
qwmvf (870) -> vxzqqkt, vnantj, qrxvlk
ygqyizj (17)
aafpdpe (94) -> nbxepbg, atwwiy, pwzfoy
ykztsh (89)
bfrlxk (99)
iabco (207) -> fqmjnw, tqjoo
bktiekj (78)
kwkcos (62)
jggsgh (91)
xrvdagj (45)
bspsg (188) -> vzyut, fjueeps, vilcpj
dqaum (33) -> ugjmo, nhpvc
eujhcc (68)
tqfnnek (60) -> gwuxmq, spjtt, rysldfw, aqdba, mpesve, iabco
efszdn (69)
uqzisq (10150) -> emqcvc, blnsa, wcehmi
nynxxb (112)
bkpka (85)
sizgye (12)
wgtdln (35)
yzdou (21)
ortdyy (34)
gogsic (67) -> dfzlxba, gmrktc
fspnagx (63)
laxqy (45) -> nuoxnzw, ehynzhf
sfmvh (72) -> jeylva, uicohi, pxbjhvu, mhzvara, irmjpg, yyqvf
hlctdjw (53)
ynxvst (67)
aiyvkt (70)
rsaruxl (37)
mruemdv (14)
riqqep (21)
mnpow (162) -> eoyji, ayslth, ygqyizj
iclynp (706) -> pcvfnve, gbnmi, cyizx
tsndvs (79) -> rvwaido, npsgry, hlfgoa, criaedy, hohun, extfuy, acxrgal
gmyydys (69)
xbxnqfz (22)
ujudb (77)
vqpzey (1876) -> mjntxj, rupou, xbypzyz, brzgxi, toprb, gqmqt, psyroai
ivlxyu (68)
jeylva (148) -> ibzlhq, rahuwnt
xpormy (97)
zlffm (6)
ulzbyy (44)
trjerm (156) -> yzdou, riqqep, rrlifdx, kosdbim
wbzccs (70) -> pulsq, zskvah
xwfnl (63)
nbxepbg (67)
fatahc (851) -> lpuutn, aemnj, fpbxkb
dzyvcxt (142) -> wrsdsa, zledwkk
asafmi (51)
puxzdr (95)
skykdl (22)
avxjqab (87)
rowhly (47) -> kdcgzbw, fpzba
lqwwuqk (118)
rqpbxi (145) -> kmpdpy, kimfdue
ffcivin (170) -> tlzng, fwfbtcw
zoibnsq (8) -> wzdgidb, imtxkez, uqzisq, mqwdr, vcezyqj, usfiw
xsiklw (38)
ucwwc (78)
opbjmvf (63) -> dbkzlc, sbaeoqo, hazdl, rghhbxs
luzuu (72)
dbkzlc (58)
jgvzkp (74)
kryid (199) -> tenjus, gocurcy
hdahal (81)
fjbvj (6)
ajjyqz (14)
fftxfs (155) -> wqljugh, sjlwjcc
vkkwtq (61232) -> buvuq, diovwj, decklvy
xqqgyu (16) -> fbxcht, uyxpx, ddpua, ayloijg
hohun (305)
cwthmnc (82) -> wmciqdi, tdmpe, wiqjl, gnmudrh, mjwsaam, fgqxuig
veeinqe (13)
gocurcy (74)
uxupys (65)
qjipfjv (58)
wgarj (35)
mceul (202) -> wtfjqnx, rpadzt, grupgc
ohwwaa (26)
fieeic (45)
urawf (88)
fjhgrk (37)
axjaxn (78)
gunuriu (73)
uetcsjx (52)
bwbufij (35) -> lshcpil, vzvwks, paokema, uetcsjx
ppptkjy (97) -> vlwnw, ivksd, etzocl, akvov
agiybx (257) -> dmyxvek, cshts
oziwk (55)
yvedw (88)
gjlsrq (84)
nehvflm (98) -> bhezu, plcqz
rlvxk (53)
eiysj (69)
cynginx (99)
lefse (51) -> jyfkl, peoneex
qfabag (12)
ffvtymw (204) -> hcywt, ttvyvv
ayslth (17)
rladyxh (78)
rnpwyut (170) -> zwfoos, urnycch
mxaoa (32)
ufense (96)
atfrn (35)
qrmkrs (94)
javnv (48) -> atfrn, wgarj
inthrl (80) -> usfag, usvvacs
ljcnfds (68)
rmdui (90)
njiof (743) -> zhqnj, ptzanxy, khykj
zqqxw (17)
zbjhd (12)
kimfdue (42)
vqtknqo (192) -> jqjyspz, dkxedzu
gmrktc (57)
lbroca (46)
kdtiuq (98)
hkhvjq (51)
ohdwce (54)
deusrct (41)
urljsxt (64)
mjesxf (74) -> zbuar, tfpqwb
qdeybxh (51)
kxxqxm (1330) -> xoseld, xpxbgh
ivohs (91) -> hlctdjw, rlvxk, ovhxqx
izcxju (176) -> ybwjsz, bfrlxk
hdwpha (62)
lnrnk (321)
vybrd (42)
kiflapm (6)
icdyrl (39)
fpbxkb (53) -> hxlve, ivlxyu
qnsnqy (310) -> gddaubt, rnnmuk, kjauj
zazlcg (68) -> nkgayde, eiysj, yheomy, xpbro
inriw (99)
jpsejb (26)
gufoi (16)
uicohi (296) -> frqiho, bnnfxqe
rxarqvt (58) -> avrjyy, seeqovh, gmdwhcy, kqkyaey
yadfos (1659) -> ysqwf, zkfwc, bqusn
gozty (13)
ocatfcc (240)
vzcvxt (54)
xochi (195) -> xdohu, lbroca
lddqu (20)
dmyxvek (7)
didkxzb (13)
hpdegx (99)
tlbakdh (7) -> vnoztu, lkozg
vvsgr (332) -> pulpbaj, mjesxf, jvyscs, pctst
avufn (28) -> urljsxt, gurmk
faifx (1004) -> rpywck, bveca, motddh
chlofj (14)
zygpcpd (39)
msjyqjc (45)
dhdhlmq (1181) -> jelwm, mtlapd, nynxxb
knczsg (51)
tenjus (74)
ocwyg (217) -> cjgtcao, tiedml
zzqwyx (62) -> vdoqb, qrmkrs
zdsag (52)
rysldfw (26) -> vyrwv, wqcpszg, mfdhdz
vfsdg (54)
pgigk (86)
tytufo (67)
bqusn (15) -> itfjcv, bkpka
gmdwhcy (25)
wqgini (563) -> xochi, xdfyw, pdfjuw, ujtmmb, dajvk
iynmj (95)
umdspul (71)
qcpdn (31)
gddaubt (49)
colwnyd (141) -> fkurfvl, frkzri
nuqlt (64) -> rfzyvg, dirdh
qgaohbj (81) -> nfefd, ysuhs
sxuafke (250)
efvcgxy (51)
kmpdpy (42)
finmnfn (6)
ohcob (901) -> rggrduc, fbnmdsm, xljdooj
bxbaua (63) -> ynodbxy, tceqj, necuat
lkozg (96)
yzahca (62)
vbypq (88) -> dgdeleb, wgtdln
lkipow (22)
mqruxqs (135) -> eujhcc, giuonbb
nitrl (425)
lnpng (70) -> pkhkeva, lzdzs, bfhoskf, vcdrqps
nhuuo (70)
nkvxvrf (12)
beoww (22)
atnshbm (58) -> taausp, unikoz
ftumhcp (12)
kojrycd (7)
lyoaqtx (184) -> gbkfxbj, xxqaj, txmua
ojwlkyk (23)
untbk (28) -> rqqirx, ftlrjo
bcqcl (278) -> emwqaz, zuruj
tpxrpfd (63)
vxqvj (58)
zfnxo (32)
klrrpn (32)
mlzxc (51) -> qxqzzw, zxozizh
porhb (240)
pelersw (70) -> nhuuo, tlitf
fpzba (75)
qgyngw (121) -> jieqymv, ujudb
negvvn (55)
nhpvc (86)
wbrop (233) -> xnfjw, ynbhq
fgqxuig (275) -> qjknns, kxbclr, uwlpeon
wmciqdi (335) -> opnqtf, ydzdome, gdjawl
laydujd (99)
vuqlmx (932) -> tlbakdh, daaexz, maokl, owqupte
zopkd (22)
azsiin (69)
hgyhzc (311)
nlbnj (13)
ygukv (115) -> wacnhz, kgfeyyg
gxrmso (242)
jfccr (53)
sfyps (20)
deargsb (11)
dqxvm (67)
mqvstxt (134) -> jrtqzng, xgelkov
twsvvzh (25)
qrnztia (145) -> bktiekj, yagjnwy
rnnmuk (49)
qxqzzw (96)
nawhukx (188) -> fkksmck, deargsb
tiwaeim (98)
iwrph (77)
feyiw (20)
akvov (90)
wacnhz (45)
mhpsnlv (93)
yfrcezm (95)
dsqvto (207) -> cpbxfaf, inuxln, bolsi
rhfpsr (55)
gnamu (397) -> pudsuxc, lddqu, qffvjvy
buxlsok (96)
ebjqfgp (20)
vdabfab (73)
memsb (10)
nkotu (29)
rctutb (98)
dlsayg (66)
megix (13)
nkgayde (69)
zfrzbb (32)
whdgk (73)
gnmudrh (331) -> zqqxw, uagtb
chpsgen (2051) -> irckle, epxtul, atnshbm, ostpegy
hyjba (82)
mcsoi (10)
uyiaqu (98) -> gjlsrq, ilrkhhd
bdzqyu (16)
necuat (58)
uymxxd (56)
qdzefux (139) -> hdahal, bjfmsgp
zqega (18)
jelwm (60) -> hkykap, jpsejb
lcpbmc (212)
nuoxnzw (99)
lpogx (65)
maokl (171) -> xdxiemt, ajjyqz
fwfbtcw (35)
ostpegy (60) -> rfvtvw, hzain
uqfiwz (29)
fllyd (54)
ntzgern (67)
wfmql (22)
hugzzwo (45)
oxjor (87)
ncpzhu (83) -> loaex, qoamv, rnfox, xsyepmg
gzwkbw (123) -> efvcgxy, knczsg
usvvacs (80)
cptmwf (75)
xdfyw (104) -> yafzoj, nazoxxl, jnack
ijibo (390) -> xkqsn, colwnyd, unehvbu
nykzm (275)
wxjuvo (56)
obumrhe (275)
fwapzg (12)
uvprg (10) -> qhavltn, yswzfg, fjwfhq
klkyckz (96)
koacic (69)
ihxivjd (902) -> rsnptie, tollmm, mceul, tfjzhz, amsxxe
lausf (38)
sakilbi (57)
gdbapcx (25)
wqljugh (41)
zvdii (20)
wdvmq (81) -> gkyvjtj, azsiin, gmyydys
aemnj (107) -> pvthg, deusrct
smamg (5382) -> hgtwnxu, eksshr, dhdhlmq
tqjoo (58)
abkzl (24)
keesuec (54)
soguk (98)
rnfox (71) -> tytufo, qsoybe
xdohu (46)
bvxxoux (56)
decklvy (1050) -> umnhed, wqgini, tqfnnek
sakwdmk (138) -> xpormy, nhhvktp
ezimk (13)
bhezu (44)
dfzlxba (57)
xypefbg (73)
tfkse (189) -> zbvvekj, kunxx, dqxvm
vcdrqps (69)
fmvna (260) -> sozke, pgarna, abkzl
fyzadg (18)
blhurw (51)
rolegtf (97)
jmrywkk (237)
kqcmyr (301) -> uyhqy, ojwlkyk
hpovffo (44)
mzsoo (25)
tuorl (90)
bzlttq (73) -> hdwpha, yzpxkx
uozna (81)
qznfyv (12)
xpxbgh (44)
abjhq (21)
criaedy (179) -> vybrd, jprgu, tldocc
amqvafo (77)
vckadf (8)
schbl (6)
veujpah (14)
vrqgow (95)
qoamv (165) -> xqfwvbf, gihhi
tlzng (35)
dkxedzu (91)
vcezyqj (8443) -> luxlkxu, dpmsnm, yxfyvw
ipbhoko (5)
krhtjha (34)
xktot (268) -> ewnboyh, sesui
lscpgcs (296) -> fxjooi, qfabag, nkvxvrf, cjyrih
kaursj (1519) -> kjibzax, lyoaqtx, iclynp, jsgvv, fzhncm, vvsgr, ohcob
kswmjp (93)
rpywck (36) -> asafmi, tkneufh
wiqjl (209) -> zygpcpd, vtprv, wrbqnx, jniuewa
wfekn (48)
tvshylj (69)
qkdikgs (21)
plcqz (44)
wccvu (169) -> ukxuw, thnxnda
pdttnl (55)
xkuubv (35)
nhhvktp (97)
qjknns (30)
ltfanj (54)
pulpbaj (28) -> bbupp, jeopi
xcobqnc (65)
aqdba (167) -> slnbq, axjaxn
qadkf (32) -> odqxcfl, kswmjp, bylpn
vrfkca (20)
fdvpljm (96)
qhavltn (73)
niixavl (43) -> siyms, tiwaeim
ieuus (79) -> jenzlcn, kchcxe
mfkfr (87)
etzocl (90)
pgarna (24)
cjgtcao (44)
ctmtk (6)
rrizdbi (20)
bltjqky (8)
ilmqyc (47)
gfrqgpm (54)
titde (75)
jsgvv (368) -> btvxwyj, kdhmnh, ucorqrv, uoxqwap
afdox (41) -> gkcpihl, yvedw, urawf
pntxt (52)
pzwxz (73)
btvxwyj (81) -> xxjcew, fvdsi
frqiho (12)
xkyjdv (44) -> soguk, rctutb
sqzdg (176) -> xrxabjj, zvctpo
ymbtxzb (19) -> qdrbdcf, nhnwqgh
slimx (1283) -> xktot, zazlcg, lscpgcs, zdrbia
eebqhug (83)
azmknz (35)
fbnmdsm (37)
laepzgh (64)
yhmavn (70)
japznn (71)
xsjptl (75)
pctst (154) -> bltjqky, vckadf
khajtz (111) -> yzahca, kwkcos
gbkfxbj (276)
kozlucm (85)
jprgu (42)
rggrduc (37)
oejnh (55)
ugskjw (17)
baqqex (54)
vxzqqkt (419)
mzvyddo (788) -> brtvs, kmzeahn, onbize, cuikvge, lcpbmc, bzhnu, xyvst
eddcns (53)
rfzyvg (56)
ctnckd (10)
kjauj (49)
plzvkk (27)
irmjpg (206) -> cxfgjp, sakilbi
lozmiv (59)
shypdye (118)
pkhkeva (69)
lbrknfc (14)
dtqejc (34)
jholt (9)
kpjzc (93)
xwtls (91)
zhqnj (225)
cjyrih (12)
chcsjda (201)
uagtb (17)
uyxpx (166) -> zscii, lozmiv
vewew (13)
ivksd (90)
yafzoj (61)
ybwjsz (99)
pwzfoy (67)
ghdqy (54)
sjlwjcc (41)
iqwjlgh (6)
gbuhi (243) -> wlqagku, wcqwomp
bpbfd (25)
mxigb (25)
lzdzs (69)
pyavtyz (239)
xdxiemt (14)
cryxpp (31) -> cptmwf, xsjptl, titde
zkfwc (81) -> pvxvvv, fldjrj
ujtmmb (141) -> xypefbg, vdabfab
emqcvc (1138) -> wfoili, zqega, fyzadg
gvupyd (129)
twyuae (37)
ddneaes (40) -> icdcn, gunuriu, whdgk, gkouyvq
vqijdaj (68)
ttvyvv (26)",
"js inc 257 if wn < 9
jq dec -586 if esb != -3
gcf inc -603 if i >= -9
gcf dec -300 if d != 1
g inc -973 if gy > -1
epp dec -79 if rjx < 9
x dec 796 if esb == 0
d inc -526 if rf < 3
qc inc -610 if dma > -8
gcf dec 831 if aqr > 5
wow dec -705 if jq >= 583
gcf dec 135 if esb < 10
gcf inc -777 if aqr != 8
esb inc 262 if rjx > -10
x inc 259 if dma > 3
g dec -784 if rjx != -2
rjx inc -969 if yzp > -3
wow inc -401 if g < -182
dma inc 995 if rjx <= -962
vyy inc 290 if g >= -194
vyy inc 4 if gy == 0
vyy dec -295 if wow >= 314
j dec 476 if j >= -8
u inc 84 if g < -195
g inc -422 if gy == -4
yzp dec -523 if cty == 0
cn dec -938 if g > -195
u dec -652 if g <= -193
aqr dec 829 if qc < -601
u dec 199 if wow < 309
aqr dec -409 if rf >= 9
dma dec -556 if dma > 990
qc inc 134 if i != 0
qc inc 238 if j >= -476
yzp dec 810 if js < 255
i dec 898 if i == 0
g inc 898 if tyg <= -5
j dec -130 if aqr == -832
tyg inc -504 if l > -7
aqr dec 465 if l >= -9
gcf dec 64 if jq != 595
wow inc -102 if yzp >= 515
rf inc -433 if i == -898
rjx inc -334 if l >= -6
rf inc -395 if cn <= 947
u inc -724 if qc < -376
epp inc 309 if gcf >= -1281
zp dec -367 if cty >= -5
aqr dec 265 if aqr <= -1287
u inc 359 if wn == 0
vyy inc 587 if aqr != -1550
uxa inc -541 if u <= 162
js inc 720 if wow >= 195
d inc -857 if wn > -3
cty inc 958 if gcf == -1279
rf dec 153 if x >= -804
cn dec -921 if dma <= 1560
zp inc -554 if qc > -381
jq dec 83 if x < -799
qc dec 912 if cty < 968
i inc -636 if tyg == -504
rjx dec 113 if aqr >= -1566
js inc -184 if wow > 198
qc inc -519 if cty != 958
aqr dec 281 if u > 166
aqr inc 392 if vyy <= 887
cn inc -179 if rjx > -1420
g inc -16 if cn != 1680
d inc 260 if uxa != -541
j inc -873 if yzp < 522
rjx inc 737 if g >= -191
wow inc 288 if qc >= -1287
j inc -844 if wn > 5
i inc 91 if uxa > -546
yzp inc -12 if d == -1383
cn dec -247 if zp >= -189
i dec -246 if dma == 1551
qc inc -919 if gy >= -3
cn dec -804 if esb < 270
uxa inc 593 if qc != -2197
yzp inc -562 if esb <= 264
rjx inc 850 if zp < -181
u dec 944 if j != -486
vyy inc 970 if vyy >= 881
gy inc 659 if u < -779
aqr dec 145 if esb == 262
yzp inc 346 if gcf == -1282
wow inc 112 if gcf != -1279
vyy inc -120 if rjx == 171
esb dec -795 if qc >= -2210
cty inc -282 if zp == -187
uxa dec -683 if l == 0
epp inc -185 if jq >= 589
js inc 542 if cn != 2741
gy inc 591 if d == -1383
wow inc -734 if rf != -987
wow dec 482 if vyy <= 1739
uxa dec 231 if u >= -785
gy dec -153 if d != -1385
dma inc 682 if js > 1330
wn dec -173 if i < -1196
tyg dec -510 if dma > 2230
aqr inc 309 if gcf <= -1289
gcf inc 831 if g > -190
g dec -457 if qc == -2203
aqr dec -682 if gcf != -447
cn dec 75 if js > 1328
vyy inc -639 if cn >= 2666
j dec 119 if uxa != 499
i inc -312 if gy > 1399
rjx dec 145 if aqr != -634
cty dec -600 if wow == -726
dma inc 79 if zp < -179
vyy inc 285 if jq <= 586
tyg inc 34 if d < -1379
u dec -320 if yzp != -44
esb inc -229 if wow >= -730
gy dec 175 if cn != 2659
gy inc 87 if js <= 1334
wn dec 725 if rjx < 35
cty dec -64 if gy == 1228
tyg inc 809 if gcf != -438
zp dec -526 if uxa > 500
rjx inc -520 if wow > -727
gy inc 608 if cty > 1345
u inc -847 if cn == 2656
dma inc -728 if zp < 343
qc dec -722 if wow > -734
gy inc -864 if wn != -552
l dec -53 if cn == 2656
rjx dec -674 if gcf > -446
i dec 647 if jq == 586
dma inc -109 if cty <= 1349
d dec -263 if j == -595
rf dec 835 if rf < -972
esb inc 504 if js >= 1332
uxa dec -215 if wow > -732
l dec 240 if qc != -1472
tyg inc 754 if aqr > -639
d dec -784 if d > -1129
u dec 528 if dma == 1482
x dec 569 if aqr != -636
uxa dec -163 if yzp < -49
qc inc -88 if x != -1365
gy inc -966 if yzp <= -43
cn inc 870 if gy == 262
x dec -511 if gy > 263
qc dec -245 if epp >= 380
rf dec -718 if vyy >= 2018
aqr inc 494 if u != -1302
esb dec 211 if wow < -734
esb dec -284 if epp > 379
x inc 354 if x >= -1372
g dec 378 if cty != 1349
wow dec -405 if js < 1338
tyg dec 615 if yzp != -51
rf inc 475 if dma == 1475
gy dec 15 if aqr <= -132
zp inc 915 if zp != 335
d dec 323 if wn >= -553
g dec -525 if cn != 3517
l dec -13 if gy <= 248
gy dec -687 if i != -2147
tyg inc -487 if gy == 934
rjx dec 444 if vyy == 2014
wn inc 449 if yzp < -46
cn inc 651 if rf != -1337
zp dec 888 if jq >= 578
uxa dec 966 if esb > 1616
wn dec -219 if tyg != 1109
dma inc 599 if cn > 4176
x inc -486 if epp >= 387
l inc -404 if yzp != -43
rjx dec -996 if aqr > -142
g dec 215 if dma == 2079
aqr dec -746 if i > -2159
gcf dec -301 if cn >= 4174
qc inc -700 if vyy == 2016
rjx dec 289 if d >= -661
vyy inc 278 if gcf < -138
g inc 659 if wn != 118
zp inc 622 if l <= -579
zp inc 644 if zp >= 373
aqr inc 468 if gy >= 931
wn inc 608 if zp > 359
js dec 985 if gy <= 942
uxa dec 509 if epp > 383
tyg inc 134 if cty <= 1340
jq dec 677 if dma < 2073
yzp dec -146 if j >= -600
l inc -500 if dma == 2072
cn inc 399 if cty <= 1341
rf inc 521 if wow != -321
gcf dec -70 if wn > 718
gcf dec 946 if tyg == 1250
cty dec 398 if qc == -1936
qc dec -105 if qc > -1936
g dec -16 if g != 1082
cty dec 932 if vyy > 2288
gcf dec -293 if esb != 1616
cn dec 244 if esb >= 1610
rf inc -790 if cn != 4338
dma dec -87 if wn == 724
cn dec -191 if j > -598
aqr inc -246 if d <= -658
cn inc 786 if vyy <= 2297
g dec 211 if qc < -1932
x inc -473 if l >= -587
rjx inc -686 if jq <= 589
dma inc -411 if gcf <= -1019
u inc -596 if qc <= -1933
l inc -60 if rjx > -476
wow dec 910 if zp == 366
gy inc -854 if u != -1916
jq inc -698 if wn <= 729
zp inc 456 if d != -665
x dec -853 if x >= -1964
aqr inc -282 if rjx < -478
tyg inc 975 if cty == 10
js inc -684 if wow >= -1239
uxa dec 702 if g == 879
g inc -602 if tyg <= 2231
cty inc 987 if dma > 1742
gy dec -57 if cn == 5317
zp dec -527 if qc > -1941
tyg dec 689 if g != 282
gcf inc -11 if rjx > -478
epp dec 44 if uxa < -325
aqr dec 936 if i < -2156
i dec -119 if aqr >= 841
yzp dec 46 if uxa > -330
u inc 549 if esb == 1619
zp dec 412 if j >= -598
cn dec -865 if gy < 89
yzp dec 366 if g > 283
j dec 372 if j <= -595
zp inc 879 if tyg > 1532
wow dec 30 if x > -1974
dma dec 288 if j < -966
zp inc -419 if epp == 344
tyg inc 304 if x < -1964
zp dec 77 if j < -960
cty inc -586 if wn < 728
wow inc 271 if wow == -1261
u inc -751 if dma != 1470
wow dec -64 if wn != 727
esb inc 294 if gy <= 75
jq inc -559 if wow > -918
cty inc -283 if gy != 73
cty dec 792 if g != 274
l inc 589 if rjx >= -482
gy inc -659 if jq < -116
cn dec -817 if aqr <= 839
l inc 682 if rjx < -467
l dec -285 if rf <= -2125
qc inc -370 if x < -1961
l dec 582 if yzp >= 47
dma inc -365 if js == -334
js inc 397 if d == -659
g inc 902 if aqr <= 840
x inc 129 if rf >= -2139
g dec -704 if gcf < -1041
cn inc 207 if uxa > -332
epp dec 517 if js < 70
cn dec 125 if qc <= -2305
x inc -187 if tyg > 1844
gy dec -1 if cty < -661
js inc -224 if qc != -2306
x inc -627 if gcf <= -1025
yzp inc -426 if g < 1186
uxa dec -839 if wow >= -933
u inc 688 if gy <= 87
jq dec -453 if aqr < 834
tyg inc -370 if d >= -660
l dec 81 if gy != 89
zp dec -184 if uxa == 510
gcf dec -97 if gcf == -1034
tyg inc -508 if x != -2468
cty inc 89 if j < -960
jq dec 416 if rf >= -2132
rf inc -846 if cty >= -577
uxa dec -315 if vyy <= 2294
g inc -191 if vyy == 2294
js dec -557 if wow == -934
u dec 382 if qc >= -2313
dma dec -841 if wow > -929
u dec 6 if dma < 1932
aqr inc 75 if js == 63
jq inc 640 if gcf < -928
wn inc -992 if j != -967
uxa dec -577 if wn != 726
g inc -458 if rjx < -467
epp dec 141 if x == -2468
l inc 617 if cty < -576
gy dec 542 if yzp != -377
cty dec -99 if uxa < 1410
tyg inc -358 if g > 521
wn dec 514 if uxa <= 1401
dma inc 110 if j > -960
js inc -359 if jq <= 569
epp dec 266 if epp < -312
x inc -473 if jq > 557
vyy inc 890 if wn == 730
zp dec 379 if gcf != -932
u dec 46 if jq == 565
tyg inc -410 if l != 256
i inc -320 if js != -303
l dec 670 if i >= -2482
i inc 788 if u < -2391
l inc 23 if cty == -476
tyg inc -364 if rf <= -2969
gy dec 394 if epp >= -584
esb dec 645 if aqr != 907
epp inc -842 if x < -2935
js dec -289 if l == -392
tyg inc -405 if gcf == -937
vyy dec -58 if epp == -1419
gy dec 110 if u == -2398
vyy dec -789 if epp > -1426
qc dec 463 if zp >= 1117
gcf inc 859 if esb == 1616
u inc -99 if js < 2
epp inc 111 if tyg == -67
jq inc -536 if d < -661
js inc 506 if dma <= 1936
rjx dec -58 if j >= -971
wn dec -513 if tyg <= -64
wow dec 816 if aqr >= 902
gcf inc -957 if x == -2941
uxa dec -39 if yzp <= -377
wn inc 824 if g <= 531
gcf dec 927 if l >= -394
x dec -655 if l == -392
g inc 767 if tyg > -59
dma inc -493 if epp != -1318
esb inc -587 if epp >= -1318
wow inc 996 if cty >= -479
l dec 119 if epp != -1311
gy dec -935 if uxa < 1444
gcf dec -428 if zp >= 1121
js inc -999 if x > -2293
uxa dec 438 if epp != -1305
gcf dec -623 if yzp <= -380
j inc -256 if gcf < -1530
l inc -183 if d >= -662
wn dec 945 if x < -2279
i inc 953 if epp == -1311
vyy inc -434 if js > -1016
yzp dec -695 if epp < -1308
d inc 975 if l == -575
uxa dec -218 if j < -1216
gcf inc 413 if tyg == -67
wow inc -380 if zp != 1125
cn dec 166 if wn > 1113
rjx dec 878 if l != -581
esb inc -745 if g <= 531
aqr dec 14 if esb < 292
j inc 405 if jq == 565
vyy inc 657 if wn == 1124
j dec -624 if qc > -2763
rf inc -711 if esb < 289
uxa inc 453 if gy >= 512
i inc 593 if js >= -1009
qc inc -413 if j > -826
tyg inc -395 if epp == -1311
cty dec -911 if g >= 524
yzp inc -259 if esb >= 277
dma dec -812 if vyy > 2643
wow inc 887 if gcf >= -1121
uxa inc 906 if rf < -3686
l inc -816 if js > -1002
aqr inc -123 if tyg >= -462
x inc 413 if uxa < 2590
tyg dec -141 if wow <= 142
cn inc -138 if gcf > -1118
i inc 665 if wn < 1117
wow inc -250 if gy > 505
wow inc 751 if l != -569
j inc 334 if x >= -1879
i inc -963 if uxa >= 2575
uxa dec -395 if wn == 1106
dma dec 130 if esb != 290
l inc 171 if zp > 1122
esb inc 288 if uxa < 2576
wn inc -672 if i > -445
uxa inc -593 if jq == 565
cty inc -247 if yzp >= 52
vyy inc 480 if cn == 6907
js dec -402 if uxa <= 1982
l dec -121 if cn < 6913
rjx dec 692 if i <= -433
g inc 827 if cty >= 186
d inc -333 if i >= -449
zp inc 438 if u > -2502
gy dec -418 if cn >= 6902
x dec 665 if uxa != 1980
zp dec 497 if l == -283
i inc -51 if rf >= -3694
dma dec -792 if gy < 931
dma dec -189 if wn <= 435
cty dec 337 if g != 1347
zp inc -436 if dma == 2919
yzp dec -184 if rjx <= -1980
gcf inc 688 if qc <= -3174
u inc -536 if zp <= 636
d dec 31 if rjx != -1989
esb dec 523 if wow >= 651
gy inc 566 if zp >= 632
vyy dec 190 if zp == 630
tyg inc 53 if i < -486
x dec -543 if i < -482
uxa dec 403 if epp != -1319
i inc 880 if u == -3033
wn dec 172 if gcf < -428
qc dec 316 if cn > 6904
gcf dec 864 if rf == -3688
esb inc -344 if uxa <= 1583
i inc -234 if js >= -1006
x dec -306 if d < -45
vyy inc -256 if rjx >= -1983
u dec -498 if epp > -1309
esb inc -915 if u == -3033
i inc 910 if esb < -624
l dec 781 if qc > -3505
g dec 803 if u < -3032
dma dec 392 if u == -3033
cty inc 216 if vyy != 2929
zp dec 12 if gy == 930
gy dec -35 if tyg >= -269
wow inc 824 if wow != 642
js inc 974 if epp >= -1319
gcf inc 883 if dma != 2527
jq inc 339 if cty != 72
yzp dec 751 if zp != 627
tyg inc 90 if x < -1679
vyy inc -212 if rjx < -1976
wow dec -642 if epp <= -1302
d dec -643 if x <= -1689
i dec -25 if vyy != 2728
js inc -522 if tyg == -178
js dec -880 if epp == -1311
l inc -899 if x == -1689
u dec 459 if g < 563
tyg dec -485 if i > 1083
zp dec -944 if dma < 2537
cn inc 867 if cty > 57
i dec -95 if jq == 904
gcf dec 548 if d == 595
gcf inc 35 if aqr >= 766
x inc -420 if yzp != -501
x inc -308 if yzp == -500
vyy inc -407 if l <= -1961
gy inc -623 if j < -476
u inc -932 if g > 547
zp dec 18 if js > 324
g dec 776 if cn > 7771
jq dec 581 if cn == 7764
zp inc 392 if g < -215
vyy inc -266 if i < 1185
tyg inc 600 if jq < 906
wow dec 480 if aqr != 770
yzp dec -97 if j <= -488
cn dec 212 if wow == 1284
vyy dec -247 if rjx == -1992
x dec -13 if uxa != 1584
wow inc 929 if vyy < 2330
qc inc 515 if vyy != 2313
wow dec -484 if u <= -4424
wn dec -251 if wow > 2699
dma inc 128 if d > 585
epp inc -429 if cn > 7564
tyg inc 401 if gy < 344
dma inc -757 if qc == -2975
rjx inc 124 if tyg == 1308
j dec -258 if wn < 279
zp inc 265 if u != -4431
epp dec -850 if epp <= -1308
jq inc 875 if cty > 64
esb dec -585 if yzp <= -509
qc inc -663 if vyy != 2313
cn dec 194 if rf < -3682
i inc -821 if rjx > -1862
qc dec -768 if i != 354
rf inc 949 if gcf != -1805
wow inc -350 if gcf == -1807
l dec -72 if dma <= 2663
qc inc -201 if x <= -2106
cty dec -576 if esb == -631
uxa dec 387 if i >= 358
uxa inc 189 if uxa >= 1193
i dec 193 if qc > -3089
gcf inc -164 if wow == 2697
aqr inc -154 if cn < 7364
dma inc 145 if j < -228
cn inc -314 if rf > -2749
qc dec 976 if yzp > -509
tyg dec -277 if j == -226
l dec -339 if yzp < -501
qc dec -615 if js > 320
x inc -678 if rf >= -2743
rjx inc -777 if g > -228
rf dec 151 if vyy <= 2320
js dec 591 if u <= -4422
i inc 166 if d != 585
cn inc -615 if esb < -634
qc dec -148 if dma > 2649
gy dec -93 if d == 595
esb dec 265 if js < -256
wn inc 784 if x != -2795
rf inc 123 if yzp < -499
tyg inc 51 if rf <= -2777
dma inc 273 if tyg != 1575
g dec 546 if uxa == 1386
j inc 787 if l < -1546
rf inc 97 if qc <= -3289
x inc 621 if uxa <= 1388
cn inc -760 if esb <= -894
jq inc -89 if zp <= 2197
uxa dec -485 if wow == 2697
j inc 971 if rjx == -2638
rjx inc 117 if l != -1552
yzp inc -335 if wn < 1059
gy dec -871 if j <= 1541
tyg dec -602 if yzp != -852
rjx dec -852 if cn > 6293
g dec 893 if cn > 6293
uxa inc -524 if cn > 6297
yzp inc 389 if i < 347
epp dec -502 if rf > -2667
dma dec 323 if yzp > -453
l inc -921 if wow <= 2706
yzp dec -534 if jq > 1777
yzp dec 94 if rjx != -1784
uxa inc 254 if vyy != 2320
i dec 224 if gy <= 1313
u inc -466 if vyy < 2318
epp dec -751 if cn <= 6299
qc dec -316 if tyg >= 2197
d inc -666 if epp == 290
g dec -398 if js != -274
rf dec 482 if cn == 6304
vyy inc 295 if cty == 643
wn dec -540 if i != 108
qc inc -334 if wow != 2691
d dec 315 if dma < 2935
qc dec -860 if d != -385
qc dec -202 if gy > 1296
wow dec -67 if vyy > 2610
esb dec 880 if vyy != 2621
cn inc -985 if d <= -383
epp inc -435 if g == -1263
gcf dec 559 if u != -4419
aqr dec 979 if yzp >= -23
jq inc 905 if wn < 1597
cn inc -679 if x >= -2165
rf inc -590 if j != 1532
l dec -73 if wn <= 1598
qc dec 804 if d > -388
j inc 42 if zp < 2198
tyg dec -636 if qc < -3366
gy inc 209 if vyy != 2613
l inc -725 if x < -2158
qc inc -54 if j >= 1531
cty inc 896 if js < -258
u dec -718 if d <= -377
l dec 654 if esb >= -1785
dma dec 912 if i <= 120
uxa inc 827 if cty <= 1543
rf dec -871 if g >= -1271
uxa inc -77 if tyg > 2814
d inc -59 if i <= 121
dma dec 792 if cn == 5309
zp inc 930 if yzp >= -15
gy inc -956 if d == -435
wn dec -205 if j <= 1540
d dec -830 if cty >= 1536
tyg inc -552 if u != -3700
rf inc -616 if gcf != -2533
yzp dec -87 if rf >= -1793
uxa inc -886 if dma <= 1226
g inc 278 if uxa < 1741
j dec 1 if vyy > 2614
d dec 899 if rjx == -1786
rjx dec 238 if vyy >= 2606
cn inc -663 if d <= -505
rjx inc -805 if l != -3785
tyg dec 610 if aqr != -206
gcf inc 842 if yzp <= -12
tyg dec 361 if l <= -3772
wow inc 717 if dma != 1219
vyy dec 182 if gcf != -1695
l inc -972 if vyy >= 2428
cn dec -106 if gy <= 1517
g dec 598 if cn >= 4749
l dec 653 if rf <= -1808
epp dec -768 if esb != -1781
rf dec -566 if uxa != 1740
aqr inc 639 if cn == 4752
wn inc -877 if u == -3699
uxa dec -7 if vyy >= 2427
yzp dec -107 if qc != -3430
yzp inc -426 if esb <= -1776
epp inc 102 if x < -2164
u dec 396 if jq == 2684
dma dec -6 if jq <= 2686
cn inc -739 if gcf > -1698
gcf inc 470 if cty == 1539
gy inc 781 if vyy != 2433
vyy dec 238 if dma < 1236
wow inc 670 if wn != 1801
i dec -230 if jq < 2686
wn dec -500 if wow == 3481
wow dec 865 if j < 1536
x inc -733 if jq == 2684
cty dec 531 if gy >= 1513
g inc 399 if esb < -1768
x inc 201 if rf == -1233
cn inc 394 if d < -515
epp inc -953 if wow >= 2615
qc inc -639 if u != -4095
i inc 935 if vyy > 2191
g inc 846 if gcf != -1218
epp dec 89 if yzp > -334
qc dec -143 if epp >= -317
gy dec 256 if zp != 3132
tyg inc 545 if rjx > -2831
cn dec 913 if gcf != -1226
esb inc -410 if d == -514
rjx dec -71 if tyg <= 1849
rjx inc -127 if yzp > -336
jq inc -206 if gcf >= -1221
i dec -800 if gcf > -1225
aqr inc 98 if gy < 1263
gcf dec 163 if gcf <= -1213
cty dec -965 if zp == 3131
rjx dec -969 if cty < 1976
uxa inc 51 if x == -2698
u dec -380 if wow == 2616
tyg dec -715 if rjx >= -1922
zp inc -825 if yzp >= -323
zp dec 292 if x > -2702
esb inc -415 if d <= -505
wn inc -240 if esb > -2609
gy inc 882 if rf == -1233
vyy dec -638 if gcf <= -1384
tyg dec -63 if jq <= 2479
dma inc -38 if uxa < 1797
qc inc 289 if wow < 2619
d inc 342 if yzp >= -330
cty inc 650 if js < -271
l inc -129 if esb == -2601
jq inc 453 if j > 1531
esb dec 11 if i > 2084
wow inc -729 if wow < 2620
x inc 132 if uxa <= 1796
l dec 818 if vyy <= 2832
tyg dec 804 if yzp != -334
wow inc 285 if rf != -1233
zp dec -329 if wn != 2061
u inc 720 if js == -265
gcf inc 85 if aqr != 528
wow dec -303 if jq != 2472
vyy inc -366 if cty < 1965
yzp inc 598 if cty == 1973
rf inc 685 if jq >= 2471
vyy inc -385 if jq <= 2487
aqr inc 103 if esb >= -2598
d inc 427 if uxa == 1793
rjx inc -736 if zp < 2849
x dec -991 if esb <= -2600
gy inc 642 if gcf <= -1391
yzp dec -726 if wow < 2195
zp dec -582 if u == -3002
rf dec -354 if cn > 3093
g inc -150 if g != -342
uxa inc -381 if wn >= 2060
wow inc -145 if qc < -3628
g inc 589 if qc <= -3626
jq dec -643 if js == -265
yzp dec -768 if cty == 1973
vyy dec 448 if aqr <= 533
uxa dec 507 if cn <= 3103
gy dec 134 if aqr >= 521
x dec 868 if rf != -194
gcf inc 355 if cn >= 3093
rf dec 639 if tyg == 1822
gcf inc -671 if wow >= 2048
jq inc 615 if gy != 2012
zp dec -910 if cn < 3096
rjx inc 949 if wow < 2055
esb dec 623 if jq != 3731
g dec -799 if qc != -3634
uxa inc -352 if rf >= -196
gcf dec 229 if jq != 3729
zp inc 785 if rf > -200
i dec 150 if gcf < -1251
gcf inc -463 if dma != 1189
j inc -424 if wn == 2061
l inc 412 if gcf <= -1715
aqr inc -646 if u <= -2994
i inc 195 if l <= -4471
rjx inc 291 if cty < 1982
epp dec 459 if dma != 1198
u inc -909 if vyy > 1998
uxa dec -208 if vyy > 1994
u dec 727 if i == 1928
x inc -402 if l == -4468
u inc -459 if cn > 3095
j inc 253 if jq < 3733
vyy dec -143 if rf > -204
rjx dec -560 if tyg < 1822
cn dec -602 if jq > 3732
wow dec -231 if wn >= 2066
js inc -40 if x > -1972
gcf dec 591 if rjx >= -842
wn inc -612 if u <= -5090
vyy inc 60 if j >= 1100
cty dec -207 if tyg == 1819
wow dec -862 if dma < 1197
cn dec 534 if i <= 1936
d dec 380 if wow == 2907
x inc 48 if dma < 1199
yzp inc 955 if dma != 1191
dma inc -372 if l <= -4467
uxa dec 83 if wow != 2909
u dec 383 if u > -5102
cn dec 691 if cn <= 3169
vyy inc 952 if jq == 3733
j dec -231 if wn < 1444
i dec -988 if x >= -1931
cn dec -459 if x > -1936
js inc -714 if epp != -779
gy dec 751 if cn <= 2936
epp inc -45 if vyy != 2203
aqr dec -251 if i != 2916
zp dec -248 if x == -1929
vyy dec -917 if tyg > 1814
l inc 906 if rjx > -855
wn dec -262 if aqr >= -120
js inc -293 if j >= 1108
g inc -507 if uxa < 671
tyg dec 848 if uxa < 680
tyg inc 449 if cn != 2931
jq inc 43 if d >= -474
uxa dec 510 if rjx != -852
u dec -15 if cn <= 2936
rf dec 467 if uxa < 688
dma dec 40 if j < 1114
cn inc 256 if l > -3564
uxa dec -193 if cty <= 2173
cn inc 472 if epp <= -773
g dec 509 if qc != -3621
x dec -909 if wow >= 2902
j inc 809 if rf > -663
epp inc 458 if x > -1028
cty dec -210 if wn > 1707
wn dec 778 if rjx <= -848
zp inc 745 if l == -3570
dma inc -86 if rjx != -852
dma inc -964 if aqr > -126
x inc -741 if wow == 2909
cn inc -751 if esb <= -3217
cty dec 910 if qc >= -3636
gcf inc 496 if epp >= -321
rf dec -516 if epp >= -322
dma inc -112 if dma <= -187
yzp dec -376 if zp >= 4451
js dec -829 if qc >= -3629
rf inc -310 if dma > -188
gy inc -312 if wow >= 2899
cn dec 889 if dma > -186
g inc 667 if uxa > 682
u inc 201 if u != -5462
cty dec 589 if yzp > 3082
gy inc -274 if wow != 2907
u dec -533 if tyg == 1420
zp inc 937 if d >= -469
jq dec 18 if cn == 2024
cn dec -59 if rjx <= -850
dma dec -172 if x > -1026
jq dec 129 if gcf > -1231
wn dec 957 if jq < 3627
uxa dec -989 if wn <= 939
jq inc -390 if x != -1013
js inc 251 if g < 385
jq inc -572 if tyg <= 1424
g inc -265 if vyy > 3122
rjx inc 789 if x > -1025
gcf dec -838 if d < -465
g dec -564 if gcf != -387
vyy dec -188 if yzp != 3087
i dec 912 if x < -1015
cty inc -768 if x <= -1019
wn dec 175 if wow == 2906
wow inc 534 if qc > -3627
js inc 247 if x >= -1026
vyy dec 894 if gy == 944
x inc -150 if rf != -445
dma dec 766 if wn < 932
gcf dec -53 if zp >= 5383
g dec 9 if u >= -4740
yzp dec -425 if vyy < 2423
aqr inc 407 if vyy > 2406
gcf inc -244 if tyg != 1429
i inc 120 if i <= 2010
cty dec -40 if rf == -455
x dec -433 if wow <= 2900
u dec -553 if cty != 162
jq dec 329 if u <= -4177
aqr dec -319 if g > 387
esb dec 259 if wow > 2904
jq dec -590 if vyy < 2415
tyg dec -211 if d == -467
rjx dec -209 if l != -3561
tyg inc -564 if aqr >= 281
aqr dec -860 if u != -4186
wn inc -298 if g >= 387
x inc -951 if i < 2122
j inc -141 if cty >= 163
l inc 780 if wow != 2910
zp dec 796 if esb > -3491
j dec 652 if qc < -3628
cn dec 101 if i <= 2128
cty inc -169 if gcf < -573
dma dec -544 if epp <= -309
vyy dec -261 if zp <= 4598
rf inc 166 if esb == -3487
aqr dec 865 if rf == -455
u dec -14 if rjx <= 148
vyy inc -518 if dma < 538
aqr dec -30 if cn <= 1988
rf dec 818 if wow <= 2912
js inc 116 if l > -2786
jq inc 707 if gy > 939
jq dec -673 if x < -1176
wow dec 94 if rjx >= 141
esb inc -654 if esb > -3488
jq inc 982 if i != 2127
cn inc 479 if i > 2122
js dec 530 if rjx >= 144
wn dec -37 if gcf != -578
esb dec -917 if cn > 2458
l dec 519 if wn > 923
uxa dec 671 if wn == 933
zp inc -269 if cn >= 2457
esb inc -329 if wn == 933
tyg inc -490 if d <= -468
dma dec 817 if wn > 930
cty inc -650 if u != -4164
rf inc 804 if l >= -3309
dma dec 629 if esb == -3549
l dec -588 if epp < -317
zp dec 738 if zp >= 4318
l dec -772 if wn != 924
epp dec -423 if js <= -317
g dec 781 if qc != -3637
uxa inc -540 if tyg > 1062
gcf dec 120 if j != 1127
d inc 294 if wow >= 2819
i inc 108 if u >= -4169
x dec -117 if zp > 3581
i inc -434 if gy > 950
jq dec -749 if zp >= 3587
gcf dec -342 if tyg != 1063
j inc -910 if wn != 932
u inc -353 if rjx > 142
qc inc -478 if esb == -3549
uxa inc -992 if wn != 933
cty dec -628 if gcf == -356
u dec -527 if dma != -916
aqr inc -354 if j >= 211
d dec 633 if x >= -1062
l dec 920 if js != -326
wn dec -122 if j > 203
x dec 907 if gcf == -356
epp inc 112 if epp != 105
wn dec 322 if u >= -3988
j inc 934 if wn != 1055
g inc -366 if uxa >= 448
uxa dec -720 if wn <= 1063
zp inc 8 if gy <= 947
esb dec -199 if epp <= 110
cn dec -122 if j != 220
l dec -810 if g <= -763
cty inc -435 if yzp < 3521
zp dec 615 if i >= 2230
jq dec -445 if uxa != 1170
gy inc -654 if u > -3994
tyg dec 48 if dma == -914
gcf inc -346 if dma < -905
wn inc -660 if zp >= 2980
x inc -242 if esb >= -3359
gy inc 28 if rjx < 154
tyg dec 602 if esb <= -3341
jq dec 380 if l > -2061
i inc 502 if js <= -308
uxa inc -255 if qc < -4109
u dec -617 if l == -2051
rjx inc -864 if esb <= -3347
u dec -465 if vyy == 2157
tyg dec 629 if i < 2740
d inc -28 if u < -2907
epp inc -430 if cn <= 2585
vyy dec -740 if yzp == 3515
js inc 387 if gy > 315
vyy dec 792 if gy >= 310
l inc -89 if wn > 389
qc dec -149 if wow > 2810
gy inc 510 if cn == 2583
d inc 381 if g != -757
x inc 633 if gcf != -694
wn dec -677 if j > 222
wn inc -605 if i >= 2729
zp dec 794 if rf < -474
dma inc -560 if js >= 61
rjx dec -364 if wow <= 2819
jq inc 429 if g != -774
rjx dec -305 if j <= 215
epp inc -254 if tyg <= -219
i dec -106 if qc < -3955
g inc -514 if zp < 2983
dma inc 617 if u >= -2911
gy dec -81 if aqr > -40
g inc -722 if wn < -201
zp dec -46 if tyg >= -210
yzp dec 720 if gy <= 830
cty inc 675 if epp >= -318
qc dec 727 if cn >= 2574
zp dec -458 if wow != 2811
jq inc -547 if yzp <= 2800
uxa dec 443 if l == -2140
x dec 76 if tyg == -212
x inc 11 if aqr != -36
x dec 1000 if aqr <= -37
l dec -384 if gy != 822
u dec -329 if aqr == -40
x inc 432 if wn > -202
uxa inc 530 if tyg < -202
i dec -798 if js < 71
dma inc -503 if i == 3645
dma inc 69 if uxa <= 1264
d dec 571 if js != 70
wn inc 134 if u >= -2588
i dec -813 if rjx > -48
epp inc -735 if cn < 2585
aqr dec -223 if dma >= -790
uxa dec -86 if dma >= -789
cty inc 127 if aqr < 188
cty inc 472 if uxa == 1349
cn inc 780 if cty >= 791
cn dec -528 if rjx == -49
l inc 669 if d != -754
gcf dec 427 if epp != -1052
rf inc 465 if dma >= -797
epp inc 452 if g != -2005
epp dec -922 if i != 3648
wow inc -516 if js != 72
esb dec -623 if epp == 314
wn inc 434 if cn == 3111
rjx inc 698 if yzp == 2794
vyy dec -150 if epp < 319
rjx inc 343 if yzp != 2797
jq inc -390 if gy <= 831
yzp dec 296 if l >= -1093
yzp dec 676 if esb > -2730
js dec 320 if epp < 318
jq inc 44 if g < -1996
esb inc 425 if zp >= 3444
i inc -817 if esb == -2727
cty inc -336 if esb != -2727
epp inc -90 if uxa < 1341
zp dec 554 if epp != 312
js dec -299 if epp != 319
zp dec 92 if wn <= 366
wow dec -443 if rf > -12
gy dec -962 if i < 2825
vyy inc 381 if d > -748
wn dec -212 if js <= 54
esb dec -944 if gcf != -1127
cty dec 985 if j != 211
aqr dec 697 if uxa >= 1340
wow dec 459 if gcf < -1120
i inc 728 if dma >= -796
cty inc -277 if tyg >= -216
rjx inc 279 if aqr == -505
yzp inc -38 if zp >= 2793
zp dec 221 if epp == 314
d dec -628 if x <= -2630
uxa dec -846 if esb == -1783
rjx dec 154 if x <= -2625
l inc 541 if wow >= 2290
wow inc -777 if vyy < 2636
rjx dec -492 if uxa >= 2195
gy inc 881 if yzp < 1791
x dec 24 if l >= -1095
jq inc -82 if x >= -2658
tyg dec 763 if aqr > -519
wn dec -242 if esb < -1773
tyg inc 574 if d == -119
d dec 423 if i > 3540",
r#"{{{{{<!>,<!o,!!!>{>,{<<a!>u!>eio!>ui>}},{},{{<!!e!>>},{{<uo'<!!!>>},{<!>'!>},<e!>,<"ea!!!>!>,<>}}}},{{{{<"a">,<!><<>}}}},{{{{{},{<{!!"!><ae<},!>!>!!!>}'!!>}},<!!a{!!!!!>!>,<!!!>,<{oiu<a!!,<>},{<>}}}},{{{{<eoa!!eeea{!,<>}},{{<!!!!,}!>!!a!!!}e!>},<!!!!!>!>i!e!>,<!u<!!{!!!>!!!>>}}}},{{{{{<!ea,uio!}"!>},<!>},<u<!!!!o!<,!>>,<!>},<">},{}},{<ua"!>,<!!!>eouaa"o}!!!>!>,<>}},{<a}!>!!<'a!!<!u!!!!!>!!!>!!!a!>},<!>,<>}},{{{},{{{},<ua}!>!!!>i}}u!!!>},<!>!ii!!'!,>},{{<{>},{<i<!!!e}!>,<!"'a!!u!!{',>}}}},{<!>},<!!!!},o}!!}{!!!">}}},{{{{},{<!>},<!!!!!>}}{'>}}},{{{{<},{a!>,<>}},{<>,<!i>}},{{{<"u!i{!i}a!!!>"'>}}}},{{{<<e!'!!i'!!}{!>,<u,""!!i<ui!!!!!>!!<>},<!!!>'u!!ea!>u'!>}!!!!!>>},{{}},{<"i}u,,>}}}},{{{{{{<',!!,uo!,"<,{!i"{<<o<o>},<<a!>},<!!ee"'!!>},<>},{{},{<{!!!!,!>,<,">}},{{}}},{{{{},<,}i!!,e!>,<!!!!<!!}!!{!>i{ou>},{{},{<!!!>a"!!!>!!!>a,!!i!>,<!<"e"i}!!i'e!!!>>}}},{{<{!>!>},<a<i!>,<uao>,{{<<>},<!!!e}a!>}>}},{{<"!"a,!!!>{}!!u'u{>}}},{{<oe!>},<'!><>},{<!!!>i!!!>!!!oa<!><oeo!>},<>},{<}!!!!"!a,!!'!!!>>,<!!!>!>},<!>>}},{{{<ei<"!">,<io!!!!},!!<!!a<!{a,'ei>},{}},{<}!!!>,<!!!>!>},<!i}<i>,<a',!>{!>},<o!>!!!!!>"o!!!!!{}!o>}}},{{{<">,{}},{{{<"}!i!!oi!!!>},<''>},{}},{{<u!>,<ea!>},<!!!>!!>}}}}}},{},{},{{{{{}},{<u}!>},<,{!!,!!{}oo!>,<!>,<>}},{{{{},<o!a}'!!!>"!iii!>,<{!>},<>},{<!u'}}!>},<!>,<!>{!>,<u}}a!!u,u>}}},{{<"!!!>!>},<!>},<'>,<{a!>},<uuo>},{},{{},{{<!!!>!,!>},<''!!!>,<!>{!!!>!!!>!!!>},<{,!!!!e}i!!,>},<!!!!!<<o!!!>'e!u!!!>e!>u!"">}}},{{{<!>},<!!u>}},{{<<"!>!!!>e!>e!!!>!>,<a!!!!!!!!!'ia>,{}}}}},{{{<!>!!{}!>!>,<>},{{}}},{{<!!!>ai}>}},{{<!!}'!!}!>,<o!>},<{!>},<!!!>,<''!>,<!>},<!!!!,!>>,{<!>,<!a!!!>,<}<'!o>}}}},{{{{{<>,{{<!!!!'!>,<!>},<<oe!!!>i!!{i<!>}e>}}},{{<!!!!!!!>ao<!>},<!>,<!>a!!,!!!>!e!>u>}}},{{},{{<"{a!>}!>},<},},!>>}}},{{<!!!>,!!a!!a!!!>},<i>},{<!!!>}!>,<>}}},{<!!!>},<!>'!!!><,!<!!!>,!ua!!!>e!>},<a!>},<<>}},{{},{{{<{u{aie!!"!!!>a>}}}}},{{{},<a!>},<u>},{{{<!!}<>}},{{{}}}},{<!>!>},<!i<}>,<"!!!>!>},<!>i!!!>o}u{!!!><{>}}},{{{<!>},<<,!>,<!!{!>},<{<o!>},<"i!<'ei>,{}},{{},{<uu!!!!!!o!!ei!!"{e''!>!!!>>}}},{},{{{<e>},<ao>},{<ai!!!!!><}!>,<>,{{<!!!>o,a<!>,<'}>},<o!!{!!!!!!a!oe!!"!,<a>}},{}}}},{{{{{{<o!>!!!!>},{{<i"a!"!e<!>},<u",oou!>ua!!!><>}},{{}}},{{{<>},<o!!!!!>,<!!!>,<a!!e!!!a"!,,!!!e}"e>},{<"!o!!{!i!!!oa!!!>!><,!>>,<e!>!!,u<u,i!!!!!>},<{'"!>e{i'>}}},{{<!>o!>!!!>!!a!e!<!!!>},<o!e}e!>},<ia">,<!!}!!!>uaao!!!>e!ai!!!>>}}},{{{<"!u,!!!>},<!>,<!oo"!>,<!!!>!!!>}i!!!>!!!>'>,{<!!!!!>!!!a{o}<e<!!!>!>>}},{<!au"!!}{,o{!>},<!!>}},{<!>},<a!>,<"!>{'ia!>{'!!!>a!!{>},{{{<}e!>},<!!o}!>{!><!iio!!>}},{{{{<'!!i!!'o{,!e!!!>oi!>},<!!!>!!o>}},{{<,,i!!!>!a!,{{!>,<e!uo!>,<>}}},{{<!!!>},<<u,a>}}},{{}}}}},{{{<!!!>!>!>},<{i!!!!oe>,{<ui!>e!!<>}},{<!!>},{{{{{<"!>},<u!>},<!!<e>},{<o!>},<,!!u{o!e'}'!>,<!!{!>,<uo>,<!!!>>}}},{{{<!!!!!>'e>},<!>,<!e,o!!!!o>}},{}},{{}},{{<a<o>},{<!!,}}i!>,<!e!>},<!!"a!>,<!!{a>}}}},{{}},{{<!>},<!!!>uoi!}eu},!!o!>,<iu{o,>},{{},<,!!!!!!!>!>,<o">},{<o>,{<!>},<!>},<!!a!><u!!!>}}!!!!!>uo>}}},{{},{<o!>},<ia!!!!!>!!u"!!u!>},<>}}},{{{<<{!!!!o!>,<!i<!!"e<!>,<!!}!>},<'a{!i>,<!>>},{{<!>,<!>},<"!>,<{{'>},{}}},{{{{{},{}},{{{{<u!>,!>!!!!!>!u<}iio"i'>}},{<}}<e!>},<o{!i'!!!>ioi"ua>}}},{<<}>}},{{{{}},{<!!>}},{{{{<{',!>},<!>},<aue"!!!>>}},{}},{{<!>},<!!!>!>!>},<>}}},{{{<!>},<'!!ao'!>,<{<!>},<<,aeu>},{<{>}},{<<!>},<!!"}'!>!'uoo!!o"{aii!>},<,a>},{{<a!>>},<!i!!<e!!!>},<,!<!>,<!!!>!!!>!>,<!i,e<'!!!>,<>}}},{{{<!!'!!!>o}i{!><!!"}",e!>},<ii!!{>},{{<<{!>,<!!!>>},<">}},{{<eo!!!>{!>,<,<!!!!i!!!>!>},<<'o>},{}}}},{{<!!!>o{}<!>,<<e>,{{<,}i!>},<!!!>},<!{>}}},{{<{ai!>,<'u,!!!>'!o!>,<i<!>,<}ai>},<ei!!a!!a}!i!!!>,<>},{<!!e!>},<{!!!>>,<!>},<!{!!!o"i!!{>}}},{{<!>},<!>},<',ia,>,{<!>,<ou!e!>'i!>,<iea}e!!o!>},<i!!e<>}}}}},{{{{{{{<e'!o'!!!>,<"!!!>!>},<iu>},{<<!!!><{>}},{<'!>!>},<>,{}}},{},{{{{{{<!!!!,a!>,<!!!>,<u>},{<!'!>},<!>},<!!{'aia!!}!>},<,<!!a!!<io>}},{{{},{{<!>,<!!,}!"}}!!!>i!!!>!>,<!!!>!!<{>,{<<!!!>},<!!e'<!!'>}}}}},{{<!!u}!{,'!>},<!!!><'>,<u"!>},<i>},{{<{!>,<!!!!a<!!!,>},<i!>ia!!!,,!i"}"!!<!>a{!!!>!!}">},{{<}!>,<,>}}}}},{{{<!>,<!>},<,!<!!},e!!!>""a'i!<eo<!!o>}}},{}},{{{{},{{<e<>}},{{{}}}},{{{<e!>>,<!!!>},<!>!!!>,<iu'uo!"!>},<o!o,i!!!><!!!!!!!!>}},{},{{},<!!!!'!>,<ia!!'!!!>!!!>!>u!>!!a>}}},{{}},{{{},{{<,>},<i>}},{{{<<'"oe!!o>},{<!>,<u!>"!"}{a>,<!!'!o<o,!>,<!>,<u}!>},<',!!!!u!o{!o,!"a>},{<!!uu'!!a,'!>{u{!!<!!}!!!!,!>,<>,{{<!>},<!!"}!!!>"!>},<!!!>!uue>}}}},{{<>,{{},{{<!!u'a!!!>!!!!<!!!>!>},<'>},{{<{,'!!!!!>aa!!,uui}!!!>!o!!,e>}}}}},{{<oe!!!!!>>}}},{{{}},{{<!e!ioe>},{<,>}}}}},{{<a!e'e,!>"'<!>,<!!"i}a!>!!>,<ee}iu!}}a<!!},}'!!o,!!>},{{{},{<e!!!>o,!!i'ui!!!!i,a!>},<}>}},<!>},<!>,<,"!>},<!>,<{!>},<,,,>}}},{{<!{eu!>,<ue>,<e!!"'{!>,<!>,<'!>,<aaao"oa{!!<u!>},<>}},{{{<!>!'!!eeu!>},<!>},<<!!}!>},<'u{!!o>,{{<!>},<!!!><!>"!!e!>},<!!u!>},<>}}}},{{{<!!'!!oa'!!!!,!!!>>},<e!!}!!{}'"!>},<!>,<}eue!!e>},{{<!!,!>!>},<o!>!!!>,<a'"!!!>{o!ie!!!>>},{}}}}},{{{{<i!>},<!>},<e!'u!>},<!>!>"!o<!>,<>},<"!!!!'>},{{<!!}ai{<,!!o!,e!!u!>},<<u!>,<>}},{{{{<>}},{}},{<"!>},<}!!!!<<!!}!>},<!>u,"e>}}},{{{<!!!!!>!!!>,<!!i{!>},<!!i{{>,<a>},{}},{{{}},{<i>}},{{{<uuua!>!e">},{<!>,<!!!>}a<,!!!!,e!!!>>}},{<!!,!>},<!>},<!!o!>>,<!!!'!!!!!!!>}"eua!!!>!!!>i!!!>,<!!a!!!>!{!>},<!!>}}},{{{}},{}}}},{{{{{<!>,<ei!!i>},<!>a{o>},{<!!u!>,<i>}},{<i'"{>},{{{{<!{!!!!aa{''!>,!!'"!>,<!!!>,"a!"!>!>,<'>}},{{<!{!"u!>,<o!!!!!>e!!!!!>},<!>!"}!i>},<e'<'!>,<i"}{!>,<!>},<"!!!>">}},{<!>!ui,e!!!>e>,<!>},<>},{}}},{},{{{<a>},<o"o!}!>,<'>},{},{{<<!!!!!>!!ao!!eei!>,<>},<eu"!!!>}<}!!<o>}},{{{},{{{<,i}aui!!}!!!>}!!{!!!>'!<'!>,<<!>},<{>},{<">}},{<!>,>,{<!>,<!!a!>,<o>,<!"au{!!e!>!!}"}>}},{{<!!!>e!>},<o!!e!>e<ea!>!!,!u!!'i!!!',>},<!!!>},'i!>},<,!>!'!>!!"o,}!!!!!>>}},{{<!!{,oai},!!<!!!>'aaa>}}},{{<}!o!>},<>}},{{{<,"}a<o{!>i>},{{<"!!e!!,!!e!>u!!!>!!!>,ue!!o,!>ei>},{{<>},<!!!>},<a'o!!!>,<'{!!!>io<!e}!><!'>}}},{{{<>},{{},{{<!i}uee"e!!!>!!!!!,!!,!!!!!>u{!!"<>,{<'eauu!!,i!!!<a<ea>}}},{}},{{{<!>},<eeiuo!!}<"!>},<ii,<>},<o>}}},{},{<"!>{>}},{{{<<!!!{!>au!!!!!>u!>},<,!>a>}}}}}},{{{},{}},{{<!!e!>!!,!>e"!>},<<e{!>},<'!aeaoi>},<{o!!!!}ue!'<'!!!>!>!'a<!}!!u!i>}},{{{{<'<o!!}>}}},{<!>,<!>!>},<!u!>,e!>,<"!<eu'!>,<,!>i>,{<>}}}},{{{{{{},<!!!>!>},<!<o!!!>i!>!!!>,'>},{<!!!>u!ua!i>}}},{{{{<i!>},<!!<e<!!!><!"!>,<,!!>}},{{{{<!!!>o!!,}!,>}}},<!>!!"}!>i!>!!u!>,<!!!>!!o!>},<i">}},{{{{{<!!u!>,<">},{{}}}},{{{<!!',>},{<"',o!>},<e!!!!ee!!oi{i,>}},{{<'>}}}},{{<>,<'!>o!>},<!!,}<!>,<{{!!!!!>i!>"'!!u>},{<"!>,<<!>!u!!'u!!!>,!!aa!>},<!>!!uo"{>,{<!>},<ei!!!>},<,uu!!!!!!,!>io{!!!!>}}}}},{{{},{}},{{<!!!>,<!!!>!!,!!!>eo}u!!!!!>>,<i{!!!!!>!oi,!!e!!!>,<<!!!>!!!>,<,!>,<'!>>},{{{},<!>},<{ao!""!>'!>,<oaei>}},{<!!!>},<!,!>,<o>,{{<!>},<oiui}"!>,<""u!!!>}!>!!,ii!!>}}}}},{{<e!>},<o!!!>,<",>,{<!!!>iuoa!!!>!>,<e!!!>!!,}a'!<a}!!!!,>}},{{},<!!!>!>,<!>},<!!!,'i}>},{{{},{{}}},{{{<>}}}}}},{{{{{<{!>},<}u!!!!!>!'!!!>!>},<}!!!>!>},<!>},<e,!!!>,!>,<>},<!>ea!!!!a<!!o!!i>}},{{{},{}}},{{},{<!{!>,<>,<<>}},{{{{{{<!>,<!!{!!!!}!!o!!!>}!eu!!!!!>,a"!!a!>ei<>,<!>,<!>},<"!"a>},{{{<!>}!!!>,<}a!i>,{<!>,!>},<i!!!>!!'>}}},{{<<oi!}e<"u!!"}!!!><'!!!>>},{<!!>}},{{<!!!>}'!>},<u!<!>,<{!>,<<>},<u!,!!!>iooi!>e,{<"<io}!>,<>}},{{<!>},<!>},<!>},<o!>ua!!!!<uoi!!!>!>au{,>},<!!!>"e!>uu!>,<}>}},{{{},{{}}},{<ioe{!>,<oe{!!"!!!>!!">}},{{{<ei"!!!>!!!!!!!>,<!!,!!!!oo!!!!e!>,<'!>},<>},{<u'!!!a!!!>},<!>},<!!!<>,<,a,iu>}}}},{{{<<<a!><<!!!!!i"i!!!>,<!!!>{!>},<i!>},<},<>,<!>u{!!"!>!!},!!!!!!ii!>{!!e{aa!>},<>}},{{<>},<<!!!><!>,<i!>},<a,!,!>!>ii<!"o>}}},{{<!!'}}<e}>,{<!!!>o{!>},<!io!!{{au!!o!i''uo,!>},<>}}},{<!>,<!!>}},{{{{},{<e!>!>},<}>,<>}}},{<!ou"!!!!!>a!u!>,<!>},<<}}!!!>,<'<aa>},{<"o!ii"!>,<"}!><!>!!{!>o,!!u!!!!!>},<a{>,<<!!i>}}}},{{{<<"}'e<!>,<}!!!!!>},<,!>,<>},<{!>!>e!!!>!!ei{!>},<{{eoao"i>},{{<{!"{!!!{!!!!!>'!eoee>},<o!!>},{<a!,!!!'u>,{<!>,<!>},<i!"o,i!!!>},<>}}},{{{<ea}a!>!!!>!>!>},<!!!!!>,<e!>},<>,<",''uooa!!'uo>},{{<!!a>},{<u<!!}!!eu!!<!">}}},{{<!>,<u!>!>,<}!a!!!!e<uu!>oa!!!>},<e>},<<!!!>'u}!'""!!!>},<!!iu>}},{{{<!e'">}},{<<'}!>!!!!!>}!!!>!!!!!>i{!>,<!>ie>}}}},{}},{},{{{{<u!>!!!!!><a>,<,<'>},{{}},{<ii"!!<i<!"u!>"!},e{,<>}},{<!>!!}!>,<"a{a,i!,{!!>,<!!i'!}a!!!>},<<!!{!!!!!>},<!!}"o>},{{{{<ou!>,<a>},{<'a,u<!!<iu}i!>!,>}},{{<,!!'!>},<e>,<!>,<!>},<a!>},<,!>aou<!!!!!>!!'!!!>'{!!!ea!>,<>}},{}},{{<!!!>!!>}},{{{{{<ouu!!!i!>},<!!e!!,!!{u}'e>},{<eeea}a>,<!{e!>!!,}a!!!>>}},{<,>}}},{<a!,u'>,{<}{'!>},<u!!',,eu!uo!!!>>}},{<!!u'>,{<",!!!!!!!>!>,<a}e}'{>}}}}},{{{{<!'!>},<!!i!!!o,u!>!!!!!!!>!!'io<!!<!>},<>,<"!!!!!!!a{!>,<>},{}},{{},<!!o!>},<!!!!!>!,!!!>,<}!u,!o!!!>!!!>>}},{{<!>,<a{'{e!,i!>},<!>,<!>,<>,<<',}!,'<!!!''!>},<!>,<,>}}},{{<!<""e!u!"!>},<i!!!>o!!!>"!!,,!!i>,{{{<"e!!!>,<o!>},<!>a!!!a!>,<,""!>},<>,{{<},!!!>!!<i!>,<"iu>,{<u,!>u>}},{<i!>},<!!!!!>u!>}'!>},<!!!>>}}},<e!!!>i!!i!!!>!!!!"o!!!!!!!>}ee<}!>>},{<a!>!!'>}}},{{{<!>,<!!!>'!>!>,<!>},<'!>},<!!u!>!>,<!>,<>},{}},{{{},{<',!!!>!>},<!!{<!!!!!>!!!>o!>},<a>}},{<uie!!!>>},{<{"o}u},!!!>"!!>,<!!!>!>,<!,i!!!>,<!!{"!!!>o!!!>!!u!!!!!>!!!!!>>}},{{{<'o!<!!e}!!e!>,<!}!!u!>},<!>},<ao!!"!>},<!!}!>,<>},{{},<!!,!>!>}ua!!!!>}}}},{<'!>!"!,!>,<!>},<!!!><eoiuu!!<!!!>!e!!!>>}},{{{<i!e<,!!!>!>},<,>},<<o!>,<<>},{{<>},{<!!!>iu!!!>!>!!!!!>!<!!!>a!!!>!!a!u!>>}},{<>,{{<a!>!>},<!>,<!!!>{!!i!u">}}}}}},{{{{{<!>},<}!!!>},<{!!}eu!>},<}i<oo!o>,<{!!ae!a'!>},<!!>},{{<>},<!>},<!>},<{!!!!!!!>,<!>{!}}">},{<{!a!>},<!!!>eu!!{'!>},<!!!i!o!!!ae}!!,>,{{<}!!!>!!<!!<<!!!>{oi{!>},<!!o<'o{>},<!>!>,<!,ea}"!!<!>},<u!}"o!><i>}}},{{<!!!>}e!!!>>}}},{{{{{<!>>}},{{<"i>}}}},{{<a>},{<u{!!!!!>!<a!!{!!}i!!{!!!>!!{{!>,<u}u>}},{{{},<!>aoa!!{!>!!e!!!>!!!>}<'!!},!!e}>}}}}},{{{{{{{{<!!!>"}>,{<a}a"!!!>,o<!!!>!>},<>}}},{{{<!!!>!!e!>!!!>>}},<!>},<a!>,<<!>,<i>},{{<!>},<"{!!!!eo!!!!!>,<",e!><u>},{<oue!>,<"!>,<>}}},{{{}},{{<!>,<e>,{<ui!!!>u!>},<!!ee{"<!!!>!!uiu!>},<,<!>},<>}},<!!{!!,"!!!>!!!!!!!>,<!!!>},<}>},{{{}},{{<>},<!>},<!!!>!>},<{<!>,<!!>}}},{{{<>},<!!}!>"<!>>},{{{{<!>,<!!uu!>e!>e}"!!ei!!!>>}}}},{<'o>,{}}},{{<,!>,<"!!a!>e!}!!!!i!>>},{{{{<>}},{{{<>}}},{<!>!{!!{>,{}}},{{<!>},<u!>oe!>ao>}}},{{}}}},{{{},{{{<!>o,,>}},{{<u!!!>{!!'!>!<''o"{>,<!o!!!!u!!!>'"e!!'<a!!!><}!!"}!>,<>},{<,}!!<a}!"{!>,<e!!!!!>!>!>!!!>}!!!>!>,<oo>}}}},{{{{<}e}!!e!!!!!>{o!!!>!>o>}},{<!'e!!!>!!!>eo!>},<>}},{{{<!,!!{!!a>},{<!!"!!}!!ei!>,<!!u!!!!!>!!au!!!>,<u>}}}},{{{<!>},<!i!!!>,<!!>},<!a{!>,<!!a!!a,!>},<!>"'{i"!>,<!!!>!!>},{{{{<i!>"!!!!'!!>,{<a!!',!!!>>}},{}},{{{<',u!>},<!!<!>!>ueuo!}!!!!e>}},{}}}},{{<,!>},<!>'>}}},{{{<!!!,!!!>i"!ae!>,<!!!>},<e>}},{<,{i{!>},<ai!!}>}}},{{{},<!>,<!>},<<o!,o,<}>},{<a,<}!>!!e>,<}!>,<!>},<!>},<!!o!!!!!>,{"ai"u>}}}},{{{{},{{<!>,<{oi!iu{!!!>!!!>ea>},<e!>!><o!!u!!!!!>!>},<ee,"o{,'>}},{{{<u!!}!!'},!{!!!>!!!>}!>,<u!!!>o!!!!!ia,>}},{{<!>a!"!!{!!!>>,{{{{<o!>!!e'<u!i!>a,e!>,<>},<!!}>},{<{a!>,<{,u!!!>i!!!>!>,<>}},{<!>},<!!!!!>{u>}}},{{<!!<!!!>o!>!!}!>},<"o!!eiia}>,<>}}}},{{<!!!>i!>},<!>!>!"<<!>a<!>o!!!>!!{!!!>!>,<>},{<!!!!!!!>e{e,a'!>},<}"!!!>,<!>,<,}>}}},{{{{{{},{{}},{{},<!!!!eo!'!>,<>}},{{<!!,oaa!a!!!!<'u,!!!>},<{!!i!!uea!>},<!>},<>},{{{<"!>,<!a'!>,<e!>a,"a>},{{{},<>},<!>!!!!>}},{{{}}},{{{<<e,'!!e!!<'a!>},<!>uo{o,!!!>!!<!!'>}},{<iu{u"i'>,{<!!,!>,<i''{}!'eo>}},{{{},{<!!!!a}>}}}}},{{<!>!!!>!!!>'!>!!!>!!!>'!>},<!!!>,<!!!>>}}},{{<!>},<a<'!>,<!i'o'!>},<"!!i!>,<}!>},<}!!,!>!>},<>,<!>,<'u!>,<aa!>,<a!!!>a!>},<i"!!!><,>},{{<!>},<!>,o!!!>!>},<!>},<ei>},{}}}},{{{{{<!>",!!!"iio!!<{'}}!>,<>}},{{<u!>!!{''!"!!!!!>,,>},{<!!!>uuoa'!""!>,<!!eaa!!<!a!!!>>}},{<!a<!>,<>}},{{},{<<"!<!>},<eu!e<!>},<>}},{<"!!!>}ii<!>,<>}},{{{{<>}}},{{<!'uo{!!io!>},<i!"!!!<<',,!!!>iu!!!>>}}},{<i<<ao!!}!>!<!>},<e!!i,!>i!!!!{!">,{<,!<!!{,!>'!!!>a!>},<u!!!>'<!>,<!!!,u!>!!e>}}},{{{{{<e{!>o",>}}},{<!>},<a<!>},<!>,<,>,{}},{}},{{{<o>,<<!>},<!>,<!!!o!!,!,}u!!{>},{<{!>},<!!!>"'>,<!!!>,uu!!eu<!>ueii>},{{<!"e'!!u!>},<u!!!!!>{e<a<>},{{},{<>}},{{<!>',!!}"i!!!!e!>!>,<,>},{<!!!!!!,>}}}}},{{{{<!>!}!!!!ui>,{}},{{<a!!!>!>!>!!!>iea"!"!!!>!u>}}},{},{<<!!!>,<!!!","!!!>!!}!>,<o,}a!!!!!>!>io>,{}}},{<u>,<!!!>!!!!!>{!>},<<"e!!eu!!!"!!eu>},{{<i!>!>},<{ii!>!!!>a!!!!i<<!>},<!>,<!!!>!>>},{<!!e!>,<!!!><a'!!!>{''ie!'>}}}},{{{<!!<!>!!}}!!!!i"i!!o}!>>},{<!>},<a"!!!>i'i!!!>},<<{u>}}}},{{{<!!!>"!!e!>}u>}}},{{{{},{}},<!!,>}},{{}}},{{},{}},{{{{<o!a!!!>!>!!,>},{{<<<!<oa>},{<o}!!!>!>},<!>},<>}}}},{{{<!>i>}},{<ea>,<!{<""<!!!>!!!>{a!i<u!>},<i!!!'!}',e>},{{<o,!!>},{{<o>}}}}}},{{<>},{<e!!!>!>},<<!!!>>,<!>},<"!<!>'{"!!!>!'<e!>o!!}!>,<>},{<"!>},<!!!>!o{a>,{}}}},{{{<u!i}'i'!!!>i!>'a>}},{{<!>,<a}}""a,>,{<a!>,<o!!"iu!>,<'""i!!!>!>!!}e{i>,<!>},<!>!!"!!!>"}<{!>},<}!oe!!!>'i>}}}}},{{{{{{{},{<uu!!!<,}e!!!>>}},{{{{{<a!>},<o}a!!!>>}},<'<'!>a!!o{a}!!!>">},{{<}!>,<"!!!o{>},{<!>},<"ao!>,<!!"a!o!!!>!!!>!o!!!>a!!e,>}}},{{<>}},{<,e{>,{}}}},{{{<!!!}'u!>!e!>,<<<>},<'}o}!">},{{<'<o!!!>a!>,<!!,"!>>},<!!!>},<!>!>},<<ue!!!!u!!!!!!ua!>},<!a>},{{{{{<!!{!>},<'!>!!!>,<!>!!!>,<!!'u!!i<o>,{<!!o!!,euua'!>,<'!>,<!!!>},<}>}},{<{!!!>!"!!{!>ao!>!>},<!!i>,{<!><"!>,<!!!>},<<o!"!!<aa!!!'}u>}},{{<!>,<{,>},{<u!,!u!>},<!!"{>}}},{{<}!!!>},<e!!!!{,!!e!!!>},<!>,<,!!',>}},{{<'i!>},<!!<ua!>},<!>,<!!!!!!!>,<''o<!!!!!>!!!e>,{{{},{{<>}}},<",>}},{{<ie!>!>,<eu<,!!!>,<!{!!!>},<!>},<{u!!i!>,<>,<!!ai!>,<o!!!>}!!u!u{!!!>ai!>,<"'e!>},<>}},{<oe!>},<!!!>!,!!!>uau<e!!>,{<!>,<<{>}}}}},{}}},{{{<e<!>},<!o'u>}},{<o"a""!>},<ae!!}!,!>,<>,<}<,}!u!>,<!!{!>"o,i{!!}>}},{{{<u!!!!!>,a!!!!!a<{e!!i!!<eu!>,<a>,{<!!!>>}},<>},{{<!>},<!><!!!>,<i!!}a!!<!!e{{!>},<e!!!>>},{}}}},{{{{{{<a!>},<o'!!''e"!!<!!!>!o>}},{{}}},{{}}},{},{{{<"<o>}}}},{<!>},<!>},<}}!>},<u!>},<o>,<!!!!!'i!!',}ea!<!>a!>!!>},{{<!>,<}u!>,<o!u!>,<o!>},<a!>,<!!!>,!!!><!>,<!>},<>}}},{{{<{!!ai!!<{o!!!!!!>},{{},<!>!!u{<e<!'!!!>,<e!>,<,{"e!>},<">},{<!><i>,{}}},{{<,!>},<"e!!a<!!a!>},<!>},<""!!!!>,{{<'u<<!>o!>,<'>},<'{i'!!"!>,<!o!!!>ue}<}a!!!!>}}}}},{},{{{{{{<e!!}i!>},<oi!>},<"!>,<!!!>eua>},<{!!!>}>}}}},{{{<!>,<,,u!!!>ue<!!!!}!>,<{{>},{}},{{}}},{{{{<'!!!!o{!>},<,'!>,<,"oe!>},<u!>},<!!o!!}!!i>}}},{{<{!>,<e<e!{'<!>,<!,o!!{!!{>,{{},{}}},{{<a,<o<!!!>!>,<!{!!!!!>!>},<<ui!!!>{{!>,<!!'>,{<!!!>ia"!!}!>,<!{>}}}},{{{},<!!ee,!!!u!!!>},<!>{!>,<a!!}!>!>!i!>},<}!>,<!!'>},{<!'>,{}},{{<!>},<!>!""{ei!!}e!>,<!>,<!!!!"u!'!!!a!>,<!>},<<>},<'!>,<!e<'<oe!!!>a!">}}},{{<u!!!>,!>,<!!ea>,{<"!!"e{,!>e!!!!uei!!!>!!!>!!!!!>!>,<a!!eo>}},{{{{},{{<ie!>},<!>},<oeu}!!!>i'!u!>},<>},{<,!>!>ea!<!>},<e!!{!,!>e>}}},{{{<>}},{{<!'o!>u!!e!>},<a!!!>!!u!!!>},<,!>,<,i!!!>,<!>a>}}},{{<!>},<<a<e'>}}},{}},{<u!!!>u!!!>},<!>,<!!!!!!<u!>,<!!!,!!"!!!!!>!>},<!!!>>,<{e!>{{!!!>!>i!>u!"<!!o!!!>"!>,<'u<>}}},{{{{},{}},{{{<!>,<o!!{!!!>u!!!>!>,<>,{}},{<!>!>},<a'}ou!!'!>},<!!io>}},{{<!!!><i!!!>'>},{<'!!e<{"!!}{o>}}},{{{{<!>}!>!!!}'!!}!>!>},<e,!!}!!e!o!!!!!>!ea>}},{{<u!>,<<!!!>!!!>i<>},<!!o"u{>},{<!!'>,{}}},{{{<<u!!!>!!}!!!!!>,<a!!u"!>},<!aae!e>},<'!!!>!!!>,<"oa"!>},<!!,<!!!>io}!>!!>},{{{{<!>i<!!u"!!ie!>,<,!>!!'a!>},<u!>>},{<u<ea}!>},<!,e<{e!!!>>}},{{<i!>,<!!!>,<!!!>!!!>,<!>},<'!>,<a!!!>!>,<}!>,<<>}},{{{<ou!!{!!""{!!!>!!<!!,"eu<!''!!ei!>},<>},{<!!!>>}},{<i!!!!!><<!>,<!,!!e!>!>},<>}}},{},{{},<{!!!>iao!>e,"o!!!>!!!!!!u!>,<o!"!<,>}},{{<!!}!>,<iei!!o>,<!>,<!!!>o!!{,i!!!>!!!!a!!!>},<,!!"o>},{{{}},{<"!!!!!!'u",!>uui!ai!!'>},{{<!,i!>,{">},{<<>}}}}},{{{},<>},{{{<}'!>,<iu',<>,{{{<'!>},<!!,!>!!!>!!<"{!!!!"a!o<!a">},<a{<!!>},<>}},{{<o}"'o!>},<!!{!!!>{>},{<ua!aeu!>},<}!!!>i{u!!!!!>!>!!>}}},{}}},{{{{<ee<!!!>!<!!o>}}},{{{<ie<!>,<!!i!!i}a<'i"{{'e!i>,<,!!"!>},<!!!>!>},<,!>,<,{!!!>o"<}!!<"!'>}}},{{}}}}},{{<"!u>}},{{{{<i!>!!!>!!!>!!!>>},{{<}<<!>},<!!!>!"a!!o>},{}}},{{{{<!!!>>}},<ea!>,<{u!<{!>,<"}!!!!!!!>"i!!">},<!!!>,<>},{<!>,<a!>,<!>,<a{>,{<!>,<!o!>!!{!>},<!!!>!!u,,!',!>,<e'>}}},{{{{<>}},{{{<!!!>},<}!!!!!!!!o!><}!!!!!>,<,,!!oi!>,<!!e>}}}}},{{{<<!<!!,!>!!!>,<>},<!<i!!e!!!o!>},<a!!!!!>aoi!>,<>},{<!><>,{<!>,<ia!!{!!!!!!}!>},<oi!>!>a}}ua!>},<>}}}}},{{{{{<uu!!!!!!!>}{!!!!!>i{!>},<e!>,<a>},{<!!'<!>},<!>},<>,{}}},{<ei{}e}!!u!!!>,<}!!!><!>u,}>},{{<,!>,<<!!iiou<!>},<{>},<u!!!!!>{}!>!!!>a!>,<>}}},{{{{{{},{<!>},<"!!}io!!i}{iu!>>}},{{{{{<ee!>,<a!!!>{,"!!!!e!!uio!!!>{!!!!>},<!>{!>oe!!!e!>},<i'e!'!!i!!'u"!>,<}!i!>>},<{"{ue''{i>},{{<o>},{<!><o!>},<}!>i!>},<!!!u!>,<}!>!>},<oaoi>}}},<!!i>},{{<oa!>},<!!!>}{!>},<i!!a,}!>},<}!!!>"o!!!>>,<<!>},<!>},<!!!>'!>e}!>,<>},{{},<i!!">}}},{{}},{{<a!!"<a>},{{<!>},<<!!!>e!><ua<,!>},<}!>},<!!!>!>,<'}>}},{<!>},<!>!,{a{!!"!!!>!!'!o<e!!!!ee"!!}<>}}}},{{{<i{,"ai!>!>},<u'i!!>,<!!!!"!!!!}o!!!>!oue}"!!!>},<{!>{!!{>}},{<!>,<{!!!>},<!!!>!>,<!!!>},<,<}!>'!a>,{<!>},<u!!a{!!!>!>},<a!'!!a!!io!>},<!>,<<>}},{{<!}e<o!>!}>}}},{{{}},{{{{}},<{'""a!>o!!!>'!>'!>},<>}}},{{{<!!!!!>!{>}},{{{<ia!>}i<!>,<a!!}>},<!!!>u!>>}},{{<>}}}},{{},{{{},<a!>,<!>!!!>},<!!'!euu,o{!!!>!><'>},{<!ua!,oe'e!ie!>,<!!!>!>},<!!'!!!>a!>},<!e>,<!>,<!!'!>,<"}!!'ou'>}},{{<uo!!o!!"!>,<,>,{}},<u"}!!au!!!!!>o!>},<}!>,<a,}>}},{{{{},{<!>,o'u!!!>},<"i{a!>,<o>}}},{{{<,i!o}u!!!>!!!!!>}>},<{u!!'!!{e<o>}},{{<}!>,<!!!!">}}}}},{{{{<<a!>,<!!!>">},<"{"!!<!{a<<!!o!>,<"o!>,<ii!!o!>},<ae>},{{<a"!>!>ui,e!e!!!><{o<">,{}}}},{{{{<i!!!>!>i<<!!!><!>!!!!!>,<ui}>}},{{},{{{<!!!>!!!!!>>}},{<e!!"!!{"<!!!>!!a!!ai!"{ou!><!!!!>}},{<{!!!>!>},<!!!!!!!!e!>u,u!!>}},{{{},{}},{{<>,{<<!!!>!!!i!>u!!!>,<>,{<!!!>a!>,<,{!!{e!!!!!>i!!!>,<!!!>,<'!'>}}}}}},{{{<<<o!>},<"iu!!ai>},{{{{<{,>}}}}},{{{{{{{<e!!a!!!>,<!u'io!!!>e!!}!!"!',!>,!>>},{<'!o'o}!>,<>}},<!!!>}!u!>,<o!!'!!u!uau,!!!>},<o!!">},{}},{<!>,<u!>"a!>},<!e<'e<"o!>,<!"i{u>}},{{<ao!!!!u!!!>>}}},{<,u>,<e"!><!!!>'o,'o>},{{{{},{<"!>},<aa<>}},{<,u!!!!!>,<!!!>oi!uo!e!!e!>!>!!}>,<!{oea!!!!!!!>!!!>!>,<!!!>!>},<>}}}}},{{},{{<>},<!!!!!',!!",iua{'!!!>},<!u!!!!o!o>}},{{{{{{{<{!!!>!>,<!!ao"!>,<!!!>'!>},<!'oe!!}}!>,<u>},{}},<i!!,"!>>},<>},{{<!<ua!!!>,<!'"!>o>},{<eo!!<!!!'<!!!>}a>,{<'a''ui'!a!!}}'>}}},{{<o!>,<!>>}}},{{{{<e''<!>,<!>},<!{,,>},{{<e,o{e}o,o,e,!!!!>}}}}}},{{<!!i{!!!!oa>},<au!>},<!>!>!!!>,<!>e!>,<!!!>},<o!>},<!!o>},{{<"!>},<!!a!!}<'!!o!!"!!{<{{<!!!>i>,<!!au}o!!'}o}}ou>},{<!!ei!!!>e<!>e!!!>,<a,!!oi>},{{{<!'!>!!!!!!i!!}!'!!!>},<!!!>,<!!!>i{}!!!>,<!!{!>},<a>},{{{<!!!>,!}!>i<>},{<"i,!>,<o"!!!>>}}}},{<}{o>}}}}},{},{{<}o!},}!>,<}u>,<<}<,<!!u!>,<iii>},{{},{{<o!>},<!!!>'e{!>},<">},{<!>a!!!>{'!!{'u,>}}}}},{{{{<!!!>!>},<o!>'ii!!!!aa!!,a!>},<'!oio!!!>>},{}},{{},{}}},{{{{<"io,"'!>,<o}>},{{<!>!!,!!!>},<"!!i}!!!>,<'<i!>>},<!>},<>}},{<!!!>!,!!!>'!>},<''!>},<o{'!>"<>,<!!!ea"!!ioi!!'"}'{>}}}},{{{{},{<!!,io!>!>},<!>,!!!>!i!>,<u!!!!!}a!"!!!>,<>}},{}},{{{},{{<i!>},<oeu!>},<}!!!!"!!!>!!<!"!>},<"'!>},<>}}},{<!!u'!>},<!!ui!!!>}>,{{{},<!>!>},<>}}},{{{<!!!>'o!!!i,e}!!!!!>a,!!'>}}}}}},{{{},{{{<!!!'i!!!!!><!>!!!>e>},{<!>!!!!u>}},{{}}},{{{{<'!>'i'!>},<!"'a!!o!!!>!>,<}!>,<<!>,<!!!!!ue>}},<!>o!!!>,<a!!u"'!!"ao!,o'!!!>!,<!"'>},{},{{<!!!>},<o!},'o!!!>!!!>>}}}},{{},{{<>,{{<!u!>},<!!!>!o!>,<<,u<'oo,!>},<!!!>ea'<>},{<!!!>!!!>!!!!}!!!>},<!!!>a,o'o<!>>}}}},{{<u"!!!>>,{<!>,<!>!>!>,<u!>>}},{<{o!!'>}},{{},{{}}}}},{{{{<>},{<!>,<!!"e!!}a!>},<>},{{},<i}i!>,'}!!!!e!!u!!'!>>}},{{{<!>},<!!!!i{!>},<!!!>!!i,'oi'">}}},{{{{<,{u!!<a',!!!>{!>}}!>},<!a,>},{<!}!><>}},{{{<!>o'!!!>,!!<!>,<!!a"}e},!>i!>,<!!!!>},{<e!!}a!>a!i"!!!!!>u!!a'>}},{{<!!!!!>},<!>u!!>,{<!!!!o>}},{}},{{{}}}},{{{<e!>},<!!i!>!!!>!>,<ae!>,<>},{{{<u!!u{!!!>},<!>'!>},<!!!>''{!>"e!>},<!!!>{<>}},<i}>}}}},{{<"!!!'o!>o<i'e>,{<!u>}},{{{{{},{}},{<i}>}},<o<}{}aa!>},<eae,!>!<e>},{<'!>,<u!!,u!!}!!<>},{<!!!>,<!!!!!!!>!!!>e<<ao!!o'a!!{{!>,<!!>,{{}}}}},{{<o}<!!a'!u!!!>!>},<e'}!!'"u''!!!>>},{{<i!>e!!!>u"}!>,<!>eou{io!!!>,<>,{}},{{<a!!<!u!>!!!!!<!>},<!!!!<{oa"!>,<!}}">}}},{<{<}!>"<{!>,<!!!>!>!>,<!>,<}eei{"a>}}},{{{{},{<!!!>!!!><!"!!e!>},<,,>}},<u!!!>ia<i!!!>{a{!!!>u!<ae!>},<<>},{{},{{{<!>,<!!oa>,<e!!}!!i>},<!>,io!>!}!>},<!!{!>},<>},{{<'uuu<!{{>}}}},{{},{<!>,<e!e!{!u!>},<'!>u!}>}}}},{{{{{<"!>},<o'<,!e'u},"!!!>!!,>,{}},{{<{!>},<u!"!!!!!>o"!!{!!<'>},{<!>ui"!!!!!>!!iu!>},<">}},{<o,ae!!!!}!!ii!iuueo!!",i!ou>,<!!"!>},<aa!!!>},<o<au<!>,<ie!>},<>}},{<!!oi!>!!,!{!>,<o!!!>oe!!!>!u!!,!!aa!!}u>,<!>!>},<!!"<!>},<u'!!!>},<!!!!!>,<>},{{{},{}},{<!!!>},<i!eea'!!!>,<>}}},{{<>}}},{{{{<a!!!!u{!!!>>}},<}!!!!!>},<}{{ua!!u{<"<"!!!>>},{{<!!!>!>,!>e!!!>,<!>!!}!!!!o!!!><'!"}!>'<>,<e!>},<!!>},<{!>},<,<,!!u!oe,"!!a!!!!!!i>},{<{,!!!>"<'i,!!!!'!!,<!>,<"o!!<!!,!i">,{}}},{{},{{},{<eue!>!{<!!e},i!>},<>,{{<ai!!!>i>},{<!,>}}}}},{{<!>},<'u>},{<!>iu'!>},<!>!!"!>,<,o!>},<'}}oe>}}}}}}"#,
"130,126,1,11,140,2,255,207,18,254,246,164,29,104,0,224",
"s,nw,s,nw,se,nw,nw,nw,nw,n,n,se,n,ne,n,ne,ne,n,se,ne,nw,ne,s,nw,ne,ne,sw,se,se,se,se,se,se,se,nw,se,se,sw,n,se,se,se,se,s,n,ne,se,nw,nw,nw,s,s,sw,se,se,s,s,s,se,s,s,n,sw,s,s,s,s,s,nw,s,se,sw,s,sw,s,s,s,ne,sw,sw,sw,s,sw,s,sw,sw,sw,s,sw,sw,sw,sw,se,sw,sw,sw,sw,ne,sw,ne,sw,sw,nw,sw,sw,ne,sw,se,sw,n,n,sw,sw,sw,nw,nw,s,nw,sw,nw,nw,sw,s,nw,sw,nw,nw,sw,sw,sw,nw,sw,sw,nw,nw,nw,sw,nw,nw,nw,nw,nw,nw,nw,nw,nw,nw,nw,nw,sw,nw,nw,nw,se,nw,s,nw,nw,nw,nw,nw,nw,n,ne,n,se,nw,nw,nw,nw,nw,n,nw,ne,se,n,nw,nw,nw,nw,n,n,nw,ne,nw,ne,ne,n,nw,n,nw,n,ne,n,s,n,ne,se,nw,n,n,n,nw,n,n,sw,n,n,nw,n,se,n,nw,n,n,n,n,n,n,n,n,n,n,se,n,nw,n,n,n,n,se,sw,n,n,n,n,n,n,ne,n,s,n,nw,se,n,n,n,n,ne,ne,ne,n,se,n,ne,sw,n,ne,ne,n,se,s,n,n,ne,se,n,ne,ne,n,ne,n,s,ne,n,ne,ne,n,se,ne,s,n,nw,ne,nw,n,ne,ne,ne,ne,ne,ne,n,ne,ne,ne,ne,ne,ne,sw,ne,n,ne,se,sw,se,n,ne,ne,n,ne,ne,ne,s,ne,sw,ne,ne,ne,ne,ne,ne,ne,ne,ne,ne,ne,sw,ne,ne,ne,ne,ne,ne,se,se,ne,nw,ne,ne,ne,ne,ne,ne,ne,ne,ne,se,se,ne,ne,se,ne,ne,ne,se,ne,ne,se,se,sw,s,ne,sw,ne,ne,sw,se,ne,ne,se,sw,ne,se,ne,ne,se,ne,ne,ne,ne,ne,se,ne,ne,se,ne,se,ne,se,se,se,s,ne,se,ne,ne,se,ne,se,nw,ne,ne,ne,se,ne,se,ne,se,se,n,se,se,se,ne,se,se,se,se,se,ne,se,ne,se,se,se,ne,se,ne,sw,se,ne,n,se,se,se,ne,se,se,ne,sw,se,se,n,se,ne,se,se,se,ne,se,se,s,se,se,se,se,se,se,se,se,se,se,se,ne,nw,se,se,se,se,se,se,se,se,se,se,se,se,se,ne,se,se,ne,se,se,se,s,se,se,ne,nw,se,s,se,se,n,se,n,se,se,se,se,nw,s,se,se,se,se,s,s,s,sw,se,se,s,se,se,se,s,s,s,n,s,se,se,se,s,se,se,nw,n,se,se,s,se,se,se,se,s,se,se,nw,nw,s,se,se,se,se,se,ne,s,s,se,se,se,s,se,s,se,se,s,se,se,se,sw,se,se,se,se,se,se,s,s,sw,s,s,s,s,s,s,s,sw,se,s,s,s,se,se,se,se,s,s,n,s,nw,se,s,s,se,se,s,s,se,s,s,s,ne,se,ne,nw,n,s,s,s,se,sw,s,s,s,se,s,s,s,s,s,s,s,s,s,ne,s,s,s,se,s,s,s,s,s,n,ne,sw,se,s,se,s,s,nw,s,sw,s,s,s,s,se,se,s,s,ne,s,s,s,s,se,s,s,se,ne,s,s,s,s,s,nw,s,s,se,s,sw,n,n,s,s,s,ne,s,s,s,s,s,s,s,s,s,sw,s,s,nw,sw,s,s,s,sw,s,s,s,sw,s,s,s,s,n,s,s,s,s,s,s,sw,nw,s,sw,sw,s,s,n,sw,s,s,s,s,se,sw,ne,ne,s,se,sw,s,sw,ne,n,sw,sw,sw,ne,s,s,sw,se,nw,s,s,s,se,nw,s,sw,nw,ne,sw,s,s,sw,sw,s,s,se,ne,s,s,sw,nw,sw,s,s,s,s,ne,s,sw,sw,s,sw,s,s,s,s,sw,sw,s,sw,s,s,s,sw,s,s,s,sw,sw,nw,sw,sw,ne,s,se,sw,sw,s,sw,sw,s,s,sw,s,s,s,n,s,s,sw,s,nw,sw,s,s,s,s,ne,ne,sw,s,sw,sw,sw,sw,s,sw,sw,se,ne,sw,s,sw,sw,s,s,sw,nw,sw,sw,s,nw,nw,s,sw,s,nw,sw,sw,n,sw,s,sw,sw,sw,s,sw,sw,sw,sw,ne,s,s,nw,sw,sw,sw,sw,sw,ne,s,sw,s,sw,ne,n,sw,sw,sw,s,sw,s,nw,sw,sw,sw,sw,sw,s,s,s,n,sw,sw,sw,se,s,ne,sw,s,s,sw,sw,sw,s,s,sw,sw,sw,sw,sw,sw,s,sw,sw,sw,ne,sw,s,s,sw,ne,sw,se,sw,sw,sw,ne,ne,sw,sw,ne,n,sw,ne,sw,sw,sw,sw,sw,sw,s,sw,sw,n,sw,nw,sw,sw,sw,sw,nw,sw,sw,sw,sw,sw,ne,sw,sw,sw,s,sw,sw,sw,sw,nw,sw,n,sw,sw,sw,sw,ne,ne,sw,sw,sw,sw,sw,sw,sw,sw,s,sw,se,sw,sw,sw,sw,sw,sw,sw,s,sw,sw,sw,sw,sw,sw,sw,sw,nw,sw,sw,sw,sw,sw,sw,sw,sw,se,sw,sw,nw,sw,sw,sw,sw,sw,nw,sw,sw,ne,sw,se,sw,sw,se,sw,nw,sw,nw,sw,nw,nw,nw,n,ne,nw,nw,sw,nw,ne,sw,sw,nw,sw,n,sw,s,sw,sw,sw,sw,sw,sw,sw,sw,nw,nw,sw,sw,sw,nw,sw,sw,sw,sw,sw,ne,sw,nw,sw,n,sw,s,n,sw,sw,sw,sw,sw,sw,nw,sw,sw,sw,sw,nw,sw,nw,nw,sw,sw,sw,s,sw,sw,s,sw,sw,nw,sw,ne,se,n,sw,sw,se,ne,sw,nw,nw,nw,sw,sw,sw,nw,sw,sw,nw,nw,nw,nw,s,sw,nw,sw,n,nw,s,sw,nw,n,sw,sw,sw,nw,nw,ne,nw,nw,sw,sw,sw,nw,nw,sw,nw,sw,sw,sw,nw,sw,s,nw,sw,nw,nw,sw,sw,nw,sw,s,nw,nw,ne,sw,sw,ne,sw,se,nw,sw,nw,sw,n,nw,sw,sw,nw,sw,sw,sw,sw,sw,nw,n,nw,nw,s,nw,nw,nw,sw,sw,n,nw,sw,nw,sw,se,sw,sw,s,nw,sw,se,nw,sw,nw,sw,ne,n,sw,ne,nw,sw,nw,sw,sw,nw,ne,nw,nw,nw,s,nw,sw,nw,se,sw,nw,s,sw,s,nw,n,nw,sw,se,nw,nw,nw,n,s,sw,nw,ne,n,sw,sw,nw,nw,sw,nw,nw,nw,nw,sw,sw,nw,ne,ne,sw,nw,n,nw,se,nw,nw,sw,nw,nw,nw,nw,nw,nw,s,nw,nw,sw,sw,nw,nw,sw,nw,nw,sw,sw,se,n,se,s,nw,nw,sw,nw,nw,nw,nw,nw,nw,nw,nw,nw,sw,s,sw,nw,nw,sw,sw,nw,nw,nw,nw,nw,nw,se,nw,n,sw,sw,nw,n,nw,nw,nw,ne,se,nw,nw,nw,nw,nw,nw,nw,nw,nw,nw,nw,nw,nw,nw,nw,nw,nw,s,nw,nw,nw,s,nw,nw,nw,nw,nw,nw,ne,nw,nw,nw,nw,ne,nw,ne,nw,nw,nw,n,sw,nw,nw,sw,nw,nw,nw,s,nw,nw,s,nw,nw,nw,sw,nw,nw,s,s,nw,nw,nw,sw,sw,nw,n,nw,nw,nw,s,nw,nw,nw,nw,nw,nw,nw,nw,ne,nw,nw,n,nw,nw,nw,nw,nw,nw,nw,nw,s,n,nw,nw,nw,n,nw,nw,nw,nw,nw,nw,nw,n,nw,nw,nw,n,sw,se,nw,nw,ne,nw,sw,nw,ne,nw,n,nw,nw,n,ne,nw,nw,nw,nw,nw,nw,nw,se,nw,nw,nw,nw,nw,nw,nw,nw,ne,nw,nw,se,nw,nw,n,nw,n,nw,nw,nw,nw,n,n,nw,se,n,n,nw,n,nw,nw,nw,nw,nw,nw,nw,nw,nw,nw,n,s,n,n,sw,nw,nw,nw,se,nw,ne,nw,nw,nw,sw,nw,nw,sw,n,nw,nw,nw,n,nw,nw,n,n,nw,nw,n,n,n,nw,nw,nw,n,nw,sw,ne,sw,sw,se,n,se,n,n,nw,n,n,n,nw,nw,nw,nw,nw,n,n,n,se,nw,n,n,sw,nw,nw,nw,nw,n,nw,s,nw,n,nw,nw,n,nw,nw,n,nw,nw,nw,n,nw,nw,s,nw,nw,nw,nw,nw,nw,n,nw,n,se,n,n,nw,nw,nw,nw,ne,n,se,sw,nw,n,n,nw,nw,n,nw,sw,sw,n,n,nw,se,nw,n,nw,nw,ne,nw,n,nw,nw,nw,n,n,nw,n,nw,nw,n,s,n,n,nw,n,n,n,nw,n,sw,n,se,sw,nw,n,n,sw,n,nw,nw,n,sw,sw,n,n,s,n,n,n,n,n,n,sw,nw,nw,n,sw,n,nw,nw,se,n,nw,ne,n,n,n,n,n,nw,sw,nw,nw,s,ne,se,nw,n,se,sw,nw,n,n,n,nw,n,nw,nw,n,n,n,nw,s,s,n,sw,n,nw,n,ne,nw,n,nw,nw,nw,sw,n,nw,n,n,sw,n,n,nw,n,n,se,n,nw,nw,nw,ne,nw,n,nw,n,nw,n,n,nw,nw,nw,n,nw,s,nw,n,s,nw,nw,n,nw,n,nw,n,n,sw,nw,n,nw,nw,n,sw,nw,n,nw,n,se,n,n,n,nw,n,nw,nw,se,nw,nw,nw,nw,n,nw,n,n,n,nw,n,n,nw,n,n,nw,n,n,n,n,n,nw,sw,n,nw,n,n,n,n,nw,n,nw,ne,nw,n,n,n,n,n,n,se,n,n,nw,n,n,n,n,s,nw,n,n,ne,n,n,se,n,n,nw,n,sw,n,s,n,nw,n,n,nw,nw,n,n,n,n,ne,n,n,se,ne,nw,n,n,nw,n,s,nw,nw,nw,n,sw,n,s,n,n,n,n,n,n,sw,n,ne,n,n,ne,n,n,n,se,sw,n,n,n,nw,n,n,n,nw,n,n,n,n,se,n,n,n,n,sw,ne,ne,n,n,n,ne,s,ne,n,n,n,nw,n,n,n,nw,n,n,n,n,n,nw,sw,nw,n,n,n,s,ne,n,n,ne,nw,n,n,n,n,n,n,nw,n,n,n,n,n,n,sw,n,nw,n,se,n,n,n,n,se,n,n,se,n,s,n,nw,n,se,n,n,sw,n,n,n,n,n,n,ne,n,sw,ne,n,n,n,n,n,n,n,n,n,n,ne,n,n,sw,sw,n,n,n,n,n,n,n,n,se,n,n,n,se,n,n,n,n,n,ne,se,n,n,n,n,n,n,n,n,n,n,ne,n,n,n,n,ne,n,n,ne,nw,se,n,n,n,n,n,n,n,n,n,ne,s,n,n,n,n,nw,n,n,n,ne,n,se,ne,ne,n,n,ne,n,sw,n,n,ne,n,n,se,ne,n,n,n,n,n,s,s,ne,nw,n,nw,ne,n,ne,nw,ne,ne,n,n,sw,n,n,ne,s,ne,n,sw,ne,n,se,nw,n,n,n,n,sw,s,se,ne,n,n,n,n,n,ne,sw,sw,s,n,nw,ne,n,n,ne,ne,ne,se,n,n,sw,n,se,n,n,se,ne,n,n,ne,nw,ne,n,sw,n,n,n,ne,ne,ne,ne,n,n,n,n,n,sw,n,n,n,nw,n,n,s,n,ne,ne,n,n,sw,sw,ne,n,n,n,n,ne,ne,ne,ne,nw,ne,se,ne,n,n,n,se,n,n,ne,n,sw,nw,n,n,s,n,n,n,n,n,n,n,n,sw,nw,n,n,n,n,n,n,n,n,ne,n,ne,n,ne,n,n,n,ne,ne,n,n,ne,n,n,n,n,ne,sw,n,se,n,n,n,n,ne,n,s,n,n,ne,ne,ne,n,n,ne,ne,n,ne,n,ne,se,n,se,sw,sw,sw,se,nw,ne,nw,ne,ne,n,ne,n,ne,n,ne,ne,ne,ne,n,n,n,ne,ne,n,ne,nw,sw,ne,n,ne,n,n,ne,nw,n,n,n,ne,se,n,n,n,ne,ne,n,n,nw,ne,n,n,s,n,ne,s,s,ne,nw,n,ne,n,n,nw,ne,ne,ne,n,n,ne,s,ne,sw,n,n,n,n,ne,ne,ne,ne,ne,n,sw,n,nw,ne,ne,ne,n,se,ne,s,ne,ne,ne,s,ne,n,n,ne,n,s,ne,n,ne,n,n,ne,n,ne,ne,ne,n,ne,n,nw,ne,ne,ne,ne,ne,ne,nw,ne,ne,ne,ne,n,sw,sw,ne,n,se,s,ne,n,n,nw,ne,ne,sw,ne,n,se,ne,n,sw,ne,n,ne,ne,ne,ne,ne,ne,ne,ne,n,n,n,ne,ne,nw,n,ne,ne,n,ne,n,sw,ne,n,n,ne,ne,n,s,nw,n,se,ne,n,se,ne,ne,ne,sw,n,ne,ne,sw,ne,n,n,ne,n,nw,n,ne,ne,n,ne,ne,ne,ne,se,se,nw,n,ne,ne,n,ne,ne,n,ne,sw,ne,se,n,ne,ne,ne,ne,ne,ne,ne,n,nw,ne,se,ne,n,ne,ne,ne,ne,ne,ne,ne,n,n,n,nw,ne,n,ne,ne,ne,ne,ne,ne,ne,ne,ne,ne,ne,ne,ne,ne,ne,ne,ne,ne,ne,ne,ne,nw,ne,s,n,ne,ne,ne,sw,sw,ne,sw,ne,ne,ne,ne,ne,ne,ne,ne,ne,ne,ne,ne,ne,ne,se,ne,ne,ne,ne,sw,ne,n,ne,ne,n,ne,ne,ne,nw,n,ne,ne,ne,ne,ne,ne,ne,ne,ne,ne,ne,s,n,ne,ne,ne,ne,ne,ne,ne,ne,ne,ne,ne,ne,ne,ne,sw,ne,ne,ne,ne,n,nw,ne,ne,ne,ne,se,ne,ne,ne,ne,sw,ne,ne,nw,ne,n,ne,ne,ne,ne,sw,ne,ne,sw,ne,ne,ne,ne,ne,se,ne,n,ne,nw,ne,sw,ne,ne,sw,ne,ne,n,ne,ne,ne,nw,n,ne,ne,n,ne,nw,n,ne,ne,ne,s,s,ne,ne,n,ne,ne,ne,ne,ne,ne,ne,ne,ne,ne,sw,nw,ne,ne,ne,ne,ne,nw,ne,sw,ne,ne,ne,ne,ne,ne,ne,ne,ne,ne,ne,ne,ne,ne,ne,ne,ne,ne,ne,ne,ne,ne,n,n,ne,sw,ne,ne,ne,ne,ne,ne,ne,ne,ne,ne,ne,ne,ne,ne,ne,ne,ne,ne,ne,ne,ne,ne,sw,ne,ne,se,n,ne,ne,ne,ne,s,ne,sw,ne,ne,ne,ne,se,ne,ne,sw,ne,ne,s,n,ne,ne,ne,ne,n,ne,se,ne,ne,nw,s,ne,ne,ne,ne,ne,ne,ne,ne,ne,nw,ne,ne,s,ne,ne,ne,se,ne,ne,ne,ne,s,ne,se,ne,ne,ne,se,ne,nw,ne,ne,ne,ne,ne,ne,ne,ne,ne,ne,ne,ne,ne,ne,ne,ne,nw,ne,ne,se,se,ne,nw,ne,ne,ne,n,ne,ne,ne,s,ne,ne,ne,ne,ne,nw,n,ne,s,ne,se,s,ne,ne,ne,ne,se,ne,ne,ne,nw,ne,se,ne,ne,ne,ne,ne,ne,ne,nw,ne,se,ne,ne,ne,se,ne,se,ne,ne,ne,ne,ne,ne,se,ne,ne,ne,ne,n,ne,ne,ne,ne,ne,ne,ne,ne,sw,ne,ne,ne,s,ne,ne,ne,ne,ne,ne,ne,ne,ne,n,sw,ne,nw,ne,nw,ne,sw,ne,ne,ne,ne,ne,ne,ne,s,ne,ne,n,ne,n,sw,ne,ne,ne,se,ne,se,se,ne,ne,n,n,ne,nw,ne,s,ne,ne,ne,ne,ne,ne,se,sw,ne,se,se,s,nw,se,ne,ne,s,n,ne,ne,ne,ne,se,se,ne,ne,ne,nw,sw,ne,ne,ne,ne,ne,se,ne,ne,se,se,se,se,se,se,ne,ne,se,ne,sw,ne,ne,ne,se,ne,ne,ne,ne,se,ne,se,ne,se,ne,sw,ne,ne,ne,se,ne,ne,ne,ne,ne,s,ne,ne,ne,ne,s,se,sw,ne,ne,s,ne,se,ne,se,ne,se,se,ne,ne,ne,ne,ne,ne,ne,ne,se,se,ne,ne,se,s,se,ne,ne,ne,ne,se,ne,se,se,nw,ne,se,ne,n,n,ne,ne,s,ne,n,nw,ne,nw,ne,ne,ne,s,s,ne,sw,ne,sw,se,se,se,ne,ne,sw,se,ne,ne,se,ne,se,ne,se,ne,se,ne,s,se,ne,ne,ne,se,ne,ne,se,ne,n,se,se,se,ne,nw,se,se,se,se,se,ne,n,ne,nw,ne,se,se,ne,ne,ne,ne,se,se,ne,ne,ne,ne,ne,se,se,ne,sw,s,se,nw,se,ne,se,n,ne,se,sw,ne,ne,ne,ne,ne,ne,n,ne,se,n,se,ne,ne,ne,n,ne,se,ne,se,ne,ne,se,n,ne,sw,s,se,se,se,sw,ne,se,ne,ne,se,se,nw,se,ne,se,ne,ne,se,se,nw,ne,nw,ne,ne,ne,n,se,ne,n,ne,sw,ne,ne,ne,sw,ne,se,se,se,ne,sw,ne,ne,sw,ne,ne,ne,ne,ne,se,se,ne,ne,ne,ne,ne,n,se,ne,ne,s,ne,ne,se,ne,nw,ne,s,nw,nw,se,se,ne,ne,n,ne,ne,nw,ne,ne,ne,se,ne,sw,s,ne,ne,ne,n,ne,ne,se,se,se,ne,sw,ne,se,ne,sw,se,se,se,ne,ne,ne,ne,ne,se,s,se,sw,se,ne,se,se,ne,se,ne,se,ne,se,ne,se,ne,ne,ne,s,se,se,n,se,se,ne,se,ne,n,s,se,se,se,se,se,nw,ne,ne,se,se,se,se,ne,se,ne,ne,se,se,se,ne,se,se,se,ne,se,ne,ne,se,ne,nw,ne,se,ne,se,ne,ne,se,se,ne,ne,ne,ne,se,ne,ne,se,n,ne,se,sw,ne,ne,ne,se,ne,sw,se,ne,se,ne,se,ne,se,se,ne,ne,nw,sw,se,ne,se,ne,se,ne,se,ne,ne,ne,se,ne,ne,se,n,se,se,se,nw,se,se,se,se,se,ne,se,ne,se,sw,sw,se,se,n,ne,ne,s,se,ne,ne,ne,se,ne,se,ne,se,se,ne,ne,se,se,se,se,ne,se,ne,ne,ne,se,ne,se,ne,ne,se,ne,ne,se,se,se,s,se,se,ne,ne,ne,nw,se,se,ne,nw,sw,ne,se,se,s,se,ne,ne,nw,se,se,n,se,se,se,se,s,se,se,se,ne,se,ne,se,se,se,ne,ne,ne,se,se,nw,se,ne,se,se,se,ne,ne,se,se,ne,ne,n,se,ne,se,ne,se,ne,ne,se,se,se,se,se,se,ne,se,ne,sw,ne,se,n,se,se,s,se,se,ne,ne,se,se,se,se,nw,se,se,se,s,sw,se,se,se,ne,se,ne,se,se,ne,ne,se,ne,ne,ne,se,sw,se,n,se,se,se,se,ne,nw,ne,se,se,s,se,se,ne,s,se,ne,s,se,se,ne,sw,sw,se,se,se,se,se,se,s,se,ne,ne,se,sw,ne,se,se,se,se,ne,se,se,se,se,ne,se,ne,se,se,se,se,ne,ne,ne,ne,ne,ne,n,se,ne,se,se,se,se,ne,nw,se,se,ne,se,n,sw,se,se,se,se,ne,se,se,se,s,se,ne,sw,se,se,se,se,n,se,ne,ne,se,se,se,se,se,se,se,nw,se,ne,ne,se,ne,se,se,se,se,se,ne,se,s,se,sw,se,ne,se,se,ne,n,se,se,se,nw,nw,se,se,se,nw,se,se,s,ne,sw,se,se,ne,se,ne,se,ne,ne,sw,se,ne,nw,se,se,se,se,ne,ne,se,n,se,sw,se,se,nw,se,se,se,se,se,se,se,ne,ne,se,se,se,nw,se,se,se,n,s,n,se,ne,se,s,ne,s,se,se,se,se,ne,se,se,se,se,se,se,se,se,se,se,se,ne,nw,sw,se,ne,se,se,n,se,se,nw,se,se,ne,se,se,s,ne,se,ne,se,se,se,se,se,s,se,nw,se,se,se,se,n,se,ne,se,nw,se,se,se,se,se,s,se,se,se,n,ne,se,se,se,ne,ne,se,se,ne,sw,se,sw,n,se,sw,se,se,se,se,se,se,se,se,ne,se,se,se,se,se,ne,se,s,se,se,se,se,n,se,se,se,se,se,se,se,se,se,se,sw,se,se,sw,se,se,sw,se,se,sw,se,nw,se,se,se,se,se,se,se,se,se,n,se,ne,se,se,se,se,se,se,se,se,se,se,se,se,se,s,nw,ne,se,se,s,se,se,s,se,se,se,se,se,se,se,sw,se,se,se,se,se,se,se,se,se,se,se,se,se,se,se,s,se,se,se,se,se,sw,se,se,ne,s,se,se,se,se,se,se,sw,se,se,se,se,ne,se,se,s,se,se,n,se,se,se,ne,se,s,se,se,s,n,se,s,se,se,sw,se,se,se,se,se,se,se,sw,sw,se,se,se,se,se,se,s,se,se,nw,se,se,se,se,se,se,se,se,se,se,s,se,se,se,se,se,se,se,sw,se,se,se,n,se,se,se,se,s,s,se,se,se,se,n,s,se,n,se,se,se,se,se,nw,se,se,se,sw,se,se,se,s,s,s,se,se,se,se,nw,s,n,se,se,se,se,ne,se,se,sw,se,se,s,se,ne,ne,se,sw,s,s,se,se,s,nw,se,se,se,se,se,nw,se,se,se,se,s,s,se,s,n,se,se,se,se,se,se,s,se,s,se,nw,se,se,se,sw,se,s,s,se,s,se,se,se,se,se,se,se,nw,se,se,nw,se,se,se,se,se,s,se,se,s,se,se,s,s,se,se,se,se,s,se,sw,s,se,se,s,se,ne,se,se,s,se,se,ne,se,se,s,se,se,s,se,s,se,se,se,se,se,s,s,s,se,se,se,se,s,s,ne,n,se,se,s,sw,se,sw,se,sw,se,se,se,se,s,se,s,s,se,se,se,se,ne,se,se,s,s,se,se,se,se,se,n,ne,ne,se,se,s,se,se,se,se,se,s,se,se,se,se,sw,se,se,se,sw,s,se,se,se,ne,se,se,se,se,se,se,se,s,ne,se,se,se,se,sw,s,s,se,se,se,se,se,n,n,se,se,ne,se,se,sw,s,se,se,nw,ne,s,se,se,se,n,n,se,s,ne,se,se,se,se,se,se,se,se,se,se,se,se,se,s,nw,se,se,s,se,se,sw,se,se,s,sw,sw,se,se,s,sw,se,s,s,s,s,se,se,nw,se,n,se,n,se,s,se,s,ne,n,se,n,se,se,se,ne,s,se,se,ne,se,se,sw,se,s,nw,se,se,s,s,se,s,sw,sw,n,se,se,s,n,se,se,se,s,se,se,s,nw,nw,se,se,se,se,se,se,s,se,se,se,se,s,nw,s,sw,se,sw,s,ne,nw,se,se,s,se,se,se,se,se,se,se,se,se,se,se,se,s,s,se,se,se,ne,se,ne,se,ne,se,se,se,se,se,se,s,se,s,n,se,se,n,s,s,s,nw,se,ne,se,se,s,se,se,sw,s,s,s,se,s,s,n,sw,s,se,se,se,s,ne,se,s,n,se,se,se,se,ne,se,nw,s,s,s,nw,se,n,s,n,se,se,se,s,se,s,se,nw,s,n,se,s,ne,s,se,s,se,s,s,sw,se,s,s,se,se,s,se,n,se,n,se,se,se,s,se,se,nw,se,se,n,se,se,ne,se,se,s,s,se,se,se,se,n,se,se,se,ne,nw,se,se,se,se,se,se,se,se,sw,s,se,s,se,s,s,sw,se,se,se,s,se,se,se,se,ne,s,s,se,se,s,se,se,se,se,sw,s,se,se,ne,nw,sw,se,s,se,s,se,n,s,sw,se,se,nw,nw,s,ne,s,se,se,s,sw,se,se,s,s,s,s,se,s,se,n,s,se,s,se,s,se,s,se,se,se,s,se,se,s,s,s,s,ne,se,se,se,ne,ne,s,sw,nw,se,se,s,s,se,s,nw,se,ne,se,se,s,sw,ne,se,se,s,sw,se,s,s,s,sw,s,se,se,ne,ne,se,sw,s,s,s,s,s,se,se,s,s,se,nw,se,s,s,s,se,s,s,se,sw,s,s,s,ne,se,se,s,se,s,se,s,se,nw,nw,s,se,s,ne,se,n,se,se,s,se,se,nw,s,s,se,s,s,se,n,se,se,s,se,se,se,se,s,se,se,se,s,s,se,s,se,ne,s,n,sw,n,sw,s,s,s,se,s,s,s,s,n,se,se,se,s,s,se,se,se,se,sw,s,s,s,se,ne,se,se,s,ne,s,s,se,sw,se,se,s,se,se,se,se,s,s,ne,s,s,s,se,s,se,s,ne,se,s,sw,nw,se,s,s,s,s,s,s,se,n,s,s,se,se,sw,se,s,se,s,se,se,se,s,sw,s,s,sw,se,se,se,se,s,se,sw,s,n,ne,se,s,s,se,n,s,sw,nw,se,s,nw,se,se,se,se,s,ne,s,sw,s,nw,s,se,se,s,se,ne,se,se,se,se,s,se,s,sw,s,n,ne,se,s,s,s,s,s,s,se,s,se,s,se,s,s,s,s,s,s,s,s,se,se,se,nw,se,se,s,s,s,s,se,s,s,nw,s,se,se,s,se,se,ne,sw,se,s,s,se,sw,s,s,se,se,se,s,se,n,sw,s,se,s,se,se,se,se,se,se,se,se,se,se,se,s,s,n,s,s,se,s,se,se,s,s,nw,se,s,se,s,se,s,s,s,ne,s,se,s,s,nw,s,se,se,se,s,n,s,se,s,s,s,s,se,s,nw,se,se,se,se,nw,nw,ne,s,se,se,s,sw,sw,se,se,s,s,s,se,s,s,s,se,se,s,se,se,ne,n,nw,s,s,s,se,se,nw,s,s,s,s,s,s,s,s,se,ne,se,s,s,se,se,s,s,s,s,ne,ne,s,sw,s,se,n,s,nw,nw,se,se,s,se,s,s,nw,s,ne,nw,se,s,s,se,se,se,s,se,s,sw,se,n,se,s,s,s,nw,se,sw,s,s,s,ne,s,s,s,s,s,s,sw,se,se,s,nw,s,se,se,s,s,s,s,s,s,s,se,nw,s,se,s,ne,s,se,s,s,n,s,s,s,se,s,s,s,s,s,se,s,s,sw,s,sw,s,n,s,nw,ne,se,s,ne,s,s,se,n,s,ne,se,s,se,s,s,s,sw,sw,s,s,s,s,sw,s,sw,se,s,n,n,s,s,se,s,se,s,ne,s,se,s,s,s,nw,s,s,se,nw,s,s,s,s,s,s,s,s,s,nw,ne,n,s,n,s,s,se,s,s,se,se,s,s,s,s,s,ne,s,s,s,n,se,ne,ne,s,s,s,s,s,s,s,s,s,s,s,s,s,s,s,s,se,s,s,n,s,s,s,nw,se,ne,s,s,s,s,s,se,s,s,s,sw,s,s,s,s,s,s,s,s,se,s,n,ne,s,s,s,nw,n,s,s,s,s,sw,se,nw,s,se,n,s,s,sw,s,s,s,sw,s,s,s,se,s,s,s,s,n,s,s,s,s,s,se,sw,s,s,s,s,nw,s,s,s,nw,s,nw,s,s,se,s,s,se,s,sw,s,s,s,s,s,se,s,s,n,s,nw,s,s,s,ne,se,se,s,ne,s,sw,s,se,s,s,s,s,s,se,s,nw,nw,se,nw,s,s,s,nw,se,s,s,s,se,s,s,s,ne,se,s,s,s,n,s,s,s,se,s,s,s,s,sw,sw,s,nw,ne,s,s,n,s,s,s,se,ne,s,s,nw,s,s,s,s,s,n,nw,s,s,s,se,s,nw,s,ne,s,s,s,sw,s,s,n,s,s,s,s,s,s,s,n,nw,ne,se,sw,s,nw,n,s,s,s,se,s,s,s,s,s,s,sw,s,s,ne,s,n,nw,s,nw,ne,s,s,s,s,s,s,s,sw,s,s,s,s,s,s,s,sw,s,s,s,s,s,se,s,s,s,s,s,n,s,s,s,s,n,s,s,s,se,ne,nw,ne,ne,n,nw,n,n,nw,nw,nw,nw,sw,s,sw,ne,sw,nw,sw,s,sw,sw,sw,se,sw,sw,sw,sw,s,s,nw,sw,n,s,n,s,s,s,s,n,s,se,se,s,se,se,se,s,se,se,s,s,se,s,ne,n,se,nw,se,se,se,se,ne,se,se,se,n,se,se,sw,se,se,se,ne,se,ne,ne,n,se,s,se,ne,ne,ne,se,ne,s,ne,ne,ne,ne,ne,se,ne,ne,ne,ne,se,n,n,ne,s,s,ne,n,n,ne,ne,se,s,sw,ne,n,s,n,ne,ne,n,s,ne,ne,n,n,nw,n,n,n,s,ne,ne,n,ne,n,ne,n,n,n,ne,ne,nw,n,sw,n,n,nw,n,n,n,n,n,n,n,n,n,sw,n,n,n,n,n,n,n,nw,se,n,nw,n,n,n,n,se,nw,n,nw,n,n,nw,n,nw,n,nw,se,n,n,n,n,s,n,se,se,n,nw,nw,nw,n,n,nw,nw,s,n,nw,nw,nw,nw,nw,nw,nw,nw,nw,sw,n,nw,nw,nw,sw,nw,nw,nw,nw,nw,nw,nw,nw,nw,ne,nw,nw,nw,sw,nw,ne,ne,se,nw,nw,nw,nw,nw,sw,nw,sw,nw,sw,s,nw,nw,nw,nw,nw,nw,nw,nw,sw,sw,nw,nw,sw,nw,sw,n,nw,sw,nw,sw,nw,se,nw,se,s,s,nw,nw,nw,nw,s,sw,n,nw,n,nw,sw,nw,nw,n,sw,s,nw,sw,sw,nw,sw,sw,sw,ne,sw,n,s,sw,sw,sw,sw,sw,n,sw,sw,s,sw,sw,sw,ne,sw,sw,nw,sw,n,s,sw,sw,sw,sw,sw,s,sw,sw,sw,sw,sw,sw,sw,sw,sw,ne,sw,nw,sw,ne,sw,sw,n,sw,sw,sw,ne,sw,nw,sw,s,ne,sw,sw,sw,sw,s,sw,sw,se,sw,sw,sw,sw,s,sw,sw,s,ne,s,sw,s,sw,sw,s,n,s,se,sw,sw,se,se,nw,s,s,sw,sw,n,s,s,s,sw,s,se,sw,sw,se,sw,sw,sw,nw,n,s,sw,sw,s,sw,s,sw,s,s,sw,s,sw,s,sw,s,se,s,sw,sw,sw,sw,sw,s,s,sw,s,s,s,ne,s,s,ne,sw,sw,nw,s,s,se,sw,sw,s,s,s,n,s,s,s,ne,s,s,s,s,n,nw,s,s,s,s,s,s,s,s,sw,s,s,s,s,s,s,s,s,s,s,se,s,se,s,s,s,s,s,s,s,s,s,nw,s,s,nw,s,s,s,s,s,ne,n,s,s,s,s,n,s,s,n,s,s,s,s,se,s,se,s,se,sw,se,s,s,se,s,s,se,s,s,s,s,se,nw,ne,n,se,s,s,s,s,s,sw,n,s,s,s,se,se,s,se,se,sw,se,ne,s,se,sw,se,nw,se,se,s,s,s,se,s,s,s,se,se,s,sw,se,s,n,s,s,se,n,se,se,s,se,s,ne,s,s,s,se,s,se,s,ne,s,se,se,nw,ne,se,se,se,s,s,s,se,s,nw,s,se,s,se,se,n,se,s,se,nw,se,se,se,se,se,se,se,s,se,s,nw,n,se,ne,se,s,se,se,ne,n,se,ne,se,se,se,s,se,ne,se,s,se,se,se,sw,s,s,se,se,s,se,nw,se,se,se,se,se,se,se,se,se,se,se,se,se,ne,se,se,s,se,se,se,s,se,se,se,se,se,se,se,se,se,se,se,s,se,sw,se,se,se,ne,se,se,se,se,se,se,ne,se,nw,se,se,se,se,ne,sw,sw,se,se,se,ne,n,se,ne,nw,se,sw,se,se,se,ne,ne,se,ne,se,se,se,sw,se,se,n,se,se,n,se,se,se,ne,se,se,sw,se,nw,s,se,se,n,ne,se,ne,ne,se,se,se,ne,ne,nw,se,se,s,sw,se,ne,se,ne,sw,s,se,ne,se,se,se,se,nw,se,se,se,se,se,se,se,se,ne,ne,ne,se,se,ne,se,n,se,se,ne,n,sw,se,ne,se,se,se,se,ne,se,sw,ne,ne,ne,ne,se,se,se,ne,se,se,se,ne,n,ne,se,se,ne,ne,ne,se,se,se,se,sw,nw,ne,ne,s,sw,ne,se,sw,se,sw,ne,nw,ne,ne,ne,ne,ne,se,ne,ne,ne,se,se,ne,ne,ne,s,se,ne,nw,se,se,ne,ne,ne,se,sw,ne,nw,sw,ne,ne,n,n,ne,ne,se,ne,nw,sw,se,ne,se,ne,se,se,ne,se,se,sw,ne,nw,s,ne,se,se,ne,sw,ne,ne,nw,s,ne,ne,ne,ne,ne,se,ne,nw,ne,ne,ne,se,ne,ne,ne,ne,sw,ne,ne,ne,ne,ne,ne,ne,ne,nw,ne,ne,se,se,ne,ne,ne,ne,se,ne,se,n,ne,ne,ne,s,se,sw,ne,ne,ne,ne,ne,ne,ne,ne,se,ne,sw,ne,ne,ne,ne,ne,ne,ne,n,n,ne,n,ne,ne,nw,ne,ne,ne,ne,se,ne,ne,ne,ne,ne,ne,ne,ne,se,ne,sw,ne,ne,ne,ne,ne,ne,ne,ne,ne,n,sw,ne,ne,ne,ne,ne,ne,sw,ne,ne,ne,s,ne,ne,ne,sw,ne,ne,n,ne,ne,ne,ne,ne,ne,ne,ne,s,n,ne,se,sw,ne,se,ne,ne,ne,ne,n,ne,ne,ne,se,n,nw,ne,ne,ne,sw,ne,n,nw,s,ne,ne,nw,s,ne,se,ne,ne,nw,n,ne,se,nw,ne,n,ne,ne,ne,ne,n,ne,ne,n,ne,n,ne,ne,n,ne,ne,n,n,ne,ne,ne,nw,ne,ne,ne,se,ne,ne,ne,n,ne,nw,ne,ne,ne,ne,n,n,ne,ne,ne,ne,sw,nw,ne,ne,n,ne,sw,n,ne,ne,ne,ne,ne,n,ne,n,n,n,s,n,ne,se,ne,n,ne,ne,s,s,ne,n,ne,n,nw,n,nw,ne,sw,s,ne,ne,ne,ne,ne,ne,n,ne,nw,s,ne,n,ne,se,ne,ne,n,ne,n,s,se,sw,ne,sw,n,ne,ne,sw,n,ne,n,n,sw,n,n,n,ne,n,sw,n,ne,ne,sw,sw,n,ne,n,ne,n,n,sw,n,n,ne,n,ne,n,ne,nw,ne,n,ne,n,n,ne,n,n,n,ne,n,ne,sw,n,n,se,n,n,ne,ne,s,n,ne,n,ne,ne,ne,ne,n,ne,s,n,ne,n,nw,n,n,ne,n,ne,n,nw,n,n,ne,ne,n,n,n,ne,n,n,n,se,n,n,ne,n,n,n,n,n,n,n,n,n,n,n,s,n,ne,ne,n,n,n,n,n,ne,n,nw,nw,n,n,sw,ne,nw,ne,n,n,ne,n,s,n,ne,ne,n,sw,ne,n,ne,n,n,n,se,n,n,ne,n,n,n,ne,n,n,n,n,n,ne,nw,n,n,s,n,n,n,ne,n,se,n,n,s,n,n,ne,se,n,n,ne,n,ne,n,n,n,ne,se,n,n,sw,sw,n,s,n,se,n,n,n,n,sw,n,n,ne,n,n,n,n,n,n,n,n,n,n,n,n,n,nw,n,n,n,n,n,se,n,s,n,se,n,n,n,n,n,sw,n,ne,n,n,s,n,n,n,nw,n,n,n,n,nw,nw,n,n,n,n,n,s,n,n,ne,n,n,n,n,nw,sw,n,n,n,s,n,nw,s,n,s,n,n,n,n,n,n,n,n,n,n,n,n,n,se,nw,n,n,n,n,nw,n,sw,s,nw,n,n,n,n,n,n,se,n,n,n,n,n,n,ne,nw,n,n,n,n,n,se,n,n,n,n,n,n,n,n,n,n,n,n,sw,n,n,nw,se,se,nw,ne,n,n,n,nw,n,n,n,n,n,n,nw,n,nw,nw,s,nw,n,n,n,n,nw,nw,nw,nw,ne,n,nw,n,nw,n,ne,s,nw,se,n,nw,nw,nw,sw,n,n,se,nw,nw,nw,n,n,nw,nw,n,n,n,nw,nw,n,nw,sw,n,n,n,n,n,nw,n,nw,n,nw,s,n,nw,ne,nw,se,n,se,n,nw,n,n,s,n,s,n,n,nw,nw,ne,nw,n,sw,ne,nw,ne,nw,n,nw,nw,nw,se,nw,n,n,nw,se,nw,nw,n,se,n,n,n,n,n,nw,n,ne,n,n,n,sw,nw,nw,nw,nw,nw,n,n,n,s,nw,se,nw,nw,n,n,s,nw,n,ne,nw,n,se,n,nw,n,nw,nw,nw,n,n,nw,nw,nw,nw,n,nw,n,nw,nw,s,n,nw,n,nw,s,nw,n,sw,ne,nw,se,nw,nw,ne,n,n,n,se,nw,n,nw,nw,nw,nw,ne,nw,s,nw,nw,nw,nw,n,n,nw,se,nw,n,nw,ne,nw,sw,n,se,n,nw,sw,nw,nw,n,s,nw,nw,s,ne,nw,n,n,n,nw,n,n,nw,n,nw,n,nw,n,n,sw,nw,n,nw,n,nw,n,se,ne,nw,nw,n,sw,nw,n,nw,nw,nw,ne,nw,n,nw,nw,nw,nw,nw,nw,n,n,n,nw,ne,nw,n,n,s,nw,nw,ne,nw,nw,nw,nw,nw,nw,se,nw,n,nw,nw,nw,nw,nw,nw,n,nw,ne,n,nw,nw,nw,n,nw,nw,nw,nw,nw,n,nw,ne,nw,sw,s,nw,sw,nw,s,nw,nw,nw,se,n,n,sw,nw,nw,s,nw,nw,nw,s,n,ne,n,nw,s,n,n,nw,n,n,n,nw,nw,nw,nw,n,nw,nw,n,nw,nw,nw,s,sw,nw,nw,n,nw,n,nw,n,s,nw,nw,n,sw,n,nw,nw,n,nw,sw,se,n,nw,nw,nw,nw,n,n,nw,nw,n,nw,n,nw,nw,nw,nw,nw,se,nw,n,nw,nw,nw,n,nw,nw,nw,ne,s,nw,n,nw,s,se,n,nw,nw,n,nw,s,se,n,n,nw,nw,nw,sw,nw,n,nw,sw,nw,nw,s,nw,nw,nw,s,nw,s,sw,nw,nw,nw,nw,nw,nw,ne,nw,nw,se,nw,nw,nw,nw,n,nw,nw,se,nw,nw,nw,nw,nw,nw,n,s,nw,nw,n,nw,sw,nw,nw,nw,nw,nw,nw,s,nw,nw,nw,nw,sw,nw,nw,nw,nw,nw,nw,nw,nw,nw,nw,nw,nw,nw,nw,nw,nw,nw,nw,nw,nw,nw,nw,n,nw,nw,sw,nw,nw,nw,nw,nw,nw,nw,nw,nw,se,nw,nw,nw,nw,n,sw,nw,sw,n,nw,nw,nw,se,nw,nw,sw,nw,sw,se,sw,nw,nw,sw,nw,nw,nw,nw,nw,n,s,ne,nw,s,nw,nw,nw,nw,n,nw,nw,s,nw,nw,nw,nw,nw,ne,ne,nw,nw,se,nw,sw,nw,nw,nw,nw,nw,ne,nw,nw,nw,nw,sw,ne,nw,nw,nw,sw,nw,nw,s,se,nw,nw,s,nw,nw,ne,nw,nw,nw,se,nw,se,n,nw,nw,sw,se,sw,nw,se,sw,nw,nw,nw,n,sw,nw,nw,nw,nw,nw,nw,se,nw,nw,sw,nw,sw,sw,nw,ne,nw,nw,sw,ne,nw,sw,sw,nw,nw,nw,sw,nw,nw,nw,n,sw,se,nw,sw,nw,nw,nw,se,nw,nw,sw,ne,nw,sw,sw,n,nw,nw,nw,sw,nw,nw,sw,nw,nw,sw,se,se,nw,nw,nw,nw,nw,nw,nw,sw,nw,sw,nw,nw,se,n,se,sw,nw,sw,nw,sw,se,nw,nw,sw,nw,nw,nw,n,sw,nw,nw,nw,sw,nw,sw,nw,nw,se,nw,nw,nw,nw,nw,ne,nw,ne,nw,nw,nw,nw,nw,nw,n,sw,nw,sw,n,nw,sw,sw,nw,nw,nw,n,n,nw,nw,sw,nw,nw,nw,sw,nw,nw,n,nw,n,nw,n,nw,nw,nw,nw,sw,sw,nw,sw,n,sw,sw,nw,nw,sw,sw,nw,sw,sw,nw,nw,nw,ne,nw,nw,nw,sw,n,nw,sw,nw,ne,sw,nw,ne,nw,nw,n,ne,nw,n,nw,s,n,sw,nw,nw,sw,nw,nw,s,nw,sw,sw,nw,se,sw,nw,sw,nw,sw,sw,sw,nw,s,se,sw,nw,nw,sw,se,sw,sw,ne,nw,nw,sw,sw,se,ne,nw,nw,s,se,ne,sw,nw,nw,nw,nw,sw,ne,ne,sw,nw,sw,nw,ne,nw,s,sw,sw,sw,nw,s,sw,nw,nw,nw,sw,nw,nw,ne,nw,n,nw,nw,nw,sw,nw,sw,sw,nw,nw,nw,nw,ne,ne,nw,nw,nw,s,s,sw,n,nw,sw,sw,sw,sw,sw,sw,nw,nw,s,s,nw,nw,sw,sw,sw,sw,nw,sw,sw,ne,nw,n,nw,nw,ne,sw,s,sw,ne,sw,ne,sw,sw,sw,sw,nw,sw,nw,sw,sw,nw,nw,s,nw,sw,nw,nw,n,sw,sw,sw,sw,nw,sw,nw,sw,n,nw,se,nw,sw,sw,sw,sw,nw,sw,sw,sw,sw,sw,nw,nw,sw,nw,sw,sw,sw,nw,sw,sw,sw,sw,sw,nw,ne,sw,nw,nw,nw,sw,nw,nw,nw,sw,sw,sw,nw,nw,sw,sw,sw,se,sw,nw,nw,sw,sw,ne,sw,sw,nw,sw,n,sw,nw,nw,sw,sw,sw,nw,sw,nw,sw,sw,s,nw,sw,sw,sw,se,s,sw,sw,sw,nw,s,sw,sw,sw,sw,sw,nw,sw,nw,sw,nw,nw,nw,sw,nw,sw,nw,nw,sw,nw,sw,sw,nw,sw,se,sw,sw,nw,sw,se,sw,ne,ne,sw,sw,sw,sw,nw,s,nw,nw,se,sw,nw,sw,nw,sw,sw,sw,sw,se,se,s,sw,sw,sw,nw,sw,nw,sw,nw,ne,nw,sw,sw,sw,s,s,sw,sw,sw,nw,se,sw,nw,sw,sw,se,nw,s,nw,sw,nw,sw,se,ne,sw,sw,sw,nw,nw,nw,nw,sw,nw,ne,sw,sw,nw,sw,sw,sw,sw,se,se,sw,sw,se,s,sw,se,ne,se,sw,sw,sw,sw,sw,sw,sw,sw,sw,sw,sw,sw,sw,sw,nw,nw,nw,nw,sw,sw,sw,sw,sw,sw,sw,nw,sw,s,sw,nw,sw,sw,sw,sw,sw,sw,sw,sw,sw,sw,sw,sw,sw,sw,sw,se,sw,n,sw,sw,sw,ne,sw,sw,sw,sw,s,ne,sw,sw,n,sw,sw,sw,nw,sw,sw,se,sw,sw,sw,sw,nw,sw,sw,sw,sw,sw,sw,sw,sw,se,ne,sw,sw,sw,nw,nw,sw,sw,se,sw,sw,sw,sw,sw,sw,sw,sw,sw,sw,sw,sw,sw,sw,sw,nw,sw,n,sw,sw,sw,sw,sw,sw,sw,nw,sw,sw,sw,sw,sw,sw,se,sw,s,n,s,s,sw,sw,sw,sw,ne,sw,n,se,sw,sw,sw,sw",
"0 <-> 396, 1867
1 <-> 1749
2 <-> 466, 675, 1661
3 <-> 3, 328, 1160
4 <-> 4, 953
5 <-> 1922
6 <-> 1273
7 <-> 7, 959
8 <-> 96, 274
9 <-> 9, 1036
10 <-> 1783
11 <-> 879
12 <-> 382, 1390
13 <-> 1214, 1473
14 <-> 434, 508, 1470
15 <-> 324, 747
16 <-> 195, 894
17 <-> 536
18 <-> 1024, 1198
19 <-> 1616
20 <-> 797
21 <-> 885
22 <-> 1001
23 <-> 569, 1496
24 <-> 392
25 <-> 32
26 <-> 820
27 <-> 655
28 <-> 1044
29 <-> 307, 675, 1518
30 <-> 1547, 1775, 1885
31 <-> 237
32 <-> 25, 1736
33 <-> 390
34 <-> 279, 1423
35 <-> 1367, 1761
36 <-> 154, 353, 1047
37 <-> 1512
38 <-> 1618, 1665
39 <-> 1004
40 <-> 276, 335
41 <-> 315
42 <-> 340, 901, 954
43 <-> 415, 652
44 <-> 550
45 <-> 145, 1352
46 <-> 1653
47 <-> 711
48 <-> 274, 1155
49 <-> 1960
50 <-> 263, 372, 395
51 <-> 51, 327, 1530
52 <-> 1506, 1875
53 <-> 53, 213, 1872
54 <-> 270
55 <-> 997
56 <-> 1649
57 <-> 789
58 <-> 98, 826, 1548
59 <-> 1918
60 <-> 447
61 <-> 448
62 <-> 966, 1152, 1895
63 <-> 883
64 <-> 1320, 1656
65 <-> 417, 1263
66 <-> 750, 1708
67 <-> 1198, 1324, 1929
68 <-> 1932
69 <-> 765
70 <-> 221, 624, 1745
71 <-> 599, 1120
72 <-> 106, 485, 986
73 <-> 765, 1008, 1822
74 <-> 464
75 <-> 1856
76 <-> 798, 1575
77 <-> 122, 706, 1720
78 <-> 236, 583, 1505
79 <-> 711, 1480, 1809
80 <-> 1379, 1705
81 <-> 315, 984, 1440
82 <-> 758
83 <-> 779, 1768
84 <-> 100, 1427
85 <-> 178
86 <-> 490
87 <-> 982, 1978
88 <-> 1329, 1485, 1845
89 <-> 327, 963
90 <-> 1740
91 <-> 242
92 <-> 351, 1290
93 <-> 172, 1052
94 <-> 1333, 1598, 1856
95 <-> 567
96 <-> 8, 732
97 <-> 97
98 <-> 58, 1219, 1330
99 <-> 231, 544, 848, 1923
100 <-> 84
101 <-> 1438
102 <-> 705
103 <-> 508, 677
104 <-> 104
105 <-> 519
106 <-> 72
107 <-> 107, 1169
108 <-> 277
109 <-> 1746
110 <-> 686
111 <-> 115, 903, 915
112 <-> 493
113 <-> 113
114 <-> 1217, 1437
115 <-> 111
116 <-> 535, 1701
117 <-> 117, 910
118 <-> 192, 982
119 <-> 119, 1274
120 <-> 1800
121 <-> 970
122 <-> 77, 1208
123 <-> 1073, 1984
124 <-> 530, 600, 1254
125 <-> 125
126 <-> 166
127 <-> 563, 607, 1389
128 <-> 260, 1160
129 <-> 683, 977, 1366
130 <-> 981
131 <-> 1113, 1209, 1461
132 <-> 411
133 <-> 794, 1478
134 <-> 134, 419
135 <-> 755
136 <-> 507
137 <-> 855
138 <-> 1434
139 <-> 1623, 1989
140 <-> 796, 1133, 1414
141 <-> 476
142 <-> 385, 1708
143 <-> 932, 1616
144 <-> 1014
145 <-> 45
146 <-> 146, 369, 489
147 <-> 1901
148 <-> 676
149 <-> 188, 377
150 <-> 318, 1128
151 <-> 549, 1478
152 <-> 641, 1090
153 <-> 1770
154 <-> 36, 585
155 <-> 1450
156 <-> 197, 1764
157 <-> 157
158 <-> 158
159 <-> 1951
160 <-> 1420, 1782
161 <-> 1587, 1990
162 <-> 1158, 1927
163 <-> 372, 1653, 1796
164 <-> 1916
165 <-> 1653, 1864
166 <-> 126, 166, 1497
167 <-> 167, 679
168 <-> 308, 1176
169 <-> 378, 475, 921, 1057
170 <-> 651, 1175
171 <-> 1275
172 <-> 93, 172
173 <-> 1763
174 <-> 174, 1318
175 <-> 243, 866, 1053, 1495
176 <-> 1212
177 <-> 949
178 <-> 85, 882
179 <-> 1427, 1704
180 <-> 1802
181 <-> 1362
182 <-> 764, 793
183 <-> 1076
184 <-> 184, 969, 1250, 1581
185 <-> 227, 833, 1835
186 <-> 568, 1378
187 <-> 1142
188 <-> 149
189 <-> 873
190 <-> 1650
191 <-> 1101, 1131, 1313
192 <-> 118, 244, 278, 1025, 1232, 1554
193 <-> 193, 570
194 <-> 772, 1595
195 <-> 16
196 <-> 1173
197 <-> 156, 925, 1880
198 <-> 320, 1489, 1675
199 <-> 199
200 <-> 342, 875, 1787
201 <-> 415
202 <-> 1472, 1846
203 <-> 360, 1187
204 <-> 558
205 <-> 1026
206 <-> 795, 1405
207 <-> 207, 658
208 <-> 780, 1587
209 <-> 661
210 <-> 367, 1620
211 <-> 1473, 1783
212 <-> 656
213 <-> 53, 724, 1017
214 <-> 1282, 1735
215 <-> 1564
216 <-> 1784
217 <-> 824
218 <-> 218, 1238
219 <-> 675, 1096
220 <-> 1154
221 <-> 70, 1743
222 <-> 922, 1268
223 <-> 1646
224 <-> 850
225 <-> 1072
226 <-> 582
227 <-> 185, 499
228 <-> 465
229 <-> 712, 1767
230 <-> 1259, 1916
231 <-> 99
232 <-> 1024, 1850
233 <-> 1443
234 <-> 397, 708, 1296
235 <-> 967, 1179
236 <-> 78, 1380, 1826
237 <-> 31, 287, 659
238 <-> 238
239 <-> 557
240 <-> 1753
241 <-> 363, 731
242 <-> 91, 1713
243 <-> 175
244 <-> 192, 721, 820, 1277, 1785
245 <-> 961
246 <-> 1509, 1986
247 <-> 1781
248 <-> 1381
249 <-> 879
250 <-> 391, 1268, 1799
251 <-> 326, 771
252 <-> 806, 1371, 1818
253 <-> 846
254 <-> 1414
255 <-> 1098, 1379, 1724
256 <-> 1078
257 <-> 705, 1078
258 <-> 1264, 1865
259 <-> 524, 1140
260 <-> 128, 1270, 1559
261 <-> 1125, 1306, 1541
262 <-> 1199, 1981
263 <-> 50
264 <-> 264
265 <-> 1123, 1453
266 <-> 774, 833, 1694, 1797
267 <-> 1562, 1574
268 <-> 425, 1611
269 <-> 595, 851, 1873
270 <-> 54, 1410, 1828
271 <-> 325, 1866
272 <-> 294
273 <-> 480
274 <-> 8, 48, 1012
275 <-> 1299
276 <-> 40, 1767
277 <-> 108, 1539
278 <-> 192, 1660
279 <-> 34, 1157, 1375
280 <-> 1654
281 <-> 627
282 <-> 288, 340
283 <-> 1323, 1618
284 <-> 284
285 <-> 1110
286 <-> 550, 756, 863, 1735
287 <-> 237, 1615, 1847
288 <-> 282, 1347, 1782
289 <-> 720, 1006, 1260
290 <-> 1272
291 <-> 1891
292 <-> 292, 1215, 1549
293 <-> 825
294 <-> 272, 484, 1789
295 <-> 818
296 <-> 974, 1870
297 <-> 551, 859
298 <-> 298, 1087
299 <-> 299, 1946
300 <-> 300, 1002, 1029, 1848
301 <-> 733, 1768
302 <-> 1156
303 <-> 1366
304 <-> 1632
305 <-> 1610
306 <-> 1292
307 <-> 29
308 <-> 168, 822
309 <-> 309, 636, 997
310 <-> 344, 836
311 <-> 311, 471
312 <-> 1498
313 <-> 738, 1221
314 <-> 1018
315 <-> 41, 81, 935, 1552
316 <-> 325, 1189
317 <-> 595
318 <-> 150, 481, 1859, 1949
319 <-> 1912
320 <-> 198, 1046, 1311
321 <-> 824, 1590
322 <-> 393
323 <-> 634
324 <-> 15, 1385
325 <-> 271, 316, 1090
326 <-> 251, 503
327 <-> 51, 89, 1186, 1617, 1696, 1869
328 <-> 3, 665
329 <-> 544, 1816
330 <-> 425
331 <-> 1214
332 <-> 332, 418, 758
333 <-> 333
334 <-> 1536
335 <-> 40, 963
336 <-> 359, 690, 1613
337 <-> 667, 1004, 1532, 1900
338 <-> 338, 1970
339 <-> 1077
340 <-> 42, 282, 1421
341 <-> 341, 616
342 <-> 200, 1514
343 <-> 368, 1273, 1570
344 <-> 310, 473
345 <-> 1031
346 <-> 1455
347 <-> 1701
348 <-> 729, 1342
349 <-> 1517
350 <-> 1918
351 <-> 92, 1235
352 <-> 1406
353 <-> 36
354 <-> 744
355 <-> 494
356 <-> 424, 1960
357 <-> 1827
358 <-> 612, 781, 1245, 1303, 1455
359 <-> 336, 993, 1193, 1693
360 <-> 203, 1937
361 <-> 1791, 1795
362 <-> 873
363 <-> 241, 1802
364 <-> 670
365 <-> 959, 1085, 1432
366 <-> 689
367 <-> 210, 1040
368 <-> 343, 716, 1382
369 <-> 146, 1013, 1305
370 <-> 948, 1935
371 <-> 1612, 1782
372 <-> 50, 163
373 <-> 373, 495
374 <-> 1127
375 <-> 913, 1844
376 <-> 620, 982, 1103
377 <-> 149, 1247, 1251
378 <-> 169
379 <-> 1902
380 <-> 701, 1654
381 <-> 381
382 <-> 12, 526, 1123, 1868
383 <-> 391, 1458
384 <-> 384
385 <-> 142, 989, 1442, 1511
386 <-> 588, 1437, 1532
387 <-> 1917
388 <-> 1738
389 <-> 629, 1511
390 <-> 33, 390
391 <-> 250, 383
392 <-> 24, 701, 1726
393 <-> 322, 450, 1900, 1962
394 <-> 394
395 <-> 50
396 <-> 0, 464, 563
397 <-> 234, 401, 1599
398 <-> 398
399 <-> 399, 1452, 1966
400 <-> 474, 903
401 <-> 397, 729
402 <-> 922
403 <-> 1544, 1756
404 <-> 1192, 1634
405 <-> 1602
406 <-> 675
407 <-> 1670
408 <-> 477
409 <-> 1442
410 <-> 1087
411 <-> 132, 705, 1741
412 <-> 938, 1134
413 <-> 530
414 <-> 1346
415 <-> 43, 201, 1381
416 <-> 1122, 1730
417 <-> 65, 417
418 <-> 332
419 <-> 134, 1044, 1592
420 <-> 651, 1466
421 <-> 882, 1295
422 <-> 486
423 <-> 1870
424 <-> 356
425 <-> 268, 330, 575
426 <-> 1850
427 <-> 427
428 <-> 497, 594, 1295, 1863
429 <-> 1263
430 <-> 1525
431 <-> 1111
432 <-> 599, 1050
433 <-> 987, 1279
434 <-> 14, 434
435 <-> 785, 1381
436 <-> 459, 1141, 1618
437 <-> 944
438 <-> 1674, 1928, 1945
439 <-> 1638
440 <-> 622
441 <-> 1648
442 <-> 1099
443 <-> 815, 967
444 <-> 484, 859, 1446, 1717
445 <-> 649, 799, 957, 1185, 1412
446 <-> 639, 1952
447 <-> 60, 743
448 <-> 61, 783, 1085
449 <-> 539, 1257, 1319
450 <-> 393, 1167
451 <-> 1918
452 <-> 1579
453 <-> 821, 1886
454 <-> 1075, 1633, 1723
455 <-> 455
456 <-> 959
457 <-> 457, 1871
458 <-> 458
459 <-> 436
460 <-> 722, 1273
461 <-> 878, 1093
462 <-> 546
463 <-> 809
464 <-> 74, 396
465 <-> 228, 577, 1483
466 <-> 2, 769
467 <-> 1304, 1368, 1531
468 <-> 835, 883, 1071
469 <-> 540, 747, 1137, 1339
470 <-> 1649, 1722
471 <-> 311, 874, 952, 1239
472 <-> 472, 1811
473 <-> 344
474 <-> 400
475 <-> 169, 889, 1738
476 <-> 141, 1566
477 <-> 408, 1611
478 <-> 629, 733
479 <-> 688
480 <-> 273, 859
481 <-> 318
482 <-> 1069
483 <-> 1609
484 <-> 294, 444
485 <-> 72, 1838
486 <-> 422, 662, 1011, 1026
487 <-> 1195
488 <-> 990, 1068
489 <-> 146
490 <-> 86, 958
491 <-> 806, 890, 1734
492 <-> 548, 1233, 1302
493 <-> 112, 1899, 1942
494 <-> 355, 494, 650
495 <-> 373, 1040, 1372, 1691
496 <-> 749
497 <-> 428
498 <-> 1800
499 <-> 227
500 <-> 1153, 1320, 1438, 1853
501 <-> 1038, 1744
502 <-> 1949
503 <-> 326, 1578
504 <-> 504
505 <-> 1672
506 <-> 506, 1149
507 <-> 136, 1454
508 <-> 14, 103, 1010, 1796
509 <-> 1376, 1838
510 <-> 1301
511 <-> 652, 842
512 <-> 512, 1102
513 <-> 789
514 <-> 1605
515 <-> 914
516 <-> 516, 1091
517 <-> 1513, 1981
518 <-> 1787
519 <-> 105, 519, 828
520 <-> 700, 1924
521 <-> 692, 1522, 1605
522 <-> 1537
523 <-> 523, 1300
524 <-> 259, 1410
525 <-> 1749, 1821
526 <-> 382
527 <-> 1174, 1571
528 <-> 942, 1429
529 <-> 768, 1177, 1820
530 <-> 124, 413, 826, 1467, 1882
531 <-> 621, 1940
532 <-> 712, 807
533 <-> 578, 1655
534 <-> 854
535 <-> 116
536 <-> 17, 929, 1482
537 <-> 537, 1265
538 <-> 1461
539 <-> 449, 1805
540 <-> 469, 1035
541 <-> 730
542 <-> 1147, 1244
543 <-> 543, 696, 1877
544 <-> 99, 329
545 <-> 545, 1936
546 <-> 462, 711, 1957
547 <-> 950, 1510
548 <-> 492, 730
549 <-> 151, 1054, 1297
550 <-> 44, 286, 1956
551 <-> 297, 1117, 1431, 1739
552 <-> 648, 772
553 <-> 1709
554 <-> 951, 1361
555 <-> 555, 1772
556 <-> 802, 1643
557 <-> 239, 557, 1142, 1893
558 <-> 204, 634
559 <-> 1792
560 <-> 612, 1693
561 <-> 561, 1007
562 <-> 809
563 <-> 127, 396, 1396
564 <-> 564, 1658
565 <-> 932, 1491
566 <-> 863, 1563, 1839
567 <-> 95, 716, 1514
568 <-> 186
569 <-> 23, 1261, 1378
570 <-> 193
571 <-> 1768
572 <-> 572, 1667
573 <-> 1977
574 <-> 835
575 <-> 425, 1039
576 <-> 1124
577 <-> 465, 732, 1110, 1625
578 <-> 533, 1876
579 <-> 946
580 <-> 580
581 <-> 837
582 <-> 226, 1738
583 <-> 78, 1526
584 <-> 847, 1106
585 <-> 154, 1723
586 <-> 1391
587 <-> 1589, 1725
588 <-> 386, 588, 991
589 <-> 589, 645
590 <-> 1296, 1972
591 <-> 591, 1556
592 <-> 1070, 1970
593 <-> 1786
594 <-> 428
595 <-> 269, 317, 619, 1638
596 <-> 1973
597 <-> 597, 750, 1109
598 <-> 1530, 1887
599 <-> 71, 432, 1904
600 <-> 124, 603, 1205
601 <-> 1694
602 <-> 1037, 1094, 1351
603 <-> 600, 689
604 <-> 667, 1143, 1460
605 <-> 643
606 <-> 953
607 <-> 127
608 <-> 836, 1000
609 <-> 609, 748, 1030
610 <-> 1879
611 <-> 1674
612 <-> 358, 560, 1918
613 <-> 784
614 <-> 1214
615 <-> 1643
616 <-> 341, 917, 1356, 1854
617 <-> 685
618 <-> 1297
619 <-> 595
620 <-> 376
621 <-> 531, 1054, 1236
622 <-> 440, 926, 992, 1080
623 <-> 1268
624 <-> 70
625 <-> 1251
626 <-> 1341
627 <-> 281, 627, 811, 1523
628 <-> 1060, 1911
629 <-> 389, 478
630 <-> 1401
631 <-> 665, 789
632 <-> 632, 654
633 <-> 797, 1402
634 <-> 323, 558, 1560
635 <-> 800, 1991
636 <-> 309, 1317
637 <-> 1803
638 <-> 1091
639 <-> 446, 1778
640 <-> 1131
641 <-> 152, 1116
642 <-> 1690
643 <-> 605, 1604
644 <-> 874, 1471
645 <-> 589, 1500
646 <-> 1517
647 <-> 1394, 1750
648 <-> 552, 838, 1807
649 <-> 445, 734
650 <-> 494, 1276
651 <-> 170, 420, 1469
652 <-> 43, 511, 1823
653 <-> 1442, 1558
654 <-> 632
655 <-> 27, 1058, 1239
656 <-> 212, 992, 1765
657 <-> 946
658 <-> 207
659 <-> 237, 659
660 <-> 831, 1162, 1540
661 <-> 209, 1588
662 <-> 486, 924
663 <-> 1172, 1650
664 <-> 944
665 <-> 328, 631, 1626
666 <-> 1350, 1714, 1979
667 <-> 337, 604
668 <-> 1067, 1983
669 <-> 669
670 <-> 364, 764, 799
671 <-> 888, 1063, 1139, 1310, 1570
672 <-> 1779
673 <-> 903
674 <-> 870
675 <-> 2, 29, 219, 406, 881
676 <-> 148, 1228
677 <-> 103, 717
678 <-> 1094
679 <-> 167, 1679
680 <-> 1605
681 <-> 1111
682 <-> 1385
683 <-> 129
684 <-> 1105
685 <-> 617, 1843
686 <-> 110, 1099, 1339
687 <-> 736
688 <-> 479, 923, 1597
689 <-> 366, 603, 1083, 1258, 1407
690 <-> 336, 1182
691 <-> 1585, 1922
692 <-> 521
693 <-> 782, 908, 1790
694 <-> 1935, 1939
695 <-> 1696
696 <-> 543, 1908
697 <-> 1039
698 <-> 1736
699 <-> 1186
700 <-> 520, 735, 1814, 1990
701 <-> 380, 392
702 <-> 1399
703 <-> 1300
704 <-> 704
705 <-> 102, 257, 411
706 <-> 77, 1016, 1561, 1701
707 <-> 1566
708 <-> 234
709 <-> 709
710 <-> 1731
711 <-> 47, 79, 546, 973
712 <-> 229, 532, 1779
713 <-> 724
714 <-> 1905, 1953
715 <-> 865, 1703
716 <-> 368, 567, 760, 1758
717 <-> 677, 727, 1340
718 <-> 1129
719 <-> 1042
720 <-> 289
721 <-> 244
722 <-> 460, 1525
723 <-> 723, 1563
724 <-> 213, 713
725 <-> 1955
726 <-> 1085
727 <-> 717, 1485
728 <-> 1940
729 <-> 348, 401, 1062
730 <-> 541, 548
731 <-> 241, 731, 1388
732 <-> 96, 577, 970, 1968
733 <-> 301, 478
734 <-> 649, 1380, 1388
735 <-> 700
736 <-> 687, 1861
737 <-> 972
738 <-> 313
739 <-> 947
740 <-> 1835
741 <-> 1153, 1528
742 <-> 786
743 <-> 447, 948, 1542, 1601
744 <-> 354, 1643
745 <-> 1213
746 <-> 746, 919, 1607
747 <-> 15, 469, 768, 905
748 <-> 609, 805, 1279
749 <-> 496, 1588, 1824
750 <-> 66, 597
751 <-> 946
752 <-> 1521
753 <-> 885, 1394
754 <-> 1413
755 <-> 135, 1532, 1801, 1833
756 <-> 286, 1938
757 <-> 1315
758 <-> 82, 332, 1721
759 <-> 1168
760 <-> 716, 1557
761 <-> 921
762 <-> 1172
763 <-> 1469
764 <-> 182, 670, 1955
765 <-> 69, 73
766 <-> 1298
767 <-> 767
768 <-> 529, 747
769 <-> 466, 1747, 1909
770 <-> 770, 1284
771 <-> 251, 1520, 1979
772 <-> 194, 552, 1397
773 <-> 1105, 1827
774 <-> 266
775 <-> 1315
776 <-> 800, 1919
777 <-> 846, 1019
778 <-> 1914
779 <-> 83, 1818
780 <-> 208, 1417
781 <-> 358, 943, 1247
782 <-> 693, 1732
783 <-> 448, 1977
784 <-> 613, 1408
785 <-> 435
786 <-> 742, 919, 1252
787 <-> 787, 992
788 <-> 844, 1845
789 <-> 57, 513, 631
790 <-> 1609, 1680, 1912
791 <-> 854, 1985
792 <-> 1296
793 <-> 182, 1857
794 <-> 133
795 <-> 206, 1413, 1632, 1929
796 <-> 140, 1700, 1789, 1804
797 <-> 20, 633, 1396
798 <-> 76, 1862
799 <-> 445, 670
800 <-> 635, 776
801 <-> 1027, 1518
802 <-> 556, 1018, 1539
803 <-> 803, 1287, 1926
804 <-> 804, 1697, 1713
805 <-> 748
806 <-> 252, 491
807 <-> 532, 942
808 <-> 1633
809 <-> 463, 562, 1163
810 <-> 817, 1498
811 <-> 627, 1888
812 <-> 1222, 1538
813 <-> 978
814 <-> 979
815 <-> 443, 1254, 1515
816 <-> 1409
817 <-> 810
818 <-> 295, 818
819 <-> 1117
820 <-> 26, 244, 820
821 <-> 453
822 <-> 308, 826
823 <-> 1824
824 <-> 217, 321, 1370
825 <-> 293, 1834
826 <-> 58, 530, 822, 1077, 1106, 1652
827 <-> 1535
828 <-> 519
829 <-> 1311, 1472
830 <-> 1836
831 <-> 660
832 <-> 1504, 1766
833 <-> 185, 266
834 <-> 834
835 <-> 468, 574
836 <-> 310, 608, 1383
837 <-> 581, 968
838 <-> 648, 1089, 1409
839 <-> 1546, 1793
840 <-> 840, 1014
841 <-> 918
842 <-> 511, 1269, 1752
843 <-> 1437
844 <-> 788, 1996
845 <-> 943
846 <-> 253, 777, 1839
847 <-> 584
848 <-> 99, 892, 1392, 1629
849 <-> 849, 1258
850 <-> 224, 1408, 1988
851 <-> 269, 851
852 <-> 1405
853 <-> 1099, 1798
854 <-> 534, 791, 1248
855 <-> 137, 1468, 1918
856 <-> 1281
857 <-> 938, 1060
858 <-> 1498
859 <-> 297, 444, 480, 1272, 1695
860 <-> 1352, 1506
861 <-> 1569, 1950
862 <-> 1895
863 <-> 286, 566
864 <-> 1289, 1737
865 <-> 715
866 <-> 175, 1988
867 <-> 1830
868 <-> 1467
869 <-> 869
870 <-> 674, 1524
871 <-> 1862
872 <-> 1726
873 <-> 189, 362, 936, 971, 1356
874 <-> 471, 644, 1322
875 <-> 200
876 <-> 1075, 1154, 1196
877 <-> 1434
878 <-> 461
879 <-> 11, 249, 1638
880 <-> 1014, 1221
881 <-> 675, 1098, 1576, 1942
882 <-> 178, 421, 882
883 <-> 63, 468
884 <-> 884
885 <-> 21, 753, 947
886 <-> 1043
887 <-> 1436, 1533, 1624, 1788
888 <-> 671, 1020
889 <-> 475
890 <-> 491
891 <-> 1709
892 <-> 848, 892
893 <-> 1914
894 <-> 16, 924, 1434
895 <-> 1363
896 <-> 1301, 1393
897 <-> 897
898 <-> 1074, 1539
899 <-> 1656
900 <-> 900, 1433, 1635, 1980
901 <-> 42
902 <-> 977
903 <-> 111, 400, 673, 1512, 1965
904 <-> 1091, 1808
905 <-> 747
906 <-> 1160, 1401, 1703
907 <-> 1223, 1666
908 <-> 693, 1117
909 <-> 1722
910 <-> 117, 1023
911 <-> 945
912 <-> 1600
913 <-> 375, 1784
914 <-> 515, 1108, 1362, 1808
915 <-> 111, 1535
916 <-> 1106
917 <-> 616
918 <-> 841, 918, 1331
919 <-> 746, 786, 1711
920 <-> 966, 1028, 1545
921 <-> 169, 761, 1669
922 <-> 222, 402, 1562
923 <-> 688
924 <-> 662, 894
925 <-> 197, 1550
926 <-> 622
927 <-> 927
928 <-> 1723
929 <-> 536, 1235
930 <-> 1409, 1566
931 <-> 1189
932 <-> 143, 565
933 <-> 1159, 1947
934 <-> 1986
935 <-> 315
936 <-> 873, 1604
937 <-> 937
938 <-> 412, 857, 1247
939 <-> 1087
940 <-> 1078
941 <-> 1350
942 <-> 528, 807, 1813
943 <-> 781, 845
944 <-> 437, 664, 1045, 1488
945 <-> 911, 979, 1913
946 <-> 579, 657, 751, 946, 1071
947 <-> 739, 885, 1685
948 <-> 370, 743, 1844
949 <-> 177, 1312, 1491
950 <-> 547, 1248, 1418
951 <-> 554, 1044, 1062
952 <-> 471
953 <-> 4, 606
954 <-> 42
955 <-> 1136
956 <-> 1204
957 <-> 445
958 <-> 490, 1994
959 <-> 7, 365, 456
960 <-> 960
961 <-> 245, 1900
962 <-> 1594
963 <-> 89, 335
964 <-> 1090
965 <-> 965
966 <-> 62, 920, 1242
967 <-> 235, 443, 1286
968 <-> 837, 968, 1229, 1866
969 <-> 184, 1451
970 <-> 121, 732, 978
971 <-> 873
972 <-> 737, 1487, 1969
973 <-> 711
974 <-> 296
975 <-> 1831
976 <-> 1212, 1381
977 <-> 129, 902, 1044
978 <-> 813, 970, 1789
979 <-> 814, 945
980 <-> 980
981 <-> 130, 1684
982 <-> 87, 118, 376
983 <-> 1214, 1543
984 <-> 81, 1825
985 <-> 985, 1527
986 <-> 72, 1476
987 <-> 433, 1393, 1958
988 <-> 988
989 <-> 385
990 <-> 488
991 <-> 588, 1105, 1145, 1201
992 <-> 622, 656, 787
993 <-> 359
994 <-> 1164, 1253
995 <-> 1933
996 <-> 1742, 1764
997 <-> 55, 309
998 <-> 1838
999 <-> 1302, 1597
1000 <-> 608, 1687, 1974
1001 <-> 22, 1149, 1965
1002 <-> 300
1003 <-> 1891, 1920
1004 <-> 39, 337
1005 <-> 1544, 1747, 1805
1006 <-> 289, 1374, 1636
1007 <-> 561
1008 <-> 73, 1221
1009 <-> 1009
1010 <-> 508
1011 <-> 486, 1228
1012 <-> 274
1013 <-> 369, 1778
1014 <-> 144, 840, 880
1015 <-> 1240, 1352, 1665
1016 <-> 706
1017 <-> 213
1018 <-> 314, 802
1019 <-> 777
1020 <-> 888, 1718
1021 <-> 1981
1022 <-> 1381, 1564, 1843
1023 <-> 910
1024 <-> 18, 232
1025 <-> 192
1026 <-> 205, 486
1027 <-> 801, 1652
1028 <-> 920
1029 <-> 300
1030 <-> 609
1031 <-> 345, 1079, 1596, 1964
1032 <-> 1918
1033 <-> 1550
1034 <-> 1034
1035 <-> 540
1036 <-> 9, 1959
1037 <-> 602, 1178
1038 <-> 501
1039 <-> 575, 697, 1311
1040 <-> 367, 495
1041 <-> 1041
1042 <-> 719, 1567, 1982
1043 <-> 886, 1778
1044 <-> 28, 419, 951, 977
1045 <-> 944
1046 <-> 320
1047 <-> 36
1048 <-> 1495, 1508, 1919
1049 <-> 1049, 1960
1050 <-> 432
1051 <-> 1793
1052 <-> 93, 1454
1053 <-> 175, 1744, 1881
1054 <-> 549, 621, 1054, 1119, 1809
1055 <-> 1333
1056 <-> 1109
1057 <-> 169
1058 <-> 655
1059 <-> 1434, 1444
1060 <-> 628, 857
1061 <-> 1560
1062 <-> 729, 951
1063 <-> 671, 1549
1064 <-> 1759, 1904
1065 <-> 1541
1066 <-> 1306
1067 <-> 668
1068 <-> 488, 1415, 1570
1069 <-> 482, 1116
1070 <-> 592, 1271
1071 <-> 468, 946
1072 <-> 225, 1072
1073 <-> 123, 1073
1074 <-> 898
1075 <-> 454, 876, 1129
1076 <-> 183, 1180
1077 <-> 339, 826
1078 <-> 256, 257, 940, 1501
1079 <-> 1031, 1092
1080 <-> 622
1081 <-> 1669, 1817
1082 <-> 1669, 1925
1083 <-> 689, 1682
1084 <-> 1413, 1764
1085 <-> 365, 448, 726, 1883
1086 <-> 1628
1087 <-> 298, 410, 939, 1951
1088 <-> 1483
1089 <-> 838
1090 <-> 152, 325, 964
1091 <-> 516, 638, 904, 1694
1092 <-> 1079, 1113
1093 <-> 461, 1355, 1686
1094 <-> 602, 678, 1094
1095 <-> 1726
1096 <-> 219
1097 <-> 1215
1098 <-> 255, 881, 1712
1099 <-> 442, 686, 853
1100 <-> 1100, 1684
1101 <-> 191, 1101
1102 <-> 512, 1449
1103 <-> 376, 1427
1104 <-> 1449
1105 <-> 684, 773, 991, 1118
1106 <-> 584, 826, 916
1107 <-> 1256
1108 <-> 914
1109 <-> 597, 1056
1110 <-> 285, 577
1111 <-> 431, 681, 1879, 1948
1112 <-> 1206, 1556
1113 <-> 131, 1092
1114 <-> 1160, 1821, 1889
1115 <-> 1491, 1571
1116 <-> 641, 1069
1117 <-> 551, 819, 908
1118 <-> 1105
1119 <-> 1054, 1950
1120 <-> 71
1121 <-> 1252
1122 <-> 416, 1196
1123 <-> 265, 382, 1509
1124 <-> 576, 1124, 1493, 1690
1125 <-> 261
1126 <-> 1155
1127 <-> 374, 1341
1128 <-> 150
1129 <-> 718, 1075
1130 <-> 1954
1131 <-> 191, 640
1132 <-> 1570
1133 <-> 140
1134 <-> 412
1135 <-> 1702, 1938
1136 <-> 955, 1513
1137 <-> 469, 1728
1138 <-> 1600, 1905
1139 <-> 671
1140 <-> 259, 1492, 1736
1141 <-> 436
1142 <-> 187, 557
1143 <-> 604
1144 <-> 1581
1145 <-> 991, 1672
1146 <-> 1571, 1683
1147 <-> 542
1148 <-> 1726
1149 <-> 506, 1001
1150 <-> 1371
1151 <-> 1151, 1403, 1967, 1993
1152 <-> 62, 1152
1153 <-> 500, 741
1154 <-> 220, 876, 1456
1155 <-> 48, 1126, 1375
1156 <-> 302, 1301
1157 <-> 279
1158 <-> 162, 1426, 1535
1159 <-> 933
1160 <-> 3, 128, 906, 1114
1161 <-> 1448, 1742
1162 <-> 660, 1435, 1579
1163 <-> 809, 1163, 1776
1164 <-> 994
1165 <-> 1803, 1870
1166 <-> 1921
1167 <-> 450
1168 <-> 759, 1168
1169 <-> 107
1170 <-> 1170
1171 <-> 1291, 1573
1172 <-> 663, 762
1173 <-> 196, 1764
1174 <-> 527
1175 <-> 170, 1786
1176 <-> 168, 1326
1177 <-> 529, 1598
1178 <-> 1037, 1989
1179 <-> 235, 1947
1180 <-> 1076, 1180
1181 <-> 1373, 1419
1182 <-> 690
1183 <-> 1183
1184 <-> 1224, 1504
1185 <-> 445
1186 <-> 327, 699
1187 <-> 203
1188 <-> 1598
1189 <-> 316, 931, 1561
1190 <-> 1603, 1647
1191 <-> 1704
1192 <-> 404
1193 <-> 359
1194 <-> 1594
1195 <-> 487, 1760
1196 <-> 876, 1122, 1475
1197 <-> 1942
1198 <-> 18, 67, 1943
1199 <-> 262, 1419
1200 <-> 1700
1201 <-> 991
1202 <-> 1457
1203 <-> 1543
1204 <-> 956, 1204
1205 <-> 600
1206 <-> 1112
1207 <-> 1553, 1910
1208 <-> 122
1209 <-> 131, 1622, 1771
1210 <-> 1446
1211 <-> 1386, 1595
1212 <-> 176, 976, 1868
1213 <-> 745, 1244, 1412
1214 <-> 13, 331, 614, 983
1215 <-> 292, 1097
1216 <-> 1723
1217 <-> 114, 1218
1218 <-> 1217
1219 <-> 98, 1450
1220 <-> 1959
1221 <-> 313, 880, 1008, 1614, 1688
1222 <-> 812, 1626
1223 <-> 907, 1853
1224 <-> 1184
1225 <-> 1241
1226 <-> 1799
1227 <-> 1227
1228 <-> 676, 1011
1229 <-> 968, 1994
1230 <-> 1650, 1914
1231 <-> 1787
1232 <-> 192
1233 <-> 492
1234 <-> 1614, 1628
1235 <-> 351, 929
1236 <-> 621
1237 <-> 1264
1238 <-> 218
1239 <-> 471, 655, 1369
1240 <-> 1015, 1256
1241 <-> 1225, 1241, 1336
1242 <-> 966, 1750
1243 <-> 1243, 1363
1244 <-> 542, 1213
1245 <-> 358
1246 <-> 1246
1247 <-> 377, 781, 938
1248 <-> 854, 950, 1347
1249 <-> 1856, 1930
1250 <-> 184
1251 <-> 377, 625, 1645
1252 <-> 786, 1121
1253 <-> 994, 1842
1254 <-> 124, 815, 1815
1255 <-> 1742
1256 <-> 1107, 1240
1257 <-> 449, 1964
1258 <-> 689, 849
1259 <-> 230
1260 <-> 289, 1898
1261 <-> 569
1262 <-> 1890
1263 <-> 65, 429
1264 <-> 258, 1237, 1416, 1770
1265 <-> 537
1266 <-> 1322, 1468, 1524, 1995
1267 <-> 1602, 1647
1268 <-> 222, 250, 623
1269 <-> 842
1270 <-> 260
1271 <-> 1070
1272 <-> 290, 859
1273 <-> 6, 343, 460
1274 <-> 119
1275 <-> 171, 1275, 1777
1276 <-> 650
1277 <-> 244, 1631
1278 <-> 1335
1279 <-> 433, 748
1280 <-> 1280
1281 <-> 856, 1452
1282 <-> 214
1283 <-> 1657, 1732
1284 <-> 770
1285 <-> 1285
1286 <-> 967
1287 <-> 803
1288 <-> 1353
1289 <-> 864, 1632
1290 <-> 92
1291 <-> 1171
1292 <-> 306, 1762
1293 <-> 1293
1294 <-> 1772
1295 <-> 421, 428
1296 <-> 234, 590, 792
1297 <-> 549, 618
1298 <-> 766, 1298, 1416, 1595, 1709
1299 <-> 275, 1884, 1981
1300 <-> 523, 703, 1428
1301 <-> 510, 896, 1156
1302 <-> 492, 999
1303 <-> 358
1304 <-> 467, 1304
1305 <-> 369, 1345
1306 <-> 261, 1066, 1946
1307 <-> 1568, 1845
1308 <-> 1994
1309 <-> 1334
1310 <-> 671
1311 <-> 320, 829, 1039, 1463, 1608
1312 <-> 949, 1590
1313 <-> 191
1314 <-> 1630
1315 <-> 757, 775, 1463
1316 <-> 1499
1317 <-> 636
1318 <-> 174
1319 <-> 449
1320 <-> 64, 500
1321 <-> 1532, 1678
1322 <-> 874, 1266, 1327
1323 <-> 283
1324 <-> 67, 1987
1325 <-> 1902
1326 <-> 1176, 1499
1327 <-> 1322
1328 <-> 1359
1329 <-> 88
1330 <-> 98, 1781
1331 <-> 918
1332 <-> 1430
1333 <-> 94, 1055, 1523
1334 <-> 1309, 1334, 1643
1335 <-> 1278, 1335
1336 <-> 1241
1337 <-> 1482, 1495
1338 <-> 1338
1339 <-> 469, 686
1340 <-> 717
1341 <-> 626, 1127, 1341, 1903
1342 <-> 348, 1944
1343 <-> 1343
1344 <-> 1407
1345 <-> 1305, 1760
1346 <-> 414, 1890
1347 <-> 288, 1248
1348 <-> 1575, 1576
1349 <-> 1583
1350 <-> 666, 941, 1462
1351 <-> 602, 1998
1352 <-> 45, 860, 1015
1353 <-> 1288, 1702
1354 <-> 1354
1355 <-> 1093, 1355, 1931
1356 <-> 616, 873, 1715
1357 <-> 1723
1358 <-> 1613
1359 <-> 1328, 1359
1360 <-> 1360
1361 <-> 554
1362 <-> 181, 914, 1627
1363 <-> 895, 1243, 1594
1364 <-> 1364
1365 <-> 1365, 1650
1366 <-> 129, 303, 1800
1367 <-> 35, 1522, 1716
1368 <-> 467
1369 <-> 1239
1370 <-> 824, 1502, 1576, 1943
1371 <-> 252, 1150
1372 <-> 495
1373 <-> 1181, 1698
1374 <-> 1006, 1999
1375 <-> 279, 1155, 1921
1376 <-> 509, 1651
1377 <-> 1796
1378 <-> 186, 569, 1552
1379 <-> 80, 255, 1634
1380 <-> 236, 734
1381 <-> 248, 415, 435, 976, 1022
1382 <-> 368
1383 <-> 836, 1913
1384 <-> 1425
1385 <-> 324, 682, 1640
1386 <-> 1211
1387 <-> 1494
1388 <-> 731, 734, 1466
1389 <-> 127
1390 <-> 12
1391 <-> 586, 1657
1392 <-> 848
1393 <-> 896, 987
1394 <-> 647, 753
1395 <-> 1429
1396 <-> 563, 797, 1434
1397 <-> 772
1398 <-> 1913
1399 <-> 702, 1572
1400 <-> 1876
1401 <-> 630, 906
1402 <-> 633
1403 <-> 1151
1404 <-> 1661
1405 <-> 206, 852
1406 <-> 352, 1743
1407 <-> 689, 1344
1408 <-> 784, 850
1409 <-> 816, 838, 930
1410 <-> 270, 524
1411 <-> 1468, 1793
1412 <-> 445, 1213
1413 <-> 754, 795, 1084
1414 <-> 140, 254
1415 <-> 1068
1416 <-> 1264, 1298
1417 <-> 780
1418 <-> 950
1419 <-> 1181, 1199
1420 <-> 160
1421 <-> 340
1422 <-> 1422, 1860
1423 <-> 34, 1971
1424 <-> 1991
1425 <-> 1384, 1450
1426 <-> 1158
1427 <-> 84, 179, 1103, 1706
1428 <-> 1300
1429 <-> 528, 1395
1430 <-> 1332, 1430
1431 <-> 551
1432 <-> 365
1433 <-> 900
1434 <-> 138, 877, 894, 1059, 1396
1435 <-> 1162
1436 <-> 887, 1859, 1949
1437 <-> 114, 386, 843, 1855
1438 <-> 101, 500, 1581
1439 <-> 1449
1440 <-> 81, 1766
1441 <-> 1767
1442 <-> 385, 409, 653
1443 <-> 233, 1443
1444 <-> 1059, 1670, 1742
1445 <-> 1669
1446 <-> 444, 1210
1447 <-> 1842
1448 <-> 1161
1449 <-> 1102, 1104, 1439
1450 <-> 155, 1219, 1425, 1677, 1727
1451 <-> 969, 1584
1452 <-> 399, 1281
1453 <-> 265
1454 <-> 507, 1052, 1834
1455 <-> 346, 358
1456 <-> 1154, 1786
1457 <-> 1202, 1619
1458 <-> 383
1459 <-> 1718, 1729
1460 <-> 604
1461 <-> 131, 538, 1740
1462 <-> 1350, 1530
1463 <-> 1311, 1315, 1582, 1982
1464 <-> 1817, 1906
1465 <-> 1465
1466 <-> 420, 1388
1467 <-> 530, 868
1468 <-> 855, 1266, 1411
1469 <-> 651, 763
1470 <-> 14, 1829
1471 <-> 644, 1775
1472 <-> 202, 829, 1562
1473 <-> 13, 211, 1891
1474 <-> 1813, 1852
1475 <-> 1196
1476 <-> 986, 1537
1477 <-> 1481
1478 <-> 133, 151, 1932
1479 <-> 1538
1480 <-> 79, 1565
1481 <-> 1477, 1481
1482 <-> 536, 1337
1483 <-> 465, 1088
1484 <-> 1553, 1974
1485 <-> 88, 727
1486 <-> 1582
1487 <-> 972, 1731, 1896
1488 <-> 944, 1845
1489 <-> 198
1490 <-> 1926
1491 <-> 565, 949, 1115, 1983
1492 <-> 1140
1493 <-> 1124
1494 <-> 1387, 1814
1495 <-> 175, 1048, 1337
1496 <-> 23
1497 <-> 166
1498 <-> 312, 810, 858, 1727
1499 <-> 1316, 1326, 1586
1500 <-> 645
1501 <-> 1078, 1847
1502 <-> 1370
1503 <-> 1992
1504 <-> 832, 1184
1505 <-> 78
1506 <-> 52, 860
1507 <-> 1642, 1647
1508 <-> 1048, 1508
1509 <-> 246, 1123
1510 <-> 547, 1876
1511 <-> 385, 389
1512 <-> 37, 903
1513 <-> 517, 1136
1514 <-> 342, 567, 1641
1515 <-> 815
1516 <-> 1722, 1992
1517 <-> 349, 646, 1517
1518 <-> 29, 801
1519 <-> 1676
1520 <-> 771
1521 <-> 752, 1754, 1916
1522 <-> 521, 1367
1523 <-> 627, 1333
1524 <-> 870, 1266
1525 <-> 430, 722
1526 <-> 583, 1878
1527 <-> 985
1528 <-> 741
1529 <-> 1559, 1954
1530 <-> 51, 598, 1462
1531 <-> 467, 1842
1532 <-> 337, 386, 755, 1321
1533 <-> 887
1534 <-> 1740
1535 <-> 827, 915, 1158
1536 <-> 334, 1536
1537 <-> 522, 1476
1538 <-> 812, 1479
1539 <-> 277, 802, 898
1540 <-> 660, 1937
1541 <-> 261, 1065
1542 <-> 743
1543 <-> 983, 1203
1544 <-> 403, 1005
1545 <-> 920
1546 <-> 839
1547 <-> 30
1548 <-> 58
1549 <-> 292, 1063
1550 <-> 925, 1033
1551 <-> 1618
1552 <-> 315, 1378, 1552
1553 <-> 1207, 1484
1554 <-> 192
1555 <-> 1555, 1710
1556 <-> 591, 1112
1557 <-> 760
1558 <-> 653, 1934
1559 <-> 260, 1529
1560 <-> 634, 1061, 1812
1561 <-> 706, 1189
1562 <-> 267, 922, 1472
1563 <-> 566, 723
1564 <-> 215, 1022
1565 <-> 1480
1566 <-> 476, 707, 930
1567 <-> 1042
1568 <-> 1307
1569 <-> 861
1570 <-> 343, 671, 1068, 1132
1571 <-> 527, 1115, 1146, 1630
1572 <-> 1399, 1591
1573 <-> 1171, 1723
1574 <-> 267
1575 <-> 76, 1348
1576 <-> 881, 1348, 1370, 1646
1577 <-> 1687
1578 <-> 503
1579 <-> 452, 1162, 1660
1580 <-> 1926
1581 <-> 184, 1144, 1438, 1648
1582 <-> 1463, 1486
1583 <-> 1349, 1583, 1644
1584 <-> 1451
1585 <-> 691, 1706
1586 <-> 1499, 1676
1587 <-> 161, 208, 1786
1588 <-> 661, 749
1589 <-> 587
1590 <-> 321, 1312
1591 <-> 1572, 1844
1592 <-> 419
1593 <-> 1593
1594 <-> 962, 1194, 1363
1595 <-> 194, 1211, 1298, 1754
1596 <-> 1031
1597 <-> 688, 999, 1816
1598 <-> 94, 1177, 1188
1599 <-> 397
1600 <-> 912, 1138
1601 <-> 743
1602 <-> 405, 1267
1603 <-> 1190
1604 <-> 643, 936
1605 <-> 514, 521, 680
1606 <-> 1606
1607 <-> 746
1608 <-> 1311, 1608
1609 <-> 483, 790
1610 <-> 305, 1610
1611 <-> 268, 477
1612 <-> 371
1613 <-> 336, 1358, 1642
1614 <-> 1221, 1234, 1732
1615 <-> 287
1616 <-> 19, 143
1617 <-> 327
1618 <-> 38, 283, 436, 1551
1619 <-> 1457, 1817
1620 <-> 210, 1791
1621 <-> 1976
1622 <-> 1209, 1928
1623 <-> 139
1624 <-> 887
1625 <-> 577
1626 <-> 665, 1222
1627 <-> 1362
1628 <-> 1086, 1234
1629 <-> 848
1630 <-> 1314, 1571
1631 <-> 1277
1632 <-> 304, 795, 1289
1633 <-> 454, 808
1634 <-> 404, 1379
1635 <-> 900
1636 <-> 1006
1637 <-> 1637
1638 <-> 439, 595, 879
1639 <-> 1639
1640 <-> 1385
1641 <-> 1514
1642 <-> 1507, 1613
1643 <-> 556, 615, 744, 1334
1644 <-> 1583
1645 <-> 1251
1646 <-> 223, 1576
1647 <-> 1190, 1267, 1507
1648 <-> 441, 1581
1649 <-> 56, 470
1650 <-> 190, 663, 1230, 1365, 1875
1651 <-> 1376
1652 <-> 826, 1027
1653 <-> 46, 163, 165
1654 <-> 280, 380, 1808
1655 <-> 533, 1668
1656 <-> 64, 899, 1746
1657 <-> 1283, 1391, 1941
1658 <-> 564
1659 <-> 1824
1660 <-> 278, 1579
1661 <-> 2, 1404
1662 <-> 1757
1663 <-> 1663
1664 <-> 1664
1665 <-> 38, 1015
1666 <-> 907
1667 <-> 572, 1824, 1830
1668 <-> 1655
1669 <-> 921, 1081, 1082, 1445
1670 <-> 407, 1444
1671 <-> 1671
1672 <-> 505, 1145
1673 <-> 1673
1674 <-> 438, 611
1675 <-> 198
1676 <-> 1519, 1586
1677 <-> 1450
1678 <-> 1321, 1819
1679 <-> 679
1680 <-> 790
1681 <-> 1938
1682 <-> 1083, 1997
1683 <-> 1146
1684 <-> 981, 1100
1685 <-> 947
1686 <-> 1093
1687 <-> 1000, 1577, 1713
1688 <-> 1221
1689 <-> 1708
1690 <-> 642, 1124
1691 <-> 495
1692 <-> 1821
1693 <-> 359, 560
1694 <-> 266, 601, 1091, 1933
1695 <-> 859
1696 <-> 327, 695
1697 <-> 804
1698 <-> 1373, 1767
1699 <-> 1985
1700 <-> 796, 1200
1701 <-> 116, 347, 706
1702 <-> 1135, 1353
1703 <-> 715, 906
1704 <-> 179, 1191
1705 <-> 80
1706 <-> 1427, 1585
1707 <-> 1927
1708 <-> 66, 142, 1689
1709 <-> 553, 891, 1298
1710 <-> 1555
1711 <-> 919
1712 <-> 1098, 1915
1713 <-> 242, 804, 1687
1714 <-> 666
1715 <-> 1356
1716 <-> 1367, 1716
1717 <-> 444
1718 <-> 1020, 1459
1719 <-> 1852
1720 <-> 77
1721 <-> 758
1722 <-> 470, 909, 1516, 1783
1723 <-> 454, 585, 928, 1216, 1357, 1573
1724 <-> 255
1725 <-> 587, 1725
1726 <-> 392, 872, 1095, 1148
1727 <-> 1450, 1498
1728 <-> 1137
1729 <-> 1459
1730 <-> 416
1731 <-> 710, 1487, 1904
1732 <-> 782, 1283, 1614
1733 <-> 1880
1734 <-> 491
1735 <-> 214, 286
1736 <-> 32, 698, 1140
1737 <-> 864
1738 <-> 388, 475, 582
1739 <-> 551
1740 <-> 90, 1461, 1534
1741 <-> 411
1742 <-> 996, 1161, 1255, 1444
1743 <-> 221, 1406, 1743, 1748
1744 <-> 501, 1053
1745 <-> 70
1746 <-> 109, 1656
1747 <-> 769, 1005, 1844
1748 <-> 1743
1749 <-> 1, 525
1750 <-> 647, 1242
1751 <-> 1907
1752 <-> 842
1753 <-> 240, 1793
1754 <-> 1521, 1595
1755 <-> 1755
1756 <-> 403
1757 <-> 1662, 1757
1758 <-> 716
1759 <-> 1064
1760 <-> 1195, 1345
1761 <-> 35
1762 <-> 1292, 1762
1763 <-> 173, 1869
1764 <-> 156, 996, 1084, 1173
1765 <-> 656, 1851
1766 <-> 832, 1440
1767 <-> 229, 276, 1441, 1698
1768 <-> 83, 301, 571
1769 <-> 1998
1770 <-> 153, 1264
1771 <-> 1209
1772 <-> 555, 1294, 1897
1773 <-> 1773, 1858
1774 <-> 1961
1775 <-> 30, 1471
1776 <-> 1163
1777 <-> 1275
1778 <-> 639, 1013, 1043, 1905
1779 <-> 672, 712
1780 <-> 1780
1781 <-> 247, 1330
1782 <-> 160, 288, 371
1783 <-> 10, 211, 1722
1784 <-> 216, 913
1785 <-> 244
1786 <-> 593, 1175, 1456, 1587
1787 <-> 200, 518, 1231
1788 <-> 887
1789 <-> 294, 796, 978
1790 <-> 693, 1902
1791 <-> 361, 1620
1792 <-> 559, 1828
1793 <-> 839, 1051, 1411, 1753
1794 <-> 1837, 1969
1795 <-> 361
1796 <-> 163, 508, 1377
1797 <-> 266
1798 <-> 853
1799 <-> 250, 1226
1800 <-> 120, 498, 1366
1801 <-> 755
1802 <-> 180, 363
1803 <-> 637, 1165
1804 <-> 796
1805 <-> 539, 1005
1806 <-> 1806
1807 <-> 648
1808 <-> 904, 914, 1654
1809 <-> 79, 1054
1810 <-> 1838
1811 <-> 472
1812 <-> 1560, 1812
1813 <-> 942, 1474, 1891
1814 <-> 700, 1494
1815 <-> 1254
1816 <-> 329, 1597
1817 <-> 1081, 1464, 1619, 1891
1818 <-> 252, 779
1819 <-> 1678
1820 <-> 529
1821 <-> 525, 1114, 1692
1822 <-> 73
1823 <-> 652
1824 <-> 749, 823, 1659, 1667
1825 <-> 984
1826 <-> 236
1827 <-> 357, 773
1828 <-> 270, 1792, 1869
1829 <-> 1470, 1831
1830 <-> 867, 1667
1831 <-> 975, 1829
1832 <-> 1832
1833 <-> 755
1834 <-> 825, 1454
1835 <-> 185, 740
1836 <-> 830, 1881
1837 <-> 1794
1838 <-> 485, 509, 998, 1810, 1943
1839 <-> 566, 846
1840 <-> 1840
1841 <-> 1951
1842 <-> 1253, 1447, 1531
1843 <-> 685, 1022
1844 <-> 375, 948, 1591, 1747
1845 <-> 88, 788, 1307, 1488
1846 <-> 202
1847 <-> 287, 1501
1848 <-> 300, 1969
1849 <-> 1849
1850 <-> 232, 426
1851 <-> 1765
1852 <-> 1474, 1719
1853 <-> 500, 1223
1854 <-> 616
1855 <-> 1437
1856 <-> 75, 94, 1249
1857 <-> 793
1858 <-> 1773
1859 <-> 318, 1436
1860 <-> 1422
1861 <-> 736, 1947
1862 <-> 798, 871, 1890
1863 <-> 428
1864 <-> 165
1865 <-> 258
1866 <-> 271, 968
1867 <-> 0
1868 <-> 382, 1212
1869 <-> 327, 1763, 1828
1870 <-> 296, 423, 1165, 1931
1871 <-> 457
1872 <-> 53, 1973
1873 <-> 269
1874 <-> 1874
1875 <-> 52, 1650
1876 <-> 578, 1400, 1510
1877 <-> 543
1878 <-> 1526
1879 <-> 610, 1111, 1879
1880 <-> 197, 1733
1881 <-> 1053, 1836
1882 <-> 530
1883 <-> 1085
1884 <-> 1299
1885 <-> 30
1886 <-> 453, 1907, 1909
1887 <-> 598
1888 <-> 811
1889 <-> 1114
1890 <-> 1262, 1346, 1862
1891 <-> 291, 1003, 1473, 1813, 1817
1892 <-> 1892
1893 <-> 557
1894 <-> 1997
1895 <-> 62, 862
1896 <-> 1487
1897 <-> 1772
1898 <-> 1260, 1902
1899 <-> 493
1900 <-> 337, 393, 961
1901 <-> 147, 1940
1902 <-> 379, 1325, 1790, 1898
1903 <-> 1341
1904 <-> 599, 1064, 1731
1905 <-> 714, 1138, 1778
1906 <-> 1464
1907 <-> 1751, 1886
1908 <-> 696
1909 <-> 769, 1886
1910 <-> 1207
1911 <-> 628
1912 <-> 319, 790, 1912
1913 <-> 945, 1383, 1398
1914 <-> 778, 893, 1230
1915 <-> 1712, 1917
1916 <-> 164, 230, 1521
1917 <-> 387, 1915
1918 <-> 59, 350, 451, 612, 855, 1032
1919 <-> 776, 1048
1920 <-> 1003
1921 <-> 1166, 1375
1922 <-> 5, 691
1923 <-> 99
1924 <-> 520
1925 <-> 1082
1926 <-> 803, 1490, 1580
1927 <-> 162, 1707
1928 <-> 438, 1622
1929 <-> 67, 795
1930 <-> 1249
1931 <-> 1355, 1870
1932 <-> 68, 1478
1933 <-> 995, 1694
1934 <-> 1558
1935 <-> 370, 694
1936 <-> 545
1937 <-> 360, 1540
1938 <-> 756, 1135, 1681
1939 <-> 694
1940 <-> 531, 728, 1901
1941 <-> 1657
1942 <-> 493, 881, 1197
1943 <-> 1198, 1370, 1838
1944 <-> 1342
1945 <-> 438
1946 <-> 299, 1306
1947 <-> 933, 1179, 1861
1948 <-> 1111
1949 <-> 318, 502, 1436
1950 <-> 861, 1119
1951 <-> 159, 1087, 1841
1952 <-> 446
1953 <-> 714
1954 <-> 1130, 1529
1955 <-> 725, 764
1956 <-> 550
1957 <-> 546
1958 <-> 987
1959 <-> 1036, 1220
1960 <-> 49, 356, 1049
1961 <-> 1774, 1961
1962 <-> 393
1963 <-> 1963
1964 <-> 1031, 1257
1965 <-> 903, 1001
1966 <-> 399
1967 <-> 1151
1968 <-> 732
1969 <-> 972, 1794, 1848
1970 <-> 338, 592
1971 <-> 1423
1972 <-> 590
1973 <-> 596, 1872
1974 <-> 1000, 1484
1975 <-> 1975
1976 <-> 1621, 1976
1977 <-> 573, 783
1978 <-> 87
1979 <-> 666, 771
1980 <-> 900
1981 <-> 262, 517, 1021, 1299
1982 <-> 1042, 1463
1983 <-> 668, 1491
1984 <-> 123
1985 <-> 791, 1699
1986 <-> 246, 934, 1986
1987 <-> 1324
1988 <-> 850, 866
1989 <-> 139, 1178
1990 <-> 161, 700
1991 <-> 635, 1424
1992 <-> 1503, 1516
1993 <-> 1151
1994 <-> 958, 1229, 1308
1995 <-> 1266
1996 <-> 844
1997 <-> 1682, 1894
1998 <-> 1351, 1769
1999 <-> 1374",
"0: 4
1: 2
2: 3
4: 4
6: 8
8: 5
10: 6
12: 6
14: 10
16: 8
18: 6
20: 9
22: 8
24: 6
26: 8
28: 8
30: 12
32: 12
34: 12
36: 12
38: 10
40: 12
42: 12
44: 14
46: 8
48: 14
50: 12
52: 14
54: 14
58: 14
60: 12
62: 14
64: 14
66: 12
68: 12
72: 14
74: 18
76: 17
86: 14
88: 20
92: 14
94: 14
96: 18
98: 18",
"nbysizxe",
"Generator A starts with 634
Generator B starts with 301",
"x10/0,s2,x6/11,s3,x12/1,s15,pb/m,x4/8,pn/c,x13/9,pj/e,x15/2,s15,x4/9,pp/h,x8/11,s10,x6/15,s15,pd/k,x11/8,s14,x0/6,s14,x1/7,s14,x2/9,pg/l,x0/15,pc/f,x9/12,s12,pi/d,x4/0,s15,x15/13,s10,x1/8,s10,x3/12,s12,pl/n,x6/8,s5,x3/13,s12,x4/6,s14,x1/13,s3,x12/2,s11,x9/1,ph/f,x2/15,s8,x14/4,pl/d,s7,x10/13,pi/h,x9/12,s7,x13/8,pb/d,x6/14,s15,x7/15,s1,pn/p,x4/11,pe/l,x7/12,s6,x6/14,pp/m,x11/10,s15,x13/12,pc/o,x14/10,pl/k,x1/6,s15,x4/14,s13,x6/2,pp/i,x1/10,s1,x12/2,s12,x1/10,pl/m,x0/15,s1,x9/10,s12,x11/3,ph/a,x4/12,s3,x8/2,s9,x10/3,s10,x12/6,s8,x13/4,s10,x1/11,s11,x9/4,s14,x11/13,s8,x1/2,s9,x14/3,pb/m,x15/5,s13,x13/8,pp/i,x2/3,s6,x10/15,s4,pc/h,x7/0,pb/l,x13/8,s2,x5/14,s5,x8/4,s11,pa/h,x2/13,s11,x5/14,s11,x3/4,pm/b,s9,x2/15,s9,x4/0,s6,x9/7,s3,x1/15,s5,x11/5,s10,x9/7,s2,x8/5,po/l,s13,pd/h,x0/2,s11,x14/12,s15,x3/13,pb/i,x0/10,s11,x14/9,s9,pp/c,x1/3,s15,pf/a,x10/6,s8,x9/14,pj/p,x8/3,s12,x10/15,pl/h,x4/14,s14,pa/d,x2/8,s13,x12/10,s2,x13/3,pi/g,s8,x7/8,s2,x15/10,s8,x3/1,s11,x9/4,pk/o,x13/6,s12,x10/0,s6,x5/9,s15,x0/11,s10,x12/13,s4,x11/3,s4,x8/13,pf/n,s4,x7/0,pi/b,x12/8,pg/d,x1/5,s13,x8/10,pk/i,x11/3,s10,x2/4,s13,x6/11,s1,x2/5,pe/n,x13/0,s7,x11/2,s4,x8/14,s8,x4/5,pf/g,x2/13,pe/i,s7,pn/m,x7/6,pj/o,x15/2,s4,x6/12,s14,x10/11,s8,pe/l,x9/12,s3,x11/15,s4,x14/10,pd/m,x11/13,pk/h,s4,x1/8,pc/m,x12/4,s3,x11/3,s2,x15/4,s14,x1/5,ph/o,x8/6,pb/c,x10/13,pg/f,x1/5,pp/a,x2/15,s7,x13/3,s12,x10/1,s11,x0/3,s8,x14/4,s12,x5/15,pe/f,s14,x13/12,s6,x4/11,s13,x3/5,s13,x2/13,pb/p,s10,x11/0,s3,x4/6,s10,x14/2,s15,x10/9,po/e,x4/11,pl/a,s10,x5/12,s15,x2/9,pg/d,x8/11,pb/f,x10/4,s7,x7/2,pp/o,x8/3,s4,x4/12,s5,x11/3,s1,x14/9,pl/e,x11/12,s15,pi/k,s1,x6/1,pe/b,x10/4,pl/a,x7/0,s13,x2/15,pf/k,x4/7,s14,x8/0,s6,x3/7,pm/i,x1/5,pb/l,x9/15,pp/k,x2/1,s5,x6/10,s3,x12/14,s10,x7/0,pi/h,x8/2,s2,pf/g,s2,x7/3,s3,ph/d,x14/13,s2,x1/15,po/m,x11/5,s15,x2/7,s4,x1/10,pj/a,s7,x15/14,po/k,x11/8,s10,pc/a,x6/15,s14,x12/1,s2,x7/9,pf/i,x5/4,pd/e,x11/13,s11,x9/6,pa/g,x13/8,s8,x2/14,s11,x13/9,s10,x7/11,s5,x2/14,s5,x10/7,s13,x0/13,s14,x5/3,s8,x14/15,s9,x4/0,ph/m,x14/1,po/k,x0/4,pf/b,x13/6,s10,x9/12,s1,x13/3,s1,x7/15,s12,pj/o,s12,x9/5,pm/p,x12/6,pk/j,x14/15,pc/n,x13/5,pk/a,x14/12,pn/f,x1/11,pa/p,x7/2,s8,x9/8,s8,pk/d,x2/12,s8,x15/3,pb/c,x14/9,s7,x10/7,s2,x12/11,s9,x2/3,pa/j,s15,x1/0,s9,x14/8,pc/f,x3/1,s4,x14/6,s1,x13/7,pd/o,x10/0,ph/c,x8/7,s3,x11/4,pj/g,x9/2,pp/n,x6/8,pc/k,x3/11,pa/i,x2/0,s6,x15/5,s12,x2/7,pn/b,x3/0,ph/m,s11,x7/1,s4,x5/10,s13,x11/0,pj/g,x13/3,s11,x7/15,s14,x6/0,s13,pk/a,s7,x4/15,pc/m,x13/6,pl/k,x14/1,s4,x15/11,s1,pg/p,x3/5,s13,x2/14,pa/c,x12/15,pi/k,x6/8,s10,x14/15,s5,x12/8,s15,x5/3,s9,x2/4,s14,x8/7,pb/f,x10/4,s10,x5/9,s3,x4/14,s4,x7/2,pd/j,x0/12,po/l,x3/11,pc/h,x9/0,s11,pp/b,x6/12,s3,x14/4,s9,x5/3,s9,x0/12,s1,x10/7,pd/h,x11/6,s8,pp/f,x14/12,s6,pj/i,s11,x11/0,s13,x7/6,pf/b,x15/1,pa/l,s5,x2/5,po/n,x0/3,ph/b,x8/7,s15,x4/12,s4,x14/11,s6,x15/7,pj/p,s7,x12/6,s5,x5/7,s5,x0/3,s15,x6/11,s4,x13/14,s12,x2/1,pf/g,x11/0,s12,x7/3,s12,x10/4,s10,x5/13,s14,x1/6,pp/h,x7/13,pe/i,x4/15,s6,x3/1,s7,x7/5,s13,x1/0,pj/h,x15/5,pc/i,x9/2,s7,x11/0,s2,x4/13,s8,pg/j,x2/11,s15,x8/14,s11,x6/9,pe/h,x2/12,s6,x14/0,s11,pd/b,x11/3,s10,x1/8,s12,x12/0,s13,x8/13,s7,x2/9,pg/c,x10/6,s5,x4/7,s6,x13/12,pm/o,x4/10,pc/n,x7/15,po/i,x11/3,pl/n,x15/5,pb/a,x13/0,pg/e,x1/5,pa/o,x13/0,pj/m,x14/5,s11,x7/8,s6,x15/9,s5,x14/12,s4,x4/3,ph/c,x12/6,pl/j,s11,x10/0,s9,pd/c,x14/15,s5,x11/3,s3,x8/0,s12,x13/9,s8,x5/7,pl/k,s2,x2/11,s7,x14/10,s14,x11/15,pd/a,x14/4,pj/i,x1/6,s10,x0/14,pl/g,x4/6,s9,x15/13,s9,x12/1,s13,x10/8,pi/c,x5/4,po/f,x0/8,pm/l,x5/11,s9,x8/7,pp/n,x15/10,pi/e,x7/9,pa/f,s13,x3/5,s3,x13/1,pb/d,x7/2,s12,x6/12,pc/g,x9/10,s7,x12/1,s7,x6/11,s5,x1/15,s11,x2/12,s3,x14/0,po/b,x5/7,s10,x13/9,pc/f,x7/15,po/a,x14/5,pl/b,x4/2,ph/j,x14/11,s4,x13/5,s15,pi/n,x3/10,s8,x1/13,s11,x10/9,s2,x3/6,s3,x2/15,s6,x10/12,pf/o,x1/13,s13,x11/7,s11,x9/15,pm/n,x2/14,s12,pp/i,x8/3,pm/n,x6/12,pd/o,x9/11,s4,x12/14,pc/g,x10/5,s6,x12/3,s8,pe/h,x8/1,s13,x0/15,pn/c,x7/10,s12,x15/3,s1,x11/9,pb/j,x5/15,pg/d,x13/7,s9,x1/8,pk/j,x9/7,pc/p,s3,x12/5,s6,x10/3,po/e,x1/12,s15,x8/9,s6,pa/n,x4/3,s9,x1/8,s12,x15/5,s2,x3/14,pe/h,x13/1,pf/l,x8/7,s1,x1/14,pd/a,x7/2,pp/m,x8/5,s7,x0/12,pi/b,s11,x11/2,s3,x8/0,s5,x11/3,s2,x15/9,pj/o,x1/5,s9,x8/4,s12,x3/6,pk/a,x11/7,s1,x14/2,pl/f,s3,x12/6,s9,x3/2,s15,x15/6,pk/i,x12/1,pe/d,x6/10,s12,x15/0,pn/a,x9/10,s4,x13/3,s2,x9/6,s4,pm/b,x4/5,s2,x1/3,s2,x13/10,s1,x1/2,s4,x11/0,po/f,x9/4,s4,x1/0,pp/m,x15/4,s7,x5/2,s4,x8/9,s14,x5/13,pb/c,x8/3,pe/d,s11,x14/13,s3,x6/9,s6,x14/1,s10,x4/0,pk/n,x3/2,pb/f,x15/13,s4,x12/4,pj/o,x11/0,s9,x4/5,s8,x3/11,pn/h,x1/6,pd/a,x14/7,s4,x0/3,s4,x5/15,ph/c,s14,x2/14,pk/p,x11/12,s1,x15/14,pg/n,x10/2,s10,x14/6,s15,x3/4,s12,x14/1,pk/f,x6/10,pn/h,x3/4,pc/a,x10/7,pk/j,x13/6,pi/d,x10/12,pl/c,x4/5,s6,x0/3,s5,x8/14,s12,x7/3,pk/n,x11/5,s7,x9/7,s1,x6/4,pg/f,x5/7,s12,x0/12,s11,pi/b,x14/2,s3,x13/7,s14,x0/3,s8,pk/o,s1,x12/5,pi/e,x2/8,s13,x3/12,s5,x10/14,s8,pd/f,s10,x0/5,s3,x2/3,s2,x14/15,s14,pj/n,s4,x0/6,s12,x13/1,pp/c,x5/12,po/j,x10/13,pb/c,x5/11,pd/o,x8/9,pn/b,x11/2,s12,x15/8,pd/p,x14/0,ph/l,x4/2,pi/f,x5/6,s1,x4/0,s10,x12/2,s4,x15/13,pe/m,x4/6,s10,x12/1,pb/c,s10,x6/3,s6,x9/10,s11,x5/12,pj/f,x7/2,pa/d,s7,pl/k,x12/4,s7,x7/2,s10,x3/9,s13,x12/8,s13,x4/1,pf/o,x9/2,s12,x3/15,pa/e,x1/5,s10,x10/6,s1,x3/9,pc/d,x6/0,pf/j,s12,x3/2,s11,x1/14,s5,x7/0,s11,x1/15,ph/k,x8/7,s15,pa/p,x14/2,s4,x0/15,s9,x3/11,s5,x8/15,pn/l,x3/4,s15,x5/13,s5,x4/14,s8,pj/e,s4,x10/8,s7,x2/11,s1,x12/1,s11,x5/15,s3,x6/10,pf/i,s12,x1/13,pe/n,x0/15,s7,x2/14,s2,x15/9,pl/b,x1/7,pj/o,x0/12,s15,x1/9,pk/i,x4/11,pb/d,s7,x8/6,pa/j,x0/3,pi/m,s7,x13/9,pe/d,x10/11,s8,x0/5,s10,x8/13,pk/b,x15/7,pf/c,x2/1,s2,x3/4,pj/h,x10/8,s10,x3/5,s14,x15/1,pc/i,x8/5,pp/e,x6/15,s2,pj/l,x12/9,po/m,x3/8,s9,x4/9,s6,x13/15,s10,x3/10,s3,x5/11,pd/e,x4/15,s6,x1/2,s1,x14/10,pl/c,x2/4,pj/d,x3/9,pi/b,x13/12,s6,x15/2,s6,x4/5,pd/m,x7/15,s12,x2/10,pl/o,x7/15,pi/f,x14/3,s11,x5/7,pk/b,x2/8,pj/e,x9/14,pk/n,s6,x0/5,pl/a,x15/14,s5,x2/4,s9,x12/0,s14,x10/15,pd/h,x3/5,s1,x6/13,s14,x15/9,s11,x4/7,pg/e,x1/0,pi/a,s4,x12/13,s13,x11/1,s4,x10/9,s15,x3/2,pg/e,s1,pl/f,x15/6,s1,x4/13,pa/o,x9/14,pk/e,x7/10,s14,x5/15,s4,x8/0,pc/o,x3/6,s8,x0/13,s4,x10/5,pn/h,x14/6,pc/g,s12,x11/12,s11,x10/5,pk/o,x0/3,s13,x8/14,s13,x7/15,pe/j,x0/12,pc/o,x15/13,s4,pl/i,x14/3,s5,pm/j,x6/15,s15,x10/1,ph/d,x5/2,pl/f,x9/6,s8,x15/3,pa/m,x0/11,s4,x14/2,s8,x10/0,s11,x1/13,s5,x15/2,pd/c,x11/1,s10,x0/5,s12,x1/11,s3,ph/p,x5/15,pb/k,x2/1,pn/m,s9,pj/i,x6/15,pe/o,x1/14,s12,x0/11,s11,x8/1,s7,x6/11,s8,pj/b,x7/15,pl/o,x12/5,pf/i,x13/2,s14,x10/5,s7,x15/7,pj/k,x12/8,pg/l,s1,x13/10,s9,x6/1,s2,x15/2,s11,x9/12,pn/m,x5/0,pa/i,x2/7,s15,pb/m,s12,pi/e,x12/8,s9,x2/1,s2,x0/12,pc/d,x2/11,s8,x8/10,s5,x2/7,pe/g,x10/3,s5,x8/15,s13,pl/a,x13/11,pe/b,x0/10,s13,x14/11,s14,x12/4,pi/h,s9,x14/2,s6,pe/m,x15/5,s4,x1/6,s12,x14/13,s2,x0/11,pp/l,x3/7,po/g,x15/11,s12,x12/14,pp/a,x5/13,s6,x10/1,s6,x9/7,s13,x5/8,s3,x10/1,s11,pb/m,x7/11,s3,x12/8,po/l,x1/2,pp/a,x8/9,s14,pe/b,x2/3,s15,x14/11,s5,pc/j,x12/3,s15,pe/b,x13/10,s9,x1/11,s10,x7/13,s13,x14/9,s2,x2/6,s12,x15/7,s3,x13/0,pa/m,x11/15,s10,x12/9,s9,x13/10,s5,x1/2,s7,x10/6,s15,x9/1,pg/f,s1,pb/k,x8/7,pg/m,x14/1,s13,x4/8,pi/h,x12/14,pa/c,x15/9,s5,x7/0,pp/m,x11/12,pb/n,x8/1,s1,x12/3,s4,x8/0,s15,x13/4,pk/a,x7/2,pg/n,x11/14,pk/e,x9/1,s3,pl/b,x14/3,s10,x10/13,s10,x8/1,pn/o,x14/9,pe/c,x1/12,pb/m,x14/11,po/n,x9/0,s10,x8/2,pb/j,x12/3,ph/l,x8/7,pi/c,x10/6,ph/p,x9/11,s7,x2/14,s14,x8/3,s13,x1/14,s5,x4/0,pf/b,x1/13,pp/j,x8/12,s1,x1/11,s15,x6/5,s6,x13/7,s13,x0/10,pb/k,x4/9,pd/a,x13/5,s5,x11/2,s8,x13/9,s1,x11/0,s3,x3/2,pi/n,x1/15,s14,x6/14,s5,x12/0,pg/h,x10/8,pa/l,x12/7,s8,x3/14,pm/n,x9/7,s13,x15/13,pp/a,x12/10,s4,x5/9,s2,x2/10,s14,x3/5,pg/h,x10/4,s12,x6/9,s10,x11/4,pf/l,x10/13,pi/o,x6/8,s5,pl/d,x3/2,s5,x10/4,s9,x2/1,s10,x8/15,s3,x0/5,s9,x2/7,pb/e,s9,x15/4,pp/j,x8/10,pk/i,s7,x15/4,s7,x6/8,pp/a,x10/3,pd/g,x15/14,s7,x4/0,pb/j,x12/8,s9,x4/0,s6,x11/10,s9,x9/4,s8,x13/3,pc/d,x7/12,s11,x10/8,s11,x13/1,s4,x5/11,s8,x12/0,s6,x11/13,s8,x2/0,s7,x9/12,pl/a,s14,x2/13,ph/f,x6/12,pm/g,x4/8,pi/l,s1,x5/10,pa/d,x1/13,pp/h,x15/5,s9,x14/0,s8,x12/4,pa/c,x3/5,s11,x2/11,s4,x3/0,s2,pi/g,x14/2,pa/d,s15,x3/4,s8,pj/p,x15/10,s14,x4/13,pm/i,s15,x11/8,pj/b,x6/0,s9,x12/13,s14,x3/10,pi/m,x4/6,po/f,x9/14,s2,pd/i,x12/0,s15,pk/g,x11/14,s1,x9/0,pj/f,x8/2,pk/m,x4/14,pb/o,x11/2,pc/d,x8/0,s14,x15/5,s13,pl/e,x6/11,pb/c,x2/0,pj/h,x15/3,s15,x7/2,s5,x10/3,pf/m,x5/11,pi/l,x14/7,pd/j,x13/5,pp/n,x1/12,s13,x6/4,pb/m,x7/12,s8,pk/l,x15/2,pf/p,x5/8,pi/m,x14/13,s10,x3/8,s1,x9/6,s15,x13/3,pe/b,x5/10,s3,x6/4,s9,x8/7,s13,x3/15,pp/j,x11/6,s7,x9/2,pm/b,x0/3,pd/j,x2/14,s4,x15/3,pm/o,x1/5,s15,x4/8,pa/i,x2/9,pn/h,x1/14,pc/i,x5/15,pk/p,x4/7,pg/n,x2/6,s3,x11/4,pb/h,x8/3,s10,x12/13,s5,x14/11,s9,x2/4,s1,x11/1,pg/l,x4/14,s4,x3/12,pn/j,x2/15,pl/p,x7/5,s6,x3/4,s2,x7/5,pf/j,x12/10,pm/c,x2/14,pi/o,x13/1,pd/m,x11/3,s5,x8/15,s7,x1/0,pi/a,x4/6,s13,x15/13,s9,x1/12,s11,x9/15,ph/k,x2/6,pf/e,x5/11,s4,x13/9,s2,pn/b,x4/7,s5,x9/6,s5,x13/1,s4,x7/8,s6,x11/10,s13,x13/1,pp/d,x9/6,pc/g,x13/7,s7,x9/2,s7,x4/11,po/f,x2/7,pd/i,x8/13,s11,x2/10,pp/h,x3/1,pe/n,x0/5,s3,x14/13,s8,x15/11,s14,x7/10,s15,x3/12,pp/j,x1/14,s12,x10/11,po/i,x5/13,pl/n,x4/0,s14,x1/3,s14,x4/14,s6,x2/6,pg/e,x10/3,pc/h,x2/0,s7,x11/4,s8,po/f,x9/1,pa/e,x4/11,s14,x1/3,s13,pn/d,x13/12,s4,x11/7,s7,x12/13,s11,x6/2,s4,x4/0,s7,pm/k,x6/8,s8,x14/13,pn/d,x8/15,s8,x13/9,s13,x12/10,s3,x3/7,s15,x9/1,pf/o,x0/14,pa/p,s9,x4/12,pl/n,x11/9,pg/i,x8/7,pe/n,x4/9,pp/d,x15/3,s5,x10/7,po/m,x12/3,s10,x4/10,pb/f,x15/6,s5,x12/13,po/l,x15/11,s8,x4/7,pj/d,x13/12,pn/g,x11/2,s5,x12/14,pl/h,x9/2,pd/g,x15/8,ph/l,x1/3,s7,x9/2,pp/d,s1,x5/1,s6,x12/7,s1,x9/8,pk/b,x12/11,s13,x8/1,s11,pl/h,x7/9,pe/n,x0/2,s2,x14/5,s4,x2/15,s4,x4/5,pa/b,x3/15,s14,x5/0,s13,x4/15,pg/o,s2,x3/13,s9,x2/15,s10,x10/12,pk/f,s12,x2/8,pl/h,x12/10,pa/o,x7/8,pi/b,x14/4,pa/c,x7/5,s3,x15/2,s6,pd/k,x11/3,s10,pj/m,x4/9,pp/e,s4,x5/15,s10,x11/1,pn/d,x4/15,s13,x1/9,s15,x15/6,s8,x1/11,s4,x0/5,s15,x11/12,s7,x7/10,s9,x13/1,s13,x5/12,s3,pm/h,s15,x10/14,s10,x11/12,pp/c,s14,x0/1,s8,x3/2,s8,x12/13,pe/g,x2/14,s15,x13/8,pd/a,x1/0,s11,x8/13,pm/g,x12/9,pe/j,s15,x4/7,s8,x6/10,pd/f,x2/1,s13,x5/15,s4,x3/12,pb/e,s7,pm/d,x5/7,pi/k,x13/14,pp/o,x12/8,s3,x4/1,pc/b,s12,x12/8,s2,x3/5,pg/h,x6/9,s15,pa/i,x10/1,s8,x13/0,pf/n,x14/1,s14,pk/a,x3/4,s7,x5/0,s13,x1/13,s6,x2/0,pd/g,x7/1,pn/m,x12/6,s6,x5/15,pc/a,x10/0,s4,x6/13,s15,x12/3,s13,x0/5,pl/f,x6/8,s15,x1/0,s15,x11/15,s8,x14/1,pk/o,x0/13,s1,x2/10,pi/d,x15/11,s2,x0/7,s5,x13/6,s8,x8/0,s13,x11/15,s14,x4/8,s2,x0/1,s7,x6/5,pk/j,x13/1,s1,x14/0,s10,x10/12,s5,x6/3,ph/n,x11/2,s2,x7/14,s3,pi/c,x2/0,s9,x12/5,s14,x9/11,pb/e,s4,x15/7,s7,x10/9,ph/g,s3,x0/11,pf/e,x12/8,s5,x1/5,s11,x2/7,s11,x10/6,s14,x7/2,s3,x6/11,s7,x9/5,s9,x6/12,s9,x15/5,pj/b,x0/7,s8,x9/3,s6,x4/15,pi/k,x7/5,pl/f,x9/14,s9,x0/3,s9,x8/4,s15,pd/g,x2/12,pb/p,x13/11,s14,x3/10,s7,x12/8,s6,x1/5,pc/k,x3/12,s3,x8/13,po/l,x6/3,s4,x9/1,pn/f,s11,x5/13,s2,x4/15,pj/k,x12/9,pl/o,x11/14,s11,x15/1,s6,x12/14,s12,pb/h,x2/4,po/g,x6/3,s4,x14/15,pa/b,x1/0,s14,x6/11,s3,x13/0,s7,pp/l,x6/7,s2,x10/5,s9,x12/2,po/k,x9/15,ph/a,x4/8,s6,x1/9,pf/m,x4/12,s5,x5/2,pj/n,x3/15,s3,x1/7,pm/g,x11/12,s3,x10/2,pf/c,x9/6,pa/n,x7/15,s14,x3/4,s8,x15/10,s10,x9/8,pm/l,x3/6,s10,x8/4,s5,x14/15,s14,x1/4,s4,x9/5,s10,x6/13,s7,po/p,x9/12,s2,x13/6,pf/j,x10/1,s3,x11/0,s2,x14/3,pk/a,s10,x5/8,pb/g,s11,pk/a,x6/7,pm/c,x9/14,s8,x12/6,pf/d,x11/8,pm/i,x4/13,pc/a,x8/10,pn/b,x3/12,s7,pg/f,x5/2,pl/b,s5,x8/0,s4,x14/11,pg/p,x3/12,s4,x10/5,pj/o,x13/7,pp/m,x3/10,s8,x9/4,pl/e,x14/10,pi/o,x13/4,s12,x11/8,s7,x0/2,s10,x4/15,s1,x0/10,pe/j,x15/13,pl/c,x6/12,s6,x4/10,ph/i,x12/11,s9,pc/l,s3,x10/2,s12,x6/0,po/f,x2/13,s5,pg/j,x5/6,s8,x3/8,s6,x9/11,pd/i,x5/13,po/p,x2/0,s11,x13/3,pf/j,x1/9,s11,x13/8,s5,x4/12,pl/i,x3/5,pe/h,x4/14,pn/d,x13/15,s13,x4/10,pb/l,x9/7,s10,x14/0,s9,x4/13,pd/n,x14/7,s13,x5/2,pf/g,x4/9,pb/c,s6,x15/12,s7,x5/10,s15,x4/14,po/l,s7,x15/0,ph/a,x14/5,pb/l,x0/10,pc/j,x14/7,s3,x8/12,s11,x15/0,s14,x10/13,s2,x9/1,pb/m,x14/3,s12,x10/8,pg/n,x15/0,s9,x9/10,pi/m,x0/12,s13,x7/6,s11,x15/8,pf/g,x12/2,s7,x6/1,s4,x2/9,pp/i,x14/8,s6,x6/11,pd/g,x2/13,s8,x0/10,pl/e,x3/14,s5,x15/13,s10,x10/11,s6,x12/9,s6,x8/1,pj/i,x2/14,pd/b,x4/3,s9,x6/1,s12,pj/f,x7/15,s12,x13/8,s8,x4/12,s4,x6/2,pb/p,x5/4,pn/f,x10/1,pb/a,s10,x7/13,s7,x0/4,pk/m,x9/3,s9,x14/5,s10,x13/15,s2,x8/2,s7,x1/3,pg/b,x2/0,s11,x3/1,s15,x10/14,s12,x0/6,s11,x8/14,pa/o,x3/7,s11,x5/14,pi/f,x10/6,s13,x14/8,pk/c,x4/3,s12,x9/2,s2,x3/1,s1,x5/15,s8,x3/8,s10,x10/11,s10,x14/3,s3,x15/7,pb/j,x8/11,s9,x5/6,s13,x12/13,s13,x1/10,pd/k,x11/14,s5,x13/10,ph/n,x5/6,s7,x14/11,pa/k,x12/6,s15,x4/15,po/d,x8/12,s7,x2/11,pa/b,x6/12,s9,x3/11,s8,x12/1,pl/e,x10/7,s10,x8/4,s10,x6/12,s13,x8/7,pk/m,x10/12,s9,x8/2,pa/i,x14/9,s12,x6/11,pn/m,x13/1,s12,x14/6,pi/g,x0/11,s2,x6/15,s1,x4/12,s7,x1/11,s2,x7/12,s12,x15/14,pp/h,x8/1,pi/o,x7/13,pc/a,x14/0,s5,x2/7,s8,x5/8,pg/k,x12/10,pj/o,x11/8,pl/d,x6/4,s15,x14/12,pj/k,x15/9,s15,x0/1,pf/o,x8/4,s14,x13/14,s12,x3/15,s1,x0/6,s8,x9/3,pk/n,x2/11,s11,x1/4,pf/c,x6/7,pb/n,x11/14,s9,x7/2,s11,x3/12,pi/d,x9/2,s6,x12/7,s15,x3/11,pc/g,x6/10,pi/e,x0/11,s6,x9/13,s1,x11/1,ph/g,x15/10,pj/a,x14/6,s10,x8/13,s8,x9/0,s5,x13/1,pb/i,x15/2,s6,pp/a,s4,x4/13,pj/h,x7/5,s6,x15/14,pm/o,x4/7,s1,x3/12,pp/a,x11/4,pg/d,s14,x10/13,s15,x1/12,s11,x5/7,pp/k,x12/1,s15,x11/5,s12,x3/6,pl/c,x14/2,s3,x15/4,pb/m,x3/1,pa/h,x5/6,pf/j,s13,x15/10,ph/e,s3,x5/11,pi/f,x15/10,s11,x7/14,pj/a,s12,x6/13,ph/c,x1/12,pg/i,x6/8,pp/f,x10/4,pg/d,x14/6,pp/b,x5/3,s6,x8/11,s10,x9/1,pe/m,x2/14,s3,x7/9,pl/b,x10/2,pf/p,s1,x15/0,pb/j,x9/8,s10,x5/10,s14,x4/2,s8,x12/1,s12,pi/d,x11/13,po/c,x4/10,pi/d,x15/7,s11,x10/8,pj/m,x7/9,pb/d,x8/5,po/f,x7/2,pe/h,x10/13,pb/i,x9/4,pm/l,x6/3,s3,x4/12,s2,pd/o,x8/5,pa/p,s14,x1/14,s1,pi/d,x2/12,pl/o,x1/13,s3,pp/f,x7/9,ph/j,x0/2,pm/e,x11/4,s13,x12/6,pj/b,x3/8,pk/a,x5/6,s10,x10/11,pe/o,x12/9,pj/p,x0/5,s11,x14/9,pe/f,x3/4,pa/p,s14,x6/0,s11,x8/3,pd/f,x11/5,s14,x9/2,s7,pg/n,x0/14,pj/c,x4/3,s11,x14/2,s1,x0/7,s11,x8/2,s13,x12/3,s11,x9/7,s14,x10/2,s9,x1/12,pi/o,x14/3,pc/b,x7/8,s7,x11/14,s4,pp/l,s5,x2/8,s11,x11/9,pk/e,x5/1,s15,x2/14,pb/d,x11/7,s15,x13/15,pk/o,x5/4,s4,x6/2,pc/p,x8/9,pj/f,x5/10,pb/i,x9/8,s10,x6/10,pj/a,s7,x5/8,s2,x13/14,s12,x11/10,s6,x5/4,pb/p,x1/10,pc/d,x6/12,pm/o,x9/15,s4,x14/2,s7,x12/8,s14,x3/1,pp/b,s2,x8/2,s4,x15/0,s14,x7/11,s4,x5/13,s11,x10/7,s8,x2/9,pe/d,x0/1,s13,pm/f,s10,x3/15,ph/l,x9/11,po/e,x15/1,pg/p,x4/0,pn/b,s12,x13/12,pm/l,x6/4,s11,x3/2,s6,x0/4,pe/p,s4,x8/7,s12,x4/3,s11,x12/10,s1,x9/6,s14,x15/13,s14,x0/4,pc/m,x9/7,s9,x11/13,ph/j,x14/1,pc/m,x11/0,ph/o,x4/15,s14,pp/f,s7,pi/b,s10,x11/12,s4,x4/9,s14,x10/13,pg/l,x1/12,pn/f,x5/8,s15,x15/12,ph/m,x14/3,pl/e,x9/5,ph/i,x4/8,s6,x9/11,s10,x13/1,s9,x0/12,pc/k,x13/4,s9,x1/7,s9,x0/15,pb/l,x12/5,pm/p,x0/8,ph/c,x5/7,s8,x11/14,s6,x0/5,pm/g,x1/12,s4,x5/8,pf/c,x6/15,s10,pb/a,x14/5,s1,pj/e,x3/4,s4,x12/15,s8,x13/3,pa/d,s14,x6/9,po/e,x5/13,s10,x12/10,s2,x0/3,pp/b,x5/15,s15,x12/14,s7,x2/7,pj/d,x0/11,pb/l,x14/8,s14,x13/2,s2,x4/7,pp/g,x11/8,s8,x3/9,pb/l,x1/10,s9,x7/0,po/g,x12/15,pd/h,s4,x10/13,s14,pm/g,x5/9,ph/e,x7/4,po/j,x5/9,s13,pg/d,x2/13,pi/n,x7/14,s14,ph/c,s2,x4/3,pl/o,x10/0,pa/k,x12/2,pd/i,x15/1,pl/n,x4/2,pe/j,x14/3,s14,x12/13,pf/n,x5/2,pa/e,s5,x11/14,s2,pc/h,x9/7,pa/d,x14/8,pc/b,x9/3,s1,pe/h,x1/11,s15,x13/2,s10,x14/1,s1,x15/2,s3,x14/7,s14,x15/1,s5,x10/2,s3,x13/15,s9,x1/4,s5,x10/8,s6,x5/14,s7,x7/6,pj/m,x8/14,s6,x10/6,pa/h,x11/13,s7,x12/9,s5,x5/2,pi/d,x6/0,pg/a,s14,x2/14,s13,pd/n,s4,x15/9,s2,x2/12,pa/f,x4/8,pi/d,x6/15,pa/g,x9/13,pc/b,x3/4,s6,x0/10,s10,x13/12,s1,x6/14,s11,x5/11,s5,x8/1,ph/i,s12,x13/11,s6,x9/1,pe/c,x12/10,pl/g,x9/5,s3,x8/3,pe/d,x6/11,s12,x12/10,s4,x8/5,s6,x0/4,s10,x9/13,pg/n,x5/14,s5,x1/6,pc/j,x11/2,s2,x3/8,s11,x5/6,s12,x0/10,pf/a,x3/11,s2,x4/8,pp/e,s7,x5/1,s4,x6/0,s6,x4/1,s6,x15/6,pi/j,x13/10,s14,x8/7,pp/f,x10/2,s12,x0/6,pk/d,x5/13,s1,x2/9,s3,x15/8,pi/b,s14,x10/0,s2,x7/5,s14,x12/15,s3,x13/11,pe/m,x14/0,s11,x4/3,s3,x15/1,po/b,x0/9,pn/f,x5/1,s12,x12/2,s5,x1/5,pg/i,s5,x11/13,pd/m,x9/6,s11,pp/i,x2/3,s15,x5/14,ph/k,x12/2,pg/m,s6,x10/14,pj/n,x12/3,s12,x14/10,s12,x15/9,pg/p,s5,x6/4,s10,x7/10,s9,x3/0,pa/h,x4/12,s13,po/f,x1/8,s9,x13/11,pi/n,x7/14,pp/o,s15,x10/4,pl/d,x1/6,s12,x5/0,s1,x2/9,pa/c,x11/15,s10,x7/1,ph/b,x4/14,s1,x11/8,pn/a,x10/5,pl/j,x11/3,s9,pi/p,x4/0,s7,x15/12,pk/d,x2/7,s5,x10/6,pa/h,x7/12,pe/c,x3/9,pa/d,x11/0,s7,x4/13,s10,x12/15,s14,ph/n,x4/8,pg/d,x5/10,s15,x0/13,pp/c,x5/9,s11,x0/11,s13,x4/15,s11,x13/7,s1,x4/8,s12,x5/7,s6,pj/d,x13/14,pm/e,x2/11,s8,x3/0,pl/i,s11,po/f,x11/12,pe/g,x14/15,pb/f,x4/8,pi/p,s11,x9/2,pn/b,x1/6,s3,x10/15,pa/p,x1/14,pi/h,x0/8,s4,x9/1,pe/d,x5/6,s5,x15/8,pa/n,x4/0,pc/b,s6,x13/12,s10,x4/2,pd/f,x9/5,s1,x3/10,pm/n,s6,x14/4,pk/l,x2/10,pe/n,x1/12,pd/i,x5/6,s7,pn/l,x15/7,s9,x8/11,pi/a,s4,x9/5,s13,x6/8,s6,x4/13,s2,x11/12,pk/l,s7,x10/5,ph/e,x4/2,s3,x14/15,s11,x0/1,pp/l,x7/10,s7,x3/12,s1,x7/6,pc/f,x5/13,pn/e,x10/8,s11,x6/2,pm/f,x8/3,pk/g,x1/9,po/p,x3/5,s2,x2/1,pd/i,x0/5,s3,x7/6,pc/b,x3/0,s11,pf/m,x12/8,s1,x1/3,pe/g,x0/12,s1,x15/5,s13,x6/4,s7,x0/7,s5,x14/9,s3,x10/12,s14,x4/7,s6,x9/15,s3,x10/11,pk/c,x7/4,s6,x8/13,s6,x5/6,s2,x12/14,pn/d,x1/10,pj/b,s14,pd/l,x5/14,s12,po/m,x9/11,pl/f,x15/3,ph/j,s8,x6/11,s8,x1/9,s3,x0/13,pf/o,x2/6,s1,pl/k,x12/4,pp/o,x8/0,pj/n,x4/15,s2,pe/c,x6/13,s3,x0/10,s5,x8/6,s10,x5/12,pf/p,s5,ph/b,x4/15,pl/i,x10/0,pa/g,s14,x3/12,pe/o,s6,x5/6,pl/g,x8/1,s7,x6/12,pp/h,x4/14,s8,x8/11,s6,x12/4,pj/e,x3/8,s15,x9/14,ph/k,x0/3,s9,x14/6,pi/f,x5/15,s13,pd/j,x11/14,s8,x6/5,s11,x3/1,s1,x2/13,s11,x7/1,pa/p,x8/11,s4,x15/6,s14,pg/e,x13/5,s8,x1/10,s8,x8/6,s3,x13/4,s8,x6/8,pj/h,s14,x10/3,pn/b,s12,x4/5,s7,x0/1,s3,x15/4,s5,x5/8,s3,x7/3,pf/c,x5/11,s15,x14/4,s5,pk/d,x0/15,pj/l,s11,x10/7,s8,ph/p,x2/5,s9,x6/9,pi/n,x3/0,pj/p,x11/1,s15,x8/3,pm/g,x9/1,pd/j,x15/3,s14,x0/9,s9,pc/h,x1/6,pl/k,x11/7,s13,x4/13,pe/b,x15/5,pk/d,s13,x8/12,pe/b,x9/1,po/i,x15/2,s11,x11/4,pj/g,x5/12,s10,x10/2,s9,x0/8,s5,x5/6,pn/p,x0/10,s2,x13/14,s2,x7/2,s4,x6/9,s6,x1/14,pj/g,s11,x3/9,s8,x14/13,pi/b,s15,x3/10,pe/f,x13/12,s14,pj/d,x3/2,pm/f,x13/10,pk/j,x5/11,pc/d,x13/8,pp/i,x11/6,s8,x7/0,s14,x3/1,s13,x2/13,s7,x6/8,s2,x7/15,s4,x10/14,s13,pb/n,x2/6,pj/h,x15/10,s14,x13/1,pb/d,x0/5,s10,x14/3,s9,x12/5,s1,x7/4,pn/e,x9/2,pf/k,s2,x12/1,s11,x5/2,s15,x8/7,s4,x12/1,pj/n,x0/7,pf/k,x11/2,s11,x15/3,pe/m,x0/4,s11,x5/8,s4,x10/15,s7,x2/14,s2,x11/5,s9,x7/6,pj/n,x11/13,s5,x2/10,s11,x15/8,s11,x11/13,pi/b,x3/8,pm/e,x9/5,s14,x4/3,s7,po/j,x12/10,s4,x14/0,s15,pi/a,x7/4,pg/n,x3/1,s3,x11/14,ph/m,x13/12,s11,x2/14,s13,x0/10,s11,x9/12,s13,x3/7,s4,x9/12,s15,x5/3,s5,x10/15,s2,x4/13,s14,x9/10,pj/p,x11/2,s14,x5/10,pn/m,x9/7,s3,x8/10,s7,x15/4,ph/f,x9/5,s15,x8/10,pi/g,x9/12,s14,x14/10,po/h,s13,x9/15,pc/n,x12/10,s7,x6/3,s13,x5/13,s8,x8/2,pd/b,s11,x7/12,s12,x10/13,pp/g,x12/9,s1,x11/13,s15,x1/14,pd/c,x0/10,pp/f,x4/15,s2,x12/8,pl/e,x0/6,s10,x4/2,s12,x5/10,s11,x8/7,pm/p,x6/14,s9,x0/11,s4,pc/d,x5/1,pe/k,x4/9,s9,x7/0,pb/j,x8/15,s10,ph/c,s11,x14/5,pe/d,x0/3,pm/g,x6/15,s15,x9/4,pj/e,x0/12,pn/h,x6/1,s4,x7/10,s14,x6/4,s4,x11/9,s9,x4/3,pg/a,x1/11,pc/e,x9/14,s5,x7/4,pg/d,x5/11,pm/n,x9/4,s14,x7/6,pe/l,x8/3,s5,x0/15,pp/g,x7/6,po/c,x14/12,s1,x2/11,s11,x9/0,s10,x5/1,s10,x10/6,s4,x3/13,s15,x10/8,s15,x13/1,s10,x10/5,s3,x9/15,s7,x8/5,pa/i,s14,x4/10,s10,x14/7,ph/p,s3,x2/13,s10,x1/14,s4,x11/0,s13,pb/j,s9,x7/8,pd/e,x1/14,pa/m,x6/2,s15,x15/13,s14,x10/0,s5,x2/4,pp/f,x8/9,pg/h,s3,x11/1,pa/l,x10/0,s3,x15/12,pp/e,x3/4,s8,x5/15,pi/m,x6/2,s8,pk/e,x13/11,pm/n,x2/0,s3,x15/3,s10,pf/i,x7/9,s1,x3/8,s9,x12/2,pn/d,x6/7,pe/j,x9/15,s8,x7/4,s3,x0/14,po/k,x12/7,s7,x13/6,pd/b,x9/8,s15,x7/5,s13,x9/15,pl/a,x5/6,s13,x13/3,s7,pm/c,x14/0,s9,x10/13,pj/b,s8,x4/14,s7,x15/5,s6,x2/4,pd/i,s4,x6/11,pk/h,x3/12,s12,x14/5,s1,x15/6,s3,x3/12,pb/e,x5/8,pm/n,x0/15,pe/b,x1/13,pn/p,x4/12,s11,x15/8,s8,x9/3,pa/b,x13/12,s5,x2/5,s14,x0/14,s3,x3/6,s9,x12/5,pp/e,x15/13,s7,x1/9,pd/o,x7/14,s11,x15/2,s1,x1/4,s13,x0/13,pg/f,x11/6,pl/m,x10/5,ph/k,x14/8,pn/c,x15/6,s12,x11/9,s10,x7/8,pb/d,x2/5,s11,x14/3,pp/g,x6/0,s12,x12/11,s6,x15/6,s7,x11/12,pd/o,x8/3,s9,x5/6,s3,pl/f,x1/2,s7,pp/i,x7/8,s10,x9/2,s13,x5/15,s4,x6/0,s11,x5/10,ph/k,x15/0,s10,x13/8,pd/n,x11/7,pa/f,s12,x2/3,s14,x5/12,pp/m,x3/10,s14,pn/o,x8/13,s1,x4/6,pi/a,x9/1,s7,x2/5,s8,x11/14,s5,x7/6,s7,x12/1,s12,x9/13,pp/k,x0/3,s13,pm/h,x2/9,pn/f,s10,x3/14,s5,x10/6,s7,pc/b,x3/2,pm/p,x12/10,s6,x4/14,s8,x8/9,s7,x13/2,s11,x14/11,pc/g,x6/9,pj/m,x2/3,pl/e,x14/15,po/j,x12/1,s9,x10/3,pc/p,x2/0,s9,x6/5,s13,pi/e,x10/12,s9,x3/6,pk/g,x14/5,s1,x3/2,pf/j,x14/13,s12,x6/9,s6,x13/10,s2,x15/4,s8,x14/1,s11,x3/11,s3,x6/8,s12,x0/14,s2,x1/12,s13,x3/14,pa/n,x12/8,s12,x15/3,s11,x4/8,s3,x10/9,pe/b,x1/14,s9,x3/8,s11,x12/15,s5,pd/i,x7/2,s8,x11/6,s11,x10/9,s3,x15/3,ph/o,x10/2,s7,x0/6,pa/m,s13,x2/9,s10,x11/0,s6,x15/5,s2,x10/14,s12,x6/2,s2,x4/5,s5,pd/c,x0/8,pk/b,x6/3,pi/a,s8,x10/13,pb/g,x8/5,s14,x10/2,pm/n,x7/9,s14,x6/13,s12,x4/5,pi/o,x12/1,pd/b,s12,x13/11,s12,x14/12,pl/h,x4/6,pd/o,x7/3,s3,x2/10,s15,x3/14,pm/n,x10/7,pj/f,x8/14,s5,x2/15,pl/e,x11/8,s12,x2/15,s8,pp/k,x14/13,s15,x9/11,s3,pa/m,x8/13,pj/c,x6/7,pe/p,x1/11,s1,x3/10,s8,x7/12,s5,x0/11,pj/d,x13/2,s12,x6/8,s11,x7/9,s7,x11/15,s6,x2/0,s1,x3/9,s12,x15/7,pn/h,x0/11,s7,pf/a,x12/4,s7,pj/i,x9/3,s2,x1/4,s9,x11/13,s13,x4/12,s7,x3/15,pn/d,x11/10,po/e,x13/5,s8,pd/a,x7/10,s1,x13/3,s15,x9/12,s1,pk/m,x15/13,s13,x12/0,pa/h,x1/8,s5,x9/14,pb/e,x0/8,s12,x2/5,s7,x8/7,pj/c,x4/3,s15,x14/15,pp/b,x7/4,ph/j,x12/15,pd/b,x2/3,pg/p,s11,x11/12,s12,x1/13,s1,x6/7,po/k,x14/10,s12,pj/e,x8/15,pk/g,x9/3,pn/m,x4/10,s15,x2/14,pa/f,x3/9,s7,x4/0,s11,x11/13,s15,pm/i,x7/9,s15,x3/2,s7,x12/0,pa/c,x15/10,pf/n,x6/8,s12,x3/10,pa/m,x5/1,s2,pe/c,x4/7,s4,x6/8,s8,pl/i,x10/11,s3,x3/14,s4,x2/15,ph/k,x1/5,s8,x3/6,pe/p,x11/1,s9,x6/10,s2,x14/8,s6,x5/15,s6,x11/4,pm/i,x7/10,pd/l,x2/11,s9,x8/10,s13,x11/12,pn/a,x14/6,s8,x3/0,pm/o,x13/2,s6,x4/8,s1,x7/2,pd/i,x12/0,pm/c,x11/5,s7,x9/6,pa/o,x0/15,s10,pg/l,s1,x11/13,pk/m,x1/7,s4,po/a,x0/3,s4,x2/1,pk/l,x6/8,s5,x2/9,pc/h,s13,pl/i,x7/5,s4,x3/6,s13,x10/1,pk/o,x11/4,pi/n,x0/6,ph/m,x12/13,s15,x5/3,s3,x15/2,s13,x3/0,pc/k,x1/9,pp/e,x12/4,s10,x3/13,pm/d,x8/10,s2,x0/3,s7,x6/12,pg/h,x8/14,pi/e,s13,x6/1,s2,x14/0,s9,x8/1,pl/o,x5/11,pc/m,x15/9,s10,x4/8,s8,x0/7,s6,x4/2,s9,x12/5,pl/j,x15/7,s11,x1/2,pa/k,x3/9,s11,x12/5,pn/i,x14/9,s13,x1/15,s6,pe/p,x9/12,pb/a,s11,x11/0,s6,x14/6,ph/g,s7,x3/2,pe/c,x6/10,s11,x12/9,pi/p,x1/14,pf/g,x12/2,s11,x5/1,pj/a,x8/7,s6,x2/4,s3,x11/15,s13,x3/2,s5,x9/0,s7,x6/5,s7,x8/10,s5,x0/13,pm/h,x2/5,s11,x9/1,s1,x10/14,s4,x15/6,pn/o,s7,x11/13,ph/k,x6/1,s5,x0/4,pc/p,s15,x6/7,pm/n,x2/9,pb/a,x0/5,s13,x15/2,pm/c,x12/4,pn/b,x13/6,s6,x0/15,s5,x10/7,s6,x1/9,s4,x11/4,s3,pa/k,x7/6,pe/i,s11,x9/12,pl/p,x13/1,pg/n,s6,x12/10,s9,x11/3,s14,x6/5,pl/m,x0/9,pe/h,x11/10,s11,pf/l,s7,pk/h,x14/5,s12,x15/13,s11,x12/4,s4,x8/6,pb/g,x0/10,s12,x2/15,s3,x7/3,s9,po/e,x11/12,pp/i,x14/0,s3,x10/1,pc/n,x15/3,s9,po/h,x7/10,s15,x15/13,s11,x7/2,pb/f,x11/9,s12,x8/14,pc/p,x11/0,s7,x7/1,pf/b,x3/2,s2,x9/7,ph/a,x2/0,s11,x4/14,s1,x13/15,s9,x6/12,s8,x11/13,s3,x7/10,s6,x0/1,pf/c,x8/5,pn/k,x2/7,pf/d,x5/15,s14,x4/10,s13,x2/0,s14,x13/9,s14,x6/2,s13,x12/4,pa/b,x5/13,s5,pg/m,s3,x1/11,s11,x6/4,pn/d,x11/1,pb/c,s15,x5/14,s2,pf/k,x3/7,pa/i,x10/8,s13,x15/1,s6,x10/9,s14,pd/o,x12/14,pi/h,x6/7,s8,x2/8,s9,x7/14,s5,x13/1,s5,x8/9,s10,x10/2,s3,x1/3,s10,x0/10,s2,x12/13,s11,x3/15,s4,x1/14,s14,x6/11,s11,x12/7,s2,x3/6,s2,x9/8,pn/c,x2/1,pf/e,x12/13,s9,x15/7,pd/n,x0/12,po/f,s1,x15/6,s12,x1/4,s8,x11/15,s5,x9/8,s12,x6/0,pm/a,x15/14,pl/o,x10/0,s5,x15/6,s11,x1/11,pe/c,x0/6,pk/l,x15/4,s2,x5/14,s5,x15/13,s9,x1/3,s15,x2/11,s6,x5/3,s5,x4/2,s6,x14/12,pm/o,x8/9,s3,x6/3,pf/n,x2/9,ph/l,x12/0,pj/e,x15/7,s11,pk/b,x2/4,pd/e,s13,x11/7,s6,x12/0,pc/p,x15/6,s13,x3/7,s7,pa/f,x9/12,pi/d,x4/2,s9,x13/15,pa/k,x5/1,pe/n,x11/0,s1,x14/3,pl/j,x4/12,s6,pg/n,s3,pd/b,x5/10,pc/e,x14/3,s6,ph/p,x1/0,s5,x11/4,pk/d,x6/12,s5,x7/3,s7,x14/6,pa/c,x15/5,s14,x13/14,s2,x15/6,s11,x13/11,pg/i,x12/1,s15,x0/13,pa/e,x12/4,po/p,x1/15,pi/c,x7/3,pd/e,x8/12,s13,x6/0,s1,x5/7,pc/k,x2/1,pp/j,x5/3,s13,ph/i,x4/7,pa/m,x8/15,s15,x11/13,s14,x9/2,s1,x6/13,s14,x5/0,pe/c,x1/13,s15,x5/6,s3,x4/9,s12,x0/1,s10,x12/6,s5,x9/3,s6,x5/8,s12,x15/4,s3,x10/3,s2,x5/8,pd/b,x3/13,s10,x6/10,pa/g,x14/1,pe/k,x5/8,s1,x1/11,s13,x10/15,pa/o,x0/1,s13,x5/12,s10,x14/4,pf/j,x5/12,pi/k,x0/3,s12,x10/9,s4,x2/6,pl/f,s7,x11/5,s6,x8/7,pc/k,x0/2,pi/g,x3/9,pd/l,x2/7,pg/c,x6/8,s14,x13/9,pb/o,x4/8,s12,x1/10,s13,x4/13,ph/d,x11/1,pk/m,x9/3,pe/p,x12/4,s13,x13/6,s14,x12/14,s10,x2/1,pb/c,s6,x12/4,s3,x1/2,s8,x3/9,s3,x8/2,s13,x12/1,pa/e,x9/15,pi/j,x11/8,s2,x6/1,s10,x12/5,s15,x8/2,pf/e,x12/4,pb/c,x15/6,s11,x13/11,pl/p,x1/14,s15,x8/3,s9,x10/1,s13,x11/7,s4,x9/2,s6,x8/10,s4,x4/15,pa/n,s15,x1/3,s1,x7/13,s9,x4/9,s10,x11/14,s8,x2/3,ph/k,x13/7,s4,x4/14,s1,x15/13,pj/o,x10/7,s14,x8/6,s13,x14/1,pe/d,x11/12,s14,x9/8,pj/h,x6/13,pm/d,x3/4,s14,pi/o,x1/5,pg/e,x14/3,s6,x2/15,s10,x14/13,s14,pi/a,x11/12,s6,x13/6,pl/j,x15/8,s2,pe/h,x0/10,s15,x14/2,pb/d,x4/6,s2,x5/14,s6,pg/o,s12,x10/13,pe/d,x14/15,s3,x9/1,pg/l,x2/4,s15,x3/10,pp/h,x5/2,s9,x14/0,pg/d,x15/5,s5,x9/11,s15,x1/13,pc/i,x5/15,s6,ph/g,x7/11,s9,x5/13,s15,x2/15,s2,x3/14,pk/o,x4/5,pj/l,x2/1,s15,x11/14,pp/f,x15/13,s4,x8/11,s4,pa/j,s15,x13/10,s15,x7/1,s1,x15/6,pb/c,x5/3,pi/h,x13/12,pg/e,s2,x5/15,pc/b,x4/13,pe/f,x15/2,pd/l,x6/14,s15,x1/13,ph/b,x15/10,s11,pp/a,x5/8,ph/d,x11/9,s15,x5/13,pn/j,s1,x3/7,s9,x9/12,s2,pk/m,x8/2,s6,x12/4,s2,x5/14,pf/b,x15/7,s9,x3/6,pj/e,s4,x1/13,pg/m,x7/12,s12,x8/15,pp/l,x10/13,s6,x12/5,s12,x4/0,pm/n,x10/8,pa/k,x14/9,pp/j,x15/7,ph/d,x11/6,s13,x13/12,s13,x14/15,s8,x4/8,s12,x13/3,s8,x10/9,s2,x7/8,s8,x14/2,s11,x10/11,pc/n,x4/13,pl/j,x5/15,pn/a,x12/0,s10,x8/11,pd/h,x1/15,s9,x0/8,pe/n,x14/12,s15,pa/h,x2/13,s7,x7/9,pg/m,x1/8,s13,x0/5,s8,x15/8,pd/n,x3/12,pm/o,x13/10,s11,x7/3,s12,x12/4,s14,x3/7,s13,x9/1,pk/b,x11/0,s11,pp/e,x4/8,pm/f,x14/5,s14,x4/10,s11,x11/12,s6,x13/2,s4,x10/0,s5,x7/4,s14,x15/1,s12,x2/12,s3,x9/13,s10,x12/3,pb/e,x8/7,pa/m,x10/6,pk/f,x14/7,pg/b,x0/12,s6,x7/3,s14,x8/2,s9,x7/3,s10,x2/6,pp/h,s4,x8/4,pi/f,x3/6,s5,x7/8,pk/l,x12/1,pi/g,s4,x3/7,s13,x2/13,pd/e,s2,x1/0,s8,x6/3,s3,x4/7,s2,x12/13,s4,x7/11,s3,x12/2,s1,x8/13,pk/m,x2/15,s12,x8/0,s4,x14/12,pb/p,x5/8,s11,x14/12,ph/f,x0/2,pe/n,x8/4,s3,x5/2,s10,x8/9,s5,x13/7,po/l,x8/11,pm/h,x1/9,s8,x14/4,s3,x2/6,s5,pk/f,x15/0,ph/g,s12,x12/1,s7,x14/11,pb/a,x12/8,s11,x4/6,s6,x8/9,s3,x2/1,pn/j,x0/9,s6,x15/5,pc/o,x12/4,pi/a,x13/8,pn/p,x7/9,s4,x6/15,pd/f,s2,x5/9,s4,x3/15,s10,ph/o,x0/12,pn/e,x13/14,s15,x4/12,pf/g,x11/0,s14,x15/9,s1,x1/12,pl/k,s13,x14/8,pp/j,x12/5,pl/f,x7/1,s11,x11/9,s9,x8/6,pb/m,s12,x9/2,s13,x7/1,pf/i,x13/11,pg/j,x10/1,s1,x0/14,pi/p,x5/12,s3,x15/3,s8,x13/8,pf/n,x3/4,s12,x10/8,pp/g,x2/11,s6,pa/c,s10,x1/8,pd/m,x6/12,ph/l,x1/11,pd/p,x12/15,s9,x3/2,s14,x7/6,s13,x10/12,s5,x3/6,s9,pb/n,x5/9,s3,x12/13,s8,x11/0,pc/f,x7/14,pa/m,x11/13,s10,x5/14,s4,x2/11,s4,x14/1,s7,x13/8,s5,x12/5,s13,x0/10,s1,x13/4,s9,x11/14,pj/c,x0/10,s3,x1/5,po/b,x13/11,s4,x5/3,s1,x1/6,pk/m,x13/3,s14,x0/5,pj/h,x14/13,s8,x11/5,s11,x8/12,pp/g,x3/2,pl/m,x5/11,s12,x9/10,s1,x1/13,s11,x3/7,pa/p,x2/4,pn/e,x10/9,s4,x1/6,s11,x3/8,s11,x10/5,s1,x8/11,s14,x4/0,s11,x5/8,pf/c,x4/2,pd/m,x12/6,s3,x8/2,s13,pf/l,x6/13,s11,x5/15,pi/e,x6/10,pb/l,x9/5,s14,pe/o,x10/6,pg/d,x11/8,pn/m,x1/2,pf/c,x4/9,pk/m,x5/15,s1,x12/11,pn/i,x14/6,s13,x0/5,s14,x6/8,s4,x12/0,pe/m,x10/15,pc/p,x3/8,pn/e,x11/0,s14,x7/13,s5,x10/14,pa/o,x3/13,s5,x11/10,pf/m,s11,x5/3,s9,x4/8,s9,x3/1,pp/l,x0/4,pc/j,x13/7,s5,x4/2,pa/d,x1/6,pc/l,s13,x7/4,s12,x3/10,pk/a,x14/7,s7,x11/5,s12,pe/j,x13/14,s2,x7/11,s4,x6/10,s10,x5/8,pn/h,x0/2,pb/j,x12/4,po/a,x5/7,pd/l,x0/15,s10,x1/6,s9,pj/h,x2/12,pf/o,x7/13,s11,x14/11,pi/m,x1/15,pf/j,x13/3,pl/k,x2/8,s3,x5/3,s11,pg/f,x13/4,s15,x3/1,pl/h,x12/15,pj/b,s13,x9/2,pl/o,x5/4,pi/f,s9,x2/3,s3,x0/13,s12,x4/12,ph/o,x3/11,pa/p,x7/14,s6,pd/b,s8,x6/0,pp/g,x15/10,pm/l,x6/8,pd/o,x7/12,s7,x8/10,s6,x2/5,s1,x0/10,s8,x5/2,s2,pl/k,x12/14,s14,x15/11,s13,x3/7,ph/n,x9/15,s13,x7/13,pl/m,x9/10,s9,x1/0,s4,x8/2,pk/b,x10/15,s15,x6/14,s8,x13/11,s9,x6/9,pl/m,x1/8,ph/a,x9/2,s6,x5/12,pm/i,x10/6,pd/n,x5/14,s4,x4/15,pa/p,x3/9,s6,x12/5,po/m,x13/0,pf/g,x15/5,s13,pa/c,x8/10,pd/f,x2/14,s13,x11/3,pa/c,x7/0,s15,x4/5,s5,x3/12,s13,x10/8,s14,x5/11,s12,x1/9,s12,x3/2,pe/b,x6/5,s5,x2/8,s11,x7/10,po/k,x4/8,pc/a,s4,x12/11,pj/b,x4/3,s13,x5/0,pk/e,x15/11,pp/f,x10/6,pe/c,x11/8,s3,x9/5,pn/i,s14,x6/10,pp/g,s9,x5/8,po/h,x13/10,s1,x11/2,s6,x9/7,s4,x6/11,s8,x0/15,s12,x14/4,s1,x6/8,pp/m,x9/10,s10,x3/5,s1,x1/13,s8,x0/14,pa/g,s9,x8/1,s1,x4/13,s5,x9/7,pd/f,x6/12,pp/c,x11/5,pb/k,x0/14,ph/m,x4/5,s7,x0/7,pe/p,x10/5,s4,x9/11,s7,pf/m,x13/14,ph/d,x15/7,s2,pc/b,x14/13,pk/g,x3/5,pa/o,x11/2,pg/k,x3/0,pf/a,x1/13,pm/l,x11/4,s13,x13/3,pn/k,x1/5,pc/o,x4/15,s10,pd/h,x1/14,s9,x6/7,pl/f,x13/8,s3,x4/0,pc/m,x3/1,pk/f,s2,x15/14,s9,x5/6,s1,x12/0,pl/c,x7/10,s5,x4/15,pb/m,x8/11,pg/k,x13/3,s5,x12/9,pe/n,s5,x5/2,pl/m,x4/15,pb/i,x1/13,pl/d,x15/12,po/m,x10/14,s10,x0/15,pk/p,x4/8,s12,x10/13,ph/g,x0/1,s11,x4/2,s15,x1/8,pf/b,x6/13,s1,x2/15,po/l,x12/9,s15,x13/6,s5,x14/8,s10,x0/11,pc/g,x12/4,ph/i,x9/5,pk/o,x13/15,pb/m,x11/5,s6,x0/7,pe/c,x6/3,s13,x14/4,ph/b,x9/6,pa/d,x3/4,po/i,x13/14,pl/m,x2/0,s7,x7/12,s5,x4/14,s5,ph/p,x11/7,s13,x9/1,s15,x3/15,s1,x8/6,pa/e,x1/11,s3,pb/c,x5/13,pn/o,x0/12,pb/g,x13/6,po/f,s9,x5/3,s12,x11/10,pb/j,x8/2,pd/g,x5/15,po/f,x8/12,pl/i,x1/9,pb/c,x5/11,s6,x4/2,s14,x0/3,s10,x4/1,s5,x11/13,pi/k,x3/8,s12,x12/5,s5,x13/6,ph/f,x4/7,s15,x2/6,pa/p,x1/12,s15,x7/0,pl/o,x5/4,ph/g,x10/9,s3,x1/8,s15,x10/11,pk/i,x9/14,s9,x0/6,s11,x14/4,s12,pl/a,x5/8,pi/b,x11/3,s10,x6/4,pf/o,x13/7,pb/m,x10/0,pg/k,x7/13,s1,x11/5,s4,x0/13,s5,x4/5,s5,x6/13,pn/h,x9/4,pd/e,x2/11,ph/o,x14/13,pn/d,x4/5,s7,x12/7,pa/p,x15/5,s3,x14/9,s10,x5/6,pj/o,x11/4,pi/a,s1,x2/12,pj/c,x4/14,s11,x8/13,pf/i,x2/9,s1,pn/o,x0/15,pp/i,x5/6,s5,x7/4,s13,pd/k,x15/12,s13,x0/14,ph/j,x15/12,s15,x6/11,s7,x10/1,s1,x4/3,s15,x0/9,s7,x2/15,pb/p,x14/7,s7,x4/12,pa/g,x6/7,s15,x14/11,s15,x7/13,s9,x9/2,pf/p,x6/10,s3,x0/7,ph/l,x1/8,s15,x0/6,s7,x14/15,s15,x7/9,pe/n,x5/15,s10,x10/11,s7,x7/5,s6,x0/4,pm/g,x7/1,s5,x5/9,s10,x10/15,s10,pj/o,x4/6,pg/b,x9/13,s11,x6/5,s4,pn/o,x15/10,s13,pb/f,s15,x14/3,s9,x4/5,s15,x11/8,s6,x1/4,s1,x3/12,s12,x14/0,pc/d,x11/4,s3,x6/12,s2,x8/9,s1,x12/4,s9,pb/m,x7/5,pf/c,x0/3,s11,x9/10,pm/o,x14/4,pe/c,s1,x15/10,pf/j,x13/5,s9,x1/12,pg/p,x0/15,ph/b,x11/1,pd/e,s8,x14/7,pb/p,s3,x11/9,s8,x8/6,po/a,x13/11,s1,x6/0,s7,x3/13,s15,x12/15,s7,x11/0,pm/b,x1/12,s11,x6/8,pg/l,x13/9,ph/c,x11/10,s9,x8/0,s15,x13/14,pp/f,x3/1,s14,x5/2,s9,x9/15,s9,x11/14,pg/k,x7/4,s4,x8/14,s3,x7/13,pe/n,x11/6,s15,x7/10,pa/m,x9/5,pj/e,x4/12,s13,x10/11,pa/m,s14,pf/k,x1/2,s9,x4/9,pg/j,x0/3,s2,x15/4,s6,pf/a,x9/7,s15,x14/5,pd/p,x4/7,pn/i,x1/10,s15,x7/13,s15,x15/10,ph/e,s1,x3/5,s10,x4/15,s7,x14/1,s8,x12/10,s15,x0/4,pi/n,x6/9,s2,x1/0,pd/o,x8/3,s2,x12/15,pk/h,s13,x10/6,pm/n,x15/2,pc/b,x6/9,s8,x7/13,s12,x2/4,pi/o,s2,x7/12,s13,x9/2,pd/p,x4/8,pk/l,s12,pj/o,x5/1,s12,x4/6,s11,ph/d,x14/9,s8,pb/e,x13/7,pi/f,x12/1,s7,x6/14,pe/j,x15/10,s11,x0/2,pg/d,x12/11,pp/b,x14/15,s5,x2/11,s4,x15/8,pm/e,x10/11,pa/d,x4/9,s11,x14/15,pj/b,x8/2,s3,x5/1,pc/f,x11/3,s8,pg/l,x2/9,po/h,x12/14,s4,x2/0,pe/l,x14/8,s9,x3/1,pn/i,x10/12,s6,x15/2,pm/p,x0/4,pa/c,x9/15,s10,ph/j,x14/1,pn/o,x10/13,s13,x7/11,s14,x15/10,s13,x0/6,s14,x7/2,pa/i,x13/3,s15,x8/1,s13,x11/12,s14,x8/10,s7,pk/p,x4/6,s2,pi/b,x8/10,s6,x5/6,s9,x4/7,pc/k,x13/1,s6,x5/12,s12,x1/0,s8,x3/2,s4,x9/5,s1,x1/8,s9,x0/5,po/i,x11/10,pp/f,x7/3,pe/j,x2/12,s1,x9/4,s7,x1/5,s6,x9/8,s12,x14/10,s10,x7/5,s10,x3/10,pm/k,x1/2,pn/i,x14/7,s9,x2/4,pf/g,x7/8,pb/h,x14/12,pk/m,x6/8,pc/f,x13/5,s8,x14/6,s3,x4/13,s3,x5/7,s1,x11/14,s3,x15/7,pa/p,s11,x11/1,s12,x9/14,s2,x10/12,po/l,x13/9,pp/m,x0/8,pi/d,x5/10,s9,x13/11,pe/k,x5/15,s10,x12/6,s15,x8/15,s4,x12/6,po/i,x13/2,pg/a,x10/5,s2,x0/6,pi/p,x3/9,pg/k,x14/5,s12,x10/7,s14,x3/9,s4,x15/4,ph/n,x13/0,s8,x10/7,pl/j,x14/11,pm/k,x12/5,ph/f,x15/13,s1,x0/5,pk/g,x3/8,ph/l,x1/2,s1,x15/5,pk/c,x8/14,s10,x2/15,s12,x10/9,s3,x3/7,s3,pl/a,s11,x8/11,s15,x1/6,s10,x7/13,po/m,s12,x11/3,ph/j,x4/2,pf/m,x14/1,pg/h,x6/12,s6,x5/7,pl/c,x1/13,s5,x5/3,s2,x8/13,pm/o,x0/2,pd/n,x5/4,pp/g,s9,x9/8,s4,x7/4,s11,x15/6,po/m,x1/10,pc/i,s15,x4/6,s3,x9/7,pa/m,s9,x0/13,s11,x2/1,s14,x15/0,s8,x11/6,s4,x9/10,pn/k,s6,x4/14,po/p,x10/15,s9,x4/12,pm/b,x9/7,s13,x1/0,s8,x2/8,s1,x7/5,s8,x8/10,s12,x5/1,s14,x13/2,pd/n,x4/8,pe/j,x3/15,pc/l,x12/6,pi/n,x13/2,pj/d,x8/3,s15,pa/m,x11/10,s11,x8/15,pf/g,x4/0,pd/h,x8/3,pg/j,x6/10,po/h,x3/11,s10,x4/15,s9,x11/1,s6,x8/9,pp/e,x4/11,pd/b,x5/3,s7,x8/14,s14,x5/2,s9,x1/13,po/k,x11/3,pg/a,x12/8,pi/d,x6/11,pn/b,x5/3,s11,x0/6,s9,pc/a,x4/1,pb/p,x9/8,s10,x2/3,s9,pk/a,x4/7,s9,x2/13,s8,x1/11,s7,x3/13,s13,x1/0,s13,x9/15,s5,x8/0,pm/i,s9,x6/12,pb/n,x7/4,pc/d,x6/9,pn/p,s3,x4/14,s4,x10/1,pa/o,s12,x2/14,s4,x6/12,s4,x4/2,pn/d,x15/8,s10,pk/i,x10/7,s4,pb/a,x14/9,s3,x13/11,s12,x12/0,s10,x11/7,s4,x0/4,pl/g,x2/5,s7,x8/10,s1,pn/k,s9,x9/2,s2,x4/6,s9,x8/10,pc/a,s9,x11/7,po/k,x6/4,pj/f,x3/12,s12,x2/10,pp/c,x5/14,s3,pk/e,x3/8,s9,x4/6,s10,x5/10,pg/o,x0/1,pa/l,s11,x3/7,s6,x13/14,s10,x8/12,pk/j,x0/1,pl/d,x3/12,s10,x10/11,s9,x7/15,s9,x3/4,pi/b,x15/0,pm/g,x4/1,s4,x0/15,po/p,s5,x2/6,s9,x13/12,pa/m,x10/14,pb/j,s14,x12/8,pk/n,x3/5,pj/p,x13/14,pi/n,x1/8,pg/l,s13,x5/9,po/j,x3/14,s12,x2/1,pc/l,x9/4,s5,x13/6,ph/f,x5/15,s9,pj/b,x12/10,pg/f,x1/15,pb/a,x5/12,ph/m,x13/3,s15,x11/8,s5,pb/e,x9/7,pa/j,s14,x13/15,s13,x8/10,s2,x14/4,pe/o,x9/12,s13,x8/1,s8,x15/11,s5,x8/5,s13,x14/12,s8,x7/5,s6,x15/9,ph/p,x14/8,s1,x10/13,s10,x8/2,s2,x15/7,pg/f,x0/11,s2,x8/14,s8,x1/13,pd/k,x14/6,s8,x11/10,s6,x0/4,pg/a,x6/14,s4,x5/10,s15,x7/2,pp/m,x9/3,pk/i,x12/15,s15,x6/11,s5,x10/3,s5,x13/6,s10,x0/10,s14,x9/1,pe/c,x11/3,pn/p,s3,x7/13,s8,x6/14,s14,x3/2,ph/a,x13/0,s14,x12/5,s7,x10/13,s11,pg/k,x8/6,s14,pd/c,x4/9,pp/o,x7/13,s12,x12/15,s10,x5/3,pa/j,x7/9,s15,x3/4,s10,x0/7,pn/g,x9/10,pe/b,x15/3,s6,x5/7,s8,x9/1,s7,x3/15,pp/l,x5/1,pj/h,x8/3,s14,pg/o,x0/10,pm/c,x1/3,s5,pl/p,x11/9,s15,x12/14,pk/f,x11/1,pc/a,x2/4,s5,x7/11,ph/d,x9/2,s6,x1/10,s6,x7/3,s10,x10/5,s14,x0/11,po/f,x8/10,s8,pe/c,x3/14,s12,x15/8,po/k,x2/7,s4,x9/13,s6,ph/i,x3/12,s5,x2/10,s11,x0/13,pc/j,x7/3,s11,x6/14,s11,x13/4,s5,x12/15,s8,x10/6,s6,x3/1,s14,x14/8,s13,pn/f,s1,x13/2,s9,x0/14,s3,x6/10,s7,x13/2,s11,x11/0,s12,x9/13,s11,x7/8,s15,x15/0,s2,x8/5,pi/a,x14/10,ph/b,s2,x1/9,s15,pp/g,x13/14,pj/e,x9/12,pc/l,s4,x5/7,s15,x11/2,ph/j,x7/8,pn/o,x2/0,pc/a,x8/9,pd/e,x14/0,s6,x15/2,s4,x4/3,s14,x9/8,s12,x7/11,s12,pc/n,x6/5,po/l,x7/14,s8,x8/4,s10,x13/9,pc/i,x7/12,pd/j,x4/15,s6,po/h,x13/1,s3,x11/8,s5,x15/0,pg/c,x3/11,s15,x14/0,pj/i,x2/7,pp/f,s4,x9/11,s8,x8/10,s8,x7/13,s15,x0/4,pb/l,s4,x3/9,ph/m,s14,x4/13,s14,x14/0,pp/d,x4/5,s13,x12/1,ph/e,x15/7,s14,x13/2,pa/g,x12/9,pc/f,x0/5,s2,x8/3,pp/h,x4/12,s15,x5/2,s15,x11/4,pb/o,x8/12,pd/m,x1/6,s15,x11/9,s5,x10/1,pa/o,x8/3,s7,x9/2,s4,x7/14,pk/d,x12/3,pg/n,x7/9,s10,x0/12,s15,x15/4,s5,x14/2,s6,x4/7,s1,x14/1,pd/e,x2/11,s12,x7/5,s4,x1/15,s14,x0/10,s7,x12/15,pb/m,x2/3,po/j,x7/1,pd/h,s6,x11/15,pb/f,x6/7,pm/d,x9/15,s3,pj/e,x13/7,s15,x8/14,s1,x2/5,ph/p,x12/11,pa/c,x3/5,pg/f,x10/6,pa/k,s8,x2/8,s5,pb/o,x14/15,s6,x12/13,pn/l,x1/10,s7,x3/12,s10,x5/8,pj/h,s1,pm/o,x13/6,s6,x5/14,s6,x8/7,s5,x13/6,pa/d,s11,x1/7,pl/h,x14/6,pb/k,s7,x10/2,s12,x8/5,s5,x14/15,pl/j,x9/11,pi/p,x5/14,pl/k,x10/9,s12,x5/7,pj/p,s13,pn/l,x0/11,s4,x6/10,pm/e,x2/5,pk/p,x6/9,s15,x14/2,s14,x9/4,pm/n,x6/11,s4,x4/8,po/d,x10/15,s11,x9/13,pg/m,x2/1,s4,pb/a,x15/4,s8,x5/3,pm/h,x0/12,s4,x15/4,pc/e,x0/11,pb/g,x9/6,pm/e,x14/3,pb/c,x8/2,pa/n,s4,pb/p,x7/10,pc/m,x12/6,s13,x5/8,pa/l,x10/3,pp/f,x9/15,s1,x10/2,s15,x9/12,pc/i,x4/11,pb/j,x14/13,s13,x2/1,s10,x5/8,pf/e,s10,x7/14,pl/p,x8/1,s8,x10/9,s3,x8/4,pj/d,s4,pf/h,x2/14,pa/g,s11,x13/1,s5,x9/3,s10,x15/12,pk/o,x13/6,s11,x1/9,s12,x4/14,s2,x11/10,pa/e,s1,x9/14,s14,x4/10,s4,x9/8,s8,pm/p,x12/14,s5,po/k,s14,x6/15,pl/d,x0/8,s12,x2/10,pn/b,x1/7,s8,x12/10,pf/o,x0/7,pl/i,x13/4,s2,x0/2,pd/e,x1/14,s1,pn/k,x8/10,s9,x4/9,s6,x1/5,s5,x7/12,s7,x11/8,pp/h,x9/1,pk/o,x7/0,pc/b,x2/9,s7,x6/15,pp/o,x9/11,pl/f,x3/6,s14,pn/m,x5/9,s2,x13/11,pd/p,x8/10,s1,x3/15,s13,x10/5,pe/g,s4,x11/4,pa/d,x6/14,s13,x7/9,s13,x1/8,pc/g,s2,x2/14,s8,x10/4,pn/o,x7/6,s12,x15/0,pc/l,x3/1,pd/b,x15/7,pp/j,x14/11,s1,x3/15,s7,x14/8,pi/d,x10/11,pb/f,x2/6,pg/l,s11,x0/9,s14,x1/14,s4,x11/10,pe/n,x12/13,pl/m,x9/2,s2,pi/e,x14/3,s6,x15/1,s9,x14/4,s7,x8/7,s13,x1/11,po/f,x6/10,s2,x1/5,s10,x6/3,s14,x9/8,pc/g,x4/1,pm/b,x8/0,pn/a,x2/12,pj/l,x10/5,pc/n,x14/6,pk/m,x8/11,s2,x2/1,pf/e,s14,x14/8,s5,x15/5,pa/d,x6/10,pf/e,s8,x3/2,s13,x0/5,pk/o,x1/3,pc/p,x0/15,s10,x1/2,s9,x3/15,s10,x12/10,s3,x8/11,s5,x0/14,s4,x7/3,s8,x0/9,s1,x14/1,ph/n,s4,x2/10,s15,x4/11,s4,x14/12,s3,x13/7,s4,x11/8,pe/m,x10/14,s9,x5/0,pj/a,x12/9,s7,x4/8,s7,x11/7,pg/c,x3/1,s12,x15/4,s3,x6/1,s14,x9/2,pb/n,x6/7,s10,x8/3,s2,x10/7,s5,x11/3,s7,pf/i,x7/8,s3,x9/15,s6,x13/7,s11,x14/6,s2,x5/4,pn/k,s11,x6/13,s13,x12/2,s12,x4/15,ph/m,x8/9,s7,x10/7,s13,po/j,x4/12,s4,x8/15,s14,x6/14,s11,pm/e,s2,x11/13,pn/f,x15/10,pm/p,s1,x14/13,pe/b,x10/2,s15,x1/15,pl/c,s6,x14/7,s9,x10/1,pk/o,x9/14,s13,x11/0,s7,x2/8,pn/p,x6/12,s10,x11/13,s7,x3/2,pc/g,x12/10,s1,x4/11,pb/j,s9,x12/9,s2,x13/4,s10,x6/11,s8,pg/i,x12/7,s6,x0/6,pb/e,x12/10,s9,x5/6,s8,po/j,x12/9,s2,x6/11,s2,x1/14,s12,x7/12,s13,pc/h,x9/2,s10,x5/10,s12,pa/e,x0/11,s8,x1/9,s7,x10/3,pk/h,x1/13,pa/l,x5/2,s2,x0/12,s15,x4/8,s2,x10/2,pg/i,x14/0,s3,x4/1,s10,x5/12,s9,x7/11,s10,x8/12,po/p,x13/6,pg/f,x2/14,pd/l,s1,x4/8,s1,x3/2,pc/i,x12/14,s10,x3/0,ph/d,x4/2,pf/b,x0/6,s14,x5/4,pi/e,x2/12,pb/h,x5/1,s3,x10/4,pd/k,x13/0,s12,x1/10,po/a,s12,x12/6,s4,x13/2,s7,x6/0,s8,x8/14,s8,x1/9,s6,x8/5,s13,pn/b,x9/0,pp/g,x8/13,s10,pl/j,x2/0,s1,x14/11,s10,x4/6,s6,x9/8,pp/i,x1/7,pa/f,x3/15,s10,x8/9,pd/k,x4/3,pa/j,x9/2,pc/b,x8/14,s3,x0/5,s9,pp/h,x15/10,pm/n,x13/6,pd/i,x2/11,s1,x12/3,s11,x6/10,s2,x9/7,pe/h,x1/5,s5,x7/9,pc/b,x14/3,pi/g,x6/2,po/m,x15/13,s2,x0/5,ph/l,x4/6,s9,x14/15,s10,x7/10,s7,x13/8,s13,x5/3,s10,x2/14,pb/i,x12/8,s8,x13/5,pa/n,x7/9,po/c,x5/0,s7,x15/13,s9,x10/5,s12,x9/13,pp/n,x14/2,pl/e,x10/11,s6,x13/3,s6,pb/c,x4/7,po/p,x1/9,pc/n,x5/12,s12,x8/0,s8,x11/2,pk/o,x6/8,s9,x4/9,pp/m,x0/8,pf/k,x1/6,s13,x2/14,ph/p,x11/8,s1,x3/4,s12,pe/f,s9,x13/10,pn/a,x14/7,s9,x8/0,s6,x6/15,s15,x11/4,s5,x15/8,s8,x13/11,s15,x3/12,s9,x10/8,ph/e,x9/0,s14,x13/5,s3,pn/d,x12/4,s10,x3/11,pk/j,x6/15,pf/c,x0/5,s8,x15/10,s3,x12/0,s11,x4/5,pl/e,x13/12,s12,x11/3,s12,x6/0,pa/f,x15/1,s15,x3/8,pn/c,s15,pi/l,x6/2,pd/c,x12/10,s15,x9/1,s4,x0/14,pa/o,x12/11,pn/i,x2/6,pg/l,x3/9,pa/h,s11,x11/14,s7,x10/13,s15,x9/6,pp/e,x4/13,s12,x9/14,s4,x2/0,s14,x6/13,pm/c,x11/12,s7,x0/2,s3,pp/f,x10/11,pn/e,s10,x12/13,s9,x5/9,s14,x1/0,po/c,x14/9,pl/b,x8/7,pc/a,x14/4,s15,x9/6,s13,x14/0,pf/n,x8/2,pp/a,x10/13,s3,x3/6,ph/g,x4/12,pl/o,x1/6,s2,x9/5,s6,x6/3,s3,x5/11,s9,x1/12,s14,x4/6,s7,x12/7,s6,x11/4,pi/g,s13,x10/3,s8,x5/6,pp/f,s14,pl/j,s7,x15/9,s7,po/g,x8/5,s12,x4/1,pd/p,s4,x15/11,s2,pc/g,x8/2,s8,x13/0,ph/k,x11/6,s9,x9/10,pg/a,x6/5,pf/c,x4/15,s2,x1/11,s9,x3/6,s8,x10/12,s1,x7/11,s5,x12/0,s10,x7/4,s14,x1/10,pl/a,x5/7,pn/j,x12/6,pa/i,x5/1,s5,x4/15,pc/b,x5/14,pf/m,x15/4,s4,x7/9,pd/o,x3/13,pf/a,s12,x0/10,pg/j,x7/8,s10,x6/15,s13,x8/10,ph/c,x7/9,s12,x14/10,pl/m,x12/9,pe/k,x4/6,s8,x13/11,s4,x1/8,s8,x6/5,pb/h,x14/15,s4,x1/8,s2,x5/2,pg/l,x0/12,pf/m,x14/1,pi/h,x11/5,s9,x12/6,pa/p,x4/13,s11,x8/11,s2,x6/13,s3,x5/15,s4,x9/10,s4,x11/5,s15,x15/12,s12,pj/f,x0/11,pg/i,x7/2,s1,x10/6,pb/h,x8/0,po/k,x5/10,s5,x15/9,s7,x8/6,s6,x11/7,pi/l,s12,x14/13,s4,x1/0,s2,pf/o,x9/14,s10,x5/1,s12,x6/12,s3,ph/j,x13/3,s5,x8/12,po/a,s11,x15/3,s9,x1/8,pc/i,x5/7,s11,x4/11,s1,x9/3,ph/f,x10/11,s8,x6/7,s12,x9/12,pj/m,x15/6,pf/i,x9/7,s9,x1/6,pl/a,s14,x10/3,s7,x4/15,s8,x7/2,pp/g,x6/4,pk/d,x11/2,pa/h,x13/10,s5,x6/7,pe/c,x4/3,s8,x9/6,s10,x15/8,pa/h,x5/2,po/k,x9/12,s5,x11/6,pj/d,x3/9,pa/i,x14/0,pc/d,x12/10,po/b,x8/2,pg/h,x3/7,s4,x5/4,pk/m,x10/14,s4,x5/13,s1,x1/14,s10,x15/10,pb/o,x9/13,s5,x15/7,s4,x2/13,s7,x5/15,s3,x14/6,s12,x5/12,s5,x9/14,pi/a,x13/11,s9,x3/10,s11,x9/14,s1,x10/11,s8,x2/15,s10,x8/0,s7,x5/1,s8,x8/12,s5,x15/4,s4,x3/0,s10,x1/5,s1,x11/0,pf/m,x1/6,pl/h,x9/11,s2,x0/12,s6,x9/3,s13,x4/5,pc/i,x2/14,s12,pe/j,s5,x15/8,s14,x10/2,s8,x12/11,s15,x1/6,s15,x10/7,s11,x15/13,pi/c,x14/8,pe/m,s14,x15/7,pk/j,x4/12,s3,x14/11,pd/h,x3/1,s15,x5/2,pi/p,x14/13,s1,x7/0,ph/j,x1/2,s6,x13/12,pn/p,x1/14,s6,x10/4,s13,x15/1,s12,x11/3,s10,x8/6,s9,x3/7,pg/f,x9/6,pa/e,x13/10,pc/j,x8/12,pp/n,x11/0,s14,x5/6,s3,pb/d,x2/0,s6,x11/9,pf/m,x13/7,s12,x15/12,s12,x10/3,s15,x11/2,s15,x12/13,s11,x0/14,s10,x8/15,s4,x13/2,s15,x7/12,pa/d,x1/10,pb/j,x5/9,s1,x4/11,pa/o,x2/10,pg/f,x0/5,s14,x8/14,pd/o,x10/13,pc/e,x2/6,pj/p,x5/7,s8,x15/1,s8,x12/4,pb/a,x5/10,s3,x9/7,s14,x1/12,s11,x4/9,pk/i,x14/7,s7,x6/10,s15,x14/1,pp/o,s13,x9/15,pe/n,x3/2,s9,x13/4,s10,x8/5,pk/o,x2/11,s3,x0/10,pm/n,x12/14,pe/a,x11/0,s10,x10/7,s9,x11/8,pi/d,x13/1,pe/p,x2/0,pk/l,x9/1,pf/n,x2/0,pa/p,x14/8,s7,x15/2,s7,x11/13,s8,x5/10,s7,x15/2,s2,x4/8,s13,x1/12,s1,x11/5,s9,pl/m,x2/9,s3,x5/11,pf/j,x1/13,s14,x8/0,pe/n,s10,x6/13,pb/o,x8/15,s11,x11/7,s10,x8/4,pi/g,x12/13,pj/a,s6,x0/3,pf/b,x14/6,s11,x13/7,pl/g,x4/9,s7,x8/7,pj/h,x0/15,s15,x9/2,s14,x11/12,pm/n,x1/14,s12,x5/13,s10,x9/10,pa/c,x12/6,s15,x2/13,s4,x12/15,s7,pg/b,x1/9,pc/o,x12/4,s12,x15/14,s7,x3/6,pa/l,x8/14,pp/h,x12/10,po/a,x8/0,s3,x1/10,s3,x3/8,pg/j,x2/7,s9,x11/14,s10,pn/i,x2/6,po/a,x14/10,s13,x15/4,pe/k,x2/6,s7,x1/8,pi/b,x13/15,s11,x12/10,pl/j,x9/3,ph/k,x2/14,s5,x4/3,pl/n,x15/1,pe/o,x13/9,s6,x7/6,s15,x8/10,s13,x4/12,s8,x5/15,s6,x13/11,pi/n,x1/6,s15,x0/3,s10,x14/11,s8,x8/3,s11,x2/7,pc/p,x10/14,pm/g,x3/13,s2,x15/5,pi/l,x3/0,s13,x11/6,s11,x7/14,s5,x10/8,s10,ph/n,x3/15,s3,x1/0,s12,po/f,x6/15,s6,x1/5,s5,x11/2,s12,ph/g,s1,pn/c,x15/13,s10,x11/10,s1,x15/5,ph/o,x11/1,s4,x3/8,s6,x6/4,s8,x2/15,pk/m,x5/8,pp/b,x3/11,s5,x12/5,s15,x7/2,pl/m,x5/10,s12,pg/i,x9/13,s11,x4/0,s10,x7/14,pf/p,x0/9,s11,x4/8,s8,x11/15,s15,x5/4,pj/e,x8/3,pd/f,x1/10,pe/o,x6/5,pb/d,x11/4,s6,x14/6,pa/p,x7/9,pg/c,x1/5,pd/k,s5,x3/15,s11,x2/6,s12,x8/4,s8,x12/3,s15,x8/15,s1,x12/1,s10,x2/15,s6,x8/5,s12,pi/f,x12/11,s5,x14/4,pp/l,x15/7,s2,x13/14,s5,x1/11,s9,pk/m,s6,x0/7,s4,x13/3,s6,pp/d,x9/10,pj/b,x3/1,s7,x11/13,pe/i,x15/10,s2,x0/8,s2,x15/12,s12,x4/0,s3,x1/8,s1,x12/7,pf/a,x6/15,s2,x11/13,pp/d,x9/6,ph/c,x8/15,pi/a,x5/2,pg/h,x12/8,pc/p,x13/3,s7,x9/12,pm/d,x5/14,pa/j,s13,x15/10,s3,x2/7,pc/e,x11/13,s3,x10/7,s2,x15/12,s12,x0/11,s4,x5/6,s12,x2/11,s10,x3/4,s10,x11/10,ph/a,x6/8,s9,x4/0,s10,x6/3,s14,x2/8,s2,x13/1,s14,x4/2,s14,x13/15,pd/o,x3/0,s7,x13/14,s13,x3/8,pe/i,x7/4,po/g,x1/10,pl/j,x12/6,s14,pb/h,s4,x9/13,pj/f,x7/10,s11,x6/15,s8,x1/9,pg/b,x0/6,s1,pc/o,x11/9,s4,x2/15,s13,x12/3,s2,pd/l,x6/5,s10,x4/9,s10,x11/5,s15,x0/13,s11,x1/4,s10,x9/3,s7,ph/i,x13/14,pk/o,s6,ph/f,x15/9,s12,pm/j,x10/11,pg/a,s10,x8/9,s5,x7/13,s7,x5/4,s7,x13/12,s15,x5/8,s2,x1/7,pj/k,x11/15,pe/a,x3/13,po/b,x6/7,pp/n,x12/2,s3,x7/0,pd/c,x6/13,s2,x15/2,pg/o,x6/8,ph/p,x12/4,s9,x9/6,pc/l,x5/0,pe/f,s10,x9/4,s5,x3/6,pa/l,x9/1,s14,x8/3,s5,x4/15,s14,x7/8,s7,x14/12,s8,pk/i,x10/7,pf/l,x12/15,s13,x10/14,s5,x7/9,s15,pb/d,x1/3,s1,x10/11,s9,x6/14,pj/a,x15/0,s7,x5/8,pi/e,x2/13,s15,x11/14,s8,x9/15,s2,x6/0,s8,x1/7,s9,x5/14,s5,x11/2,s10,pl/k,s7,x13/15,po/h,x14/1,s14,x4/5,pa/f,x9/10,ph/d,x0/13,pp/k,x15/1,s13,x4/7,pm/b,x8/10,s3,x6/3,pe/d,x4/13,s6,x11/3,pc/p,x9/6,pe/a,x11/0,s12,x12/8,pc/g,x7/4,pl/h,x8/11,s9,x0/3,s4,x10/9,s10,pg/k,x15/8,s3,x11/0,s11,x13/15,ph/j,x1/5,pb/d,x11/9,s14,x10/0,s10,x2/7,pi/m,x14/11,s5,x2/4,s8,x13/12,pn/g,x5/14,pb/d,x9/3,s13,x12/4,s7,x11/5,pf/a,x9/1,s9,x11/0,s14,pn/o,x6/10,s11,x13/15,pm/p,x0/11,pc/o,x6/8,s5,x0/10,s1,pa/d,x1/4,s6,x12/7,pc/g,s10,pm/p,x0/2,s7,x10/5,s2,x0/4,s3,x3/5,s7,pd/g,x2/13,pn/l,x0/10,pk/c,s1,x15/11,pp/f,x14/7,s13,x9/4,s10,x3/8,s15,x7/11,pa/i,x13/6,s11,x1/3,s10,x2/0,pn/f,x3/15,pk/a,s6,x9/12,pl/h,x7/3,s7,x0/9,s6,x15/14,s10,x8/2,s12,x10/9,s14,x2/12,s6,x13/15,s12,x2/14,po/j,s7,x12/7,s3,x15/3,pb/n,x6/2,s5,x11/8,s5,x13/4,pi/c,x14/10,s2,x0/12,s4,pf/o,x8/13,s4,x9/6,s1,x5/0,s14,x1/15,s1,x8/9,s9,x13/12,s2,x14/3,s7,x4/7,s3,x10/2,s11,x0/13,pn/d,x6/9,s10,x11/14,s3,x13/6,s2,x7/5,s1,x14/8,pj/b,x10/4,pc/i,x1/13,s12,x15/3,pb/m,x2/13,pc/h,x4/0,pl/d,x1/10,s3,x7/0,s2,x11/3,pp/a,x1/0,ph/b,x15/12,s5,x10/7,s2,x5/9,po/g,x2/15,s6,pl/j,x12/3,s5,x13/4,s13,x15/8,s7,x10/9,ph/c,x4/14,s15,x6/11,s10,x5/13,s1,x4/0,s5,x6/14,s11,x12/8,s5,x13/15,s6,x3/9,pf/j,s2,x13/15,s4,x4/11,pl/d,x1/10,s2,x6/12,s10,x4/11,s11,x2/15,s4,x4/12,s4,x14/15,po/e,x1/5,s6,x8/15,s6,x6/7,s1,x8/11,pb/f,x10/14,pi/g,s6,x8/1,s11,x9/11,pk/a,x15/0,pl/p,x8/14,pd/f,x5/9,pe/n,x14/8,s2,x12/13,s3,x5/3,pc/i,x14/7,pk/d,x8/5,s2,pa/p,x3/13,s1,x1/5,s12,x10/15,s14,x11/8,s11,x14/3,s6,x13/8,s12,x9/14,s14,x1/2,pc/n,x15/4,pe/a,x13/3,s15,x0/1,pj/m,x8/7,pa/n,x4/15,s12,pj/p,s5,x0/12,po/l,x8/13,s1,x14/15,s1,x12/9,s11,x7/6,s13,x1/10,pg/a,x3/9,ph/n,x12/4,s13,pi/k,x7/6,pf/m,s10,x13/14,s6,x1/6,s9,x5/10,s14,x13/2,pd/l,x4/8,s5,x11/6,pk/g,x15/1,ph/j,x8/5,s15,x11/9,pd/e,x14/6,pi/p,x3/4,po/h,x13/5,s7,x2/0,pk/f,x1/10,s14,x11/5,s5,x12/3,pg/m,x0/14,s2,x11/5,s10,x1/13,pj/f,x14/10,s6,x5/7,s9,pl/a,x1/14,s10,x7/9,s6,pm/n,s7,pa/l,x11/2,pp/h,x13/12,pa/l,x6/8,s12,x4/5,s9,x8/3,pg/j,x12/14,pb/a,x6/1,s13,pl/f,x7/8,s13,x0/9,s14,pb/e,x10/8,s11,pd/a,x5/2,s5,x6/8,s11,x1/3,s1,x2/9,s11,x0/10,s13,x9/6,s11,x15/2,s6,pg/f,x3/1,s3,x15/7,s14,x4/11,s6,x2/6,pc/o,s3,x10/3,s15,x4/13,s7,x3/9,s9,x14/10,pm/p,x12/0,pf/a,s3,x4/15,pg/e,x6/7,s9,x15/10,s15,x7/2,pj/b,x6/1,s12,x0/2,po/f,x1/4,s12,pk/c,x9/2,s12,x15/12,ph/a,s6,pf/j,x1/2,s4,pa/d,x0/11,ph/o,x14/6,pc/n,x3/4,s7,x2/12,s7,x6/8,pa/b,x14/12,s15,x13/0,s6,pk/o,x6/10,s11,x5/0,s8,pj/g,s12,x2/8,s5,pd/h,x9/11,s9,x7/0,pm/a,x2/11,s15,pn/b,s2,x13/7,s9,x15/5,s8,x4/8,pp/k,x3/13,s9,x4/15,s10,x11/1,s8,pb/l,x7/10,s1,x3/5,s11,x1/9,s5,pg/d,x14/12,pf/p,x2/13,s7,x6/7,s2,x15/4,pg/a,x11/9,pp/f,x8/10,pb/k,x14/12,s5,x3/6,pc/e,x1/0,s8,pb/g,x3/12,s11,x2/0,s10,x15/9,s15,x13/4,s11,x3/12,s15,x4/6,s2,x7/5,pp/d,x6/13,s13,pb/l,x5/12,s12,x8/6,s11,x1/12,pi/n,x2/8,pf/a,x14/10,s11,pn/l,x11/15,s15,x13/12,s11,x14/0,pm/h,x12/13,s10,x9/0,s5,pa/i,x7/5,s2,x11/9,pc/f,s12,x6/10,pi/g,s13,x9/4,s4,x11/2,ph/o,x8/5,s8,x15/9,s5,x11/12,s9,x2/7,pp/b,x6/4,s4,pe/m,x2/12,s11,x14/6,s11,x8/15,s1,x3/13,ph/k,x15/5,pj/m,x11/14,pn/i,x12/3,s11,x0/10,s6,x12/7,s8,x8/9,s1,x4/10,pf/o,x11/0,pg/a,s5,x10/15,pl/h,s3,x4/2,s14,x3/5,s2,x13/6,pk/b,x5/11,s2,x13/2,pe/p,x14/15,pj/c,x7/10,pg/h,x1/2,pb/p,x15/6,s4,x11/9,s2,x5/4,s4,x13/10,s11,x0/6,s6,x14/3,po/a,x10/6,pj/h,x5/3,pb/d,x10/4,s12,x13/12,pn/p,x1/9,s6,x3/13,s5,x0/11,s6,x9/14,ph/d,x8/3,s7,x0/1,s2,x3/8,s15,x4/13,pl/b,x2/5,pd/p,x0/3,pj/c,x11/6,pg/n,x15/9,po/h,x11/13,pf/p,x0/12,s4,x6/13,pn/o,x9/10,s6,x15/3,pg/f,s11,x11/10,s15,x3/15,pi/b,x11/1,s15,x4/9,s5,x15/3,pc/o,x13/10,s7,x4/8,s10,x1/14,s11,x6/10,pp/b,x8/3,s5,x1/10,s3,pn/j,x9/15,s11,pe/b,x8/5,s9,x0/6,s1,x10/9,s5,pc/n,s14,x0/12,pf/p,x1/15,pk/e,x6/2,pl/b,x12/5,s14,x6/14,s12,x1/7,pg/o,x6/8,pn/l,s6,x14/9,s1,x6/0,s15,x14/5,s11,pf/e,x9/11,pk/n,x7/13,pp/e,s6,x0/10,pj/l,x6/11,po/f,x0/8,s6,x15/11,s8,pj/b,x8/4,s14,x10/2,pi/k,x7/9,s15,x2/13,s13,pm/d,x3/11,pk/c,x9/4,pn/e,x0/8,pc/p,x15/12,pe/g,x10/11,pb/l,x14/4,s14,x13/2,s2,pe/j,x4/1,pn/o,x12/5,pl/i,x1/11,s12,x0/13,ph/m,x15/6,s5,x7/11,s6,x0/9,s13,x11/10,s11,x6/7,s13,x3/15,s9,x8/11,pg/l,x4/5,ph/f,x11/7,s12,pg/p,x5/10,s13,x13/15,s15,x5/8,s10,x6/9,s15,x4/2,s13,x9/6,pe/a,x2/0,pc/l,x14/9,pf/d,x12/5,s11,x2/9,pn/m,x3/11,s6,x1/8,s11,x5/9,s4,x11/0,s1,pc/b,s1,x8/10,pg/f,x15/3,s9,x8/11,pi/n,x5/3,pe/m,x9/10,s10,pc/k,x3/6,s9,x10/1,pf/g,x4/13,pn/a,x12/0,s10,pp/m,x3/10,s12,x6/0,pn/e,x12/9,s12,pg/b,x4/6,s9,x9/2,s6,x0/15,pi/j,s12,x1/5,po/p,x15/11,s4,x12/4,pj/n,x0/8,s12,x13/6,s10,x2/4,pc/d,x11/15,pf/h,x0/8,s15,x5/7,s13,x14/2,s1,pn/l,x12/7,s15,x11/6,pf/j,x7/0,pl/o,x9/2,pc/p,x11/13,s2,x12/15,pd/i,s2,pk/j,x5/4,pb/f,x10/1,s2,x13/11,s8,x2/3,s15,x10/15,s9,x11/1,s14,x2/8,s2,x4/15,pe/l,x6/7,s5,x8/13,s6,x2/5,s8,x7/6,po/i,x8/5,s6,pe/g,x1/15,s6,pf/d,x3/4,pb/m,x8/9,s12,x2/0,s13,x12/13,s7,x1/15,pp/d,x4/3,s15,x13/2,s9,x9/5,pl/h,x15/2,pp/b,x4/9,s10,x0/7,pg/d,x14/11,s2,x8/15,s7,x6/12,s12,x15/0,pk/a,x7/14,pf/j,s8,x15/11,s8,x8/9,pc/o,s8,x12/13,s6,x6/9,pm/a,x5/4,s7,x3/2,s7,x10/13,s5,x1/12,pf/c,x6/9,s2,x8/1,s1,x5/3,ph/l,x14/0,pg/p,x5/4,pa/e,x8/9,pm/o,x14/12,pe/k,x9/10,s1,x0/6,s7,x15/10,pf/g,x8/13,s11,x4/2,pe/m,s11,x7/12,s12,x10/11,s2,x5/13,s11,x12/3,pd/c,x13/4,s7,x0/14,s8,x8/4,pe/n,s15,x13/7,pb/c,x0/1,s9,x5/14,s12,x12/11,pf/m,x2/1,s6,x0/12,s15,x10/4,s5,x13/2,s11,x10/15,pl/e,x1/14,s12,x2/7,s7,x14/12,s3,po/m,x9/8,s2,x4/11,s7,x0/2,s15,x3/9,s3,x15/14,s11,x2/5,s9,x10/6,s11,x15/4,s11,pb/g,x1/10,pi/e,x3/15,s8,x10/8,pg/j,x1/3,pn/a,x12/13,s5,x14/2,pf/c,x3/7,s4,x12/5,s10,x2/8,pg/m,x0/9,s11,x15/7,s3,pi/b,s11,pa/e,x11/6,po/g,x7/1,s14,x10/8,s12,pk/i,x13/11,pb/p,x12/8,s11,x14/2,s10,x6/10,s6,x0/1,s4,x14/3,pi/l,x12/7,pd/e,s5,x1/13,s15,x12/5,s8,x15/0,s12,x3/13,pi/b,x7/11,po/g,x2/3,s8,x5/0,pk/f,x10/13,pp/o,x0/7,pj/l,x12/5,s12,x13/4,s13,x14/3,pc/b,x4/0,pf/l,x5/1,s13,x13/10,pk/g,x8/2,s13,x0/3,s4,x4/2,s9,x10/11,s1,pm/p,x14/2,s6,pe/h,x8/5,s6,x7/11,pb/j,x6/9,s10,x0/7,pa/l,x6/10,s5,x3/15,s12,x5/12,s10,pn/d,x11/13,s6,x9/3,pc/b,s13,x13/12,po/h,x7/8,s12,x15/1,pm/k,x0/4,pj/h,x10/1,s4,x14/6,pe/p,x9/1,pi/f,x11/0,pa/n,x9/13,s7,x15/14,pe/o,x3/0,pa/i,x14/13,s9,x15/8,pk/o,x14/5,s8,x4/11,s5,pn/a,x6/5,pg/j,x14/8,pa/d,x13/15,s10,x10/7,s13,pc/e,x13/8,pg/k,x12/4,pj/n,x7/0,s10,x5/12,s5,x7/14,s7,x11/2,pf/b,x4/15,pe/h,x0/10,pa/o,x13/3,pf/e,s8,x7/5,s12,x9/6,s6,x1/7,s8,x0/6,s15,pd/h,x1/12,pb/e,x11/15,s10,x13/7,pn/c,x6/9,pb/o,x11/14,s5,x9/4,s5,x2/3,s2,x6/14,pg/j,x5/12,pc/b,x15/7,ph/f,x9/14,s14,x2/7,pa/j,x6/8,s4,x9/14,s9,x10/5,s2,x7/9,s9,x6/12,s3,x8/1,pe/n,s15,pl/g,x15/7,s15,x13/3,s12,x14/1,ph/o,s9,x7/10,s6,pi/c,x9/1,pd/f,s4,x0/13,pc/o,x15/2,s10,x9/7,s14,x14/6,s7,x13/12,pp/k,x3/5,s11,x4/6,pg/m,x12/9,pl/b,x13/10,s1,x9/14,pk/j,x4/1,pi/a,x0/10,s6,x4/15,s10,x9/1,s6,x6/14,s6,x0/7,s7,x5/1,s13,x11/0,s4,x10/7,pm/d,x15/14,s5,x5/3,s2,x13/12,pp/e,x14/15,pg/n,x6/10,s3,x7/4,s4,x5/14,pl/j,x7/1,pm/c,x4/6,pg/e,x11/9,s3,x3/6,pa/o,x5/9,s15,x4/0,pn/e,x2/13",
"301",
"set i 31
set a 1
mul p 17
jgz p p
mul a 2
add i -1
jgz i -2
add a -1
set i 127
set p 618
mul p 8505
mod p a
mul p 129749
add p 12345
mod p a
set b p
mod b 10000
snd b
add i -1
jgz i -9
jgz a 3
rcv b
jgz b -1
set f 0
set i 126
rcv a
rcv b
set p a
mul p -1
add p b
jgz p 4
snd a
set a b
jgz 1 3
snd b
set f 1
add i -1
jgz i -11
snd a
jgz f -16
jgz a -19",
"                                                       |                                                                                                                                                 
+---+       +-+ +-------------------------------------|-------------------------------------------------------------+     +-------------------------------------------------------------------+         
|   |       | | |                                     |                                                             |     |                                                                   |         
| +-|-------|-------------------------------------------------------------------------------------------------------|---------------+             +-------------------------+     +-+       +-+         
| | |       | | |                                     |                                                             |     |         |             |                         |     | |       |           
| | | +-----|-|-+ +---+ +-------------------------------------+       +-----------------------+       +-------------------|---------+           +-|---+         +-+     +---------+ |       |   +---+   
| | | |     | |   |   | |                             |       |       |                       |       |             |     |                     | |   |         | |     |   |       |       |   |   |   
| | | |     | |   |   | |             +---------------|---------------+ +---------------------|---------------------|---------+     +-----------|-|-+ |         | |   +-+   |     +-|-------|-------+   
| | | |     | |   |   | |             |               |       |         |                     |       |             |     |   |     |           | | | |         | |   |     |     | |       |   |       
| | | |     | | +-------|-------------|---------------|-------|-+ +-----+ +-------------------|---------------------------|---------|---------------|-|---------|-----------------|-|-------+   |       
| | | |     | | | |   | |             |               |       | | |       |                   |       |             |     |   |     |           | | | |         | |   |     |     | |           |       
| | +-|-------|---|-+ | | +-----------|-----------------------------------|---------------------------+             |     |   |     |           | | | |         | |   |     |     | |           |       
| |   |     | | | | | | | |           |               |       | | |       |                   |                     |     |   |     |           | | | |         | |   |     |     | |           |       
| | +-|-------|-------|-|-|---------+ |               |   +---|-|---------------------------------------------------|---------|-----------------|-----|---------|-|---------|---------------------+     
| | | |     | | | | | | | |         | |               |   |   | | |       |                   |                     |     |   |     |           | | | |         | |   |     |     | |           | |     
| | | |   +-|-----|-|---------------------------------|---------|---------------+ +---+       |         +-----------|-----|---------|---+       | | | |         | |   |     |     | |           | |     
| | | |   | | | | | | | | |         | |               |   |   | | |       |     | |   |       |         |           |     |   |     |   |       | | | |         | |   |     |     | |           | |     
| | | |   | | | | | | +-|-|-------------------------------|---+ | |     +-------|-----|-----------------------------|-----|---|-----|---+       | | | |         | |   |     |     | |           | |     
| | | |   | | | | | |   | |         | |               |   |     | |     | |     | |   |       |         |           |     |   |     |           | | | |         | |   |     |     | |           | |     
| | | |   | | | | | |   | |         | |               |   |     | +-------|-----------------------------|---------------------------|---------------------------|-----------|---------------------|-+   
| | | |   | | | | | |   | |         | |               |   |     |       | |     | |   |       |         |           |     |   |     |           | | | |         | |   |     |     | |           | | |   
| | | |   | | | | | |   | |         | +---------------|---|-+   |     +---|---+ | |   +-------|-----+   +-----------|---------------+           | | +-----------|-|---|---------------------------|-+   
| | | |   | | | | | |   | |         |                 |   | |   |     | | |   | | |           |     |               |     |   |                 | |   |         | |   |     |     | |           | |     
| | | |   | | | | | |   | |         |                 |   +-|-------------------|-|---------------------+           |     |   |                 +-----|---+     | |   |     |     +-----------+ | |     
| | | |   | | | | | |   | |         |                 |     |   |     | | |   | | |           |     |   |           |     |   |                   |   |   |     | |   |     |       |         | | |     
| | | |   | | | | +-|-------------+ |         +---------+   |   +-------|-----|-|-|-----------|-----|---|---------------------|---------------------+ |   +-----|-|---------------------------|-|-|---+ 
| | | |   | | | |   |   | |       | |         |       | |   |         | | |   | | |           |     |   |           |     |   |                   | | |         | |   |     |       |         | | |   | 
| | | |   | | | |   |   | |       | |         +-----+ | +---------------|-------|---+         |     |   |           |     |   |                   | | |         | |   +-+   |       |         | | +---+ 
| | | |   | | | |   |   | |       | |               | |     |         | | |   | | | |         |     |   |           |     |   |                   | | |         | |     |   |       |         | |       
| | | +-----|-|-|-----------------|-|---------------|-|-----|---------|-|-|-----+ | |         |     |   | +---------------|---|-------------------|-|-----------+ |     |   |       |         | |       
| | |     | | | |   |   | |       | |               | |     |         | | |   |   | |         |     |   | |         |     |   |                   | | |           |     |   |       |         | |       
| | |     | | | | +-------|-------|-------------------------------------|-|---|---|---------+ |     |   | |         |     |   |                   +-------------------+ |   |       +---+ +---|-|-+     
| | |     | | | | | |   | |       | |               | |     |         | | |   |   | |       | |     |   | |         |     |   |                     | |           |   | |   |           | |   | | |     
| | |     | | | | | |   | |       | |         +-------|-----|---------|---|---|-----|-------+ |     +---------------------|---+                     | |           |   | |   |           | |   | | |     
| | |     | | | | | |   | |       | |         |     | |     |         | | |   |   | |         |         | |         |     |                         | |           |   | |   |           | |   | | |     
| | |     | | | | | |   | |       | |         |     | |     |         | | |   |   | |         |         | |         |     |                         | |       +---+   | |   |   +---------------+ |     
| | |     | | | | | |   | |       | |         |     | |     |         | | |   |   | |         |         | |         |     |                         | |       |       | |   |   |       | |   |   |     
| | |     | | | | | |   | |       | |         |   +-|-|-------------------|---------|---------|-+       | |         |     |                         | |       |   +-----|-------+       | |   |   | +-+ 
| | |     | | | | | |   | |       | |         |   | | |     |         | | |   |   | |         | |       | |         |     |                         | |       |   |   | |   |           | |   |   | | | 
+-|-------|-------|-|---------------|---------|---|-------------------------------|-----------|---------------------------|-----------------------+ | |       |   |   | |   |           | |   |   | | | 
  | |     | | | | | |   | |       | |         |   | | |     |         | | |   |   | |         | |       | |         |     |                       | | |       |   |   | |   |           | |   |   | | | 
  | |     | | | | | |   | |       | |         | +-------+   |   +-----|-------|-----|---------|---------|-|---------------------------------------|-|---------+   |   | |   |           | |   |   | | | 
  | |     | | | | | |   | |       | |         | | | | | |   |   |     | | |   |   | |         | |       | |         |     |                       | | |           |   | |   |           | |   |   | | | 
  | |     | | | | | |   | |       +-|---------|---|-|---|---|---+   +---|-----|---|---------------------------------|-------------------------------+ |           |   | |   |           | |   |   | | | 
  | |     | | | | | |   | |         |         | | | | | |   |       | | | |   |   | |         | |       | |         |     |                       |   |           |   | |   |           | |   |   | | | 
  | |     | | | | | |   | +-+       |   +-----------|---|---|-----------------------------------|---------|---------|-----------------------------|---------------|-----|---------+ +---|-------+ | | | 
  | |     | | | | | |   |   |       |   |     | | | | | |   |       | | | |   |   | |         | |       | |         |     |                       |   |           |   | |   |     | |   | |   | | | | | 
  | |     | | | | | |   |   |       |   | +---|---------|---|-------|-|-|-|-------|-|---+     | |       | |         |     |                       |   |           |   | |   |     | |   | |   | | +-+ | 
  | |     | | | | | |   |   |       |   | |   | | | | | |   |       | | | |   |   | |   |     | |       | |         |     |                       |   |           |   | |   |     | |   | |   | |     | 
  | |     | | | | | |   |   |       |   | |   | | | | | |   |       | | | |   |   | |   |     | |       | |         |     |                       |   |           |   | |   |     | |   | |   | |     | 
  | |     | | | | | |   |   |       |   | |   | | | | | |   |       | | | |   |   | |   |     | |       | |         |     |                       |   |           |   | |   |     | |   | |   | |     | 
  | |     | | | | | |   |   |       |   | |   +-|-|-|-|-|----G------|---|-------+ +-------------|-------|---------------------------------------------------------|-----|-------+ | |   | |   | |     | 
  | |     | | | | | |   |   |       |   | |     | | | | |   |       | | | |   | |   |   |     | |       | |         |     |                       |   |           |   | |   |   | | |   | |   | |     | 
  | |     | | | | | |   |   |       |   | |     | +-|-----------------|-|-|---+ | +-----------|---------|-|---------|-----|---------------------------|-----------------|-----------------|-----|-+   | 
  | |     | | | | | |   |   |       |   | |     |   | | |   |       | | | |     | | |   |     | |       | |         |     |                       |   |           |   | |   |   | | |   | |   | | |   | 
  | |     | | | | | |   |   |       |   | |     |   | | +-----------|---+ |     | | |   |     | |       | |         |     |                       |   |           |   | |   |   | | |   | |   | | |   | 
  | |     | | | | | |   |   |       |   | |     |   | |     |       | |   |     | | |   |     | |       | |         |     |                       |   |           |   | |   |   | | |   | |   | | |   | 
  | |     | | | | +-|---+   |       |   | |     |   | | +---|-+ +---------|---------|-+ |     | |       +-----------|-------+                     |   |       +---|---|-------+ +-------|-----|---|-+ | 
  | |     | | | |   |       |       |   | |     |   | | |   | | |   | |   |     | | | | |     | |         |         |     | |                     |   |       |   |   | |   | |   | |   | |   | | | | | 
  | |     | | | |   |       |       |   | |     | +---|-----|-|-------|-----------------------+ |         |         |     | |                     |   |       |   |   | |   | |   | |   | |   | | | | | 
  | |     | | | |   |       |       |   | |     | | | | |   | | |   | |   |     | | | | |       |         |         |     | |                     |   |       |   |   | |   | |   | |   | |   | | | | | 
  | | +---|-|-|-----|-------|-------|---|-----------|---|---|---|---|-------------|-|---|-----------------|---------|-------------------------------------------------|-------|---|-+   | |   | | | | | 
  | | |   | | | |   |       |       |   | |     | | | | |   | | |   | |   |     | | | | |       |         |         |     | |                     |   |       |   |   | |   | |   |     | |   | | | | | 
  | | |   | | | |   |       |       |   | +-------|-|-+ |   | | |   | |   |     | | | | |       +-------------------|-----|--O------------------------|-------|---|-----|---|-----|-----+ |   | | | | | 
  | | |   | | | |   |       |       |   |       | | |   |   | | |   | |   |     | | | | |                 |         |     | |                     |   |       |   |   | |   | |   |       |   | | | | | 
  | +-+   | | | |   |       |       |   |       | | |   |   | | |   | |   |     | | | | |               +-|---------|-----|-------+               |   |       |   |   | |   | | +-------+ | Y | | | | | 
  |       | | | |   |       |       |   |       | | |   |   | | |   | |   |     | | | | |               | |         |     | |     |               |   |       |   |   | |   | | | |     | | | | | | | | 
  |       | | | |   |       |       |   |       | | |   |   | | |   | |   |     | | | | |               | |         |     | |     |               |   |       |   |   | |   | | | |     | | | | | | | | 
  |       | | | |   |       |       |   |       | | |   |   | | |   | |   |     | | | | |               | |         |     | |     |               |   |       |   |   | |   | | | |     | | | | | | | | 
  |   +---|-|-|-|-----------|-------|-------------|---------|---|-----|---|-----|-|---|-----------------|-|-----------------|---------------------|-----------|-------|-|---|--N--|-+   | | | | | | | | 
  |   |   | | | |   |       |       |   |       | | |   |   | | |   | |   |     | | | | |               | |         |     | |     |               |   |       |   |   | |   | | | | |   | | | | | | | | 
  |   |   | | | |   |       |       |   |       | | |   |   | | |   | |   |     | | | | |               | |         |     | |     |               |   |       |   |   | |   | | | | |   | | | +-+ | | | 
  |   |   | | | |   |       |       |   |       | | |   |   | | |   | |   |     | | | | |               | |         |     | |     |               |   |       |   |   | |   | | | | |   | | |     | | | 
  | +-----|-|-------|---------------------------|-------|-----|-|---|-|---------------|-|-----+         | |         |     | |     |               |   |       |   |   | |   +-|-|-|---+ | | |     | | | 
  | | |   | | | |   |       |       |   |       | | |   |   | | |   | |   |     | | | | |     |         | |         |     | |     |               |   |       |   |   | |     | | | | | | | |     | | | 
  | | | +-|-|-----------------------------------|-|-+   |   | | |   | |   |     | | | | |     |         | |         |     | |     |               |   |       |   |   | |     | | | | | | | |     | | | 
  | | | | | | | |   |       W       |   |       | |     |   | | |   | |   |     | | | | |     |         | |         |     | |     |               |   |       |   |   | |     | | | | | | | |     | | | 
  | | | | | | | |   |       |       |   |       | |     |   | | |   | |   |     | | | | |     |         | |         |     | |     |               |   |       |   |   | |     | | | | | | | |     | | | 
  | | | | | | | |   |       |       |   |       | |     |   | | |   | |   |     | | | | |     |         | |         |     | |     |               |   |       |   |   | |     | | | | | | | |     | | | 
  | | | | | | | |   |       |       |   |       | |     |   | | |   | |   |     | | +-------------------|-|-------+ |     | |     |               |   | +-----|---------|-------|-|-----|---|-----|-+ | 
  | | | | | | | |   |       |       |   |       | |     |   | | |   | |   |     | |   | |     |         | |       | |     | |     |               |   | |     |   |   | |     | | | | | | | |     |   | 
  | | | | | | | |   |       |       |   |       | |     |   | | |   | |   |     | |   | |     |         | |       | |     | |     |               |   | |     |   |   | |     | | | | | | | |     |   | 
  | | | | | | | |   |       |       |   |       | |     |   | | |   | |   |     | |   | |     |         | |       | |     | |     |               |   | |     |   |   | |     | | | | | | | |     |   | 
  | | | | | | | |   |   +---|------------B--------+ +-----------+   | |   |     | |   | |     |         +-|-------+ |     | |     |               |   | |     |   +---|-|---+ | | | | | | | |     |   | 
  | | | | | | | |   |   |   |       |   |       |   |   |   | |     | |   |     | |   | |     |           |         |     | |     |               |   | |     |       | |   | | | | | | | | |     |   | 
  | | +-|-|-|---|-------|---|-----------------------|-------|-------|-----|-----|-------|-----|---------------------|-------|---+ |               |   | |     |       | |   | | | | | | | | |     |   | 
  | |   | | | | |   |   |   |       |   |       |   |   |   | |     | |   |     | |   | |     |           |         |     | |   | |               |   | |     |       | |   | | | | | | | | |     |   | 
  | |   | | | | |   |   |   |       |   |       |   |   |   | |     | |   |     | |   | |     |           |         |     | |   | |       +-------------|-----|-------|---------|-----------+     |   | 
  | |   | | | | |   |   |   |       |   |       |   |   |   | |     | |   |     | |   | |     |           |         |     | |   | |       |       |   | |     |       | |   | | | | | | | |       |   | 
  | |   | | | | |   |   |   |       |   |       |   |   |   | |     | |   |     | |   | |     |           |         |     | |   +-|-------|-------|---|-|-----|-----+ +-+   | | | | | | | |       |   | 
  | |   | | | | |   |   |   |       |   |       |   |   |   | |     | |   |     | |   | |     |           |         |     | |     |       |       |   | |     |     |       | | | | | | | |       |   | 
  | |   | +---|-|---|-------|-----+ |   |       |   |   |   | |     | |   |     | |   | |     |           |     +-------+ +-----+ |       |       |   | |     |     | +-----|---+ | | | +-----------+ | 
  | |   |   | | |   |   |   |     | |   |       |   |   |   | |     | |   |     | |   | |     |           |     |   |   |   |   | |       |       |   | |     |     | |     | |   | | |   |       | | | 
  | | +-|-+ | | |   |   |   +-----|-|---|---------------------|-------|---------------|-|-----|---------------------------------+ |       |       |   | |     |     | |   +---------|---+ |       | | | 
  | | | | | | | |   |   |         | |   |       |   |   |   | |     | |   |     | |   | |     |           |     |   |   |   |     |       |       |   | |     |     | |   | | |   | | | | |       | | | 
  | | | | | | | |   |   |         | |   |   +-------|---------------|-|---|-------|-----------------------|-----|-------|---------|-------|-----------|-|-----------------|-----------|-+ |       | | | 
  | | | | | | | |   |   |         | |   |   |   |   |   |   | |     | |   |     | |   | |     |           |     |   |   |   |     |       |       |   | |     |     | |   | | |   | | |   |       | | | 
  | | | | | | | |   |   |         +-|-------|---|-------------------------------|-----|-------------------------|-------|---|-----|---------------------------|-----|-------------|---------------|-+ | 
  | | | | | | | |   |   |           |   |   |   |   |   |   | |     | |   |     | |   | |     |           |     |   |   |   |     |       |       |   | |     |     | |   | | |   | | |   |       |   | 
  | | | | | | | |   |   |           |   |   |   |   |   |   | |     | |   |     | |   | |     |           |     |   |   |   |     |       |       |   | |     |     | |   | | |   | | +-------+   |   | 
  | | | | | | | |   |   |           |   |   |   |   |   |   | |     | |   |     | |   | |     |           |     |   |   |   |     |       |       |   | |     |     | |   | | |   | |     |   |   |   | 
  | | | | | | | |   |   |           |   |   |   |   |   |   | |     | |   |     | |   | |     |           |     |   |   |   |     |       |       |   | |     |     | |   | | |   | |     |   |   |   | 
  | | | | | | | |   |   |           |   |   |   |   |   |   | |     | |   |     | |   | |     |           |     |   |   |   |     |       |       |   | |     |     | |   | | |   | |     |   |   |   | 
  | | | | | | | |   |   |           |   |   |   |   |   |   | |     | |   |     | |   | |     |           |     |   |   |   |     |       |       |   | |     |     | |   | | |   | |     |   +-----+ | 
  | | | | | | | |   |   |           |   |   |   |   |   |   | |     | |   |     | |   | |     |           |     |   |   |   |     |       |       |   | |     |     | |   | | |   | |     |       | | | 
  | | | | | | | | +-|-+ | +---------|---|-------|---|-------|-------|-----|-----|-----|-|-----|-----------------|-------------------------|-------+   | |     |     | |   | | |   | |     |       | | | 
  | | | | | | | | | | | | |         |   |   |   |   |   |   | |     | |   |     | |   | |     |           |     |   |   |   |     |       |           | |     |     | |   | | |   | |     |       | | | 
  | | | | | | | | | | | | |         |   |   |   +-------|---|-|-------+   |     | |   | |     |         +-|-----|---|-------------|-------------------|-|-----|-----|-----+ | |   | |     | +-------+ | 
  | | | | | | | | | | | | |         |   |   |       |   |   | |     |     |     | |   | |     |         | |     |   |   |   |     |       |           | |     |     | |     | |   | |     | |     |   | 
  | | | | | | | | | | | | |         |   |   |       |   |   | |     |     |     +-|-----------|---------+ |     |   |   |   |     |       |     +-----|-----+ |     | |     | |   | |     | |     |   | 
  | | | | | | | | | | | | |         |   |   |       |   |   | |     |     |       |   | |     |           |     |   |   |   |     |       |     |     | |   | |     | |     | |   | |     | |     |   | 
  | | | | | | | | | | | | |         |   |   |     +-|---|---|-------|-----------------|-|-----|-----------|-----|---|---|-----------------+     | +---|-----|-|---+ | |     | |   | |     | |     |   | 
  | | | | | | | | | | | | |         |   |   |     | |   |   | |     |     |       |   | |     |           |     |   |   |   |     |             | |   | |   | |   | | |     | |   | |     | |     |   | 
  | | | | | | | | | | | | |         |   |   +-----|-----------|-----------|-------|---|-|-----|---------------------|-------------|---------------------------|---------------|---+ |     | |     |   | 
  | | | | | | | | | | | | |         |   |         | |   |   | |     |     |       |   | |     |           |     |   |   |   |     |             | |   | |   | |   | | |     | |     |     | |     |   | 
  | | | | | | | | | | | | |       +-|-+ |         | |   |   | |     |     |       |   | |     |   +-------------|-----------|---------------------------|---|-|-+ +-|-|-----|-|---+ |   +-|-+     |   | 
  | | | | | | | | | | | | |       | | | |         | |   |   | |     |     |       |   | |     |   |       |     |   |   |   |     |             | |   | |   | | |   | |     | |   | |   | |       |   | 
  +-|-------+ | | | | | | |       | | | |         | | +-+   | |     |     |       |   | |     |   |       |     |   |   |   |     |             | |   | |   | | |   | |     | |   | |   | |       |   | 
    | | | |   | | | | | | |       | | | |         | | |     | |     |     |       |   | |     |   |       |     |   |   |   |     |             | |   | |   | | |   | |     | |   | |   | |       |   | 
+---|-|-|---------|-|-|-|-|-------------|-------------|-------|-------------------|---|-|-------+ |       |     |   |   |   |     |             | |   | |   | | |   | |     | |   | |   | |       |   | 
|   | | | |   | | | | | | |       | | | |         | | |     | |     |     |       |   | |     | | |       |     |   |   |   |     |             | |   | |   | | |   | |     | |   | |   | |       |   | 
+-+ | | | |   | | | | | | |       | | | |         | +---+ +-|-|-----|-----------------|-----------------------------|-------|-----|-------------------|---------|-+ | |     | |   | |   | |       |   | 
  | | | | |   | | | | | | |       | | | |         |   | | | | |     |     |       |   | |     | | |       |     |   |   |   |     |             | |   | |   | | | | | |     | |   | |   | |       |   | 
+-|---|---|-----|-|---|-------------|-----------------|-|-+ | |     |     |       |   | |     | | |       |     |   |   |   |     |             | +-----+ +-----|-+ +-|-------------|---|-+       |   | 
| | | | | |   | | | | | | |       | | | |         |   | |   | |     |     |       |   | |     | | |       |     |   |   |   |     |             |     |   | | | |     |     | |   | |   |         |   | 
| | | | | |   | | | | | | |   +-----|-----+       |   | |   +-------|-------------------|-----------------------|---|---|---|-----|-------------|-----+   +-+ | |     |     | |   | |   |         |   | 
| | | | | |   | | | | | | |   |   | | | | |       |   | |     |     |     |       |   | |     | | |       |     |   |   |   |     |             |             | |     |     | |   | |   |         |   | 
| | | | | | +-|-|-|-|-|-|-|M--+   | | | | +-------+   | |   +-------|-----|-------|-----------|---------------------|---|---|-----------------------------------------|-----|-----------|-----+   |   | 
| | | | | | | | | | | | | |       | | | |             | |   | |     |     |       |   | |     | | |       |     |   |   |   |     |             |             | |     |     | |   | |   |     |   |   | 
| | | | | | | | | | | | | |     +-|-|-|-|-------------|-----|-------------|-------|---|-|---------|-------|-----|---------------------------------+           | |     |     | |   | |   |     |   |   | 
| | | | | | | | | | | | | |     | | | | |             | |   | |     |     |       |   | |     | | |       |     |   |   |   |     |             | |           | |     |     | |   | |   |     |   |   | 
| +-----|-|-----|-|-|---|-|-----|-------|-+   +---------+   +-|-----------|-------------------|-+ |       |     |   |   |   |     |             | |           | |     |     | +---|-----------|---+   | 
|   | | | | | | | | | | | |     | | | | | |   |       |       |     |     |       |   | |     |   |       |     |   |   |   |     |             | |           | |     |     |     | |   |     |       | 
|   | | | | | | | | | | | |     | | | | +-|---|---------------------------|-----------|-----------|-------|-----|---|---|-------------------------------------|-------|---------------+ |     |       | 
|   | | | | | | | | | | | |     | | | |   |   |       |       |     |     |       |   | |     |   |       |     |   |   |   |     |             | |           | |     |     |     | | | |     |       | 
|   | | | | | | | | | | | |     | | | |   |   |       | +-----|-----|-----|-----------|-|-----------------|-----+   |   +---|-----|-------------|-|-------------|---------------------|---+ +---+     | 
|   | | | | | | | | | | | |     | | | |   |   |       | |     |     |     |       |   | |     |   |       |         |       |     |             | |           | |     |     |     | | | | | | | |     | 
|   | +-|---+ | | | | | | |     | | +-|-------------+ +-|-----|-----+     |       |   | |     |   +-------------------------|---------------------|-----------|-|-----|-----------|---|-|---|-|-|-+   | 
|   |   | |   | | | | | | |     | |   |   |   |     |   |     |           |       |   | |     |           |         |       |     |             | |           | |     |     |     | | | | | | | | |   | 
|   |   | |   | | | | | | |     | |   |   |   |     |   |     |           |       |   | |     |           |         |       |     |             | |           | |     |     |     | | | | | +-|---|-+ | 
|   |   | |   | | | | | | |     | |   |   |   |     |   |     |           |       |   | |     |           |         |       |     |             | |           | |     |     |     | | | | |   | | | | | 
|   +-+ | |   | | | | | | | +---|-|-----------------|---------------------|-----------|-|---+ |           |         |       |     |             | |           | |     |     |     | | | | |   | | | | | 
|     | E |   | | | | | | | |   | |   |   |   |     |   |     |           |       |   | |   | |           |         |       |     |             | |           | |     |     |     | | | | |   | | | | | 
|     | | +-------|-|-|---|-----|---+ |   |   |     |   |     |           |       |   | |   | |           +-----------------|-----------------------+         | |     |     |     | | | | |   | | | | | 
|     | |     | | | | | | | |   | | | |   |   |     |   |     |           |       |   | |   | |                     |       |     |             | | |         | |     |     |     | | | | |   | | | | | 
|     | | +---------------+ |   | | | |   |   |     +-------+ |           |       |   | |   | | +-------------------|-------------+             | | |         | +---+ |     |     | | | | |   | | | | | 
|     | | |   | | C | | |   |   | | | |   |   |         |   | |           |       |   | |   | | |                   |       |                   | | |         |     | |     |     | | | | |   | | | | | 
|     | | |   | | | | | |   |   | | | |   |   | +-----------|-------------------------|-----+ | |                   |       |                   | | |         |     | |     |     | | | | |   | | | | | 
|     | | |   | | | | | |   |   | | | |   |   | |       |   | |           |       |   | |     | |                   |       |                   | | |         |     | |     |     | | | | |   | | | | | 
|     | | |   | | | | | |   |   | | | |   |   | |       |   | |           |       |   +-------|-----------------------------|-------------------|---|-----+   |     | |     |     +-|-|-|---+ | | | | | 
|     | | |   | D | | | |   |   | | | |   |   | |       |   | |           |       |     |     | |                   |       |                   | | |     |   |     | |     |       | | | | | | | | | | 
|     | | |   | | | | | |   |   | +-+ |   | +-+ |       |   | |           |       |     |     | |                   |       |                   | | |     |   |     | |     |       | | | | | | | | | | 
|     | | |   | | | | | |   |   |     |   | |   |       |   | |           |       |     |     | |                   |       |                   | | |     |   |     | |     |       | | | | | | | | | | 
|     | | |   | | | | | |   |   |     |   | | +---+ +-------|-------------|-------------|---------------------------|-------------------------------+     +---|-------|-----|-------|-------+ | | | | | 
|     | | |   | | | | | |   |   |     |   | | | | | |   |   | |           |       |     |     | |                   |       |                   | |           |     | |     |       | | | |   | | | | | 
|     | +-----|-----+ | |   |   |     |   | | | | | | +-|---|---------------------------------|-----------------------------+                   | |           |     | |     |       | +-----+ | | | | | 
|     |   |   | | |   | |   |   |     |   | | | | | | | |   | |           |       |     |     | |                   |                           | |           |     | |     |       |   | | | | | | | | 
|     |   | +---------|-|---|---|---+ |   | | | | | | | |   | |           |       |     |     +-------------------------------------------------|-|---+       |     | |     |       |   | | | | | | | | 
|     |   | | | | |   | |   |   |   | |   | | | | | | | |   | |           |       |     |       |                   |                           | |   |       |     | |     |       |   | | | | | | | | 
|     |   | | | | |   | |   |   |   | |   | | | | | | | |   | |           |       |     |       |                   |                           | |   |       |     | |     |       |   | | | | | | | | 
|     |   | | | | |   | |   |   |   | |   | | | | | | | |   | |           |       |     |       |                   |                           | |   |       |     | |     |       |   | | | | | | | | 
|     |   | +-----|-------------|---|-|-----------|-|-------|-|-----------|-------|-----|---------------------------|---------------------------|-|---|-------|-------------|-----+ | +-+ | | | | | | | 
|     |   |   | | |   | |   |   |   | |   | | | | | | | |   | |           |       |     |       |                   |                           | |   |       |     | |     |     | | |   | | | | | | | 
|     |   +-------|-----|---|---|-----|-----|---------|-------|---------------+   +-----|-------|-+                 |                           | |   |       |     +-------------|-|---+ | | | | | | | 
|     |       | | |   | |   |   |   | |   | | | | | | | |   | |           |   |         |       | |                 |                           | |   |       |       |     |     | | | | | | | | | | | 
|     |       | | |   +---------|---|-----|---|-|-|-|-------|-|---------------|---------|-------|-|-----------------|-------------------------+ | |   |       |       |     |     +-|-+ | +-|---+ | | | 
|     |       | | |     |   |   |   | |   | | | | | | | |   | |           |   |         |       | |                 |                         | | |   |       |       |     |       |   |   | |   | | | 
|     |       +-|-|---+ |   |   |   | |   | | +-|-|-|---------|-----------|-------------|-----------------------------------------------------+ | |   |       |       |     |       |   |   | | +---+ | 
|     |         | |   | |   |   |   | |   | |   | | | | |   | |           |   |         |       | |                 |                           | |   |       |       |     |       |   |   | | | |   | 
|     |     +---|-|-----|-------------|-----|---|-----+ |   | |           |   |         |       | |                 |                           | |   |       |       |     |       |   |   | | | |   | 
|     |     |   | |   | |   |   |   | |   | |   | | |   |   | |           |   |         |       | |                 |                           | |   |       |       |     |       |   |   | | | |   | 
+-----|-----------------|-------|---+ +-------+ +-|-----|-----------------|---------------------|-|---------------------------------------------|-|---|-------|-------|-----|---+   |   |   | | | |   | 
      |     |   | |   | |   |   |         | | |   | |   |   | |           |   |         |       | |                 |                           | |   |       |       |     |   |   |   |   | | | |   | 
      |     |   | |   | |   |   +-----------|-------|-------|-|-------------------------|-------|-|-----------------|---------------------------|-----|-------|-------|---------|---|---------|-|-+   | 
      |     |   | |   | |   |             | | |   | |   |   | |           |   |         |       | |                 |                           | |   |       |       |     |   |   |   |   | | |     | 
      |     |   | |   | |   |             | | |   | |   |   | |           |   |         |       | |                 |                           | |   |       |       |     |   |   |   |   | +-----+ | 
      |     |   | |   | |   |             | | |   | |   |   | |           |   |         |       | |                 |                           | |   |       |       |     |   |   |   |   |   |   | | 
      |     |   | |   | |   |             | | |   | |   |   | |           |   |         |       | |                 |                         +-+ |   |       |       |     |   |   |   |   |   |   | | 
      |     |   | |   | |   |             | | |   | |   |   | |           |   |         |       | |                 |                         |   |   |       |       |     |   |   |   |   |   |   | | 
      |     |   | |   | |   |   +---------|-|-----------|---+ +-----------|---|---------+       +-----------------------------------------------------|---------------------------------|---|-------+ | 
      |     |   | |   | |   |   |         | | |   | |   |                 |   |                   |                 |                         |   |   |       |       |     |   |   |   |   |   |     | 
      |     |   | |   | |   |   +-------------|-+ +-----+ +-------------------+                   |           +-----|-+                       |   |   |       |       |     |   |   |   |   |   |     | 
      |     |   | |   | |   |             | | | |   |     |               |                       |           |     | |                       |   |   |       |       |     |   |   |   |   |   |     | 
      |     |   | |   +-----------------------------|---------------------------------------------|-----------|-----------------------------------|-----------+       |     |   |   |   |   |   |     | 
      |     |   | |     |   |             | | | |   |     |               |                       |           |     | |                       |   |   |               |     |   |   |   |   |   |     | 
      |     |   | |     |   |     +-------|-|-|-----------+               +-----------------------------------|-----+ |                       |   |   +-+             |     |   |   |   |   |   |     | 
      |     |   | |     |   |     |       | | | |   |                                             |           |       |                       |   |     |             |     |   |   |   |   |   |     | 
      +-----|---+ |     |   +-------------|-------------------------------------------------------+           |       |       +---------------|---|-----|-------------|-----|---------------|---------+ 
            |     |     |         |       | | | |   |                                                         |       |       |               |   |     |             |     |   |   |   |   |   |       
            +---------------------|-------|---|-----|-----------------------------------------------------------------|-------+               |   |     +-------------+     |   | +-|---+   |   |       
                  |     |         |       | | | |   |                                                         |       |                       |   |                         |   | | |       |   |       
      +---+       |   +-|---------+       +-|---|---+                                                         |       +---------------------------+                         |   | | |       |   |       
      |   |       |   | |                   | | |                                                             |                               |                             |   | | |       |   |       
      |   |       +---|-|-------------------|---------------+                                                 |                       +-------|-----------------------------|---|---|-------|---+       
      |   |           | |                   | | |           |                                                 |                       |       |                             |   | | |       |           
      +---------------+ |                   +-+ +-----------+                                                 |                       +-------+                             |   | | |       |           
          |             |                                                                                     |                                                             |   | | |       |           
          +-----------+ +-------------------------------------------------------------------------------------+                                                       +-----+   +-|-+       |           
                      |                                                                                                                                               |           |         |           
                      +-----------------------------------------------------------------------------------------------------------------------------------------------+           +---------+           
                                                                                                                                                                                                        ",
"p=<2366,784,-597>, v=<-12,-41,50>, a=<-5,1,-2>
p=<-2926,-3402,-2809>, v=<-55,65,-16>, a=<11,4,8>
p=<2290,257,-3040>, v=<41,119,57>, a=<-10,-10,5>
p=<3090,-493,-1340>, v=<-134,19,-37>, a=<1,0,7>
p=<-1338,2295,4101>, v=<-158,34,-48>, a=<19,-12,-12>
p=<-1140,1657,-981>, v=<17,-75,45>, a=<3,0,0>
p=<1544,40,141>, v=<-59,33,-75>, a=<-1,-3,6>
p=<3711,-950,-574>, v=<-8,55,61>, a=<-14,-1,-3>
p=<1643,-235,-1267>, v=<-52,11,35>, a=<-2,0,2>
p=<2501,-4492,4189>, v=<-68,124,-75>, a=<-4,7,-10>
p=<-2141,-48,2297>, v=<51,-101,-104>, a=<4,9,0>
p=<4,-2226,-1432>, v=<80,-2,8>, a=<-7,9,5>
p=<-1106,542,1857>, v=<102,-88,-64>, a=<-3,7,-11>
p=<-1405,-706,1623>, v=<13,78,-74>, a=<13,-3,-7>
p=<-664,-1226,-743>, v=<19,13,-11>, a=<4,12,10>
p=<-612,217,-1523>, v=<-34,7,105>, a=<11,-3,2>
p=<-248,1374,-639>, v=<-13,-47,58>, a=<4,-8,-1>
p=<2326,-1122,-314>, v=<-106,12,-30>, a=<-11,11,8>
p=<-14,1556,-678>, v=<-17,-131,-2>, a=<2,2,8>
p=<796,1891,-5519>, v=<-82,-9,191>, a=<4,-6,4>
p=<1854,856,-3288>, v=<-20,-24,94>, a=<-5,-1,4>
p=<-1021,2443,-9360>, v=<9,-69,178>, a=<3,-3,19>
p=<1187,2972,-2851>, v=<-63,-92,-81>, a=<1,-3,17>
p=<-2148,1753,4877>, v=<34,-27,27>, a=<5,-4,-20>
p=<957,-524,185>, v=<-41,-36,-9>, a=<0,5,0>
p=<-2723,3961,-2322>, v=<71,-87,-56>, a=<4,-7,13>
p=<-697,-666,-285>, v=<-21,22,126>, a=<7,2,-12>
p=<1700,1187,2775>, v=<-99,-69,-45>, a=<0,0,-13>
p=<2261,-2128,-2002>, v=<-105,63,110>, a=<-3,7,1>
p=<-3179,2156,2724>, v=<107,-81,-6>, a=<9,-5,-17>
p=<-2261,1442,1602>, v=<44,-3,-111>, a=<10,-9,2>
p=<7832,-6160,6204>, v=<-60,53,-17>, a=<-8,6,-8>
p=<506,10083,-1973>, v=<-71,13,52>, a=<3,-15,0>
p=<-1344,2868,-16292>, v=<-59,-58,78>, a=<5,-1,19>
p=<5316,1647,-6598>, v=<103,-120,139>, a=<-13,4,2>
p=<-863,796,-10039>, v=<-53,17,80>, a=<4,-2,10>
p=<-2047,2498,1098>, v=<17,9,-31>, a=<2,-4,0>
p=<-3246,-1044,-8976>, v=<104,-62,1>, a=<0,6,18>
p=<1869,3544,2215>, v=<35,62,-72>, a=<-6,-11,0>
p=<1714,6241,3734>, v=<8,-9,-169>, a=<-4,-12,3>
p=<-625,-2093,2556>, v=<-21,133,-91>, a=<3,-4,0>
p=<2021,-819,1800>, v=<-101,73,-64>, a=<2,-3,0>
p=<6263,-4795,-3044>, v=<-35,99,51>, a=<-13,5,4>
p=<2567,-273,-5382>, v=<-19,10,62>, a=<-5,0,9>
p=<-4125,-133,-2428>, v=<46,5,0>, a=<7,0,6>
p=<-6043,707,-8742>, v=<-16,4,153>, a=<16,-2,11>
p=<-1241,1617,-5620>, v=<117,73,-31>, a=<-5,-9,16>
p=<4042,-89,-2931>, v=<19,-60,50>, a=<-10,4,3>
p=<5422,3571,-1506>, v=<-89,-120,49>, a=<-6,0,0>
p=<-4748,1951,3579>, v=<33,89,-12>, a=<8,-10,-7>
p=<2107,-2309,474>, v=<-25,14,-17>, a=<-3,4,0>
p=<3877,2596,-3261>, v=<-84,114,-1>, a=<-3,-13,7>
p=<-4343,5221,4524>, v=<97,-20,-28>, a=<3,-10,-8>
p=<3482,2207,105>, v=<-30,-20,-52>, a=<-11,-7,4>
p=<1524,-1126,2668>, v=<-79,51,50>, a=<1,0,-15>
p=<-2084,986,688>, v=<108,-91,-44>, a=<-1,4,1>
p=<2954,-1544,3537>, v=<-75,-45,-70>, a=<-5,10,-8>
p=<3284,2515,534>, v=<-44,-149,-14>, a=<-9,3,-1>
p=<-1006,18,2250>, v=<13,-70,0>, a=<3,6,-9>
p=<-467,4121,1645>, v=<-115,-130,-53>, a=<12,-5,-2>
p=<-6800,1215,-1164>, v=<113,-70,140>, a=<5,2,-6>
p=<4114,4224,-59>, v=<37,-36,-15>, a=<-9,-5,1>
p=<3723,-3715,-9358>, v=<31,110,101>, a=<-8,0,10>
p=<-1653,897,2988>, v=<8,-51,-98>, a=<5,1,-2>
p=<-4281,1905,-1488>, v=<55,-43,-74>, a=<10,-3,11>
p=<-981,1293,1980>, v=<80,20,-81>, a=<-3,-6,0>
p=<843,321,168>, v=<-46,48,-68>, a=<1,-5,5>
p=<4791,2909,-5915>, v=<-130,-6,93>, a=<-4,-8,10>
p=<6949,-4137,1066>, v=<-24,130,0>, a=<-18,2,-3>
p=<-617,-3227,-8190>, v=<-57,122,140>, a=<6,0,13>
p=<-6,-5580,104>, v=<95,37,-17>, a=<-7,13,1>
p=<2906,5106,1326>, v=<-44,-104,-10>, a=<-5,-7,-3>
p=<387,522,-444>, v=<-61,38,-6>, a=<5,-12,6>
p=<-1845,-324,708>, v=<190,37,-102>, a=<-5,-1,6>
p=<-147,204,846>, v=<29,-85,-3>, a=<-2,11,-11>
p=<-1119,-612,1284>, v=<71,139,-111>, a=<4,-13,0>
p=<-237,-12,-1074>, v=<-22,11,-12>, a=<7,-1,15>
p=<-771,96,-696>, v=<16,28,-11>, a=<8,-5,10>
p=<2187,-396,1056>, v=<-68,4,-92>, a=<-17,5,0>
p=<-4115,4114,7904>, v=<-27,29,-166>, a=<12,-12,-8>
p=<-5305,1034,-608>, v=<30,-93,-7>, a=<11,4,2>
p=<3978,-4037,1771>, v=<25,71,84>, a=<-7,2,-7>
p=<2054,-1484,-1448>, v=<1,-17,38>, a=<-3,3,0>
p=<8677,-9069,7321>, v=<-83,-40,-142>, a=<-8,15,-3>
p=<-3903,-4925,1068>, v=<10,171,46>, a=<5,-2,-4>
p=<-1054,-7737,-3150>, v=<-10,-95,-125>, a=<2,16,11>
p=<-259,1092,-1848>, v=<61,-51,79>, a=<-5,-2,4>
p=<613,788,2888>, v=<-36,-66,-98>, a=<0,2,-10>
p=<-1963,-2212,280>, v=<23,96,-37>, a=<12,5,2>
p=<2348,-322,604>, v=<-86,-16,-51>, a=<0,2,2>
p=<3860,-970,-3743>, v=<-100,64,110>, a=<-3,-2,2>
p=<1295,-1969,-800>, v=<-33,59,15>, a=<-1,1,1>
p=<974,-1657,369>, v=<-40,134,-98>, a=<-6,0,10>
p=<-70,1361,825>, v=<-5,-124,-58>, a=<2,1,-2>
p=<-634,1445,-411>, v=<55,-66,136>, a=<0,-9,-16>
p=<290,485,-1707>, v=<-48,-25,101>, a=<4,-3,6>
p=<-256,833,-1017>, v=<-22,-28,102>, a=<7,-7,-3>
p=<-34,857,-693>, v=<-34,-17,49>, a=<6,-9,1>
p=<469,6266,5306>, v=<-92,10,57>, a=<5,-15,-16>
p=<-10174,495,2493>, v=<170,-61,109>, a=<12,3,-13>
p=<-865,2670,-523>, v=<-91,-46,18>, a=<8,-3,0>
p=<-923,-1187,-3162>, v=<31,57,139>, a=<0,-1,-2>
p=<-2344,350,637>, v=<-100,-146,38>, a=<12,9,-4>
p=<1281,-3014,-929>, v=<45,-75,32>, a=<-6,12,0>
p=<1368,1249,-5482>, v=<27,63,114>, a=<-5,-7,5>
p=<7371,-3043,5422>, v=<-120,16,-7>, a=<-9,6,-12>
p=<-633,-2898,1623>, v=<-9,-49,109>, a=<2,10,-11>
p=<2644,3105,2783>, v=<-47,-61,24>, a=<-3,-3,-8>
p=<-1139,788,2402>, v=<91,-53,-158>, a=<2,-3,-10>
p=<1017,1646,499>, v=<-93,-47,-45>, a=<0,-17,0>
p=<-2800,-158,-370>, v=<206,27,70>, a=<8,-2,-6>
p=<808,-598,763>, v=<-74,49,-45>, a=<0,1,-4>
p=<-270,854,-480>, v=<12,-173,128>, a=<2,16,-14>
p=<-1040,139,-733>, v=<94,-12,19>, a=<0,0,8>
p=<6803,-9003,-1545>, v=<-25,195,10>, a=<-10,4,2>
p=<1295,-860,-406>, v=<-3,-27,99>, a=<-2,3,-5>
p=<-1782,2353,6360>, v=<-105,-69,-135>, a=<9,0,-3>
p=<1006,2863,4133>, v=<-82,21,18>, a=<3,-6,-8>
p=<1431,-4617,-8770>, v=<28,66,135>, a=<-4,4,7>
p=<-3329,585,1447>, v=<98,-17,-8>, a=<0,0,-2>
p=<-1935,-3308,-746>, v=<22,80,4>, a=<2,1,1>
p=<2845,-2530,-3705>, v=<-53,42,75>, a=<-4,4,5>
p=<-1978,6856,3406>, v=<-70,-103,-158>, a=<11,-12,2>
p=<6069,993,-3705>, v=<-123,55,129>, a=<-8,-7,1>
p=<-1809,564,780>, v=<18,4,51>, a=<4,-2,-6>
p=<-262,1968,-1612>, v=<26,58,8>, a=<-1,-10,4>
p=<219,1630,-3523>, v=<75,-64,95>, a=<-6,0,3>
p=<2140,-366,-1287>, v=<-97,-28,24>, a=<0,4,3>
p=<963,195,3586>, v=<175,-111,-140>, a=<-19,9,-2>
p=<-1875,877,-363>, v=<74,-27,74>, a=<1,-1,-5>
p=<-1818,2412,-2470>, v=<-116,-31,-41>, a=<9,-2,6>
p=<-4716,-612,-4144>, v=<57,16,24>, a=<4,0,5>
p=<108,2106,-3082>, v=<71,-78,87>, a=<-4,1,0>
p=<-1386,-3258,2048>, v=<57,-3,0>, a=<-1,5,-3>
p=<1728,-5310,-7780>, v=<-11,128,-23>, a=<-2,1,13>
p=<-8658,-10674,1670>, v=<37,129,-45>, a=<11,9,0>
p=<-6606,-7668,-2578>, v=<54,101,-1>, a=<7,6,4>
p=<3206,-711,8158>, v=<-23,-49,-101>, a=<-4,4,-8>
p=<-7827,-3975,6543>, v=<39,-128,-106>, a=<11,14,-5>
p=<-160,-2904,-4167>, v=<6,-2,-1>, a=<0,5,7>
p=<265,1431,6696>, v=<-24,28,-58>, a=<1,-4,-8>
p=<-1673,377,4333>, v=<-72,-11,-6>, a=<7,0,-7>
p=<-3411,-85,-3544>, v=<80,-79,55>, a=<4,6,6>
p=<3570,-5220,-2049>, v=<-67,24,-43>, a=<-5,13,9>
p=<3960,1592,57>, v=<53,5,-16>, a=<-15,-5,1>
p=<-135,-72,-3466>, v=<62,96,79>, a=<-4,-7,4>
p=<-2735,2151,1578>, v=<54,-3,-115>, a=<4,-6,4>
p=<-3593,-2282,135>, v=<6,73,62>, a=<10,1,-5>
p=<-577,-2425,-515>, v=<-29,65,-21>, a=<4,2,3>
p=<-4308,1696,2254>, v=<20,-53,-60>, a=<11,-1,-2>
p=<-938,798,-2662>, v=<71,26,155>, a=<-1,-10,3>
p=<-128,2538,893>, v=<25,-138,-18>, a=<-2,-4,-5>
p=<-1714,-2056,-2548>, v=<82,40,104>, a=<1,7,3>
p=<1953,-2721,948>, v=<-131,55,-50>, a=<3,9,0>
p=<-1695,-650,-4144>, v=<-19,-64,188>, a=<11,10,3>
p=<-859,129,340>, v=<47,25,-18>, a=<0,-3,0>
p=<-3557,-61,5451>, v=<129,-5,-127>, a=<6,1,-16>
p=<-1220,-1277,1005>, v=<86,49,-103>, a=<-2,2,5>
p=<-459,-973,-680>, v=<52,77,45>, a=<-2,0,1>
p=<2115,1406,1166>, v=<-132,-64,-41>, a=<-4,-6,-7>
p=<-69,847,-1772>, v=<-13,-63,101>, a=<3,0,5>
p=<542,-1532,542>, v=<-39,78,-21>, a=<0,6,-3>
p=<1712,158,-1291>, v=<-122,-52,36>, a=<-1,6,9>
p=<6007,-3696,1991>, v=<-64,95,40>, a=<-7,1,-6>
p=<3202,2409,1760>, v=<-64,29,81>, a=<-2,-6,-8>
p=<-5642,-2310,-1837>, v=<0,-49,-14>, a=<10,7,4>
p=<9703,3861,6512>, v=<-91,-83,-12>, a=<-12,-2,-11>
p=<-131,2343,-5995>, v=<71,-71,129>, a=<-4,0,3>
p=<-2804,693,-7282>, v=<16,30,-2>, a=<4,-3,13>
p=<-3728,6435,-748>, v=<10,60,72>, a=<6,-15,-3>
p=<430,891,209>, v=<3,-10,-8>, a=<-1,-1,0>
p=<-3827,-3465,671>, v=<-21,-82,-39>, a=<8,11,1>
p=<1234,-468,427>, v=<-59,-9,-51>, a=<-4,6,3>
p=<-600,575,371>, v=<72,-76,28>, a=<-4,5,-7>
p=<-1790,1492,1211>, v=<82,61,13>, a=<6,-22,-13>
p=<597,1548,343>, v=<-21,-78,-120>, a=<-3,-4,13>
p=<-1048,-2351,2569>, v=<74,178,-99>, a=<0,-1,-11>
p=<345,1135,812>, v=<-18,-26,-71>, a=<-1,-7,2>
p=<1640,372,-903>, v=<32,-24,74>, a=<-20,0,-1>
p=<429,-1462,910>, v=<-99,77,12>, a=<9,4,-10>
p=<4879,988,-2595>, v=<-46,-43,93>, a=<-15,0,2>
p=<336,955,4236>, v=<34,-30,-114>, a=<-4,-1,-7>
p=<4109,-1608,5193>, v=<-11,-63,-169>, a=<-15,12,-6>
p=<1634,-321,-714>, v=<21,-18,19>, a=<-8,3,1>
p=<2283,152,210>, v=<-89,64,69>, a=<-1,-6,-7>
p=<1667,-3907,1585>, v=<-107,7,-28>, a=<3,15,-4>
p=<-16,141,3642>, v=<-19,30,-18>, a=<2,-3,-13>
p=<1623,-201,4473>, v=<35,9,-148>, a=<-8,0,-3>
p=<4911,-3189,1101>, v=<-27,21,-145>, a=<-14,9,8>
p=<-2397,2871,129>, v=<40,-119,108>, a=<5,0,-9>
p=<-3345,-369,3057>, v=<-33,-59,-14>, a=<14,6,-9>
p=<-3670,-2084,-4116>, v=<-72,45,60>, a=<14,2,6>
p=<3428,-4044,5054>, v=<-108,28,-21>, a=<-1,8,-11>
p=<4086,-236,2296>, v=<-146,-79,5>, a=<0,6,-6>
p=<2420,-4114,6888>, v=<-72,74,-72>, a=<-1,5,-12>
p=<374,473,215>, v=<12,-101,-65>, a=<-9,9,9>
p=<1134,773,425>, v=<-130,12,-42>, a=<3,-17,1>
p=<1059,-292,760>, v=<-73,47,-48>, a=<-6,-4,-4>
p=<224,-282,-2905>, v=<-50,2,192>, a=<5,4,19>
p=<-371,-87,-1345>, v=<114,32,146>, a=<-14,-5,-1>
p=<834,403,1200>, v=<-56,-72,-92>, a=<-5,5,-4>
p=<-841,113,-245>, v=<-4,23,36>, a=<16,-7,-1>
p=<4653,2787,-3179>, v=<-90,21,-6>, a=<-12,-14,14>
p=<-1143,-3114,-1310>, v=<32,93,-51>, a=<2,5,10>
p=<2301,2010,-1037>, v=<0,-19,68>, a=<-10,-7,-2>
p=<411,-2337,-617>, v=<-64,111,70>, a=<4,0,-4>
p=<-1752,3375,1168>, v=<72,-7,-4>, a=<1,-14,-5>
p=<-1395,1002,-2528>, v=<44,-37,216>, a=<2,-1,-9>
p=<2838,3573,263>, v=<-9,-30,-7>, a=<-4,-4,0>
p=<-277,8788,963>, v=<-100,-17,-45>, a=<6,-13,1>
p=<3328,5428,1593>, v=<103,-155,81>, a=<-11,0,-7>
p=<6338,-3532,12863>, v=<17,47,-25>, a=<-11,3,-19>
p=<-347,948,-2677>, v=<100,45,-31>, a=<-5,-4,6>
p=<-6122,6758,-3587>, v=<67,-85,103>, a=<6,-6,0>
p=<3297,6583,3637>, v=<-90,-148,-6>, a=<-1,-4,-7>
p=<4165,8908,3482>, v=<-6,-63,-17>, a=<-8,-14,-6>
p=<-640,8288,-4020>, v=<85,-75,65>, a=<-4,-12,4>
p=<2770,-3461,8318>, v=<-73,0,-125>, a=<-1,7,-9>
p=<1158,5436,3885>, v=<-101,65,-14>, a=<4,-15,-7>
p=<-5414,5963,4009>, v=<-1,-96,-98>, a=<11,-6,-2>
p=<-6065,-2159,3048>, v=<52,-42,-83>, a=<9,7,-1>
p=<-1527,-712,781>, v=<1,29,-68>, a=<5,0,3>
p=<-39,-3280,-1847>, v=<64,61,104>, a=<-5,6,-2>
p=<3633,788,-1379>, v=<-164,54,47>, a=<1,-7,1>
p=<1629,116,4321>, v=<-118,32,-128>, a=<4,-3,-4>
p=<3549,4220,1633>, v=<27,-114,-41>, a=<-14,-5,-2>
p=<1941,2540,2953>, v=<-56,-19,-121>, a=<-2,-7,0>
p=<1565,-279,-7907>, v=<88,102,0>, a=<-9,-6,17>
p=<-1090,-1404,-4232>, v=<99,31,-45>, a=<-4,1,12>
p=<635,4566,-452>, v=<-5,-13,-16>, a=<-1,-9,2>
p=<-2860,-6564,-4667>, v=<-28,17,-15>, a=<8,13,11>
p=<-490,-84,163>, v=<-76,80,41>, a=<6,-5,-3>
p=<-4540,906,-617>, v=<28,140,67>, a=<8,-11,-3>
p=<635,-6954,-5357>, v=<-129,92,132>, a=<7,9,3>
p=<-1510,3576,5248>, v=<-11,51,-113>, a=<4,-11,-4>
p=<-850,2586,2608>, v=<-188,-9,-25>, a=<14,-5,-4>
p=<2354,-627,2732>, v=<-72,171,-83>, a=<-5,-14,-6>
p=<1005,513,148>, v=<-61,-29,23>, a=<1,0,-3>
p=<-249,-1140,-859>, v=<-15,-2,66>, a=<3,6,-2>
p=<1936,2166,3245>, v=<40,14,-80>, a=<-14,-13,-9>
p=<-1256,2223,1193>, v=<38,-49,-22>, a=<3,-7,-4>
p=<-78,-3382,-1619>, v=<-24,66,-24>, a=<3,11,11>
p=<-136,-1372,1436>, v=<63,-18,-88>, a=<-5,7,2>
p=<-1423,6933,4318>, v=<41,-85,-196>, a=<2,-20,0>
p=<920,-2241,655>, v=<-77,102,97>, a=<3,0,-11>
p=<1668,707,633>, v=<50,-32,-17>, a=<-11,0,-1>
p=<1360,-239,941>, v=<-51,-35,-31>, a=<-1,4,-1>
p=<-4382,113,-3866>, v=<72,-5,107>, a=<11,0,6>
p=<-4470,-470,1744>, v=<76,10,-125>, a=<11,1,4>
p=<172,-3308,-929>, v=<49,93,31>, a=<-5,5,1>
p=<-3876,47,622>, v=<-20,44,41>, a=<17,-4,-6>
p=<2910,-3933,1100>, v=<-36,34,18>, a=<-3,5,-3>
p=<-1710,2799,-1078>, v=<104,68,-35>, a=<-3,-9,4>
p=<534,-13569,8459>, v=<-49,105,-137>, a=<2,18,-7>
p=<-1116,11577,-121>, v=<120,-45,4>, a=<-5,-18,0>
p=<-390,-9840,-1045>, v=<-55,77,-53>, a=<4,13,5>
p=<-5208,1941,-3652>, v=<142,-93,26>, a=<1,2,5>
p=<-7188,-567,1760>, v=<117,-68,-53>, a=<6,5,0>
p=<11292,-1887,-1276>, v=<-86,57,5>, a=<-15,0,2>
p=<1212,-6101,-497>, v=<1,93,96>, a=<-4,13,-6>
p=<-2124,-3077,1855>, v=<65,-33,23>, a=<2,13,-8>
p=<-1284,1567,883>, v=<-20,-14,26>, a=<6,-4,-5>
p=<1668,5371,-2033>, v=<57,-35,-40>, a=<-10,-15,10>
p=<-180,-161,-1697>, v=<34,-42,-79>, a=<-2,4,12>
p=<-2939,-929,-114>, v=<7,89,-9>, a=<10,-4,1>
p=<-1237,106,1082>, v=<65,-28,-37>, a=<-1,2,-1>
p=<-1306,-2470,2669>, v=<8,84,-58>, a=<4,2,-5>
p=<1109,474,1473>, v=<-85,40,-126>, a=<3,-5,5>
p=<2604,-3919,2140>, v=<-6,87,-47>, a=<-9,7,-4>
p=<2512,-2884,-5749>, v=<-86,102,116>, a=<-2,2,11>
p=<-6205,4614,2186>, v=<65,-44,-37>, a=<17,-13,-5>
p=<1891,1647,3911>, v=<-35,73,-136>, a=<-4,-12,-3>
p=<-12265,-6083,-1408>, v=<108,36,30>, a=<18,10,1>
p=<-237,1326,-1191>, v=<-24,117,39>, a=<2,-10,0>
p=<-3965,437,1378>, v=<112,-159,0>, a=<0,8,-2>
p=<8797,-2713,1756>, v=<-113,95,-103>, a=<-7,-1,3>
p=<-1103,1211,4366>, v=<-97,-162,-120>, a=<7,7,0>
p=<2749,-4405,2512>, v=<-19,68,-198>, a=<-3,3,7>
p=<3757,2399,-6812>, v=<-47,-84,-50>, a=<-3,1,13>
p=<-5711,-2821,8290>, v=<-80,98,-7>, a=<13,-1,-12>
p=<11263,2849,-4130>, v=<-15,-78,153>, a=<-16,0,-2>
p=<-2417,-8149,-872>, v=<69,61,-67>, a=<0,9,5>
p=<1566,465,-1809>, v=<-7,1,92>, a=<-12,-4,4>
p=<-1539,-690,2256>, v=<72,30,-147>, a=<4,2,0>
p=<6918,1222,10204>, v=<-81,-52,-142>, a=<-7,1,-9>
p=<-579,848,-10672>, v=<17,-6,52>, a=<0,-1,15>
p=<1784,-937,3438>, v=<105,-76,-48>, a=<-9,6,-3>
p=<1495,-36,-5113>, v=<-9,20,46>, a=<-2,-1,6>
p=<-233,-329,-6590>, v=<34,-16,62>, a=<-2,2,13>
p=<1279,5071,1294>, v=<76,-62,-34>, a=<-9,-9,-1>
p=<5572,-3650,5182>, v=<43,-33,-80>, a=<-18,12,-8>
p=<-314,-3758,1402>, v=<65,-29,-80>, a=<-4,12,2>
p=<2305,5071,-3134>, v=<-130,-104,144>, a=<3,-6,-2>
p=<1225,2803,1699>, v=<-62,36,-35>, a=<1,-10,-2>
p=<-1826,2803,4183>, v=<79,92,27>, a=<-1,-14,-13>
p=<3277,535,3778>, v=<-110,-20,-28>, a=<-1,0,-8>
p=<-5957,49,8773>, v=<-6,12,-101>, a=<16,-1,-16>
p=<-2852,1507,-3674>, v=<89,-56,94>, a=<1,0,3>
p=<130,-2221,-667>, v=<-27,122,42>, a=<2,-2,-1>
p=<-1674,-263,-777>, v=<9,10,116>, a=<6,0,-7>
p=<-1256,-1341,-2713>, v=<36,105,43>, a=<2,-4,7>
p=<1890,3444,235>, v=<54,-124,-22>, a=<-12,-3,1>
p=<-618,320,5680>, v=<30,18,-74>, a=<0,-3,-16>
p=<2759,-21,-810>, v=<-89,45,37>, a=<-3,-4,0>
p=<31,-142,-1723>, v=<-11,-30,90>, a=<1,3,-1>
p=<3067,-1781,510>, v=<-57,79,0>, a=<-7,0,-2>
p=<2922,-2005,-1552>, v=<-39,15,9>, a=<-6,5,4>
p=<4022,645,748>, v=<-18,-52,-96>, a=<-11,2,5>
p=<1672,3945,2748>, v=<-106,-54,19>, a=<3,-8,-10>
p=<-7378,-1180,2598>, v=<113,-135,-27>, a=<14,14,-6>
p=<3946,2459,2750>, v=<-154,-40,-118>, a=<-3,-7,-1>
p=<2245,-502,-358>, v=<-95,-20,19>, a=<-1,4,0>
p=<-4753,4381,-9193>, v=<63,2,48>, a=<4,-7,12>
p=<5782,6411,12>, v=<86,52,1>, a=<-14,-13,0>
p=<847,2281,-2088>, v=<29,-100,7>, a=<-3,2,3>
p=<-343,5956,-233>, v=<-135,11,-64>, a=<8,-10,4>
p=<-1743,-7974,-2753>, v=<13,-23,62>, a=<2,14,1>
p=<-5393,-1767,1927>, v=<-79,34,-92>, a=<13,1,2>
p=<627,-11952,1997>, v=<37,127,-76>, a=<-3,12,1>
p=<-878,-997,3047>, v=<62,66,-88>, a=<-2,-2,0>
p=<-1578,-6282,-7768>, v=<28,1,131>, a=<1,10,5>
p=<4092,-2327,2>, v=<-152,-22,17>, a=<2,5,-1>
p=<4477,-1767,-278>, v=<-19,106,-11>, a=<-6,-3,1>
p=<-1873,4441,1975>, v=<74,-172,-109>, a=<3,-8,0>
p=<-1396,2506,274>, v=<76,-131,-81>, a=<0,-1,7>
p=<539,2443,-1670>, v=<-3,-42,122>, a=<-3,-10,-3>
p=<-874,2074,373>, v=<-10,-31,-20>, a=<6,-9,0>
p=<-2224,1840,-1850>, v=<65,-75,-39>, a=<6,-3,15>
p=<-3673,-644,-1841>, v=<117,44,46>, a=<9,-1,6>
p=<745,2476,575>, v=<-54,-81,-69>, a=<1,-7,4>
p=<3941,1065,3363>, v=<-80,74,-80>, a=<-17,-15,-13>
p=<14,640,1969>, v=<88,36,-79>, a=<-10,-8,-4>
p=<2761,2482,47>, v=<-95,-69,1>, a=<-5,-6,0>
p=<1431,1760,1719>, v=<75,-91,-67>, a=<-15,0,-2>
p=<1260,-2781,693>, v=<44,118,67>, a=<-11,3,-10>
p=<-3490,2748,-523>, v=<54,77,21>, a=<13,-22,1>
p=<3559,1722,-447>, v=<-67,-59,-3>, a=<-12,-3,3>
p=<1317,4648,218>, v=<-39,-123,2>, a=<-3,-12,-1>
p=<3939,-653,-1131>, v=<-147,26,83>, a=<-6,1,-2>
p=<-393,924,-86>, v=<-79,-67,-2>, a=<10,2,1>
p=<-1647,-653,-295>, v=<27,136,-31>, a=<6,-10,5>
p=<2609,3641,-1682>, v=<-67,-100,92>, a=<-7,-9,0>
p=<4458,-44,3498>, v=<0,0,-33>, a=<-6,0,-3>
p=<1380,-804,857>, v=<42,20,-22>, a=<-4,0,0>
p=<-1961,69,-4356>, v=<-4,31,-13>, a=<4,-2,9>
p=<-201,1205,4652>, v=<7,-120,19>, a=<0,5,-10>
p=<-4681,-3211,4092>, v=<147,-48,-112>, a=<0,9,-1>
p=<5063,-1371,2140>, v=<-9,10,-18>, a=<-9,2,-3>
p=<-4713,5365,-4484>, v=<148,-118,90>, a=<0,-3,3>
p=<3042,-4034,2292>, v=<-174,-15,-140>, a=<3,17,3>
p=<1403,-173,1918>, v=<-65,51,-8>, a=<0,-4,-7>
p=<1491,1202,-579>, v=<69,0,48>, a=<-12,-5,-2>
p=<94,465,-337>, v=<6,-1,-9>, a=<-1,-2,2>
p=<-940,-866,-1833>, v=<-16,48,59>, a=<5,-1,2>
p=<-6227,-3076,-5246>, v=<-41,-61,-66>, a=<10,7,10>
p=<-7982,-970,3997>, v=<4,-95,-143>, a=<10,6,2>
p=<-4082,122,-371>, v=<-36,-83,9>, a=<7,4,0>
p=<-2834,-6391,5635>, v=<52,144,-85>, a=<1,1,-3>
p=<-728,1136,-11915>, v=<18,-49,105>, a=<0,1,10>
p=<7813,3164,-4739>, v=<-41,-1,-39>, a=<-8,-4,8>
p=<-798,-223,-1670>, v=<1,-18,142>, a=<14,7,4>
p=<-73,597,-545>, v=<-22,-12,24>, a=<5,-9,5>
p=<207,327,95>, v=<49,-51,4>, a=<-13,3,-3>
p=<-593,-1238,-210>, v=<-14,111,-26>, a=<13,2,8>
p=<517,1042,-885>, v=<-37,-29,80>, a=<-3,-14,1>
p=<-258,-808,755>, v=<24,35,-29>, a=<0,8,-9>
p=<112,-598,-750>, v=<-46,36,39>, a=<6,4,6>
p=<867,202,-620>, v=<-50,-44,37>, a=<-7,4,4>
p=<-1008,752,-745>, v=<99,-44,66>, a=<0,-6,1>
p=<-2313,-4479,2367>, v=<23,57,107>, a=<3,5,-11>
p=<-1801,-1871,-225>, v=<73,58,-76>, a=<-1,0,5>
p=<-745,5089,-689>, v=<106,-77,-45>, a=<-5,-5,4>
p=<-281,7201,-9361>, v=<75,-110,-38>, a=<-4,-7,20>
p=<-2313,-2959,1103>, v=<23,59,97>, a=<3,2,-8>
p=<-7161,-4735,-2993>, v=<125,-1,27>, a=<6,9,4>
p=<-4937,193,3567>, v=<105,10,-46>, a=<3,-1,-4>
p=<657,2664,-4372>, v=<-3,44,147>, a=<-1,-7,-1>
p=<-6432,-2164,4672>, v=<48,46,-119>, a=<8,1,-1>
p=<11452,-2929,1153>, v=<-93,-19,2>, a=<-14,6,-2>
p=<470,376,-3300>, v=<54,-40,68>, a=<-7,2,8>
p=<533,817,858>, v=<-81,-39,13>, a=<5,0,-5>
p=<-181,3085,-171>, v=<19,-15,106>, a=<-1,-12,-9>
p=<-2512,1825,585>, v=<97,-120,59>, a=<2,3,-8>
p=<-1777,208,4596>, v=<18,-10,-44>, a=<6,0,-16>
p=<2108,-3446,3819>, v=<-90,164,-62>, a=<-1,0,-11>
p=<1289,1405,2727>, v=<-73,-78,-54>, a=<1,1,-7>
p=<-1042,-3572,5919>, v=<38,60,-129>, a=<1,10,-14>
p=<-1738,-1725,-2107>, v=<-8,1,43>, a=<9,8,6>
p=<-4318,-2115,1143>, v=<79,31,38>, a=<13,7,-9>
p=<-2828,-1195,-847>, v=<36,48,64>, a=<10,1,-2>
p=<-308,1075,-1087>, v=<15,-34,-8>, a=<0,-2,6>
p=<-1938,-585,-1917>, v=<44,28,107>, a=<5,0,-1>
p=<-168,3025,1303>, v=<50,-79,-12>, a=<-4,-7,-5>
p=<3682,-2605,-727>, v=<-111,3,100>, a=<-7,12,-6>
p=<-768,-403,-567>, v=<-18,77,50>, a=<7,-6,-2>
p=<813,-386,-805>, v=<33,40,19>, a=<-9,-2,3>
p=<-3522,1773,-448>, v=<144,-69,16>, a=<7,-4,1>
p=<-1737,600,1167>, v=<102,0,29>, a=<0,-4,-11>
p=<1102,-2902,487>, v=<-65,143,-66>, a=<0,3,4>
p=<-326,-471,623>, v=<-44,27,-56>, a=<7,0,2>
p=<4553,3065,3972>, v=<-142,-109,-100>, a=<-14,-8,-15>
p=<405,-709,1966>, v=<-69,23,-90>, a=<5,2,-3>
p=<3469,-4,1122>, v=<-54,-22,-41>, a=<-9,2,-1>
p=<2732,-1907,-2464>, v=<14,76,-16>, a=<-12,1,11>
p=<664,216,-1089>, v=<-53,60,2>, a=<2,-6,4>
p=<4965,-1907,-4609>, v=<-145,53,116>, a=<-7,3,8>
p=<-4781,3747,-2882>, v=<137,-89,49>, a=<7,-7,7>
p=<588,2176,1304>, v=<-63,-114,-62>, a=<3,-4,-3>
p=<-162,226,-1591>, v=<-45,16,51>, a=<7,-4,7>
p=<-666,-2672,-1816>, v=<-25,82,-20>, a=<8,10,16>
p=<-1458,-400,-784>, v=<101,76,94>, a=<-1,-6,-5>
p=<-2514,-1256,400>, v=<99,2,-82>, a=<7,9,7>
p=<-1258,2040,-1168>, v=<12,-34,67>, a=<8,-11,1>
p=<-1170,-200,-2080>, v=<66,55,107>, a=<1,-5,3>
p=<-2114,880,-640>, v=<57,30,34>, a=<9,-10,1>
p=<1678,-1056,1640>, v=<-180,49,-49>, a=<9,2,-6>
p=<750,1592,-352>, v=<-54,-74,16>, a=<1,-3,1>
p=<-1442,1968,-520>, v=<-19,-55,52>, a=<13,-8,-2>
p=<7148,4362,-302>, v=<-171,-92,56>, a=<-5,-4,-3>
p=<-1262,2013,-2738>, v=<29,19,-10>, a=<1,-6,7>
p=<-2153,-669,-1959>, v=<111,46,153>, a=<4,0,-3>
p=<892,1266,-69>, v=<52,-35,-5>, a=<-14,-6,1>
p=<-1493,-1524,-174>, v=<27,15,-46>, a=<9,11,7>
p=<-2033,-1899,-1359>, v=<39,16,57>, a=<12,14,4>
p=<37,-954,-174>, v=<53,57,66>, a=<-7,1,-7>
p=<3562,-189,681>, v=<-118,-74,-119>, a=<-15,11,9>
p=<4502,5500,3471>, v=<-134,-88,-133>, a=<-3,-9,0>
p=<-61,-1663,3250>, v=<82,39,-3>, a=<-6,2,-9>
p=<-61,-3275,1651>, v=<-26,47,-9>, a=<2,6,-4>
p=<2745,-1628,703>, v=<393,-235,102>, a=<-28,14,-6>
p=<-2193,1468,1686>, v=<-313,207,240>, a=<17,-16,-16>
p=<-2432,2061,173>, v=<-347,291,21>, a=<25,-20,-3>
p=<2109,104,-1194>, v=<297,14,-170>, a=<-17,-1,6>
p=<1189,-1470,-2850>, v=<163,-212,-409>, a=<-11,14,30>
p=<1481,324,2466>, v=<213,41,356>, a=<-11,0,-27>
p=<2220,610,-2496>, v=<318,90,-356>, a=<-23,-7,24>
p=<-2481,1240,111>, v=<-355,179,15>, a=<21,-7,-1>
p=<2234,230,-2139>, v=<322,36,-305>, a=<-21,4,21>
p=<-1349,2138,1465>, v=<-192,303,205>, a=<15,-21,-12>
p=<-1348,768,2878>, v=<-192,106,416>, a=<8,-6,-33>
p=<3288,-44,539>, v=<470,-11,73>, a=<-32,0,-7>
p=<-2759,-41,-809>, v=<-392,-5,-111>, a=<28,0,7>
p=<-2900,-230,-1061>, v=<-415,-39,-152>, a=<30,1,9>
p=<-14,2975,219>, v=<-2,423,31>, a=<0,-32,-1>
p=<1783,-2670,260>, v=<255,-381,37>, a=<-24,27,-4>
p=<932,778,-2980>, v=<134,110,-424>, a=<-10,-8,32>
p=<1755,-2644,1480>, v=<245,-377,211>, a=<-17,22,-15>
p=<-621,705,3540>, v=<-84,98,498>, a=<11,-7,-36>
p=<2133,1423,1847>, v=<305,200,264>, a=<-20,-12,-16>
p=<2791,-220,-1644>, v=<393,-29,-232>, a=<-27,-2,15>
p=<2139,-1302,-1533>, v=<309,-185,-218>, a=<-21,13,17>
p=<1109,1580,1674>, v=<156,226,237>, a=<-3,-14,-16>
p=<-2535,97,-1203>, v=<-360,13,-169>, a=<23,-1,12>
p=<1737,-659,-2322>, v=<242,-94,-330>, a=<-14,6,18>
p=<3013,795,-30>, v=<430,112,-8>, a=<-30,-11,1>
p=<888,-411,2377>, v=<127,-61,342>, a=<-7,6,-23>
p=<2580,-340,-1825>, v=<364,-44,-264>, a=<-27,0,20>
p=<2324,1533,-830>, v=<332,222,-115>, a=<-23,-16,14>
p=<-1191,-1908,1349>, v=<-170,-275,190>, a=<12,21,-13>
p=<-176,2656,1556>, v=<-24,379,225>, a=<3,-25,-11>
p=<-2946,259,-954>, v=<-415,37,-137>, a=<25,-3,15>
p=<-378,3118,25>, v=<-56,445,-1>, a=<3,-28,5>
p=<2,2477,-264>, v=<0,354,-35>, a=<-2,-24,3>
p=<2264,634,1871>, v=<321,93,263>, a=<-27,-10,-19>
p=<1507,-2302,-972>, v=<220,-334,-137>, a=<-17,20,5>
p=<-2759,-1106,370>, v=<-392,-153,52>, a=<27,9,-2>
p=<1080,-1607,1647>, v=<152,-232,235>, a=<-10,20,-13>
p=<-1193,-1711,-2860>, v=<-170,-246,-412>, a=<10,18,30>
p=<1311,819,2101>, v=<184,113,303>, a=<-14,-4,-21>
p=<-2981,-1139,346>, v=<-428,-158,50>, a=<27,11,-4>
p=<378,176,-3454>, v=<54,25,-492>, a=<-5,-1,33>
p=<-3192,-143,841>, v=<-456,-18,117>, a=<32,3,-10>
p=<2198,1336,-1875>, v=<310,190,-267>, a=<-21,-13,19>
p=<203,2865,-1437>, v=<32,406,-204>, a=<-2,-28,19>
p=<2702,-868,-1243>, v=<386,-120,-176>, a=<-27,6,9>
p=<-1373,1484,-2247>, v=<-198,212,-321>, a=<13,-17,24>
p=<-515,-2806,-868>, v=<-76,-405,-119>, a=<5,27,2>
p=<2064,668,1417>, v=<294,96,201>, a=<-18,1,-16>
p=<-1342,-977,2545>, v=<-191,-139,365>, a=<13,9,-22>
p=<-139,2682,-2073>, v=<-19,385,-292>, a=<8,-21,15>
p=<-2525,917,959>, v=<-359,129,136>, a=<26,-8,-9>
p=<-1767,-914,2290>, v=<-260,-130,325>, a=<19,5,-22>
p=<979,1838,-2045>, v=<140,260,-290>, a=<-9,-18,25>
p=<1259,2250,-1270>, v=<178,319,-180>, a=<-11,-24,12>
p=<-744,1978,-1905>, v=<-107,279,-273>, a=<7,-21,23>
p=<352,2140,2324>, v=<50,301,332>, a=<-4,-17,-29>
p=<556,2800,-677>, v=<79,403,-94>, a=<-8,-25,5>
p=<1000,-2365,1843>, v=<137,-343,260>, a=<-9,23,-18>
p=<3198,-403,-105>, v=<455,-57,-8>, a=<-33,5,9>
p=<-1297,-1294,1901>, v=<-185,-187,273>, a=<13,15,-20>
p=<1364,-2583,802>, v=<199,-364,114>, a=<-14,24,-8>
p=<-815,741,2679>, v=<-116,105,386>, a=<5,-9,-28>
p=<-2492,-2369,847>, v=<-358,-335,120>, a=<24,15,-8>
p=<513,-208,2733>, v=<73,-24,389>, a=<-7,6,-27>
p=<-2471,-1800,384>, v=<-354,-260,55>, a=<21,19,-6>
p=<65,-2499,2144>, v=<8,-357,308>, a=<-2,25,-21>
p=<-194,-2962,1330>, v=<-30,-428,190>, a=<2,31,-17>
p=<3025,-962,561>, v=<425,-139,83>, a=<-29,11,-7>
p=<-728,-3063,731>, v=<-105,-437,103>, a=<4,25,-6>
p=<930,-2786,1428>, v=<129,-398,209>, a=<-5,24,-18>
p=<-3455,312,-397>, v=<-493,47,-56>, a=<39,2,2>
p=<302,-2001,1325>, v=<43,-285,194>, a=<-2,25,-19>
p=<2298,-1360,-1280>, v=<328,-192,-183>, a=<-21,13,12>
p=<-794,1408,-2426>, v=<-114,201,-345>, a=<7,-11,25>
p=<2040,455,2002>, v=<291,67,289>, a=<-20,-6,-22>
p=<2040,1572,1423>, v=<285,221,202>, a=<-21,-19,-14>
p=<-3011,1114,-637>, v=<-430,160,-94>, a=<31,-9,11>
p=<782,-1341,-2262>, v=<107,-195,-320>, a=<-9,15,18>
p=<3019,-888,-101>, v=<431,-124,-17>, a=<-26,6,1>
p=<-1414,1805,1648>, v=<-202,264,236>, a=<14,-19,-15>
p=<-1143,964,2590>, v=<-161,133,368>, a=<12,-8,-29>
p=<-3127,-826,1008>, v=<-450,-118,142>, a=<34,8,-14>
p=<-2417,-2045,1291>, v=<-348,-297,186>, a=<18,23,-12>
p=<-2275,-792,2149>, v=<-325,-112,303>, a=<26,7,-15>
p=<2219,1393,1517>, v=<320,200,219>, a=<-21,-12,-16>
p=<-1610,1306,2537>, v=<-227,186,357>, a=<14,-13,-25>
p=<975,-1518,2508>, v=<140,-213,358>, a=<-10,18,-26>
p=<-897,-2180,-1>, v=<-128,-309,1>, a=<7,21,4>
p=<-1564,-2273,1796>, v=<-226,-324,264>, a=<15,25,-15>
p=<77,2231,1405>, v=<13,313,197>, a=<-3,-22,-14>
p=<649,1887,1404>, v=<95,268,200>, a=<-6,-22,-17>
p=<333,3043,-875>, v=<47,434,-124>, a=<-3,-26,5>
p=<-925,-1572,2168>, v=<-133,-220,307>, a=<8,15,-22>
p=<571,2386,1925>, v=<83,336,277>, a=<-6,-24,-18>
p=<616,3182,603>, v=<85,452,90>, a=<-6,-28,-6>
p=<251,-2200,2170>, v=<33,-315,314>, a=<-6,26,-21>
p=<-1997,-1181,-1510>, v=<-285,-167,-216>, a=<22,10,18>
p=<847,2152,-1616>, v=<118,307,-230>, a=<-6,-24,15>
p=<223,424,-2903>, v=<34,54,-416>, a=<1,-8,27>
p=<2514,2109,67>, v=<360,300,3>, a=<-26,-22,0>
p=<-679,2750,1365>, v=<-96,389,197>, a=<6,-26,-13>
p=<-2486,-1751,20>, v=<-354,-247,2>, a=<23,20,1>
p=<371,-2855,1294>, v=<54,-408,183>, a=<-5,27,-12>
p=<1625,190,-2759>, v=<231,27,-394>, a=<-16,-6,24>
p=<-1821,-433,2182>, v=<-260,-65,308>, a=<20,-1,-21>
p=<1310,129,2956>, v=<184,14,422>, a=<-13,-3,-29>
p=<-2025,2177,988>, v=<-289,308,146>, a=<26,-25,-6>
p=<-2782,1356,-383>, v=<-400,196,-55>, a=<29,-10,3>
p=<2409,848,797>, v=<349,121,113>, a=<-22,-6,-7>
p=<245,-2709,406>, v=<35,-386,57>, a=<-5,29,-5>
p=<2683,1619,723>, v=<383,234,106>, a=<-25,-17,-7>
p=<-786,2197,-822>, v=<-110,312,-121>, a=<7,-18,8>
p=<926,2250,-1822>, v=<132,319,-256>, a=<-14,-23,18>
p=<-2062,-994,-2342>, v=<-289,-139,-334>, a=<13,9,21>
p=<189,1430,-2492>, v=<29,204,-358>, a=<-1,-19,24>
p=<-2385,-18,363>, v=<-340,-4,52>, a=<23,5,-6>
p=<2970,-1118,234>, v=<424,-159,34>, a=<-30,9,-1>
p=<-625,2414,-2608>, v=<-90,341,-374>, a=<6,-24,28>
p=<-1041,2464,617>, v=<-146,345,97>, a=<8,-29,-2>
p=<-2162,-1047,-998>, v=<-311,-149,-143>, a=<21,8,4>
p=<-2256,1575,-1461>, v=<-323,225,-210>, a=<25,-11,20>
p=<1109,1777,1006>, v=<158,253,148>, a=<-11,-21,-11>
p=<1426,-2508,865>, v=<206,-362,121>, a=<-13,26,-8>
p=<-240,3005,-117>, v=<-33,433,-16>, a=<3,-26,-3>
p=<-910,1926,1437>, v=<-127,275,205>, a=<9,-20,-12>
p=<-1480,1847,2940>, v=<-211,270,418>, a=<15,-18,-30>
p=<6,-2358,-2058>, v=<-6,-336,-294>, a=<-3,23,21>
p=<-2058,2013,2178>, v=<-296,287,313>, a=<23,-17,-24>
p=<404,-902,3143>, v=<58,-126,449>, a=<-7,9,-28>
p=<-2322,-476,-1672>, v=<-330,-65,-237>, a=<23,4,23>
p=<-2745,-589,1792>, v=<-393,-80,259>, a=<28,5,-17>
p=<1941,-1109,-2190>, v=<276,-163,-308>, a=<-23,14,21>
p=<1406,-2513,721>, v=<202,-361,104>, a=<-18,29,-6>
p=<188,-2221,1756>, v=<26,-314,249>, a=<1,18,-13>
p=<-331,2557,616>, v=<-46,368,86>, a=<0,-22,-3>
p=<85,-133,-3419>, v=<13,-24,-491>, a=<-3,8,34>
p=<3151,1215,316>, v=<449,173,45>, a=<-28,-9,-3>
p=<-844,-3341,-2>, v=<-117,-477,1>, a=<8,35,-1>
p=<502,1686,-2468>, v=<71,238,-348>, a=<-8,-18,19>
p=<-1517,1356,2155>, v=<-216,188,305>, a=<15,-13,-21>
p=<15,-1497,-2545>, v=<5,-211,-368>, a=<0,9,27>
p=<931,-559,2872>, v=<131,-80,410>, a=<-14,6,-26>
p=<-900,-2508,-83>, v=<-124,-365,-15>, a=<7,28,-1>
p=<255,-760,-2936>, v=<33,-110,-418>, a=<-4,13,29>
p=<882,-2392,-1417>, v=<127,-336,-202>, a=<-7,26,13>
p=<2538,286,236>, v=<361,37,33>, a=<-26,-3,-2>
p=<-657,1296,-3087>, v=<-94,184,-440>, a=<2,-8,29>
p=<-1129,654,2225>, v=<-162,93,312>, a=<12,-9,-22>
p=<1135,898,-2700>, v=<164,131,-386>, a=<-10,-8,28>
p=<-101,1379,-2551>, v=<-15,196,-368>, a=<0,-12,30>
p=<-875,88,2620>, v=<-124,12,368>, a=<11,3,-28>
p=<1020,2762,1602>, v=<142,391,226>, a=<-7,-34,-9>
p=<1901,1517,1468>, v=<269,215,211>, a=<-21,-15,-14>
p=<1994,-2383,-204>, v=<284,-335,-31>, a=<-17,24,5>
p=<-2662,1237,1016>, v=<-380,176,146>, a=<31,-14,-9>
p=<-1299,-518,2783>, v=<-183,-75,395>, a=<11,10,-25>
p=<2154,1800,1755>, v=<307,258,252>, a=<-16,-14,-19>
p=<2580,-1697,740>, v=<363,-242,106>, a=<-26,14,-11>
p=<-1554,1745,-1718>, v=<-217,247,-253>, a=<17,-21,18>
p=<2121,-997,-1810>, v=<305,-142,-263>, a=<-23,9,20>
p=<-1836,-2297,-503>, v=<-262,-332,-73>, a=<18,18,7>
p=<1616,1801,-364>, v=<235,257,-51>, a=<-13,-19,4>
p=<-293,-389,-3095>, v=<-42,-57,-446>, a=<4,7,32>
p=<-1720,2558,-643>, v=<-244,369,-89>, a=<15,-25,6>
p=<-115,-2484,-2147>, v=<-15,-354,-308>, a=<1,24,21>
p=<-470,926,-2673>, v=<-64,133,-382>, a=<5,-10,26>
p=<484,-2506,-1709>, v=<76,-353,-247>, a=<-5,25,18>
p=<618,-1377,2736>, v=<85,-196,392>, a=<-3,13,-27>
p=<2049,-2070,-199>, v=<292,-298,-26>, a=<-23,19,1>
p=<-1025,2128,-2160>, v=<-149,303,-307>, a=<16,-15,18>
p=<-981,2213,1897>, v=<-134,315,270>, a=<9,-19,-19>
p=<-1626,1942,1425>, v=<-228,277,197>, a=<14,-16,-18>
p=<-2430,-124,-914>, v=<-346,-10,-130>, a=<22,3,11>
p=<-2010,963,2401>, v=<-287,136,343>, a=<24,-10,-27>
p=<-18,1069,2874>, v=<-1,152,412>, a=<0,-15,-23>
p=<-410,2304,-2158>, v=<-58,325,-306>, a=<6,-19,18>
p=<-5,247,2825>, v=<2,37,406>, a=<4,-5,-30>
p=<-1441,-1918,-1739>, v=<-205,-274,-248>, a=<10,20,13>
p=<1718,-421,-1985>, v=<244,-62,-278>, a=<-14,4,14>
p=<1815,2181,-1091>, v=<262,308,-155>, a=<-18,-22,10>
p=<-645,-2243,2268>, v=<-90,-316,326>, a=<8,21,-23>
p=<-901,306,-2703>, v=<-126,46,-380>, a=<10,-1,29>
p=<2442,1155,799>, v=<349,165,114>, a=<-26,-17,-4>
p=<2537,-1585,170>, v=<368,-222,24>, a=<-25,15,1>
p=<784,-366,-3120>, v=<110,-55,-448>, a=<-10,5,30>
p=<831,-1139,-2824>, v=<117,-159,-398>, a=<-8,15,27>
p=<1244,726,-2075>, v=<174,105,-299>, a=<-12,-6,20>
p=<2987,-752,982>, v=<427,-107,134>, a=<-33,7,-9>
p=<-64,-2938,1182>, v=<-9,-419,172>, a=<-3,34,-12>
p=<-1002,2238,2211>, v=<-143,316,316>, a=<13,-18,-22>
p=<1608,-1418,-1969>, v=<226,-197,-278>, a=<-18,17,20>
p=<605,986,2623>, v=<88,140,374>, a=<-6,-7,-24>
p=<-1347,-1657,-2682>, v=<-194,-242,-388>, a=<12,11,30>
p=<-1952,849,2157>, v=<-276,122,309>, a=<19,-7,-20>
p=<-1433,-337,-2598>, v=<-204,-44,-372>, a=<18,7,28>
p=<1436,161,-2403>, v=<199,29,-340>, a=<-14,0,23>
p=<-1329,1914,-2561>, v=<-189,273,-365>, a=<16,-19,28>
p=<-1494,-565,-3490>, v=<-213,-82,-496>, a=<13,5,35>
p=<1996,1649,1039>, v=<277,234,147>, a=<-19,-12,-10>
p=<-1414,-1687,1919>, v=<-202,-243,276>, a=<11,14,-18>
p=<-722,-3069,-330>, v=<-103,-435,-47>, a=<9,33,3>
p=<-3507,397,-497>, v=<-501,56,-73>, a=<38,-2,2>
p=<-2284,359,1985>, v=<-326,51,284>, a=<19,-3,-19>
p=<-2088,-1086,1090>, v=<-298,-157,157>, a=<20,15,-10>
p=<-2059,-2438,453>, v=<-297,-343,63>, a=<23,20,-6>
p=<1792,-2292,-1052>, v=<260,-327,-144>, a=<-20,18,9>
p=<311,3065,-789>, v=<44,436,-116>, a=<-3,-25,2>
p=<1741,-2758,-994>, v=<248,-390,-146>, a=<-9,28,6>
p=<-1555,1747,-1795>, v=<-223,249,-256>, a=<13,-17,16>
p=<-2042,2344,620>, v=<-297,336,90>, a=<21,-26,-3>
p=<2090,2154,155>, v=<303,308,18>, a=<-21,-18,-3>
p=<2346,498,-1495>, v=<335,69,-218>, a=<-21,-7,13>
p=<-2803,-353,516>, v=<-400,-54,80>, a=<28,-5,-4>
p=<761,-2062,1872>, v=<113,-296,269>, a=<-3,24,-21>
p=<-2026,1077,2346>, v=<-293,154,337>, a=<20,-14,-19>
p=<-814,1445,-2431>, v=<-114,209,-346>, a=<6,-15,23>
p=<-82,-2733,75>, v=<-10,-390,10>, a=<0,26,-3>
p=<1841,-641,-1620>, v=<268,-94,-231>, a=<-15,10,13>
p=<2462,-2255,501>, v=<352,-324,71>, a=<-25,22,0>
p=<1272,243,2345>, v=<180,31,338>, a=<-13,0,-23>
p=<724,-2434,-1202>, v=<99,-344,-170>, a=<-4,24,11>
p=<-2839,-688,1217>, v=<-406,-97,169>, a=<31,7,-21>
p=<1526,265,-2309>, v=<218,35,-329>, a=<-18,-2,21>
p=<-1693,-2321,2132>, v=<-241,-334,300>, a=<19,21,-23>
p=<-1692,1711,1227>, v=<-245,242,176>, a=<13,-14,-12>
p=<1849,1849,-2232>, v=<269,258,-318>, a=<-18,-14,20>
p=<-2054,838,1098>, v=<-296,118,153>, a=<16,-7,-10>
p=<785,99,-2675>, v=<112,12,-382>, a=<-3,-1,26>
p=<1016,-561,2794>, v=<145,-80,399>, a=<-13,3,-22>
p=<915,1974,2332>, v=<132,279,331>, a=<-9,-19,-23>
p=<2292,-2185,299>, v=<327,-311,47>, a=<-22,22,0>
p=<-2128,763,-1650>, v=<-306,107,-235>, a=<13,-7,21>
p=<3013,-969,-715>, v=<432,-134,-102>, a=<-33,12,13>
p=<316,-1640,2411>, v=<44,-238,342>, a=<-6,17,-23>
p=<1383,145,2575>, v=<196,20,369>, a=<-13,1,-21>
p=<2200,-820,-1757>, v=<316,-117,-250>, a=<-20,8,18>
p=<-2184,1716,390>, v=<-306,245,57>, a=<23,-20,-1>
p=<-2723,2073,-886>, v=<-392,297,-125>, a=<25,-20,8>
p=<-1049,173,-3401>, v=<-147,26,-482>, a=<7,-3,34>
p=<-1557,-2558,-1440>, v=<-218,-368,-205>, a=<17,20,19>
p=<-1066,-2225,-1298>, v=<-149,-314,-184>, a=<10,20,13>
p=<-2245,49,-1494>, v=<-324,6,-210>, a=<21,-1,15>
p=<1131,-1165,2377>, v=<162,-166,337>, a=<-15,11,-23>
p=<735,-455,2431>, v=<104,-62,349>, a=<-1,3,-24>
p=<1994,2482,311>, v=<284,356,43>, a=<-19,-29,-2>
p=<-1472,701,3119>, v=<-207,99,449>, a=<8,-6,-30>
p=<-2821,1091,-1093>, v=<-405,155,-155>, a=<28,-11,8>
p=<309,-2174,-2358>, v=<38,-313,-333>, a=<-5,23,20>
p=<-2940,-1360,-18>, v=<-420,-190,0>, a=<29,8,-2>
p=<-936,-968,2621>, v=<-125,-135,376>, a=<5,6,-22>
p=<-2018,-1763,-1814>, v=<-286,-250,-260>, a=<18,11,14>
p=<1989,749,-2228>, v=<287,108,-319>, a=<-21,-7,25>
p=<-1194,815,2585>, v=<-166,117,370>, a=<12,-8,-22>
p=<-2193,1590,1652>, v=<-310,223,233>, a=<19,-8,-17>
p=<1861,-1415,1504>, v=<264,-198,215>, a=<-19,16,-14>
p=<-1328,2363,849>, v=<-189,333,117>, a=<13,-24,-11>
p=<-2238,2021,-362>, v=<-325,294,-50>, a=<25,-16,7>
p=<-2228,-832,-1337>, v=<-318,-118,-191>, a=<20,4,14>
p=<2461,1069,153>, v=<350,150,18>, a=<-20,-7,-1>
p=<2496,-958,-105>, v=<357,-137,-15>, a=<-20,4,1>
p=<2119,-394,2826>, v=<298,-54,405>, a=<-22,6,-25>
p=<1055,-3092,686>, v=<152,-442,103>, a=<-15,27,-6>
p=<-2173,-909,-1613>, v=<-310,-127,-227>, a=<23,13,16>
p=<-2484,1605,-319>, v=<-353,229,-48>, a=<23,-16,2>
p=<357,631,2589>, v=<47,91,373>, a=<-1,-3,-25>
p=<272,2775,1048>, v=<39,395,152>, a=<5,-26,-8>
p=<-1504,-2830,-801>, v=<-207,-404,-117>, a=<17,29,6>
p=<1351,3181,1010>, v=<195,455,142>, a=<-13,-31,-9>
p=<530,-1446,-3084>, v=<77,-206,-439>, a=<-3,9,33>
p=<-1952,1119,1099>, v=<-279,164,156>, a=<22,-16,-5>
p=<232,-1333,-2866>, v=<28,-187,-408>, a=<-3,13,31>
p=<-1627,-2063,2010>, v=<-234,-294,291>, a=<22,22,-22>
p=<-2775,-964,-1321>, v=<-394,-137,-188>, a=<27,9,12>
p=<-1603,2665,1942>, v=<-231,381,277>, a=<16,-29,-18>
p=<-872,-2797,-770>, v=<-128,-399,-111>, a=<9,27,12>
p=<563,2846,618>, v=<80,402,86>, a=<-4,-27,-2>
p=<-1784,1654,-1446>, v=<-258,236,-203>, a=<20,-24,18>
p=<1893,1629,-2424>, v=<270,228,-343>, a=<-18,-16,22>
p=<29,-2731,-765>, v=<4,-394,-109>, a=<4,26,8>
p=<2971,-552,520>, v=<429,-78,76>, a=<-31,5,-6>
p=<-20,2794,-979>, v=<7,399,-142>, a=<0,-28,12>
p=<-1580,-1745,1680>, v=<-224,-248,234>, a=<11,14,-11>
p=<-328,-1946,2017>, v=<-46,-279,290>, a=<4,21,-27>
p=<-845,2947,-811>, v=<-120,423,-112>, a=<6,-28,8>
p=<-216,2818,754>, v=<-31,400,100>, a=<2,-27,-7>
p=<106,2953,-578>, v=<17,421,-85>, a=<0,-25,4>
p=<-608,-2556,-1827>, v=<-86,-367,-262>, a=<4,23,20>
p=<-581,2701,-743>, v=<-84,386,-103>, a=<4,-28,11>
p=<-2807,1109,363>, v=<-404,160,49>, a=<28,-8,-3>
p=<-1309,851,2893>, v=<-187,121,413>, a=<13,-7,-30>
p=<1933,1884,-496>, v=<276,269,-66>, a=<-17,-19,4>
p=<1113,2644,1081>, v=<164,376,154>, a=<-11,-22,-12>
p=<351,-2543,-1660>, v=<46,-363,-237>, a=<-9,29,16>
p=<-2152,-1819,784>, v=<-305,-261,115>, a=<20,18,-3>
p=<639,-2441,1378>, v=<91,-348,194>, a=<-3,26,-15>
p=<-2222,-1057,-1157>, v=<-320,-152,-163>, a=<22,10,7>
p=<3311,-711,1150>, v=<470,-101,166>, a=<-34,3,-9>
p=<2195,245,2137>, v=<313,34,306>, a=<-28,1,-20>
p=<-1274,-1659,978>, v=<-176,-239,133>, a=<13,13,-12>
p=<480,1761,1821>, v=<71,253,261>, a=<-1,-18,-17>
p=<505,-2920,-1064>, v=<74,-418,-148>, a=<-2,26,12>
p=<580,1922,1361>, v=<80,276,190>, a=<-8,-22,-13>
p=<412,-2422,-113>, v=<54,-340,-18>, a=<-4,21,4>
p=<-780,2764,-1540>, v=<-108,399,-221>, a=<8,-24,15>
p=<-2953,-900,-1397>, v=<-424,-126,-200>, a=<31,9,10>
p=<2784,-1400,-75>, v=<395,-196,-10>, a=<-30,19,-4>
p=<524,2195,613>, v=<71,309,88>, a=<-10,-20,-6>
p=<-2858,795,-1974>, v=<-410,111,-282>, a=<28,-9,21>
p=<2343,-1353,-691>, v=<335,-191,-103>, a=<-23,14,2>
p=<1266,2574,-1283>, v=<180,374,-179>, a=<-7,-21,10>
p=<1847,1777,-1545>, v=<261,260,-223>, a=<-20,-17,18>
p=<2685,745,1112>, v=<385,106,161>, a=<-29,-8,-13>
p=<-1298,832,-2251>, v=<-189,117,-321>, a=<18,-10,22>
p=<990,-2338,1295>, v=<139,-334,176>, a=<-10,22,-13>
p=<-3102,-122,533>, v=<-439,-17,76>, a=<32,2,3>
p=<-2399,802,434>, v=<-342,110,62>, a=<26,-11,-3>
p=<1256,-2642,-2070>, v=<177,-377,-296>, a=<-12,22,17>
p=<960,-2780,-588>, v=<137,-395,-88>, a=<-9,27,6>
p=<993,-3243,-1465>, v=<140,-468,-213>, a=<-12,34,17>
p=<-142,-2559,1059>, v=<-27,-361,153>, a=<-3,25,-5>
p=<3210,955,-451>, v=<455,136,-70>, a=<-27,-9,6>
p=<-1267,-871,-2445>, v=<-181,-127,-349>, a=<14,15,26>
p=<2551,1269,48>, v=<364,181,6>, a=<-30,-15,0>
p=<-2991,-480,1163>, v=<-426,-68,171>, a=<36,6,-13>
p=<1302,1541,2038>, v=<181,223,293>, a=<-13,-15,-21>
p=<-532,1036,-2819>, v=<-70,150,-406>, a=<5,-7,33>
p=<1875,-1593,1724>, v=<266,-228,240>, a=<-24,11,-19>
p=<2828,-1420,520>, v=<402,-200,74>, a=<-28,9,-3>
p=<1951,329,1653>, v=<275,49,238>, a=<-16,2,-16>
p=<-2936,1272,-395>, v=<-419,181,-57>, a=<30,-14,2>
p=<557,-218,-2958>, v=<81,-28,-422>, a=<-3,4,27>
p=<-918,-3137,1683>, v=<-133,-450,236>, a=<10,34,-23>
p=<-1998,1004,-2080>, v=<-285,147,-293>, a=<16,-9,18>
p=<1551,-320,2384>, v=<223,-46,344>, a=<-14,2,-29>
p=<2892,746,228>, v=<411,106,35>, a=<-32,-11,0>
p=<1335,-1122,-2785>, v=<190,-163,-397>, a=<-15,11,26>
p=<2710,715,1453>, v=<387,105,201>, a=<-29,-7,-13>
p=<1450,1012,2195>, v=<207,147,316>, a=<-17,-8,-24>
p=<3075,-284,146>, v=<433,-41,20>, a=<-31,3,2>
p=<1804,-2040,-520>, v=<259,-290,-71>, a=<-25,22,8>
p=<107,387,3666>, v=<15,55,525>, a=<-2,-3,-39>
p=<-2838,-613,-864>, v=<-399,-89,-124>, a=<29,9,5>
p=<1287,1454,1815>, v=<180,210,259>, a=<-14,-17,-14>
p=<-1119,-3166,-696>, v=<-154,-455,-99>, a=<16,30,6>
p=<-2005,-1780,-881>, v=<-289,-252,-125>, a=<21,17,8>
p=<232,2388,1738>, v=<34,347,247>, a=<0,-23,-19>
p=<-2465,-1095,260>, v=<-352,-155,39>, a=<31,10,-2>
p=<-2291,1207,2392>, v=<-329,175,342>, a=<22,-12,-28>
p=<146,-1931,2174>, v=<26,-278,311>, a=<-1,14,-21>
p=<-1336,2330,2194>, v=<-190,333,313>, a=<16,-27,-15>
p=<1073,-2979,1785>, v=<153,-422,254>, a=<-11,28,-17>
p=<-2081,2226,-534>, v=<-299,318,-78>, a=<17,-18,5>
p=<1254,2653,523>, v=<182,379,70>, a=<-11,-32,-5>
p=<-2073,848,1586>, v=<-301,122,227>, a=<25,-7,-14>
p=<1493,2064,-1217>, v=<209,294,-174>, a=<-10,-20,14>
p=<2275,-1677,49>, v=<325,-239,5>, a=<-16,22,1>
p=<-1664,439,-2478>, v=<-236,63,-357>, a=<15,-3,28>
p=<-1998,1841,-1355>, v=<-285,262,-191>, a=<19,-22,16>
p=<-734,-1428,2437>, v=<-104,-198,344>, a=<11,15,-26>
p=<122,-2631,-1084>, v=<17,-379,-156>, a=<-1,21,10>
p=<-2357,1267,-1432>, v=<-337,186,-199>, a=<23,-6,10>
p=<1792,2381,-353>, v=<251,343,-47>, a=<-19,-29,1>
p=<2300,-981,-1410>, v=<324,-135,-205>, a=<-27,10,22>
p=<2076,453,-2196>, v=<298,63,-313>, a=<-22,-3,20>
p=<-1018,-2075,2241>, v=<-145,-296,320>, a=<8,20,-19>
p=<1098,867,2225>, v=<156,122,316>, a=<-16,-9,-25>
p=<-2699,1266,-1929>, v=<-386,177,-277>, a=<26,-9,21>
p=<2266,-2555,301>, v=<323,-364,45>, a=<-22,25,-3>
p=<1776,1506,-1079>, v=<254,215,-154>, a=<-21,-15,12>
p=<134,-2512,517>, v=<19,-357,72>, a=<-4,25,-2>
p=<-505,-2904,1442>, v=<-70,-414,202>, a=<3,30,-19>
p=<-793,1096,2909>, v=<-113,156,415>, a=<10,-11,-26>
p=<-1006,-1239,2919>, v=<-141,-178,417>, a=<12,15,-25>
p=<2390,108,1476>, v=<341,14,210>, a=<-18,-2,-17>
p=<2248,1821,141>, v=<317,257,16>, a=<-18,-15,-9>
p=<-419,-973,-2511>, v=<-59,-139,-359>, a=<3,9,29>
p=<1279,-2209,-1121>, v=<188,-315,-159>, a=<-12,21,10>
p=<-2678,-381,1727>, v=<-383,-59,245>, a=<27,-2,-21>
p=<-1050,-111,3464>, v=<-153,-15,493>, a=<8,3,-35>
p=<-668,1778,1977>, v=<-92,254,282>, a=<6,-19,-17>
p=<-333,-1542,1916>, v=<-43,-214,273>, a=<1,18,-17>
p=<1981,-41,-2417>, v=<284,-8,-346>, a=<-20,-3,21>
p=<690,2578,-1314>, v=<98,368,-189>, a=<-2,-25,13>
p=<534,-2151,2291>, v=<76,-311,327>, a=<-2,21,-19>
p=<-519,1718,-2056>, v=<-77,239,-288>, a=<4,-18,16>
p=<344,158,-3042>, v=<47,22,-435>, a=<-4,-4,25>
p=<-1568,-1527,2263>, v=<-224,-220,326>, a=<15,15,-29>
p=<-2860,507,-67>, v=<-411,71,-6>, a=<27,-4,0>
p=<2379,-357,1061>, v=<342,-50,152>, a=<-21,6,-12>
p=<-1321,1199,-1961>, v=<-185,171,-280>, a=<11,-11,22>
p=<-1827,-114,-2909>, v=<-255,-21,-411>, a=<15,-5,34>
p=<468,2593,2487>, v=<70,367,359>, a=<-2,-17,-20>
p=<637,1292,2082>, v=<90,187,295>, a=<-5,-13,-16>
p=<-1312,-1771,-1992>, v=<-182,-255,-283>, a=<9,16,14>
p=<86,392,3060>, v=<10,59,441>, a=<0,1,-28>
p=<-987,1799,-2530>, v=<-146,260,-361>, a=<4,-21,24>
p=<2165,1739,717>, v=<306,248,103>, a=<-21,-17,-10>
p=<-652,-2330,1633>, v=<-93,-332,233>, a=<4,26,-14>
p=<-502,-2082,-2395>, v=<-71,-299,-344>, a=<5,23,22>
p=<-1424,277,2478>, v=<-203,38,357>, a=<12,-1,-24>
p=<-932,-1401,2206>, v=<-131,-198,317>, a=<13,18,-19>
p=<-2744,1200,234>, v=<-390,171,33>, a=<30,-16,-2>
p=<-1711,-2047,1117>, v=<-240,-291,156>, a=<12,24,-11>
p=<-955,-1516,1897>, v=<-135,-214,271>, a=<9,19,-14>
p=<3188,-1071,-88>, v=<455,-150,-12>, a=<-32,10,4>
p=<1092,-764,2456>, v=<156,-110,349>, a=<-11,7,-24>
p=<-279,2480,-1367>, v=<-39,355,-196>, a=<2,-32,16>
p=<-2046,-345,-1904>, v=<-289,-49,-267>, a=<17,1,18>
p=<1227,947,-2286>, v=<177,138,-319>, a=<-8,-10,23>
p=<-2122,420,-2228>, v=<-304,65,-320>, a=<24,-8,22>
p=<2545,-1184,-150>, v=<366,-174,-19>, a=<-19,16,0>
p=<-1717,-1624,2020>, v=<-243,-235,288>, a=<17,17,-19>
p=<398,2910,523>, v=<57,415,74>, a=<-4,-26,-9>
p=<-3419,113,229>, v=<-489,13,34>, a=<37,-1,-4>
p=<1781,345,-2493>, v=<254,53,-359>, a=<-19,-4,24>
p=<-1513,2248,366>, v=<-219,321,52>, a=<10,-23,-2>
p=<-2669,-1445,-911>, v=<-379,-207,-128>, a=<25,18,12>
p=<1786,1285,-1423>, v=<256,183,-201>, a=<-22,-17,8>
p=<-1100,2717,688>, v=<-149,386,97>, a=<6,-29,-6>
p=<2116,-874,2249>, v=<303,-124,319>, a=<-23,4,-22>
p=<-1122,2510,-1821>, v=<-158,353,-258>, a=<12,-27,15>
p=<489,-1820,2157>, v=<73,-256,308>, a=<-5,20,-17>
p=<-2631,-1665,-971>, v=<-378,-237,-133>, a=<30,17,10>
p=<1390,3117,-326>, v=<205,443,-45>, a=<-13,-26,3>
p=<2202,1284,1507>, v=<311,186,218>, a=<-28,-12,-17>
p=<1988,-125,1719>, v=<282,-20,245>, a=<-15,-5,-10>
p=<-1409,-1789,-1224>, v=<-202,-260,-173>, a=<11,17,11>
p=<1556,-1697,-1457>, v=<223,-233,-210>, a=<-14,19,14>
p=<2654,611,-1584>, v=<380,89,-224>, a=<-22,-12,15>
p=<-1057,3100,974>, v=<-151,444,139>, a=<14,-35,-13>
p=<-2545,1545,-214>, v=<-363,222,-24>, a=<21,-11,5>
p=<-1014,-1434,1241>, v=<-144,-203,180>, a=<14,19,-11>
p=<74,-724,2914>, v=<10,-95,418>, a=<3,7,-29>
p=<2548,862,-1116>, v=<364,129,-159>, a=<-25,-8,11>
p=<998,740,2388>, v=<141,107,345>, a=<-9,-8,-23>
p=<1171,-2442,354>, v=<167,-348,47>, a=<-10,27,-1>
p=<1564,-2205,-1141>, v=<224,-316,-162>, a=<-13,22,8>
p=<897,-1715,-2190>, v=<126,-248,-311>, a=<-9,20,22>
p=<113,194,-3187>, v=<21,28,-458>, a=<3,0,34>
p=<-2901,837,1268>, v=<-414,125,181>, a=<29,-8,-11>
p=<-2669,263,-789>, v=<-377,37,-113>, a=<26,0,6>
p=<2271,-2151,-1233>, v=<323,-309,-175>, a=<-18,21,12>
p=<-3035,-879,208>, v=<-436,-127,33>, a=<33,7,-3>
p=<927,-2798,-579>, v=<137,-399,-79>, a=<-13,28,1>
p=<2615,-373,664>, v=<377,-50,90>, a=<-25,0,-7>
p=<2526,-875,1241>, v=<360,-120,175>, a=<-27,7,-9>
p=<1475,-2741,243>, v=<210,-391,35>, a=<-16,32,4>
p=<1568,1680,-1392>, v=<216,234,-198>, a=<-14,-16,12>
p=<-2639,1179,1362>, v=<-379,168,194>, a=<26,-13,-11>
p=<-2697,-554,426>, v=<-386,-76,58>, a=<23,2,-4>
p=<-625,852,-2636>, v=<-88,119,-374>, a=<5,-8,27>
p=<2894,1356,579>, v=<413,191,80>, a=<-22,-9,-6>
p=<-477,3442,-25>, v=<-70,497,-1>, a=<0,-30,-3>
p=<1957,-1935,1398>, v=<275,-275,202>, a=<-19,23,-11>
p=<-1098,62,2996>, v=<-155,8,428>, a=<10,-1,-27>
p=<-457,1398,-2512>, v=<-64,197,-361>, a=<3,-13,27>
p=<106,-1781,-3135>, v=<15,-252,-450>, a=<1,14,31>
p=<113,-2766,-627>, v=<16,-392,-89>, a=<-1,23,6>
p=<-1065,-2843,1520>, v=<-155,-407,215>, a=<9,31,-11>
p=<-1255,-2866,-1144>, v=<-181,-409,-163>, a=<14,30,14>
p=<1550,-1700,2519>, v=<219,-239,357>, a=<-16,18,-26>
p=<1244,-1136,-2392>, v=<177,-159,-341>, a=<-7,11,23>
p=<3029,-1447,-462>, v=<432,-209,-69>, a=<-36,14,2>
p=<563,2823,137>, v=<80,408,17>, a=<-6,-28,1>
p=<-404,552,-3428>, v=<-56,80,-484>, a=<9,-8,36>
p=<2138,-270,2124>, v=<301,-35,304>, a=<-21,1,-24>
p=<-3501,-392,-218>, v=<-499,-54,-30>, a=<34,4,-4>
p=<1122,-2971,-683>, v=<162,-422,-97>, a=<-13,29,7>
p=<3094,-797,736>, v=<442,-113,99>, a=<-28,5,-7>
p=<89,-2730,-416>, v=<6,-390,-59>, a=<-4,24,4>
p=<-1756,2362,-1159>, v=<-254,337,-164>, a=<21,-23,14>
p=<-2090,-2397,-1211>, v=<-293,-342,-171>, a=<20,20,13>
p=<267,2047,-2406>, v=<45,296,-343>, a=<-3,-19,23>
p=<-791,-1538,-2557>, v=<-118,-224,-360>, a=<6,17,24>
p=<-729,-2964,-1002>, v=<-104,-422,-146>, a=<7,31,11>
p=<1095,-375,-2275>, v=<154,-60,-326>, a=<-11,6,21>
p=<134,3102,-474>, v=<21,449,-67>, a=<-2,-34,2>
p=<2696,419,237>, v=<388,63,33>, a=<-31,-10,4>
p=<1051,-514,-2632>, v=<150,-76,-374>, a=<-8,7,28>
p=<-95,1194,2464>, v=<-10,169,352>, a=<2,-8,-24>
p=<116,1820,2060>, v=<21,265,298>, a=<-1,-18,-27>
p=<1137,-583,-2833>, v=<162,-80,-405>, a=<-8,10,25>
p=<597,2132,1785>, v=<83,308,255>, a=<-5,-24,-17>
p=<-2893,-510,397>, v=<-413,-69,60>, a=<28,12,-6>
p=<1326,-2526,-174>, v=<188,-355,-26>, a=<-16,21,-5>
p=<2363,-1715,29>, v=<338,-244,5>, a=<-25,18,3>
p=<3268,-674,385>, v=<468,-98,55>, a=<-32,5,0>
p=<-1907,2327,-1359>, v=<-276,327,-194>, a=<15,-22,13>
p=<1871,-498,1901>, v=<269,-73,271>, a=<-14,5,-19>
p=<-685,2520,824>, v=<-94,360,120>, a=<11,-29,-8>
p=<1477,190,-2102>, v=<213,26,-300>, a=<-17,-3,22>
p=<-2607,1051,-728>, v=<-372,153,-103>, a=<24,-11,11>
p=<-798,-1889,-2102>, v=<-117,-269,-294>, a=<6,18,18>
p=<1459,1900,608>, v=<206,273,91>, a=<-12,-20,-7>
p=<1976,-1760,1559>, v=<282,-248,224>, a=<-22,18,-15>
p=<379,2443,-1604>, v=<57,351,-229>, a=<-1,-26,15>
p=<2448,174,-1634>, v=<348,24,-236>, a=<-24,3,10>
p=<-1102,2476,-860>, v=<-154,348,-122>, a=<11,-22,8>
p=<-2586,385,-67>, v=<-370,57,-11>, a=<27,-2,0>
p=<-2933,447,-1080>, v=<-421,63,-149>, a=<30,-4,7>
p=<2722,-778,592>, v=<385,-114,77>, a=<-29,8,-7>
p=<-1364,1786,-2688>, v=<-196,252,-385>, a=<12,-17,26>
p=<-881,2946,-1634>, v=<-124,420,-236>, a=<9,-29,19>
p=<-1606,-1555,-2462>, v=<-226,-226,-348>, a=<18,15,24>
p=<225,2145,1674>, v=<33,302,239>, a=<6,-19,-17>
p=<169,2788,-958>, v=<27,403,-136>, a=<-6,-27,8>
p=<2060,1504,2049>, v=<298,214,297>, a=<-20,-16,-18>
p=<899,-2216,305>, v=<125,-320,44>, a=<-4,16,-5>
p=<-777,-768,2593>, v=<-106,-109,373>, a=<7,7,-24>
p=<-1703,2998,-1377>, v=<-239,429,-198>, a=<19,-29,15>
p=<2407,-1677,-1103>, v=<342,-235,-156>, a=<-21,17,6>
p=<-2225,1617,411>, v=<-311,228,64>, a=<25,-20,-5>
p=<-1362,1799,-2016>, v=<-194,257,-287>, a=<10,-21,15>
p=<278,-848,-2184>, v=<40,-116,-311>, a=<-2,11,20>
p=<1849,-751,-1606>, v=<268,-104,-226>, a=<-16,7,24>
p=<1935,-1226,2484>, v=<271,-175,350>, a=<-19,11,-24>
p=<-2656,1005,-1618>, v=<-379,142,-233>, a=<29,-12,16>
p=<-2561,-1255,488>, v=<-368,-179,69>, a=<26,6,-7>
p=<2844,-41,828>, v=<407,-5,112>, a=<-27,-5,-8>
p=<1955,-2382,1068>, v=<279,-340,153>, a=<-19,18,-9>
p=<1998,2280,342>, v=<282,322,49>, a=<-22,-21,-2>
p=<1423,2824,516>, v=<207,401,73>, a=<-16,-25,-2>
p=<-611,2063,1513>, v=<-85,295,212>, a=<5,-25,-14>
p=<-2533,-552,912>, v=<-356,-81,127>, a=<31,5,-8>
p=<2190,1542,488>, v=<309,221,66>, a=<-22,-14,-8>
p=<3364,222,-696>, v=<489,29,-99>, a=<-37,-3,9>
p=<-195,-2901,-9>, v=<-25,-413,-2>, a=<1,30,0>
p=<-1965,-2019,1592>, v=<-281,-287,228>, a=<22,18,-15>
p=<1379,2381,1120>, v=<199,340,166>, a=<-10,-20,-12>
p=<-2981,1450,-856>, v=<-422,211,-120>, a=<26,-18,5>
p=<1804,2104,-526>, v=<256,305,-79>, a=<-20,-23,7>
p=<-49,-1044,2462>, v=<-7,-149,352>, a=<-6,7,-24>
p=<2033,-429,-2363>, v=<293,-60,-341>, a=<-19,8,18>
p=<-1922,-579,-1416>, v=<-276,-78,-207>, a=<17,0,13>
p=<-3136,-1214,173>, v=<-450,-177,19>, a=<34,11,4>
p=<2336,1073,-95>, v=<333,155,-10>, a=<-22,-10,0>
p=<-586,-2557,619>, v=<-84,-365,86>, a=<3,28,-3>
p=<840,157,-2757>, v=<122,25,-395>, a=<-8,4,27>
p=<-1097,-1519,1850>, v=<-156,-217,265>, a=<10,20,-22>
p=<2188,516,2054>, v=<309,72,297>, a=<-22,-4,-22>
p=<3141,115,1479>, v=<444,11,212>, a=<-26,-8,-9>
p=<1399,354,-2638>, v=<197,48,-375>, a=<-11,0,28>
p=<-926,2122,1918>, v=<-132,303,276>, a=<9,-23,-19>
p=<-2852,-308,28>, v=<-407,-43,6>, a=<33,3,-1>
p=<-1258,1760,-1548>, v=<-182,248,-219>, a=<12,-19,13>
p=<-860,2008,-2187>, v=<-122,285,-315>, a=<10,-16,22>
p=<-612,-2820,223>, v=<-89,-404,37>, a=<6,29,-2>
p=<2983,1129,-643>, v=<426,162,-94>, a=<-32,-12,5>
p=<-2164,-1260,-436>, v=<-308,-178,-62>, a=<22,16,1>
p=<-117,-1598,-2780>, v=<-19,-228,-396>, a=<1,19,33>
p=<2568,1371,-634>, v=<365,191,-83>, a=<-25,-13,7>
p=<1348,-1896,-2216>, v=<195,-265,-316>, a=<-13,17,23>
p=<-33,969,-2834>, v=<-4,139,-403>, a=<-2,-12,23>
p=<1468,-522,2281>, v=<209,-71,324>, a=<-17,8,-19>
p=<-377,2072,2027>, v=<-53,297,288>, a=<6,-20,-18>
p=<-2661,346,-1704>, v=<-376,44,-240>, a=<28,0,16>
p=<-970,2021,-1566>, v=<-139,292,-223>, a=<8,-23,18>
p=<-1839,1994,-1675>, v=<-262,286,-239>, a=<26,-15,17>
p=<-1077,-1477,-1848>, v=<-148,-214,-264>, a=<10,11,12>
p=<124,243,-2828>, v=<14,38,-407>, a=<5,0,28>
p=<-2674,1373,316>, v=<-378,196,50>, a=<17,-10,-4>
p=<-227,-2451,-1109>, v=<-32,-351,-157>, a=<2,26,5>
p=<880,-940,-2673>, v=<124,-133,-378>, a=<-4,8,26>
p=<2778,198,1439>, v=<398,29,205>, a=<-28,-1,-18>
p=<206,2864,614>, v=<32,410,88>, a=<-3,-29,-9>
p=<-1369,2288,-1446>, v=<-190,326,-209>, a=<11,-13,13>
p=<518,2025,1367>, v=<70,289,197>, a=<1,-16,-11>
p=<-887,-2131,-2169>, v=<-123,-304,-305>, a=<10,20,26>
p=<992,1365,-2469>, v=<150,192,-352>, a=<-12,-12,30>
p=<-2241,1405,-737>, v=<-318,200,-100>, a=<23,-14,7>",
""
    ];
    let mut x: Vec<String> = vec![];
    for i in n {
        x.push(i.to_string());
    }
    return x[(day - 1) as usize].to_owned();
}
