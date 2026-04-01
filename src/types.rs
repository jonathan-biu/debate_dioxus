#[derive(Clone, Debug, Default, PartialEq)]
pub struct Speech {
    pub speaker: String,
    pub speech: String,
    pub rebuttal: String,
    pub poi: String,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Debate {
    pub id: String,
    pub motion: String,
    pub infoslide: String,
    pub pm:  Speech,
    pub lo:  Speech,
    pub dpm: Speech,
    pub dlo: Speech,
    pub mg:  Speech,
    pub mo:  Speech,
    pub gw:  Speech,
    pub ow:  Speech,
}

impl Debate {
    pub fn get_speech(&self, role: &str) -> &Speech {
        match role {
            "PM"  => &self.pm,
            "LO"  => &self.lo,
            "DPM" => &self.dpm,
            "DLO" => &self.dlo,
            "MG"  => &self.mg,
            "MO"  => &self.mo,
            "GW"  => &self.gw,
            "OW"  => &self.ow,
            _     => &self.pm,
        }
    }
}

pub const SPEAKER_ORDER: &[&str] = &["PM","LO","DPM","DLO","MG","MO","GW","OW"];
