use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Data {
    pub component: Vec<Component>,
}

#[derive(Debug, Deserialize)]
pub struct Component {
    pub polyline: Vec<Polyline>,
}

#[derive(Debug, Deserialize)]
pub struct Polyline {
    pub line_seg: Vec<LineSeg>,
    pub arc_seg: Option<Vec<ArcSeg>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LineSeg {
    #[serde(rename = "@x")]
    pub x: String,
    #[serde(rename = "@y")]
    pub y: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ArcSeg {
    #[serde(rename = "@x")]
    pub x: String,
    #[serde(rename = "@y")]
    pub y: String,
    #[serde(rename = "@i")]
    pub i: String,
    #[serde(rename = "@j")]
    pub j: String,
    #[serde(rename = "@cw")]
    pub cw: String,
}
