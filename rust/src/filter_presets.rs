//! Authored recipes for the shared two-stage Filters. These use the existing
//! models, including their drive/resonance behaviour; no SSI2140 clone is implied.
use serde_json::{Value, json};

pub fn factory() -> Vec<Value> {
    [
        ("silk","Silk · HP → Ladder","radias","high_pass",0.12,0.08,0.12,"ladder_lp24","low_pass",0.78,0.22,0.2),
        ("vowel","Vowel · BP → LP","sem","band_pass",0.58,0.45,0.2,"radias","low_pass",0.85,0.18,0.15),
        ("acid","Acid · MS-20 pair","ms20_hp12","high_pass",0.22,0.15,0.25,"ms20_lp12","low_pass",0.62,0.5,0.2),
        ("brass","Brass · Pro → Ladder","pro_lp24","low_pass",0.75,0.2,0.3,"ladder_lp24","low_pass",0.82,0.15,0.12),
        ("hollow","Hollow · Notch → LP","linear_notch12","low_pass",0.6,0.4,0.0,"radias","low_pass",0.85,0.2,0.18),
        ("air","Air · SVF → Pro","radias","high_pass",0.18,0.1,0.1,"pro_lp24","low_pass",0.88,0.18,0.22),
    ].into_iter().map(|(id,name,m1,t1,c1,r1,d1,m2,t2,c2,r2,d2)|json!({"id":format!("factory:filter-pair:{id}:v1"),"name":name,"params":{
        "filter_routing":"serial","filter_model":m1,"vcf_type":t1,"filter":c1,"filter_resonance":r1,"filter_drive":d1,"filter_morph":if id=="vowel" {0.5}else{0.0},
        "filter2_model":m2,"vcf2_type":t2,"filter2":c2,"filter2_resonance":r2,"filter2_drive":d2,"filter2_morph":0.0
    }})).collect()
}
