use rust_big_int::BigInt;
use std::str::FromStr;

#[test]
fn catalan_number() {
    let mut calculated_numbers: Vec<BigInt> = vec![1.into()];

    for _i in 1..100 {
        let number_pairs = calculated_numbers
            .iter()
            .zip(calculated_numbers.iter().rev());

        let result: BigInt = number_pairs.fold(BigInt::default(), |result, (x, y)| {
            x.clone() * y.clone() + result
        });

        calculated_numbers.push(result);
    }

    let catalan_numbers: Vec<BigInt> = vec![
        BigInt::from_str("1").unwrap(),
        BigInt::from_str("1").unwrap(),
        BigInt::from_str("2").unwrap(),
        BigInt::from_str("5").unwrap(),
        BigInt::from_str("14").unwrap(),
        BigInt::from_str("42").unwrap(),
        BigInt::from_str("132").unwrap(),
        BigInt::from_str("429").unwrap(),
        BigInt::from_str("1430").unwrap(),
        BigInt::from_str("4862").unwrap(),
        BigInt::from_str("16796").unwrap(),
        BigInt::from_str("58786").unwrap(),
        BigInt::from_str("208012").unwrap(),
        BigInt::from_str("742900").unwrap(),
        BigInt::from_str("2674440").unwrap(),
        BigInt::from_str("9694845").unwrap(),
        BigInt::from_str("35357670").unwrap(),
        BigInt::from_str("129644790").unwrap(),
        BigInt::from_str("477638700").unwrap(),
        BigInt::from_str("1767263190").unwrap(),
        BigInt::from_str("6564120420").unwrap(),
        BigInt::from_str("24466267020").unwrap(),
        BigInt::from_str("91482563640").unwrap(),
        BigInt::from_str("343059613650").unwrap(),
        BigInt::from_str("1289904147324").unwrap(),
        BigInt::from_str("4861946401452").unwrap(),
        BigInt::from_str("18367353072152").unwrap(),
        BigInt::from_str("69533550916004").unwrap(),
        BigInt::from_str("263747951750360").unwrap(),
        BigInt::from_str("1002242216651368").unwrap(),
        BigInt::from_str("3814986502092304").unwrap(),
        BigInt::from_str("14544636039226909").unwrap(),
        BigInt::from_str("55534064877048198").unwrap(),
        BigInt::from_str("212336130412243110").unwrap(),
        BigInt::from_str("812944042149730764").unwrap(),
        BigInt::from_str("3116285494907301262").unwrap(),
        BigInt::from_str("11959798385860453492").unwrap(),
        BigInt::from_str("45950804324621742364").unwrap(),
        BigInt::from_str("176733862787006701400").unwrap(),
        BigInt::from_str("680425371729975800390").unwrap(),
        BigInt::from_str("2622127042276492108820").unwrap(),
        BigInt::from_str("10113918591637898134020").unwrap(),
        BigInt::from_str("39044429911904443959240").unwrap(),
        BigInt::from_str("150853479205085351660700").unwrap(),
        BigInt::from_str("583300119592996693088040").unwrap(),
        BigInt::from_str("2257117854077248073253720").unwrap(),
        BigInt::from_str("8740328711533173390046320").unwrap(),
        BigInt::from_str("33868773757191046886429490").unwrap(),
        BigInt::from_str("131327898242169365477991900").unwrap(),
        BigInt::from_str("509552245179617138054608572").unwrap(),
        BigInt::from_str("1978261657756160653623774456").unwrap(),
        BigInt::from_str("7684785670514316385230816156").unwrap(),
        BigInt::from_str("29869166945772625950142417512").unwrap(),
        BigInt::from_str("116157871455782434250553845880").unwrap(),
        BigInt::from_str("451959718027953471447609509424").unwrap(),
        BigInt::from_str("1759414616608818870992479875972").unwrap(),
        BigInt::from_str("6852456927844873497549658464312").unwrap(),
        BigInt::from_str("26700952856774851904245220912664").unwrap(),
        BigInt::from_str("104088460289122304033498318812080").unwrap(),
        BigInt::from_str("405944995127576985730643443367112").unwrap(),
        BigInt::from_str("1583850964596120042686772779038896").unwrap(),
        BigInt::from_str("6182127958584855650487080847216336").unwrap(),
        BigInt::from_str("24139737743045626825711458546273312").unwrap(),
        BigInt::from_str("94295850558771979787935384946380125").unwrap(),
        BigInt::from_str("368479169875816659479009042713546950").unwrap(),
        BigInt::from_str("1440418573150919668872489894243865350").unwrap(),
        BigInt::from_str("5632681584560312734993915705849145100").unwrap(),
        BigInt::from_str("22033725021956517463358552614056949950").unwrap(),
        BigInt::from_str("86218923998960285726185640663701108500").unwrap(),
        BigInt::from_str("337485502510215975556783793455058624700").unwrap(),
        BigInt::from_str("1321422108420282270489942177190229544600").unwrap(),
        BigInt::from_str("5175569924646105559418940193995065716350").unwrap(),
        BigInt::from_str("20276890389709399862928998568254641025700").unwrap(),
        BigInt::from_str("79463489365077377841208237632349268884500").unwrap(),
        BigInt::from_str("311496878311103321137536291518809134027240").unwrap(),
        BigInt::from_str("1221395654430378811828760722007962130791020").unwrap(),
        BigInt::from_str("4790408930363303911328386208394864461024520").unwrap(),
        BigInt::from_str("18793142726809884575211361279087545193250040").unwrap(),
        BigInt::from_str("73745243611532458459690151854647329239335600").unwrap(),
        BigInt::from_str("289450081175264899454283846029490767264392230").unwrap(),
        BigInt::from_str("1136359577947336271931632877004667456667613940").unwrap(),
        BigInt::from_str("4462290049988320482463241297506133183499654740").unwrap(),
        BigInt::from_str("17526585015616776834735140517915655636396234280").unwrap(),
        BigInt::from_str("68854441132780194707888052034668647142985206100").unwrap(),
        BigInt::from_str("270557451039395118028642463289168566420671280440").unwrap(),
        BigInt::from_str("1063353702922273835973036658043476458723103404520").unwrap(),
        BigInt::from_str("4180080073556524734514695828170907458428751314320").unwrap(),
        BigInt::from_str("16435314834665426797069144960762886143367590394940").unwrap(),
        BigInt::from_str("64633260585762914370496637486146181462681535261000").unwrap(),
        BigInt::from_str("254224158304000796523953440778841647086547372026600").unwrap(),
        BigInt::from_str("1000134600800354781929399250536541864362461089950800").unwrap(),
        BigInt::from_str("3935312233584004685417853572763349509774031680023800").unwrap(),
        BigInt::from_str("15487357822491889407128326963778343232013931127835600").unwrap(),
        BigInt::from_str("60960876535340415751462563580829648891969728907438000").unwrap(),
        BigInt::from_str("239993345518077005168915776623476723006280827488229600").unwrap(),
        BigInt::from_str("944973797977428207852605870454939596837230758234904050").unwrap(),
        BigInt::from_str("3721443204405954385563870541379246659709506697378694300").unwrap(),
        BigInt::from_str("14657929356129575437016877846657032761712954950899755100").unwrap(),
        BigInt::from_str("57743358069601357782187700608042856334020731624756611000").unwrap(),
        BigInt::from_str("227508830794229349661819540395688853956041682601541047340").unwrap(),
        BigInt::from_str("896519947090131496687170070074100632420837521538745909320").unwrap(),
    ];

    let result_check = calculated_numbers.iter().zip(catalan_numbers.iter());

    for result in result_check {
        assert_eq!(result.0, result.1);
    }
}
