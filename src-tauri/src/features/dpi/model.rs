use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct DpiStages {
    pub active: u8,
    pub stages: Vec<DpiStage>,
}

impl From<(u8, Vec<(u16, u16)>)> for DpiStages {
    fn from((active, stages): (u8, Vec<(u16, u16)>)) -> Self {
        Self {
            active,
            stages: stages.into_iter().map(|(x, y)| DpiStage { x, y }).collect(),
        }
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct DpiStage {
    pub x: u16,
    pub y: u16,
}

impl From<Vec<i32>> for DpiStage {
    fn from(dpi: Vec<i32>) -> Self {
        Self {
            x: dpi[0] as u16,
            y: dpi[1] as u16,
        }
    }
}

impl From<u16> for DpiStage {
    fn from(dpi: u16) -> Self {
        Self { x: dpi, y: dpi }
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct DeviceDpi {
    pub dpi: DpiStage,
    pub stages: DpiStages,
    pub max_dpi: i32,
}
