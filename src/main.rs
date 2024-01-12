mod borze;
mod brm;
mod sportaktiv;

static SEP: &str = ";";
static WEEK_DAYS: [&str; 7] = [
    "hétfő",
    "kedd",
    "szerda",
    "csütörtök",
    "péntek",
    "szombat",
    "vasárnap",
];

fn main() {
    brm::scrape();
    borze::scrape();
    sportaktiv::scrape();
}
