from enum import Enum

raw_data_string = """
[(<[(({{<{{[{({})}{<{}{}><()<>>}]}[{[({}{})]([{}()](<>{}))}[<<[]()]>[<[]<>>{[]()}]]]}{[[[{[][]}{[]()}]
(<<{((({<(<{([()[]]<[]<>>)}>({[{<>()}]<[<>()]({}())>})){{[(<<><>>{<>()})<{[]{}}>][{[[]()]<[]()>}
{({([({[[([<{(())([][])}<(<>())[[][]]>>{{{{}{}}{<>{}}}({{}<>}<<>()>)}]<<((<><>))<<[]>((){})>>[<{{}{}}
(<[{{{[[[<[[[([]<>)]{{()()}<()()>}][({()<>}(()()))[<[][]>[[]<>]]]]([{[(){}]}(<[]{}>{{}()))]<
{(<<({<{<<[[<[[]{}]>]<[<{}{}>[{}[]]]{[<>{})({})}>]><([[{(){}}{()[]}]])>><{({<[{}()]<{}[]>>(<
<<[[<<[({[<[{{[]()}[()()]}[<{}{}>[<>]]]><[[(<>[])]]{{{{}<>}[{}()]}{<()>[{}[]]}}>](({<((){}
<{[[((<{{[[(([(){}][()<>]))[(<()()>[[]<>]){[(){}][()[]]}]]]<([<[{}{}]([]<>)><<{}<>>{{}{}}>]<({[]{}}[{}{}]
<<<{<[[[[([<([[]<>])[[<>{}][[]<>]]>{{[{}{}]}[[[]()]]}][[<((){})<[]<>>>]<{[(){}][[]<>]}<<[][]](()())>>]){<((
[<{{({<<[{{<<(()())[{}[]]>{[<>{}]<[]<>>}>{[([]<>)[{}<>]](({}<>)[(){}])}}}[[(<{()}{<>()}>)<{<{}{}>([
<{<[{[[<[({{[[{}[]][()()>]((<><>)[[]{}])}{({{}{}}[()])<{<>{}}(<>[])>}}<<(<<>[]>[()])>>)]({{[([<>])(<{}[]>{
(({<{<{[[([<<{{}{}}>(<<>{}><()()>)>]{{(<{}>{{}[]})}(<([][]][<>[]]>{{<>{}}[<><>]})})(<[{{<>()}({}{})}{{{}[]}{<
{{[<<{{(({{{<{<>{}}{[]{}}>[<[]{}>{()()})}}(<({[]()}[()<>])[{<>()}<[]()>]>[[[{}[]][<>{}]]<{<
{{{<<<[(<[({([()()]))[<[()<>]{()()}>((()[])([]()))])][([<{{}()}<[]<>>>](([<>]<<>{}>){({}<>){[]<>
<<({{[([{{<[[<()()>{(){}}]{{()<>}[(){}]}]{[({}{})([]<>)]}>(<([{}()](()[]))({<>()}<<>[]>)>[<[()<>]<
{(<{{([{<{<(<({}{})([]())>)>{[{{{}[]}(<>{})}{([])<{}<>>}]<{<{}()><()[]>}<{[]{}}<[]>>>>}>}<(<<([
([([(<{<[(<[{<<><>>(()<>)}(([]{}))][<(<>{})<{}>>{[()[]](<><>)}]>[[[{[]{}}[[]{}]]]<{[()[]]}[[{}{}]<<><>>]>])
{<<({({{{[({[[<>[]]({}())]{[{}()]}}({[()()]{[]<>}}((()<>)<[]{}>))){[{{{}()}{{}()}}]}]}<<<[[<[]()>][
{{[[[{<<[(([{[<>{}]<<><>>}{{<>}{[]<>}}]<[<<>[]>({}{})]>)({<{()()}>{{()[]}({}())}})}]<([{[<[][]>{()[]}]}]
(<<[<[[[<{{[{{()}[{}[]]}[<{}<>>{[]{}}]]{{[()[]]<(){}>}}}(((<{}{}>{()()}))[<{<>()}{()<>}><[
[[{[<([([<<[[[()()]{()<>}]({<>{}}[<>[]])]>(<([<>()]<()<>>)[[{}()]]>)>])]){({{{[{<({}[]){[]()}><<[
(<[<[(([({{[({<>[]}[<>[]])[([]{})[<>[]]]]([<{}}[{}<>]][(()()){{}()}])}([<({}())[[]<>]>([[]{}]
{([[((<(<([{([<>[]]{()})[(()[])]}])(<[<{{}()}>]>)>)>)){{{<<[<[<(<><>){<>[]}>[[<>{}]]]({<<>{}>
<<(((({<<[{([({}{})[<>()]]<<(){}>({})>)}{<<{[]{}}{<><>}><{<>{}}[()[]]>}<(([][])(<>[]))<[<>[
{{{[{[[([({{{<{}{}>{()}}((()[]]<<>{}>)}}<[{[{}[]]<<>[]>}{[{}[]][()<>]}]<<({}[]){()()}><{<>{}}(<>[]
[[([(([({<[<([<>()][[]()])>{{<[]{}>[<>()]>[([]{})<(){}>]}]({[(()<>){{}()}]}<{(<><>)({}{})}{[[]<>]{{}()}}>
(<{([{{([<{<<<<>[]><[]()>>[(()[])[<>()]]>{({()<>}{(){}})<<{}<>>{<>{}}>}}[[{<{}()>}<(()<>)[()<>]>]{[[<>
<[[{({[<<<{{<[<>{}]({}<>)>(<()[]><[]>)}(<{{}()}<{}{}>>{{()}<[][]>})}[{({[]{}}(()[]))<[[]{}][[]{}]>}<{{<>{
{({[[<{<([<((({}{})[{}{}])){{[()<>]<<>()>}[(<><>){[]}]}><<<<()<>>(()<>)>>[{([]){<>{}}}]>]){({{[{{}
(<<[{[<<({<{<[<>[]][{}<>]>([{}<>]<<>()>)}{(<{}{}>{{}()})<([])>}>{(({[]())(<>[])){[[]<>][<><>]}
[({(((<(<<{({<()()>({}()>}((<>[])[[]]))}>{<(<({}<>)([]())>[{()<>}<<>>])<<([])<[]()>>{(<>{})[<>{}]}>>[[
<<{[(({<<{[{{<<>()>{<>{}}}({[]}{()})}][<[[[]()]]<<{}{}}[<>()]>>]}[<(([{}{}]<()<>>){{()<>}[[]<>]})<{<[
(<[[{{{<<(<<[<()[]>(()[])]<[()()]([][])>><{({}<>)[<><>]}>><[(({}){[][]})((()()))]>){{<[([]<>)[<><>]
[[[[<[<[[[<(([<>{}]<<>()>)(<<>()>{{}{}}))[([()()]([]<>))]>(<[([]())<[]<>>]{<{}{}>{{}{}}}){(<()[]
{[<(<[[<{<{[{({}[])[<>{}]}<[()<>]>][(<{}<>>(()[])){{()()}<()[]>}]}[<[(()[])]({{}{}}(<>{}))>]>}<{{[{([]{})<(){
(<[{{{{{{[{[{(<><>)({}<>)}[(()[]){<>[]}]]}[{([{}<>])}]]}((([{[[]][[]{}]}({<>()}[()[]])][({[
(<<<<[[{{<<((<{}()>[<><>]))<[<<><>>([][])]<{{}<>}([]<>)>}>{{[(<><>){<>{}}]{<<>()>(()())}}[<[<><>]{{}
{({[[<({<{[[[[[]<>](<>{})]((<>{})<[]()>)]([<()[]>({}<>)][[[]{}>[{}()]])]{({<[][]><[]()>}<(<>{})>)((({})
{[([[{<<[{<<[<()()>[{}[]]]>{(({}{})([]<>))[[{}()][()<>]]}>[<(([][])){[{}{}][{}{}]}>{{<(){}>([]<>)}
([{<{{{(<[{[{{[]}[()<>]}(<[]{}>{()<>})][(<[]><{}<>>)<([]{})(<>[])>]}][[{[<[]{}>][[<>[]]{<>[]}]}<((()[])(
<(({{<<[[<<[[(<>[]){[][]}]{({}())(()<>)}]{({{}<>})}><([<<>()><{}[]>][[<>{}]({}())])[(([][])({}<
[<<{{{[[[((<{([][])<<>()>}((<><>)([]{}))>))[<(<[<>()]><([][])<<><>>>)>[[<<{}{}>(<><>)><{[]{}}>}{({
<<{<<[[[(((({{[][]}[{}[]]}(<<>{}><{}<>>))[(([]<>)<()[]>)<[<><>]>]))<<(<<<>{}>>{(<>[])})<[<<
[<<{({{<[<[<<({}[])(<><>)>(([]{})([]<>))>]>]{[(<<{(){}}[[]<>]>>){([<{}()>[()()]](([]())[<><>]))
{({<[<[{(<{[([[]<>]{<>})<{<>[]}([])>]}<{((<>[])[{}[]}){{{}{}}<[]<>>}}<{[[]{}](()[])}<(()())
(({{<{<[[(<[([<>()][[]()])<({}<>)(()())>]{<<<>[]>{(){}}>}>([{({}())}{<[]>(())}]{<<[][]>({}[])>}))]
{{<{{(<[[[{<<<[][]><{}[]>><(())[{}[]]>>}][<(([[][]][<>{}])(<[][]>)){{([][])<{}{}>}[[[]{}]{[
[([{<[[<[{([{<[]{}>(<>{})}]<{[[]()]}[<()<>>]>)}][([[[([])<<>()>]{<{}<>>[()<>]}}((<{}{}>(()<>))(<{}
([<{(<({((<{<{()[]]{[]<>}>(<[][]>(()<>))}{<[()[]](()())>{([][])<<>>}}>))(<[(({[]()}(()[]))([(){}]{<>
([{{[{(<[((<<[[]()](()())>{<{}()}{[]}}>{(({}<>))<<<>{}>[{}()]>})[[((<>)<[]<>>)([()()]{<>{}})]<{[(){}]{[
<[<[<[[[({{{[[()<>]([]())]}}{({[()()]}{{<>[]}<<>{}>})[[{[]()}[{}{}]][{<>()}<{}{}>])}})<<{([{()[]}<[]
<([[<{<<{(<(<{[]}[{}()]>({()}{<>()})){{[[]()](<>[])}(<{}{}>(<><>))}><{{{(){}}<<><>>}}>){(([[()<>]<<><>
{(([(<[<{[[<{(()[])[{}<>]}<<{}()>>>([({}<>){<>{}}]([()[]]{{}()}))]({[[[][]]]})]}>][({[{<<([]<>){()<>}>({<><
([<(((<[<({<[({}{})((){})]{{()<>}<[]<>>}>{[{()[]}<<><>>]({{}<>}[[]<>])}})[[{([[][]](()[]))({()[]})}((((
{<[{[(([<(<{<[<>{}]>[((){})({}())]}{(<(){}>{[]<>})([()<>](()<>})}>[(<{[][]}{<>[]}>[<[]<>><<
[<<{((<(<((<<{[]<>}{[]<>}>{[<><>](()())}>{([<><>])[([]{})(<>[])]))(<<{[][]}{{}()}>[<<>{}>{{}{}}]><[({}{
(<([((({{([[<<{}()><()[]>>[{()<>}{{}{}}]]{<({}{})[[]<>]>{(()[]){<>}}}][{{({}{})}<([]<>)<()[]>>
({([[<<[[[[([<()()>{{}<>}]({{}[]}))]<{(<(){}>[()()])}>]{{<([<><>]{[]{}})<<{}<>>[()<>]>>}{[
<({{(<<((<<(<({}{})<{}()>>{{<>}})([[{}()]{{}()}][<<><>>])><{[<[]()>[()()]]<([]())>}>>[<<<{()}<
[[[(<((({{{{<{[][]}{<>{}}><{[]{}}<()[]>>}<([{}{}]{{}<>})>}<({[[]<>][[]<>]}[[()<>]<(){}>])>}{{(
(<<{({[{({[{(([]<>)({}[])}[<(){}>]}]})}][({(<<{(()())}[<(){}>(()[])]><({<>{}}<()()>){<[][]>[(){}]}>>
([({{{{{([{[{([][])<[][]>}<{()<>}<{}<>>>]}{[{([][])}<<()()>[{}[]]>]{<[[][]]{[]<>}>}}]<[(<<[]<
[([<({(<({[<{[{}<>]}>{[(()<>){<>()}]}](([([])[<>{}]])<[[()()]{[]}](((){}]{[]{}})>)}[<[<{<>[]
[({<[(<<{<[{[[(){}]<<><>>][<(){}>[{}[]]]}(({{}()}<[]<>>){([]()){{}()}})]><<(<<<>[]>{<>}>(({}()){<>()}))[[(<
{{{{<{{<<{[<([()[]][{}[]])[{<>{}}[(){}])><((()[]){()[]})[[{}[]](()<>)]>]}>>}}>{{[[{<(<(({}<>)[<>()]){[
([<{[<<[{({([<(){}>([])][(()<>)<<>()>])}{[([[][]]{{}()})<[{}]({}<>]>](([[]()]<{}<>>){<{}()>[[]()]})})[<
<((<{({[<(<(<<{}<>>{[]()}><<()<>>([][])>){<<[]{}>{<>[]}>{[<>[]][()()]}}>(({{()}{()<>}}({[]<>}(<>{})))
<[(<{{{<[(([{{[]<>}{{}()}}<[<><>]<<>[]>>]({((){})}))<(([<>()]<[][]>)(<()<>>([]<>)))>)]>[[[[[{[
(({{(([{{((<{<<><>><{}<>}}{{{}{}}{[]()}}>[[(<>[])[{}{}]]{<()[]>[[]{}]}])){{{{{()<>}<<><>>}{<[]<>>[[]<>]
(<{(((<<(({{{{[]()}}<<()()>{[]>>}[<{<>{}}{<><>}><{{}()}({}<>)>]}))(([((<(){}>({}()))<{<><>}<[]()>>)<(
<[[<[(<<{((<[<[]()>([]<>)]{(<>[])[<>()]}>[<(()<>){[]()}>{{[]<>}([][])}])[<(([]<>){<>()})([{}()]<[]{}>)>({
<<[[({<<({<{[({}[])<()<>>][[(){}]{[]()}]}[{<(){}>[<>[]]}{(<>[]>({}<>)}]><<([[]<>]({}))<{[]{}}(<><>
<(({([<<(({[{[[]<>]({}())}[{<><>}[[]()]]][(<{}()>)[{{}{}}[{}{}]]]}<{(<()<>>{()<>})<[()[]][{}{}]>}[[(<>[])
[<{<({{(({(<[{(){}})(<<>()><<>()>)>)<{<({}<>)[{}[]]>(<()()>(<><>))}{([<>[]][()<>])[{[]()}{[]}]}>}))<(
[<{<[[(<([<[<<{}<>>>{[<>[]]({}())}>{[(<>{})[(){}]]{[<>[]]}}>][{<{<[]()>{{}{}}}[[[]<>]<(){}>]>([<<><>>[{}{}
<<(({[<<{<([[{[]()}{<>()}]<[[][]]>][(((){}){<>{}})([{}()]<[]<>>)])>}>([[<<(<[]<>><()()>)<[()(
{<({[{<<<{[({<()[]>}[<<>()>[(){}]])<[(<>())<[][]>](<[]()>[{}<>])>]((<(<>{}){<>{}}>{{[]()}([])})[<{{}
[<([(<[{(<([<(<>[])[()()]>{[()<>][<><>]}]<[{{}[])]((<>())[[][]])>)>({(({<><>}({}[]))<{{}<>}([]<>)>)
{<[[({<[([{<{({}()){{}{}>}>{{[()[]]}[(<>[])<{}()>]}}])[[<(([<>]<{}<>>)([{}]{<>}))<({<><>}([]<>))<[<>[]][
{(<[[([[([{{[((){})(()<>)]}}[<(([]<>){{}{}})>{<[[]{}]{()()}>({{}()}[<>])}]][[(<[[]<>]<<><>>>
<{[<{{((<({{{[()[]]([]<>)}({{}<>}<{}()>)}(((()<>)[[]{}])[([]<>)<{}()>])]([[<()[]><<>()>]]<{{[][]}[[]]}>))>)<<
{([([<<((([(<([]<>){[][]}><{[]()}<<>{}>>)(<[<>()]([]<>)>[[[][]]<<>()>])]))(((<{<[]()><()<>>}([[]][()[]])>)<(
{<[(<<((<<({{[{}[]][<>[]]}[{{}<>}((){})]}<([()<>][{}[]])<{<>[]}({}())>>)(<[<(){}>]{{()[]}<()()>}>(<<()[]>
<(<{[({{<<{[({<>()}{<><>})(<{}{}>(<><>))]{<<{}<>>({}{})>}}[({(<>{})({}{})}{<{}{}><{}{}>})]}[<(
{<[([<(([([<(<()[]>(<>[]))[[()][<>()]]>[[{{}{}}(()())][<(){}><{}<>>]]]<((<()>((){}))<{{}{}}
((({{(({{[<[<{<>}<[]{}>>{([]{})<{}<>>}](<{<>{}}<<>()>><{(){}}[[]()]>)>({<([][]}<<>{}>><<{}()><{}{}>>}
[([{[<[[({[(<{<>[]}[{}[]]>(({}[]}{[]<>}))[<{<>{}}{<>()}>(([]<>))]][<(([])<{}()>)[<{}<>>{[]<
([<{(<[<[{([<[()[]]<()<>>>]{[{[]{}}<<>()>]}){[[[<><>][<>[]]]<({}[])<[]()>>](([[][]]<{}()>)[[<>{}]{()[]}])}}{
[[[([{{(({<[[<<>{}>(<>())][((){})]]]{((<()<>>{[][]})((<>())(()<>))){<<<><>>(<>[])>}}}(({<([]
({{[(([<(<([<{(){}}({}())><[{}<>]>]{(<<>{}><(){}>)})>){[<[<(()()}[<>{}]>((<><>)<<>()>)][([{}{}]){(<>[])[[
{([{((({(((([(()<>){[]()}](<{}[]>[{}{}]))<((()()){{}{}})[<[]()>]>)[((<<>[]>[()[]])(<{}<>><<
([[{[[[{<{[{{([]<>){()[]}}}]{({(()<>)({}{})}[<{}()>{[]{}}])<{[<>{}]}>}}>}]]]{<[({(<[<([]()){
((({[<<<{{(([({}<>)<[]<>>][{{}{}}])){([[()<>]]({(){}}[<><>]))((<()()>[{}<>]){{()<>}(()[])})}}<({((
<<<(<{(<([<{[[{}{}]]<{(){}}({}())>}[<<{}()><{}{}>><(()()){()<>}>]>]]>{<[([<<()<>>((){})>][[[()[]]<{}[]>]<(<>{
([{<[[[[[[{(<<()<>>([][]>>)({{()[]}<[][]>}((()()){()[]}))}((([{}<>]([]<>)){<<>()>{[][]}}))]<<[[[[]()][[][]]
{<<[[{{[([<<<{()<>}>[((){}){[]{}}]><({<>[]}[{}])([[][]]({}{}))>>[(([[]<>]){<()<>>[<>{}]}){<<(){}>({}{})>[{
<{(<(<([(<<([<[]<>>(<>{})][([]())<[][]>])([[<>()](()[])])>({(<{}<>>([][]))}[{[[][]]{(){}]}]
{[({{{[([{<((({}()){(){}})((<>[]){<>{}}))<<([][]){{}[]})[{{}{}}{{}[]}]>>}[{[[[<>[]]([]{})]<[{}()]<<><>>>](<((
[[{[(({<[[[({((){})<()<>>}{{<>{}}[()()]}){[([]{})([]<>)][{{}}<<>()>])][{([{}<>]({}())){([]{}){
((<[{(({<{<(((<>)<{}[]>)([<>[]]{()()}))>}>}[[<[{({[]{}}{[][]})}(<{{}[]}(()[])>(({}()){<>[]}))]<<([{}][()[]])>
<(({<[[<{{<<<[{}{}]{(){}}>{<()<>>[<>{}]}>>}[[{[([][])(()())]<[()<>]>}{<<(){}>[()[]]>([[]<>]<[][]>)}][{
<{((<({{({<<{([]<>]{<><>}}((<>[]){<><>})>>{<{[{}{}][<>[]]}>({{<><>}[[]]}<{(){}}[<>{}]>)}})}})<((<{[(<<()(
[(({({{([([{{<[]<>>([][])}<[()<>]>}{(({}())[<>()])}]<({{{}{}}}({{}<>}<[]{}>))>)(<[[<()>((){})]
"""


