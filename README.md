# cepd

cepd is a small and fast caching proxy-server for CEP records (brazilian zipcode).

API: `GET /q/:cep`

Example:

```
curl -sf http://localhost:3000/q/01311200 | jq
{
  "ibge": "3550308",
  "gia": "1004",
  "zip": "01311-200",
  "address": "Avenida Paulista",
  "complement": "de 1047 a 1865 - lado ímpar",
  "neighborhood": "Bela Vista",
  "city": "São Paulo",
  "state_initials": "SP"
}
```
