use lm_sensors::{FeatureRef, value::Kind};

pub fn get_sub_feature(feature: &FeatureRef, kind: Kind) -> Option<f64> {
    feature
        .sub_feature_by_kind(kind)
        .iter()
        .map(|sub_feature| sub_feature.value().unwrap().raw_value())
        .next()
}
