macro_rules! gen_json_schema {
    ($name:ty) => {
        let dest_path =
            std::path::Path::new(&"schemas").join(&format!("{}.json", stringify!($name)));
        std::fs::write(
            &dest_path,
            serde_json::to_string_pretty(&schemars::schema_for!($name)).unwrap(),
        )
        .unwrap();
    };
}

pub fn export_schemas() {
    let schema_path = std::path::Path::new(&"schemas");
    if !schema_path.exists() {
        std::fs::create_dir(schema_path).unwrap();
    }
    gen_json_schema!(cml_cip36::CIP36Delegation);
    gen_json_schema!(cml_cip36::CIP36DelegationDistribution);
    gen_json_schema!(cml_cip36::CIP36DeregistrationCbor);
    gen_json_schema!(cml_cip36::CIP36DeregistrationWitness);
    gen_json_schema!(cml_cip36::CIP36KeyDeregistration);
    gen_json_schema!(cml_cip36::CIP36KeyRegistration);
    gen_json_schema!(cml_cip36::CIP36RegistrationCbor);
    gen_json_schema!(cml_cip36::CIP36RegistrationWitness);
}
