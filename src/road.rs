pub enum RoadType {
    Input,
    Output,
    NoSignalIntersection,
    SignalIntersection,
    PedestrianCrossing,
    Roundabout,
    MergeLane,
    ExitRamp,
    HighwaySegment,
    SchoolZone,
    ConstructionZone,
    TollBooth,
}

pub struct RoadCollisionInfo {
    pub rule_breaker_penalty: u32,
    pub base_risk: u32
}


pub struct Road {
    pub road_type: RoadType
}

impl Road {
    pub fn new(road_type: RoadType) -> Self {
        Road { road_type }
    }
}