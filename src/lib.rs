use pyo3::prelude::*;
use mpl_token_metadata::accounts::Metadata;
use pyo3::types::PyString;
use std::collections::HashMap;
use mpl_token_metadata::types::{UseMethod, CollectionDetails, ProgrammableConfig};
use serde::Serialize;

#[pyclass]
#[pyo3(get_all)]
#[derive(Clone, Serialize)]
struct Creator{
    address: String,
    verified: bool,
    share: u8,
}

#[pymethods]
impl Creator{
    #[new]
    pub fn new(address: &str, verified: bool, share: u8) -> Self{
        Creator{address: address.to_string(), verified, share}
    }
}

#[pyclass]
#[pyo3(get_all)]
#[derive(Clone, Serialize)]
struct MyMetadata{
    key: String,
    update_authority: String,
    mint: String,
    name: String,
    symbol: String,
    uri: String,
    seller_fee_basis_points: u16,
    creators: Vec<Creator>,
    primary_sale_happened: bool,
    is_mutable: bool,
    edition_nonce: Option<u8>,
    token_standard: Option<String>,
    collection: HashMap<String, String>,
    uses: HashMap<String, String>,
    collection_details: HashMap<String, String>,
    programmable_config: HashMap<String, String>
}

#[pymethods]
impl MyMetadata{
    #[new]
    #[pyo3(signature = (key,
            update_authority,
            mint,
            name,
            symbol,
            uri,
            seller_fee_basis_points,
            creators,
            primary_sale_happened,
            is_mutable,
            edition_nonce,
            token_standard,
            collection,
            uses,
            collection_details,
            programmable_config))]
    pub fn new(
            key: String,
            update_authority: String,
            mint: String,
            name: String,
            symbol: String,
            uri: String,
            seller_fee_basis_points: u16,
            creators: Vec<Creator>,
            primary_sale_happened: bool,
            is_mutable: bool,
            edition_nonce: Option<u8>,
            token_standard: Option<String>,
            collection: HashMap<String, String>,
            uses: HashMap<String, String>,
            collection_details: HashMap<String, String>,
            programmable_config: HashMap<String, String>
        ) -> Self{
        MyMetadata {
            key,
            update_authority,
            mint,
            name,
            symbol,
            uri,
            seller_fee_basis_points,
            creators,
            primary_sale_happened,
            is_mutable,
            edition_nonce,
            token_standard,
            collection,
            uses,
            collection_details,
            programmable_config

        }
    }
    fn __str__(&self) -> PyResult<String>   {
        let serialized_data = serde_json::to_string(&self).unwrap();
        Ok(format!("MyMetadata{{
            {:?}
        }}", serialized_data))
    }
}



#[pyfunction]
fn read_meta(bytes_data: &[u8]) -> PyResult<MyMetadata>{
    let metadata = match Metadata::from_bytes(bytes_data){
        Ok(data) => data,
        Err(_) => {
                return Err(PyErr::new::<PyString, _>("Connot read meta!"));
            }
    };
    
    let key = metadata.key;
    let mut creators: Vec<Creator> = vec![];
    if let Some(cs) = metadata.creators{
        for c in cs{
            creators.push(Creator{
                address: c.address.to_string(),
                verified: c.verified,
                share: c.share
            }); 
        }
    }
    let edition_nonce = metadata.edition_nonce;
    let token_standard = match metadata.token_standard{
        Some(x) => Some((format!("{x:?}")).to_string()),
        None => None
    };
    let mut collection: HashMap<String, String> = HashMap::new();
    if let Some(col) = metadata.collection{
        collection.insert("key".to_string(), col.key.to_string());
        collection.insert("verified".to_string(), col.verified.to_string());
    };
    let mut uses :HashMap<String, String> = HashMap::new();
    if let Some(us) = metadata.uses{
        match us.use_method{
            UseMethod::Burn => {
                uses.insert("use_method".to_string(), "Burn".to_string());
            },
            UseMethod::Multiple => {
                uses.insert("use_method".to_string(), "Multiple".to_string());
            },
            UseMethod::Single => {
                uses.insert("use_method".to_string(), "Single".to_string());
            }
        }
        uses.insert("remaining".to_string(), us.remaining.to_string());
        uses.insert("total".to_string(), us.total.to_string());
    };
    let mut collection_details :HashMap<String, String> = HashMap::new();
    if let Some(opt) = metadata.collection_details{
        match opt{
            CollectionDetails::V1{size} => {
                collection_details.insert("label".to_string(), "V1".to_string());
                collection_details.insert("size".to_string(), size.to_string());
            },
        }
    };
    let mut programmable_config :HashMap<String, String> = HashMap::new();
    if let Some(prgcf) = metadata.programmable_config{
        match prgcf{
            ProgrammableConfig::V1{rule_set} => {
                    if let Some(x) = rule_set{
                        programmable_config.insert("label".to_string(), "V1".to_string());
                        programmable_config.insert("rule_set".to_string(), x.to_string());
                    }
                },
        }
    };
    Ok(MyMetadata{
        key: (format!("{key:?}")).to_string(),
        update_authority: metadata.update_authority.to_string(),
        mint: metadata.mint.to_string(),
        name: metadata.name,
        symbol: metadata.symbol,
        uri: metadata.uri,
        seller_fee_basis_points: metadata.seller_fee_basis_points,
        creators: creators,
        primary_sale_happened: metadata.primary_sale_happened,
        is_mutable: metadata.is_mutable,
        edition_nonce,
        token_standard,
        collection,
        uses,
        collection_details,
        programmable_config
    })
}

#[pymodule]
fn meta_read(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Creator>()?;
    m.add_class::<MyMetadata>()?;
    m.add_function(wrap_pyfunction!(read_meta, m)?)?;
    Ok(())
}
