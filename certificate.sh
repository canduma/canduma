#!/usr/bin/env bash

_authority="Canduma Labs"
_name="rust.localhost"

_cert_pem=certs/${_name}.pem
_cert_crt=certs/${_name}.crt
_cert_key=certs/${_name}.key

_key=/tmp/${_name}.key
_cn=${_name}
_csr_cnf=/tmp/${_name}.csr.cnf
_csr=/tmp/${_name}.csr
_v3_ext=/tmp/${_name}.v3.ext

    cat << EOF > ${_csr_cnf}
[req]
default_bits = 2048
prompt = no
default_md = sha256
distinguished_name = dn

[dn]
O=${_authority}
CN=${_name}
EOF

    cat << EOF > ${_v3_ext}
authorityKeyIdentifier=keyid,issuer
basicConstraints=CA:FALSE
keyUsage = digitalSignature, nonRepudiation, keyEncipherment, dataEncipherment
subjectAltName = @alt_names

[alt_names]
DNS.1 = ${_name}
DNS.2 = *.${_name}
EOF
mkdir -p certs
openssl genrsa -out ${_key} 2048
openssl req -x509 -new -nodes -key ${_key} -sha256 -days 3650 -out ${_cert_pem} -subj "/O=${_authority}/CN=${_cn}"
openssl req -new -sha256 -nodes -out ${_csr} -newkey rsa:2048 -keyout ${_cert_key} -config <( cat ${_csr_cnf} )
openssl x509 -req -in ${_csr} -CA ${_cert_pem} -CAcreateserial -CAkey ${_key} -out ${_cert_crt} -days 3650 -sha256 -extfile ${_v3_ext}
rm certs/*.srl