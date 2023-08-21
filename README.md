# cepd

cepd is a small and fast caching proxy-server for CEP records (brazilian postalcode).

API: `GET /q/:postalcode`

Example:

```
curl -sf http://localhost:3000/q/01311200 | jq
{
  "zip": "01311-200",
  "address": "Avenida Paulista",
  "complement": "de 1047 a 1865 - lado ímpar",
  "neighborhood": "Bela Vista",
  "city": "São Paulo",
  "state_initials": "SP"
}
```
