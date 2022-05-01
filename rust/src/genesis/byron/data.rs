use crate::crypto::BlockHeaderHash;

pub fn get_test_genesis_data(genesis_prev: &BlockHeaderHash) -> Result<&str, BlockHeaderHash> {
    if genesis_prev
        == &BlockHeaderHash::from_hex("5f20df933584822601f9e3f8c024eb5eb252fe8cefb24d1317dc3d432e940ebb")
            .unwrap()
    {
        Ok(include_str!(
            "./test_data/5f20df933584822601f9e3f8c024eb5eb252fe8cefb24d1317dc3d432e940ebb.json"
        ))
    } else if genesis_prev
        == &BlockHeaderHash::from_hex("b7f76950bc4866423538ab7764fc1c7020b24a5f717a5bee3109ff2796567214")
            .unwrap()
    {
        Ok(include_str!(
            "./test_data/b7f76950bc4866423538ab7764fc1c7020b24a5f717a5bee3109ff2796567214.json"
        ))
    } else if genesis_prev
        == &BlockHeaderHash::from_hex("c6a004d3d178f600cd8caa10abbebe1549bef878f0665aea2903472d5abf7323")
            .unwrap()
    {
        Ok(include_str!(
            "./test_data/c6a004d3d178f600cd8caa10abbebe1549bef878f0665aea2903472d5abf7323.json"
        ))
    } else if genesis_prev
        == &BlockHeaderHash::from_hex("96fceff972c2c06bd3bb5243c39215333be6d56aaf4823073dca31afe5038471")
            .unwrap()
    {
        Ok(include_str!(
            "./test_data/96fceff972c2c06bd3bb5243c39215333be6d56aaf4823073dca31afe5038471.json"
        ))
    } else {
        Err(genesis_prev.clone())
    }
}
