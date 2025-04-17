use crate::road::{Road, RoadType, RoadCollisionInfo};
use crate::car::Car;
use rand::Rng;

fn get_collision_info(road_type: &RoadType) -> RoadCollisionInfo {
    match road_type {
        RoadType::NoSignalIntersection => RoadCollisionInfo {
            rule_breaker_penalty: 45,
            base_risk: 15,
        },
        RoadType::SignalIntersection => RoadCollisionInfo {
            rule_breaker_penalty: 25,
            base_risk: 5,
        },
        RoadType::PedestrianCrossing => RoadCollisionInfo {
            rule_breaker_penalty: 50,
            base_risk: 20,
        },
        RoadType::Roundabout => RoadCollisionInfo {
            rule_breaker_penalty: 30,
            base_risk: 10,
        },
        RoadType::MergeLane => RoadCollisionInfo {
            rule_breaker_penalty: 35,
            base_risk: 12,
        },
        RoadType::ExitRamp => RoadCollisionInfo {
            rule_breaker_penalty: 15,
            base_risk: 5,
        },
        RoadType::HighwaySegment => RoadCollisionInfo {
            rule_breaker_penalty: 10,
            base_risk: 3,
        },
        RoadType::SchoolZone => RoadCollisionInfo {
            rule_breaker_penalty: 60,
            base_risk: 25,
        },
        RoadType::ConstructionZone => RoadCollisionInfo {
            rule_breaker_penalty: 40,
            base_risk: 18,
        },
        RoadType::TollBooth => RoadCollisionInfo {
            rule_breaker_penalty: 20,
            base_risk: 8,
        },
        RoadType::Input | RoadType::Output => RoadCollisionInfo {
            rule_breaker_penalty: 0,
            base_risk: 0,
        },
    }
}


pub fn collision_check(car_one: &Car, car_two: &Car, road: &Road) -> bool {
    let collision_info = get_collision_info(&road.road_type);
    
    let mut rng = rand::rng();
    let value: u32 = rng.random_range(0..=100);

    let mut odds: u32 = collision_info.base_risk;

    if car_one.is_rule_breaker {
        odds += collision_info.rule_breaker_penalty;
    }
    if car_two.is_rule_breaker {
        odds += collision_info.rule_breaker_penalty;
    }

    odds > value
}