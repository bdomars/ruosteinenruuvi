use macaddr::MacAddr6;
use ruuvi::Config;
use ruuvi::RuuviMessage;

#[derive(Debug)]
pub struct TagReport {
    pub name: Option<String>,
    pub data: RuuviMessage,
}

pub struct Tagger {
    pub config: Config,
}

impl Tagger {
    pub fn new(c: Config) -> Self {
        Tagger { config: c }
    }

    pub async fn tag(&self, data: RuuviMessage) -> TagReport {
        TagReport {
            name: self.lookup(data.mac()),
            data,
        }
    }

    fn lookup(&self, mac: MacAddr6) -> Option<String> {
        let tag = &self.config.tags.iter().find(|tag| tag.address == mac);

        match tag {
            Some(t) => return Some(t.name.clone()),
            None => return None,
        }
    }
}
