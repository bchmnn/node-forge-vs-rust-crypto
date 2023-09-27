const forge = require("node-forge");

const p7 = forge.pkcs7.createEnvelopedData();
p7.content = forge.util.createBuffer("test");
p7.encrypt(undefined, undefined);
const der = forge.asn1.toDer(p7.toAsn1()).getBytes();
const der_b64 = forge.util.encode64(der);
process.stdout.write(der_b64);
