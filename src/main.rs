use base64::{engine::general_purpose, Engine as _};
use cms::cert::x509::der::Decode;
use cms::content_info::ContentInfo;
use cms::enveloped_data::EnvelopedData;
use der::Encode;
use openssl::pkcs7::Pkcs7;
use std::fs;

fn main() {
    let der: String = fs::read_to_string("der").unwrap();

    let der = &general_purpose::STANDARD.decode(der).unwrap();
    let ci = ContentInfo::from_der(der.as_slice()).unwrap();

    assert_eq!(ci.content_type, const_oid::db::rfc5911::ID_ENVELOPED_DATA);

    let bytes = ci.content.to_der().unwrap();
    let ed = EnvelopedData::from_der(&bytes);

    match ed {
        Ok(_) => println!("Won't happen"),
        Err(err) => println!("{:?}", err),
    }

    // fix enveloped data format by forth- and back-parsing using openssl

    let p7 = Pkcs7::from_der(der.as_slice()).unwrap();
    let der = p7.to_der().unwrap();

    fs::write("der_fixed", &general_purpose::STANDARD.encode(der.as_slice())).unwrap();

    let ci = ContentInfo::from_der(der.as_slice()).unwrap();

    assert_eq!(ci.content_type, const_oid::db::rfc5911::ID_ENVELOPED_DATA);

    let bytes = ci.content.to_der().unwrap();

    let ed = EnvelopedData::from_der(&bytes);

    match ed {
        Ok(res) => println!("{:?}", res),
        Err(_) => println!("Won't happen"),
    }
}
