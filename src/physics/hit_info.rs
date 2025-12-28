use glam::IVec3;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HitInfo {
    pub pos: IVec3,
    pub face: HitFace,
    pub normal: IVec3,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HitFace {
    PosX,
    NegX,
    PosY,
    NegY,
    PosZ,
    NegZ,
}

impl HitFace {
    pub fn normal(&self) -> IVec3 {
        match self {
            HitFace::PosX => IVec3::new(1, 0, 0),
            HitFace::NegX => IVec3::new(-1, 0, 0),
            HitFace::PosY => IVec3::new(0, 1, 0),
            HitFace::NegY => IVec3::new(0, -1, 0),
            HitFace::PosZ => IVec3::new(0, 0, 1),
            HitFace::NegZ => IVec3::new(0, 0, -1),
        }
    }

    pub fn opposite(&self) -> HitFace {
        match self {
            HitFace::PosX => HitFace::NegX,
            HitFace::NegX => HitFace::PosX,
            HitFace::PosY => HitFace::NegY,
            HitFace::NegY => HitFace::PosY,
            HitFace::PosZ => HitFace::NegZ,
            HitFace::NegZ => HitFace::PosZ,
        }
    }
}

impl HitInfo {
    pub fn new(pos: IVec3, face: HitFace) -> Self {
        Self { pos, face, normal: face.normal() }
    }
}
