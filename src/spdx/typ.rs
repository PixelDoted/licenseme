use super::SPDXFind;

// ---- List -----

#[derive(serde::Deserialize, serde::Serialize)]
pub struct SPDXList {
    #[serde(rename = "licenseListVersion")]
    pub version: String,

    pub licenses: Vec<SPDXListedLicense>,
}

impl SPDXList {
    pub fn find(&self, id: &str) -> SPDXFind {
        let mut ids = Vec::new();

        for l in &self.licenses {
            if l.license_id == id {
                return SPDXFind::Exact(&l.license_id);
            }

            ids.push(SPDXId::new(&l.name, id, &l.license_id));
        }

        ids.sort_by(|a, b| a.dist.cmp(&b.dist));
        SPDXFind::Closest(ids)
    }
}

// ---- Listed License ----

#[derive(serde::Deserialize, serde::Serialize)]
pub struct SPDXListedLicense {
    #[serde(rename = "isDeprecatedLicenseId")]
    pub is_deprecated_license_id: bool,

    #[serde(rename = "detailsUrl")]
    pub details_url: String,

    pub name: String,

    #[serde(rename = "licenseId")]
    pub license_id: String,
}

// ---- ID ----

pub struct SPDXId<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub dist: usize,
}

impl<'a> SPDXId<'a> {
    pub fn new(name: &'a str, id: &str, other: &'a str) -> Self {
        let dist = lev(id, other);
        Self {
            name,
            id: other,
            dist,
        }
    }
}

fn lev(a: &str, b: &str) -> usize {
    let al = a.len();
    let bl = b.len();
    if bl == 0 {
        return al;
    }

    if al == 0 {
        return bl;
    }

    if a[0..1] == b[0..1] {
        return lev(&a[1..], &b[1..]);
    }

    let l0 = lev(&a[1..], b);
    let l1 = lev(a, &b[1..]);
    let l2 = lev(&a[1..], &b[1..]);
    1 + l0.min(l1).min(l2)
}
