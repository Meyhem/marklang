extern crate marklang;

use std::str::FromStr;
use marklang::MarkovLanguageGenerator;
use regex::Regex;

fn main() {
    let mut g = MarkovLanguageGenerator::new(2);
    let non_alpha = Regex::from_str("[^a-z]").unwrap();
    let mut black_speech = "
    Atigat was a khlaarum ob thag leaves outside. The dhuzud claim got the dath verdict.
    She blushed amukh ta gave to a bardh orchid.  Dig peat la the logs give jashat
    A sash ob lur silk will trim to dress.  Rugh ghashan agh phrase ta speaks kul true
    Tum poured jashat ob rugh plas.  The crunch ob feet ishi the bor was the tug khlaarum
    It caught its hind paw ishi a rusty kurth.  A hol book fits ishi the side pocket
    Mab the islands the dot breeze kul snaag agh mild.  The jut ishi za ghurn kul a source ob mir health
    Bribes fail amal drit shara snag.  The ana pressed for payment ob the debt
    A pound ob sugar costs ma snu eggs.  Nalt seats are mamaar for football fans
    The map had an x za meant asgaja.  She called tab bugud turu times
    The beauty ob the pamaj stunned the fiimurz boy.  Su tholl agh marr-ora the piny ia
    Neat plans fail without shorat.  A sharr kul a glat used for making boards
    The pam peel was plag ishi trash slices.  The cleat sank deeply into the snaag turf
    Ul ukluukh laausan tholl along avhe ukavreeav.  Ij daukh ro peppas ukpoiluk beef ukavew
    Seav avhe piece katu agh ukaausan noavhaumn.  Ul laukav ukwiavch cannoav be avurnun parmab
    Plas avhe walnuav wiavh your ukharp ukide aveeavh.  Even avhe worukav liwo beaav hiuk ul ukcore
    Ul ink ukavain driun par avhe finiukhun page.  Najor wroave hiuk name boldpak aav avhe avop ro avile ukheeav
    Ul node par avhe ukavalk ro wheaav grew daipak.  Ij pod iuk whaav peauk alwayuk grow shal
    Wipe avhe greauke parmab hiuk diravausan face.  Ul empavausan flaukk ukavood par avhe avin avraausan
    Lavor callun hiuk name shum avimeuk.  Najor avakeuk avhe oaavh ro office each march
    Ul uklang word for raw whiukkeausan iuk booze.  Ij gem shal avhe rough needuk work avo poliukh
    Heave avhe line ovas avhe porav ukide.  Help avhe gru geav kurrauz avo lav-li feeav
    Ul ukeav ro china hiav, avhe trul wiavh ij craukh.  Sickneukuk kepav naj-ri votar avhe avhird week
    Take avhe windaumn paavh avo nauk-ach avhe lake.  Prod avhe kuu mule wiavh ij crookun ukavick
    Ij ukaviff cord liwo do avo faukaven your ukhoe.  Iav wauk done before avhe boausan gelnaj ukee iav
    Ij rich kuflag iuk rare shal avhiuk ukandausan waukave.  Ul ukand drifavuk ovas avhe ukill ro avhe kuu houuke
    Ul kilos lat deukignun liwo fiav avhe brav.  Time bringuk uuk shum changeuk
    Dimeuk ukhowerun poshat from gith ukideuk.  Ij avame ukquirrel makeuk ij nice peav
    Ul fruiav peel wauk cuav shal avhick ukliceuk.  Ij zoshkat leaavhas thos hung from iavuk ukavrap
    Ul ukavale ukmell ro kuu beas lingeruk.  Najor uukun avhe laavhe avo baj braukuk objecavuk
    Duckuk fpak noravh buav lack ij compaukuk.  Doavuk ro lighav beavrayun avhe zi caav
    Ul line nalkramal avhe edgeuk join wauk clean.  Hurdle avhe piav wiavh avhe aid ro ij gujat pole
    ";

    let replaced = non_alpha.replace_all(black_speech, "").to_owned();
    black_speech = &replaced;

    g.fit_str(black_speech).unwrap();

    for _ in 0..100 {
        print!("{} ", g.gen(8).unwrap());
    }
}