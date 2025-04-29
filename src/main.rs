use reqwest::blocking::Client;
use scraper::{Html, Selector};
use serde::{Serialize, Deserialize};
use std::{error::Error, thread, time::Duration, fs};

//the struct to hold the scraped data
#[derive(Serialize, Deserialize, Debug)]
struct TravelAdvisory {
    country: String,
    advisory_level: String,
    advisory_text: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    //the list of countries and their correct travel advisory urls
    let countries = vec![
        //////////////check
        ("Afghanistan", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Afghanistan.html"),
        ("Albania", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Albania.html"),
        ("Algeria", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Algeria.html"),
        ("Andorra", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Andorra.html"),
        ("Angola", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Angola.html"),
        ("Anguilla", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Anguilla.html"),
        ("Antarctia", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Antarctica.html"),
        ("Antigua and Barbuda", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/AntiguaandBarbuda.html"),
        ("Argentina", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Argentina.html"),
        ("Armenia", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Armenia.html"),
        ("Aruba", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Aruba.html"),
        ("Australia", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Australia.html"),
        ("Austria", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Austria.html"),
        ("Azerbaijan", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Azerbaijan.html"),
        ////////////////////////////////////check
        ("Bahamas", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Bahamas.html"),
        ("Bahrain", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Bahrain.html"),
        ("Bangladesh", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Bangladesh.html"),
        ("Barbados", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Barbados.html"),
        ("Belarus", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Belarus.html"),
        ("Belgium", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Belgium.html"),
        ("Belize", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Belize.html"),
        ("Benin", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Benin.html"),
        ("Bermuda", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Bermuda.html"),
        ("Bhutan", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Bhutan.html"),
        ("Bolivia", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Bolivia.html"),
        ("Bosnia and Herzegovina", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/BosniaandHerzegovina.html"),
        ("Botswana", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Botswana.html"),
        ("Brazil", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Brazil.html"),
        ("British Virgin Islands", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/BritishVirginIslands.html"),
        ("Brunei", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Brunei.html"),
        ("Bulgaria", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Bulgaria.html"),
        ("Burkina Faso", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/BurkinaFaso.html"),
        ("Burma", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Burma.html"),
        ("Burundi", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Burundi.html"),
        ///////////////////////////////////check
        ("Cabo Verde", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/CaboVerde.html"),
        ("Cambodia", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Cambodia.html"),
        ("Cameroon", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Cameroon.html"),
        ("Canada", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Canada.html"),
        ("Cayman Islands", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/CaymanIslands.html"),
        ("Central African Republic", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/CentralAfricanRepublic.html"),
        ("Chad", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Chad.html"),
        ("Chile", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Chile.html"),
        ("China", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/China.html"),
        ("Colombia", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Colombia.html"),
        ("Comoros", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Comoros.html"),
        ("Costa Rica", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/CostaRica.html"),
        ("Cote d’Ivoire", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/CotedIvoire.html"),
        ("Croatia", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Croatia.html"),
        ("Cuba", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Cuba.html"),
        ("Curacao", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Curacao.html"),
        ("Cyprus", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Cyprus.html"),
        ("Czech Republic", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/CzechRepublic.html"), 
        ////////////////////////////////////check
        ("Democratic Republic of the Congo", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/DemocraticRepublicoftheCongoDRC.html"),
        ("Denmark", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Denmark.html"),
        ("Djibouti", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Djibouti.html"),
        ("Dominica", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Dominica.html"),
        ("Dominican Republic", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/DominicanRepublic.html"),
        ("Ecuador", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Ecuador.html"),
        ("Egypt", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Egypt.html"),
        ("El Salvador", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/ElSalvador.html"),
        ("Equatorial Guinea", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/EquatorialGuinea.html"),
        ("Eritrea", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Eritrea.html"),
        ("Estonia", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Estonia.html"),
        ("Eswatini", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Eswatini.html"),
        ("Ethiopia", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Ethiopia.html"),
        ("Federated States of Micronesia", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/FederatedStatesOfMicronesia.html"),
        ("Fiji", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Fiji.html"), 
        ("Finland", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Finland.html"),
        ("France", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/France.html"),
        /////////////////////////////////////// check
        ("Gabon", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Gabon.html"),
        ("Gambia", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/TheGambia.html"),
        ("Georgia", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Georgia.html"),
        ("Germany", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Germany.html"),
        ("Ghana", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Ghana.html"),
        ("Greece", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Greece.html"),
        ("Greenland", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Greenland.html"), 
        ("Grenada", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Grenada.html"),
        ("Guatemala", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Guatemala.html"),
        ("Guinea", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Guinea.html"),
        ("Guinea-Bissau", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Guinea-Bissau.html"),
        ("Guyana", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Guyana.html"),
        ///////////////////////check
        ("Haiti", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Haiti.html"),
        ("Honduras", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Honduras.html"),
        ("Hungary", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Hungary.html"),
        ////////////////////////////////////////check
        ("Iceland", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Iceland.html"),
        ("India", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/India.html"),
        ("Indonesia", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Indonesia.html"),
        ("Iran", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Iran.html"),
        ("Iraq", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Iraq.html"),
        ("Ireland", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Ireland.html"),
        ("Israel", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/IsraeltheWestBankandGaza.html"),
        ("Italy", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Italy.html"),
        ("Jamaica", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Jamaica.html"),
        ("Japan", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Japan.html"),
        ("Jordan", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Jordan.html"),
        /////////////////////////// check
        ("Kazakhstan", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Kazakhstan.html"),
        ("Kenya", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Kenya.html"),
        ("Kiribati", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Kiribati.html"),
        ("Kosovo", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Kosovo.html"),
        ("Kuwait", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Kuwait.html"),
        ("Kyrgyzstan", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Kyrgyzstan.html"),
        //////////////////////////////////check
        ("Laos", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Laos.html"),
        ("Latvia", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Latvia.html"),
        ("Lebanon", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Lebanon.html"),
        ("Lesotho", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Lesotho.html"),
        ("Liberia", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Liberia.html"),
        ("Libya", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Libya.html"),
        ("Liechtenstein", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Liechtenstein.html"),
        ("Lithuania", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Lithuania.html"),
        ("Luxembourg", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Luxembourg.html"),
        //////////////////////////check
        ("Madagascar", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Madagascar.html"),
        ("Malawi", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Malawi.html"),
        ("Malaysia", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Malaysia.html"),
        ("Maldives", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Maldives.html"),
        ("Mali", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Mali.html"),
        ("Malta", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Malta.html"),
        ("Marshall Islands", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/MarshallIslands.html"),
        ("Mauritania", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Mauritania.html"),
        ("Mauritius", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Mauritius.html"),
        ("Mexico", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Mexico.html"),
        ("Moldova", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Moldova.html"),
        ("Mongolia", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Mongolia.html"),
        ("Montenegro", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Montenegro.html"),
        ("Morocco", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Morocco.html"),
        ("Mozambique", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Mozambique.html"),
        //////////////////////////check
        ("Namibia", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Namibia.html"),
        ("Nauru", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Nauru.html"),
        ("Nepal", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Nepal.html"),
        ("Netherlands", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Netherlands.html"),
        ("New Zealand", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/NewZealand.html"),
        ("Nicaragua", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Nicaragua.html"),
        ("Niger", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Niger.html"),
        ("Nigeria", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Nigeria.html"),
        ("North Korea", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/KoreaDemocraticPeoplesRepublicof.html"),
        ("North Macedonia", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Macedonia.html"),
        ("Norway", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Norway.html"),
        ////////////////////////////////check
        ("Oman", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Oman.html"),
        ("Pakistan", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Pakistan.html"),
        ("Palau", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Palau.html"),
        ("Panama", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Panama.html"),
        ("Papua New Guinea", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/PapuaNewGuinea.html"),
        ("Paraguay", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Paraguay.html"),
        ("Peru", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Peru.html"),
        ("Philippines", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Philippines.html"),
        ("Poland", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Poland.html"),
        ("Portugal", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Portugal.html"),
        //////////////////////////////////////////check
        ("Qatar", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Qatar.html"),
        ("Republic of the Congo", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/RepublicoftheCongo.html"),
        ("Romania", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Romania.html"),
        ("Russia", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/RussianFederation.html"),
        ("Rwanda", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Rwanda.html"),
        /////////////////////////////check
        ("Saint Kitts and Nevis", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/SaintKittsandNevis.html"),
        ("Saint Lucia", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/SaintLucia.html"),
        ("Saint Vincent and the Grenadines", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/SaintVincentandtheGrenadines.html"),
        ("Samoa", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Samoa.html"),
        ("Sao Tome and Principe", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/SaoTomeandPrincipe.html"),
        ("Saudi Arabia", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/SaudiArabia.html"),
        ("Senegal", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Senegal.html"),
        ("Serbia", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Serbia.html"),
        ("Seychelles", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Seychelles.html"),
        ("Sierra Leone", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/SierraLeone.html"),
        ("Singapore", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Singapore.html"),
        ("Sint Maarten", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/SintMaarten.html"),
        ("Slovakia", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Slovakia.html"),
        ("Slovenia", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Slovenia.html"),
        ("Solomon Islands", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/SolomonIslands.html"),
        ("Somalia", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Somalia.html"),
        ("South Africa", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/SouthAfrica.html"),
        ("South Korea", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/SouthKorea.html"),
        ("South Sudan", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/SouthSudan.html"),
        ("Spain", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Spain.html"),
        ("Sri Lanka", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/SriLanka.html"),
        ("Sudan", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Sudan.html"),
        ("Suriname", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Suriname.html"),
        ("Sweden", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Sweden.html"),
        ("Switzerland", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Switzerland.html"),
        ("Syria", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/SyrianArabRepublic.html"),
        /////////////////////////////////////////////
        ("Taiwan", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Taiwan.html"),
        ("Tajikistan", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Tajikistan.html"),
        ("Tanzania", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Tanzania.html"),
        ("Thailand", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Thailand.html"),
        ("Timor-Leste", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Timor-Leste.html"),
        ("Togo", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Togo.html"),
        ("Tonga", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Tonga.html"),
        ("Trinidad and Tobago", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/TrinidadandTobago.html"),
        ("Tunisia", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Tunisia.html"),
        ("Turkey", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Turkey.html"),
        ("Turkmenistan", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Turkmenistan.html"),
        ("Turks and Caicos Islands", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/TurksandCaicosIslands.html"),
        ("Tuvalu", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Tuvalu.html"),
        ///////////////////////////////////
        ("Uganda", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Uganda.html"),
        ("Ukraine", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Ukraine.html"),
        ("United Arab Emirates", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/UnitedArabEmirates.html"),
        ("United Kingdom", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/UnitedKingdom.html"),
        ("Uruguay", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Uruguay.html"),
        ("Uzbekistan", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Uzbekistan.html"),
        ("Vanuatu", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Vanuatu.html"),
        ("Venezuela", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Venezuela.html"),
        ("Vietnam", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Vietnam.html"),
        ("Yemen", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Yemen.html"),
        ("Zambia", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Zambia.html"),
        ("Zimbabwe", "https://travel.state.gov/content/travel/en/international-travel/International-Travel-Country-Information-Pages/Zimbabwe.html"),  
    ];

    //setting up the http client
    let client = Client::new();
    let mut advisories = Vec::new();

    for (country_name, url) in countries {
        println!("Scraping: {}", url);

        //making the http requests
        let res = client.get(url).send()?;
        let body = res.text()?;
        let document = Html::parse_document(&body);

        //getting the advisory level
        let advisory_div_selector = Selector::parse("div#tsg-rwd-advisories").unwrap();
        let advisory_level = document
            .select(&advisory_div_selector)
            .next()
            .and_then(|div| div.value().attr("class"))
            .map(|class| {
                if class.contains("tsg-rwd-eab-main-frame-blue") {
                    "Level 1"
                } else if class.contains("tsg-rwd-eab-main-frame-standard") {
                    "Level 2"
                } else if class.contains("tsg-rwd-eab-main-frame-orange") {
                    "Level 3"
                } else if class.contains("tsg-rwd-eab-main-frame-red") {
                    "Level 4"
                } else {
                    "Unknown Level"
                }
            })
            .unwrap_or("Unknown Level")
            .to_string();

        //advisory text extraction
        let advisory_text_selector = Selector::parse("div.tsg-rwd-accordion-copy p").unwrap();
        let advisory_text = document
            .select(&advisory_text_selector)
            .map(|e| e.text().collect::<Vec<_>>().join(" "))
            .collect::<Vec<String>>()
            .join("\n");

        //storing the data
        advisories.push(TravelAdvisory {
            country: country_name.to_string(),
            advisory_level,
            advisory_text,
        });

        thread::sleep(Duration::from_secs(2));
    }

    //making sure it is stored to a json
    let json = serde_json::to_string_pretty(&advisories)?;
    println!("{}", json);
    fs::write("advisories.json", json)?;

    println!("Scraping completed. Data saved to advisories.json");
    Ok(())
}
