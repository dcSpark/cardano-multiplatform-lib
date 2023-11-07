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
    gen_json_schema!(cml_cip36::Delegation);
    gen_json_schema!(cml_cip36::DelegationDistribution);
    gen_json_schema!(cml_cip36::DeregistrationCbor);
    gen_json_schema!(cml_cip36::DeregistrationWitness);
    gen_json_schema!(cml_cip36::KeyDeregistration);
    gen_json_schema!(cml_cip36::KeyRegistration);
    gen_json_schema!(cml_cip36::RegistrationCbor);
    gen_json_schema!(cml_cip36::RegistrationWitness);
}
