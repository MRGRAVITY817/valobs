use serde::{Deserialize, Serialize};

/// A value object representing a country.
/// The format is ISO 3166-1 alpha-3, which is a three-letter country code.
/// For example, the country code for the United States is "USA".
///
/// For more info, see [ISO 3166-1 alpha-3](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-3).
///
/// ## What is a _Country_?
///
/// A country is a region that is identified as a distinct national entity in political geography.
///
/// A country may be an independent sovereign state or one that is occupied by another state, as a non-sovereign or formerly sovereign political division, or a geographic region associated with sets of previously independent or differently associated people with distinct political characteristics.
///
/// ## When to use
///
/// Use this type when you want to represent a country.
/// For example, when you want user to pick a country from a list from their profile.
///
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Copy)]
pub enum Country {
    /// Aruba
    ABW,
    /// Afghanistan
    AFG,
    /// Angola
    AGO,
    /// Anguilla
    AIA,
    /// Åland Islands
    ALA,
    /// Albania
    ALB,
    /// Andorra
    AND,
    /// United Arab Emirates
    ARE,
    /// Argentina
    ARG,
    /// Armenia
    ARM,
    /// American Samoa
    ASM,
    /// Antarctica
    ATA,
    /// French Southern Territories
    ATF,
    /// Antigua and Barbuda
    ATG,
    /// Australia
    AUS,
    /// Austria
    AUT,
    /// Azerbaijan
    AZE,
    /// Burundi
    BDI,
    /// Belgium
    BEL,
    /// Benin
    BEN,
    /// Bonaire, Sint Eustatius and Saba
    BES,
    /// Burkina Faso
    BFA,
    /// Bangladesh
    BGD,
    /// Bulgaria
    BGR,
    /// Bahrain
    BHR,
    /// Bahamas
    BHS,
    /// Bosnia and Herzegovina
    BIH,
    /// Saint Barthélemy
    BLM,
    /// Belarus
    BLR,
    /// Belize
    BLZ,
    /// Bermuda
    BMU,
    /// Bolivia (Plurinational State of)
    BOL,
    /// Brazil
    BRA,
    /// Barbados
    BRB,
    /// Brunei Darussalam
    BRN,
    /// Bhutan
    BTN,
    /// Bouvet Island
    BVT,
    /// Botswana
    BWA,
    /// Central African Republic
    CAF,
    /// Canada
    CAN,
    /// Cocos (Keeling) Islands
    CCK,
    /// Switzerland
    CHE,
    /// Chile
    CHL,
    /// China
    CHN,
    /// Côte d'Ivoire
    CIV,
    /// Cameroon
    CMR,
    /// Congo, Democratic Republic of the
    COD,
    /// Congo
    COG,
    /// Cook Islands
    COK,
    /// Colombia
    COL,
    /// Comoros
    COM,
    /// Cabo Verde
    CPV,
    /// Costa Rica
    CRI,
    /// Cuba
    CUB,
    /// Curaçao
    CUW,
    /// Christmas Island
    CXR,
    /// Cayman Islands
    CYM,
    /// Cyprus
    CYP,
    /// Czechia
    CZE,
    /// Germany
    DEU,
    /// Djibouti
    DJI,
    /// Dominica
    DMA,
    /// Denmark
    DNK,
    /// Dominican Republic
    DOM,
    /// Algeria
    DZA,
    /// Ecuador
    ECU,
    /// Egypt
    EGY,
    /// Eritrea
    ERI,
    /// Western Sahara
    ESH,
    /// Spain
    ESP,
    /// Estonia
    EST,
    /// Ethiopia
    ETH,
    /// Finland
    FIN,
    /// Fiji
    FJI,
    /// Falkland Islands (Malvinas)
    FLK,
    /// France
    FRA,
    /// Faroe Islands
    FRO,
    /// Micronesia (Federated States of)
    FSM,
    /// Gabon
    GAB,
    /// United Kingdom of Great Britain and Northern Ireland
    GBR,
    /// Georgia
    GEO,
    /// Guernsey
    GGY,
    /// Ghana
    GHA,
    /// Gibraltar
    GIB,
    /// Guinea
    GIN,
    /// Guadeloupe
    GLP,
    /// Gambia
    GMB,
    /// Guinea-Bissau
    GNB,
    /// Equatorial Guinea
    GNQ,
    /// Greece
    GRC,
    /// Grenada
    GRD,
    /// Greenland
    GRL,
    /// Guatemala
    GTM,
    /// French Guiana
    GUF,
    /// Guam
    GUM,
    /// Guyana
    GUY,
    /// Hong Kong
    HKG,
    /// Heard Island and McDonald Islands
    HMD,
    /// Honduras
    HND,
    /// Croatia
    HRV,
    /// Haiti
    HTI,
    /// Hungary
    HUN,
    /// Indonesia
    IDN,
    /// Isle of Man
    IMN,
    /// India
    IND,
    /// British Indian Ocean Territory
    IOT,
    /// Ireland
    IRL,
    /// Iran (Islamic Republic of)
    IRN,
    /// Iraq
    IRQ,
    /// Iceland
    ISL,
    /// Israel
    ISR,
    /// Italy
    ITA,
    /// Jamaica
    JAM,
    /// Jersey
    JEY,
    /// Jordan
    JOR,
    /// Japan
    JPN,
    /// Kazakhstan
    KAZ,
    /// Kenya
    KEN,
    /// Kyrgyzstan
    KGZ,
    /// Cambodia
    KHM,
    /// Kiribati
    KIR,
    /// Saint Kitts and Nevis
    KNA,
    /// Korea, Republic of
    KOR,
    /// Kuwait
    KWT,
    /// Lao People's Democratic Republic
    LAO,
    /// Lebanon
    LBN,
    /// Liberia
    LBR,
    /// Libya
    LBY,
    /// Saint Lucia
    LCA,
    /// Liechtenstein
    LIE,
    /// Sri Lanka
    LKA,
    /// Lesotho
    LSO,
    /// Lithuania
    LTU,
    /// Luxembourg
    LUX,
    /// Latvia
    LVA,
    /// Macao
    MAC,
    /// Saint Martin (French part)
    MAF,
    /// Morocco
    MAR,
    /// Monaco
    MCO,
    /// Moldova, Republic of
    MDA,
    /// Madagascar
    MDG,
    /// Maldives
    MDV,
    /// Mexico
    MEX,
    /// Marshall Islands
    MHL,
    /// North Macedonia
    MKD,
    /// Mali
    MLI,
    /// Malta
    MLT,
    /// Myanmar
    MMR,
    /// Montenegro
    MNE,
    /// Mongolia
    MNG,
    /// Northern Mariana Islands
    MNP,
    /// Mozambique
    MOZ,
    /// Mauritania
    MRT,
    /// Montserrat
    MSR,
    /// Martinique
    MTQ,
    /// Mauritius
    MUS,
    /// Malawi
    MWI,
    /// Malaysia
    MYS,
    /// Mayotte
    MYT,
    /// Namibia
    NAM,
    /// New Caledonia
    NCL,
    /// Niger
    NER,
    /// Norfolk Island
    NFK,
    /// Nigeria
    NGA,
    /// Nicaragua
    NIC,
    /// Niue
    NIU,
    /// Netherlands, Kingdom of the
    NLD,
    /// Norway
    NOR,
    /// Nepal
    NPL,
    /// Nauru
    NRU,
    /// New Zealand
    NZL,
    /// Oman
    OMN,
    /// Pakistan
    PAK,
    /// Panama
    PAN,
    /// Pitcairn
    PCN,
    /// Peru
    PER,
    /// Philippines
    PHL,
    /// Palau
    PLW,
    /// Papua New Guinea
    PNG,
    /// Poland
    POL,
    /// Puerto Rico
    PRI,
    /// Korea (Democratic People's Republic of)
    PRK,
    /// Portugal
    PRT,
    /// Paraguay
    PRY,
    /// Palestine, State of
    PSE,
    /// French Polynesia
    PYF,
    /// Qatar
    QAT,
    /// Réunion
    REU,
    /// Romania
    ROU,
    /// Russian Federation
    RUS,
    /// Rwanda
    RWA,
    /// Saudi Arabia
    SAU,
    /// Sudan
    SDN,
    /// Senegal
    SEN,
    /// Singapore
    SGP,
    /// South Georgia and the South Sandwich Islands
    SGS,
    /// Saint Helena, Ascension and Tristan da Cunha
    SHN,
    /// Svalbard and Jan Mayen
    SJM,
    /// Solomon Islands
    SLB,
    /// Sierra Leone
    SLE,
    /// El Salvador
    SLV,
    /// San Marino
    SMR,
    /// Somalia
    SOM,
    /// Saint Pierre and Miquelon
    SPM,
    /// Serbia
    SRB,
    /// South Sudan
    SSD,
    /// Sao Tome and Principe
    STP,
    /// Suriname
    SUR,
    /// Slovakia
    SVK,
    /// Slovenia
    SVN,
    /// Sweden
    SWE,
    /// Eswatini
    SWZ,
    /// Sint Maarten (Dutch part)
    SXM,
    /// Seychelles
    SYC,
    /// Syrian Arab Republic
    SYR,
    /// Turks and Caicos Islands
    TCA,
    /// Chad
    TCD,
    /// Togo
    TGO,
    /// Thailand
    THA,
    /// Tajikistan
    TJK,
    /// Tokelau
    TKL,
    /// Turkmenistan
    TKM,
    /// Timor-Leste
    TLS,
    /// Tonga
    TON,
    /// Trinidad and Tobago
    TTO,
    /// Tunisia
    TUN,
    /// Türkiye
    TUR,
    /// Tuvalu
    TUV,
    /// Taiwan, Province of China
    TWN,
    /// Tanzania, United Republic of
    TZA,
    /// Uganda
    UGA,
    /// Ukraine
    UKR,
    /// United States Minor Outlying Islands
    UMI,
    /// Uruguay
    URY,
    /// United States of America
    USA,
    /// Uzbekistan
    UZB,
    /// Holy See
    VAT,
    /// Saint Vincent and the Grenadines
    VCT,
    /// Venezuela (Bolivarian Republic of)
    VEN,
    /// Virgin Islands (British)
    VGB,
    /// Virgin Islands (U.S.)
    VIR,
    /// Viet Nam
    VNM,
    /// Vanuatu
    VUT,
    /// Wallis and Futuna
    WLF,
    /// Samoa
    WSM,
    /// Yemen
    YEM,
    /// South Africa
    ZAF,
    /// Zambia
    ZMB,
    /// Zimbabwe
    ZWE,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_country_with_valid_data() {
        assert_eq!(Country::USA, Country::USA);
    }

    #[test]
    fn equality_between_two_country_values() {
        // Arrange
        let country1 = Country::USA;
        let country2 = Country::USA;

        // Act
        let result = country1 == country2;

        // Assert
        assert!(result);
    }

    #[test]
    fn inequality_between_two_country_values() {
        // Arrange
        let country1 = Country::USA;
        let country2 = Country::CAN;

        // Act
        let result = country1 == country2;

        // Assert
        assert!(!result);
    }

    #[test]
    fn serialize_country_to_json() {
        // Arrange
        let country = Country::USA;

        // Act
        let result = serde_json::to_string(&country).unwrap();

        // Assert
        assert_eq!(result, "\"USA\"");
    }

    #[test]
    fn deserialize_country_from_json() {
        // Arrange
        let country = Country::USA;

        // Act
        let result = serde_json::from_str::<Country>("\"USA\"").unwrap();

        // Assert
        assert_eq!(result, country);
    }
}
