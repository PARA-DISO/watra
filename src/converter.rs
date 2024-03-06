use regex::Regex;

// use std::collections::HashMap;
type HashMap<K, V> = indexmap::IndexMap<K, V, fnv::FnvBuildHasher>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
pub enum EraFormat {
    // 元号を漢字、1年を元年と表記
    Kanji,
    // 元号をローマ字イニシャル、1年を元年と表記
    Initial,
}
impl std::string::ToString for EraFormat {
    fn to_string(&self) -> String {
        match self {
            Self::Initial => "initial".to_string(),
            Self::Kanji => "kanji".to_string(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EraNames {
    Meiji,
    Taisho,
    Showa,
    Heisei,
    Reiwa,
}
impl EraNames {
    pub fn into_u32(self) -> u32 {
        match self {
            EraNames::Meiji => 1868,
            EraNames::Taisho => 1912,
            EraNames::Showa => 1926,
            EraNames::Heisei => 1989,
            EraNames::Reiwa => 2019,
        }
    }
    pub fn into_string_with_format(self, format: EraFormat) -> String {
        match format {
            EraFormat::Initial => match self {
                EraNames::Meiji => "M".to_string(),
                EraNames::Taisho => "T".to_string(),
                EraNames::Showa => "S".to_string(),
                EraNames::Heisei => "H".to_string(),
                EraNames::Reiwa => "R".to_string(),
            },
            EraFormat::Kanji => match self {
                EraNames::Meiji => "明治".to_string(),
                EraNames::Taisho => "大正".to_string(),
                EraNames::Showa => "昭和".to_string(),
                EraNames::Heisei => "平成".to_string(),
                EraNames::Reiwa => "令和".to_string(),
            },
        }
    }
}
impl From<&str> for EraNames {
    fn from(s: &str) -> Self {
        match s {
            "明治" | "M" | "m" => EraNames::Meiji,
            "大正" | "T" | "t" => EraNames::Taisho,
            "昭和" | "S" | "s" => EraNames::Showa,
            "平成" | "H" | "h" => EraNames::Heisei,
            "令和" | "R" | "r" => EraNames::Reiwa,
            _ => panic!("{} is not a valid era name", s),
        }
    }
}

static WAREKI_MAP: std::sync::OnceLock<HashMap<u32, EraNames>> = std::sync::OnceLock::new();
fn init_wareki_map() {
    WAREKI_MAP.get_or_init(|| {
        let mut m = HashMap::with_capacity_and_hasher(5, Default::default());
        m.insert(1868, EraNames::Meiji);
        m.insert(1912, EraNames::Taisho);
        m.insert(1926, EraNames::Showa);
        m.insert(1989, EraNames::Heisei);
        m.insert(2019, EraNames::Reiwa);
        m
    });
}
// Parse to Era from year
pub fn western_to_japanese(year: u32) -> Result<(EraNames, u32), String> {
    init_wareki_map();
    let mut era: Option<(EraNames, u32)> = None;
    for (&y, j) in WAREKI_MAP.get().unwrap().iter() {
        if y <= year {
            era = Some((*j, year - y + 1));
        } else {
            break;
        }
    }
    if let Some(era) = era {
        Ok((era.0, era.1))
    } else {
        Err(format!("{} can not convert to Japanese Calender", year))
    }
}

pub fn cvt_era_string(era_name: EraNames, era_number: u32, format: EraFormat) -> String {
    let is_kanji = format == EraFormat::Kanji;
    format!(
        "{}{}{}",
        era_name.into_string_with_format(format),
        if era_number == 1 && is_kanji {
            String::from("元")
        } else {
            era_number.to_string()
        },
        if is_kanji { "年" } else { "" }
    )
}
pub fn japanese_to_western(era: impl AsRef<str>) -> Result<u32, String> {
    let re = Regex::new(r"([明治大正昭和平成令]+|[a-zA-Z]+)(\d+|元)(.*)").unwrap();
    let era = era.as_ref().trim();
    if let Some(caps) = re.captures(era) {
        let era = EraNames::from(&caps[1]).into_u32();
        let year = if &caps[2] == "元" {
            1
        } else {
            caps[2].parse::<u32>().unwrap()
        };
        if !(&caps[1].is_empty()) && &caps[1] != "年" {
            Err(format!("{} is invalid format.", era))
        } else {
            Ok(era + year - 1)
        }
    } else {
        Err(format!("{} can not convert to Western Calender", era))
    }
}
// Tests
#[cfg(test)]
mod test {
    use crate::converter::*;
    use rstest::rstest;
    #[test]
    fn w_to_j() {
        assert_eq!(western_to_japanese(1868).unwrap(), (EraNames::Meiji, 1));
        assert_eq!(western_to_japanese(1880).unwrap(), (EraNames::Meiji, 13));
        assert_eq!(western_to_japanese(1912).unwrap(), (EraNames::Taisho, 1));
        assert_eq!(western_to_japanese(1926).unwrap(), (EraNames::Showa, 1));
        assert_eq!(western_to_japanese(1989).unwrap(), (EraNames::Heisei, 1));
        assert_eq!(western_to_japanese(2019).unwrap(), (EraNames::Reiwa, 1));
        assert_eq!(western_to_japanese(2024).unwrap(), (EraNames::Reiwa, 6));
        assert!(western_to_japanese(1867).is_err());
    }
    #[test]
    fn j_to_w() {
        assert_eq!(japanese_to_western("明治1年").unwrap(), 1868);
        assert_eq!(japanese_to_western("M13年").unwrap(), 1880);
        assert_eq!(japanese_to_western("大正元年").unwrap(), 1912);
        assert_eq!(japanese_to_western("昭和1年").unwrap(), 1926);
        assert_eq!(japanese_to_western("H1").unwrap(), 1989);
        assert_eq!(japanese_to_western("R元年").unwrap(), 2019);
        assert_eq!(japanese_to_western("令和6").unwrap(), 2024);
        assert!(japanese_to_western("ほげ5年").is_err());
    }
    #[rstest]
    #[case((EraNames::Meiji, 1, EraFormat::Kanji), "明治元年".to_string())]
    #[case((EraNames::Meiji, 1, EraFormat::Initial), "M1".to_string())]
    #[case((EraNames::Reiwa, 5, EraFormat::Initial), "R5".to_string())]
    #[case((EraNames::Heisei, 30, EraFormat::Initial), "H30".to_string())]
    fn era_string_check(#[case] inputs: (EraNames, u32, EraFormat), #[case] expected: String) {
        assert_eq!(cvt_era_string(inputs.0, inputs.1, inputs.2), expected);
    }
}