class Species(Enum):
    parenO = 0
    parenC = 1
    bracketO = 2
    bracketC = 3
    curlyO = 4
    curlyC = 5
    angleO = 6
    angleC = 7


class Status(Enum):
    Complete = 0
    Incomplete = 1
    Corrupted = 2


open_species = [
    Species.parenO,
    Species.bracketO,
    Species.curlyO,
    Species.angleO
]

closed_species = [
    Species.parenC,
    Species.bracketC,
    Species.curlyC,
    Species.angleC
]

species_match = {
    "(": Species.parenO,
    ")": Species.parenC,
    "[": Species.bracketO,
    "]": Species.bracketC,
    "{": Species.curlyO,
    "}": Species.curlyC,
    "<": Species.angleO,
    ">": Species.angleC
}

culprit_reward = {
    Species.parenC: 3,
    Species.bracketC: 57,
    Species.curlyC: 1197,
    Species.angleC: 25137
}


def species_matching(species1, species2):
    return species1.name.split(".")[-1][0:-1] == species2.name.split(".")[-1][0:-1]


def reverse_match(species):
    return [x for x in species_match if species_match[x] == species][0]


class Line:
    lines = []

    def __init__(self, string):
        self.line_string = string
        self.line_data = [species_match[x] for x in self.line_string]
        self.status = Status.Complete
        self.culprit = ''

        to_be_closed = []
        for species in self.line_data:
            break_later = False
            if species in open_species:
                to_be_closed.append(species)
            else:
                if (to_be_closed[-1] in open_species) and species_matching(to_be_closed[-1], species):
                    to_be_closed.pop(-1)
                else:
                    self.status = Status.Corrupted
                    self.culprit = species
                    break_later = True

            # print(f"{reverse_match(species)} : {''.join([reverse_match(x) for x in to_be_closed])}")

            if break_later:
                break

        if self.status != Status.Corrupted:
            if len(to_be_closed) == 0:
                self.status = Status.Complete
            else:
                self.status = Status.Incomplete

        # print(f"{self.status} {len(to_be_closed)} : {''.join([reverse_match(x) for x in to_be_closed])}")


for x in raw_data_string.strip().split("\n"):
    Line.lines.append(Line(x))

total = 0
for x in Line.lines:
    if x.status == Status.Corrupted:
        total += culprit_reward[x.culprit]
print(total)
