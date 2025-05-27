```bash
# 生成 CA 私钥和证书
mkdir ca

openssl req -x509 -newkey rsa:4096 -nodes -days 3650 \
  -keyout ca/key.pem -out ca/cert.pem \
  -subj "/C=CN/ST=Beijing/L=Beijing/O=Test CA/CN=ca.lan"


# 生成 peer0 私钥和 CSR
mkdir peer0

openssl req -newkey rsa:4096 -nodes \
  -keyout peer0/key.pem -out peer0/req.pem \
  -subj "/C=CN/ST=Beijing/L=Beijing/O=Test peer0/CN=peer0.lan" \
  -addext "subjectAltName=DNS:peer0.lan"

openssl x509 -req -in peer0/req.pem -CA ca/cert.pem -CAkey ca/key.pem \
  -CAcreateserial -out peer0/cert.pem -days 3650 \
  -extfile <(printf "subjectAltName=DNS:peer0.lan")


# 生成 peer1 私钥和 CSR
mkdir peer1

openssl req -newkey rsa:4096 -nodes \
  -keyout peer1/key.pem -out peer1/req.pem \
  -subj "/C=CN/ST=Beijing/L=Beijing/O=Test peer1/CN=peer1.lan" \
  -addext "subjectAltName=DNS:peer1.lan"

openssl x509 -req -in peer1/req.pem -CA ca/cert.pem -CAkey ca/key.pem \
  -CAcreateserial -out peer1/cert.pem -days 3650 \
  -extfile <(printf "subjectAltName=DNS:peer1.lan")


# 生成 peer2 私钥和 CSR
mkdir peer2

openssl req -newkey rsa:4096 -nodes \
  -keyout peer2/key.pem -out peer2/req.pem \
  -subj "/C=CN/ST=Beijing/L=Beijing/O=Test peer2/CN=peer2.lan" \
  -addext "subjectAltName=DNS:peer2.lan"

openssl x509 -req -in peer2/req.pem -CA ca/cert.pem -CAkey ca/key.pem \
  -CAcreateserial -out peer2/cert.pem -days 3650 \
  -extfile <(printf "subjectAltName=DNS:peer2.lan")


# 生成 peer3 私钥和 CSR
mkdir peer3

openssl req -newkey rsa:4096 -nodes \
  -keyout peer3/key.pem -out peer3/req.pem \
  -subj "/C=CN/ST=Beijing/L=Beijing/O=Test peer3/CN=peer3.lan" \
  -addext "subjectAltName=DNS:peer3.lan"

openssl x509 -req -in peer3/req.pem -CA ca/cert.pem -CAkey ca/key.pem \
  -CAcreateserial -out peer3/cert.pem -days 3650 \
  -extfile <(printf "subjectAltName=DNS:peer3.lan")


# 生成 peer4 私钥和 CSR
mkdir peer4

openssl req -newkey rsa:4096 -nodes \
  -keyout peer4/key.pem -out peer4/req.pem \
  -subj "/C=CN/ST=Beijing/L=Beijing/O=Test peer4/CN=peer4.lan" \
  -addext "subjectAltName=DNS:peer4.lan"

openssl x509 -req -in peer4/req.pem -CA ca/cert.pem -CAkey ca/key.pem \
  -CAcreateserial -out peer4/cert.pem -days 3650 \
  -extfile <(printf "subjectAltName=DNS:peer4.lan")


# 生成 peer5 私钥和 CSR
mkdir peer5

openssl req -newkey rsa:4096 -nodes \
  -keyout peer5/key.pem -out peer5/req.pem \
  -subj "/C=CN/ST=Beijing/L=Beijing/O=Test peer5/CN=peer5.lan" \
  -addext "subjectAltName=DNS:peer5.lan"

openssl x509 -req -in peer5/req.pem -CA ca/cert.pem -CAkey ca/key.pem \
  -CAcreateserial -out peer5/cert.pem -days 3650 \
  -extfile <(printf "subjectAltName=DNS:peer5.lan")



sudo vim /etc/hosts
127.0.0.1 peer0.lan
127.0.0.1 peer1.lan
127.0.0.1 peer2.lan
127.0.0.1 peer3.lan
127.0.0.1 peer4.lan
127.0.0.1 peer5.lan

```