use json;

fn main() {
    let parsed = json::parse(
        r#"
{
    "code": 200,
    "success": true,
    "payload": {
        "features": [
            "awesome",
            "easyAPI",
            "lowLearningCurve"
        ]
    }
}
"#,
    )
    .unwrap();

    let instantiated = json::object! {
        // quotes on keys are optional
        "code": 200,
        success: true,
        payload: {
            features: [
                "awesome",
                "easyAPI",
                "lowLearningCurve"
            ]
        }
    };

    assert_eq!(parsed, instantiated);

    println!("{}", parsed);
    println!("{}", instantiated);
}
