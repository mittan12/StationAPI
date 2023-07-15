use fake::Dummy;

use super::{company::Company, line_symbol::LineSymbol, station::Station, train_type::TrainType};

#[derive(Dummy, Clone, Debug)]
pub struct Line {
    pub line_cd: u32,
    pub company_cd: u32,
    pub company: Option<Company>,
    pub line_name: String,
    pub line_name_k: String,
    pub line_name_h: String,
    pub line_name_r: String,
    pub line_name_zh: Option<String>,
    pub line_name_ko: Option<String>,
    pub line_color_c: String,
    pub line_color_t: String,
    pub line_type: u32,
    pub line_symbols: Vec<LineSymbol>,
    pub line_symbol_primary: Option<String>,
    pub line_symbol_secondary: Option<String>,
    pub line_symbol_extra: Option<String>,
    pub line_symbol_primary_color: Option<String>,
    pub line_symbol_secondary_color: Option<String>,
    pub line_symbol_extra_color: Option<String>,
    pub line_symbol_primary_shape: Option<String>,
    pub line_symbol_secondary_shape: Option<String>,
    pub line_symbol_extra_shape: Option<String>,
    pub lon: f64,
    pub lat: f64,
    pub zoom: u32,
    pub e_status: u32,
    pub e_sort: u32,
    pub station: Option<Station>,
    pub train_type: Option<TrainType>,
}

impl Line {
    #![allow(clippy::too_many_arguments)]
    pub fn new(
        line_cd: u32,
        company_cd: u32,
        company: Option<Company>,
        line_name: String,
        line_name_k: String,
        line_name_h: String,
        line_name_r: String,
        line_name_zh: Option<String>,
        line_name_ko: Option<String>,
        line_color_c: String,
        line_color_t: String,
        line_type: u32,
        line_symbols: Vec<LineSymbol>,
        line_symbol_primary: Option<String>,
        line_symbol_secondary: Option<String>,
        line_symbol_extra: Option<String>,
        line_symbol_primary_color: Option<String>,
        line_symbol_secondary_color: Option<String>,
        line_symbol_extra_color: Option<String>,
        line_symbol_primary_shape: Option<String>,
        line_symbol_secondary_shape: Option<String>,
        line_symbol_extra_shape: Option<String>,
        lon: f64,
        lat: f64,
        zoom: u32,
        e_status: u32,
        e_sort: u32,
        station: Option<Station>,
        train_type: Option<TrainType>,
    ) -> Self {
        Self {
            line_cd,
            company_cd,
            company,
            line_name,
            line_name_k,
            line_name_h,
            line_name_r,
            line_name_zh,
            line_name_ko,
            line_color_c,
            line_color_t,
            line_type,
            line_symbols,
            line_symbol_primary,
            line_symbol_secondary,
            line_symbol_extra,
            line_symbol_primary_color,
            line_symbol_secondary_color,
            line_symbol_extra_color,
            line_symbol_primary_shape,
            line_symbol_secondary_shape,
            line_symbol_extra_shape,
            lon,
            lat,
            zoom,
            e_status,
            e_sort,
            station,
            train_type,
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use fake::{Fake, Faker};

//     use super::Line;

//     #[test]
//     fn new() {
//         let Line {
//             line_cd,
//             company_cd,
//             company: _,
//             line_name,
//             line_name_k,
//             line_name_h,
//             line_name_r,
//             line_name_zh,
//             line_name_ko,
//             line_color_c,
//             line_color_t,
//             line_type,
//             line_symbols: _,
//             line_symbol_primary,
//             line_symbol_secondary,
//             line_symbol_extra,
//             line_symbol_primary_color,
//             line_symbol_secondary_color,
//             line_symbol_extra_color,
//             line_symbol_primary_shape,
//             line_symbol_secondary_shape,
//             line_symbol_extra_shape,
//             lon,
//             lat,
//             zoom,
//             e_status,
//             e_sort,
//             station: _,
//         } = Faker.fake();

//         let actual_station = Line::new(
//             line_cd,
//             company_cd,
//             None,
//             line_name.clone(),
//             line_name_k.clone(),
//             line_name_h.clone(),
//             line_name_r.clone(),
//             line_name_zh.clone(),
//             line_name_ko.clone(),
//             line_color_c.clone(),
//             line_color_t.clone(),
//             line_type,
//             vec![],
//             line_symbol_primary.clone(),
//             line_symbol_secondary.clone(),
//             line_symbol_extra.clone(),
//             line_symbol_primary_color.clone(),
//             line_symbol_secondary_color.clone(),
//             line_symbol_extra_color.clone(),
//             line_symbol_primary_shape.clone(),
//             line_symbol_secondary_shape.clone(),
//             line_symbol_extra_shape.clone(),
//             lon,
//             lat,
//             zoom,
//             e_status,
//             e_sort,
//             None,
//         );

//         assert_eq!(actual_station.line_cd, line_cd);
//         assert_eq!(actual_station.company_cd, company_cd);
//         // assert_eq!(actual_station.company, company);
//         assert_eq!(actual_station.line_name, line_name);
//         assert_eq!(actual_station.line_name_k, line_name_k);
//         assert_eq!(actual_station.line_name_h, line_name_h);
//         assert_eq!(actual_station.line_name_r, line_name_r);
//         assert_eq!(actual_station.line_name_zh, line_name_zh);
//         assert_eq!(actual_station.line_name_ko, line_name_ko);
//         assert_eq!(actual_station.line_color_c, line_color_c);
//         assert_eq!(actual_station.line_color_t, line_color_t);
//         assert_eq!(actual_station.line_type, line_type);
//         // assert_eq!(actual_station.line_symbols, line_symbols);
//         assert_eq!(actual_station.line_symbol_primary, line_symbol_primary);
//         assert_eq!(actual_station.line_symbol_secondary, line_symbol_secondary);
//         assert_eq!(actual_station.line_symbol_extra, line_symbol_extra);
//         assert_eq!(
//             actual_station.line_symbol_primary_color,
//             line_symbol_primary_color
//         );
//         assert_eq!(
//             actual_station.line_symbol_secondary_color,
//             line_symbol_secondary_color
//         );
//         assert_eq!(
//             actual_station.line_symbol_extra_color,
//             line_symbol_extra_color
//         );
//         assert_eq!(
//             actual_station.line_symbol_primary_shape,
//             line_symbol_primary_shape
//         );
//         assert_eq!(
//             actual_station.line_symbol_secondary_shape,
//             line_symbol_secondary_shape
//         );
//         assert_eq!(
//             actual_station.line_symbol_extra_shape,
//             line_symbol_extra_shape
//         );
//         assert_eq!(actual_station.lon, lon);
//         assert_eq!(actual_station.lat, lat);
//         assert_eq!(actual_station.zoom, zoom);
//         assert_eq!(actual_station.e_status, e_status);
//         assert_eq!(actual_station.e_sort, e_sort);
//     }
// }