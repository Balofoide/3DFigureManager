import requests

url = "https://sandbox.melhorenvio.com.br/api/v2/me/cart"

payload = {
    "service": 2,  # ID do serviço (ex: 1 = PAC)
    "agency": None,
    "from": {
        "postal_code": "01001000",
        "name": "Loja Exemplo",
        "phone": "1133334444",
        "email": "loja@exemplo.com",
        "document": "12345678909",  # CPF/CNPJ
        "address": "Rua Principal, 123",
        "complement": "Sala 4",
        "number": "123",
        "district": "Centro",
        "city": "São Paulo",
        "state_abbr": "SP",
        "country_id": "BR"
    },
    "to": {
        "postal_code": "02002000",
        "name": "Cliente Exemplo",
        "phone": "1199998888",
        "email": "cliente@exemplo.com",
        "document": "98765432100",  # CPF/CNPJ
        "address": "Avenida Secundária, 456",
        "complement": "Ap 102",
        "number": "456",
        "district": "Jardim Paulista",
        "city": "São Paulo",
        "state_abbr": "SP",
        "country_id": "BR"
    },
    "volumes": [
        {
            "height": 20,
            "width": 15,
            "length": 10,
            "weight": 1.5
        }
    ],
    "options": {
        "insurance_value": 100.00,
        "receipt": True,
        "own_hand": False
    }
}

headers = {
    "Accept": "application/json",
    "Content-Type": "application/json",
    "Authorization": "Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9.eyJhdWQiOiI5NTYiLCJqdGkiOiI3Mjc4YzkyMWI2MzFhMDg4NGMyMzVmNDA2YzZjZjk1NjBjNDUxNTBiMjRhZTNiY2RmNTRhZDYwNmJlMjkwNDFlMGFkNGRmYWZmMDI2ODQ1MiIsImlhdCI6MTc0NzgzNzY0NS42OTA2NzcsIm5iZiI6MTc0NzgzNzY0NS42OTA2OCwiZXhwIjoxNzc5MzczNjQ1LjY4MTI0LCJzdWIiOiI5ZWY2ZTRjMS1kMDU1LTQxZWYtOGVjYy1hOGE5NTY4OGFkOTYiLCJzY29wZXMiOlsiY2FydC1yZWFkIiwiY2FydC13cml0ZSIsImNvbXBhbmllcy1yZWFkIiwiY29tcGFuaWVzLXdyaXRlIiwiY291cG9ucy1yZWFkIiwiY291cG9ucy13cml0ZSIsIm5vdGlmaWNhdGlvbnMtcmVhZCIsIm9yZGVycy1yZWFkIiwicHJvZHVjdHMtcmVhZCIsInByb2R1Y3RzLWRlc3Ryb3kiLCJwcm9kdWN0cy13cml0ZSIsInB1cmNoYXNlcy1yZWFkIiwic2hpcHBpbmctY2FsY3VsYXRlIiwic2hpcHBpbmctY2FuY2VsIiwic2hpcHBpbmctY2hlY2tvdXQiLCJzaGlwcGluZy1jb21wYW5pZXMiLCJzaGlwcGluZy1nZW5lcmF0ZSIsInNoaXBwaW5nLXByZXZpZXciLCJzaGlwcGluZy1wcmludCIsInNoaXBwaW5nLXNoYXJlIiwic2hpcHBpbmctdHJhY2tpbmciLCJlY29tbWVyY2Utc2hpcHBpbmciLCJ0cmFuc2FjdGlvbnMtcmVhZCIsInVzZXJzLXJlYWQiLCJ1c2Vycy13cml0ZSIsIndlYmhvb2tzLXJlYWQiLCJ3ZWJob29rcy13cml0ZSIsIndlYmhvb2tzLWRlbGV0ZSIsInRkZWFsZXItd2ViaG9vayJdfQ.Lq-VVG6jWsjYoE_ryQq4vq2Z5cgOPPXBV-zrz3VMe2eH1Ge0nxRndRtIEoMTb_JEpFSEkvIYdPlkiGlkDTnw9fZwY3rtezfurQ7L2vrw0S4RJ_MSnuCBd43plN3sJ3cvqQ6T4cv-eAtXZNChm6z2H9zS9hwCETKBUSy378vk2PuSsbJuVCeWKLxF7P4vjBnNqmjYTCErOQOulmUUKQ3t1sPr53q_9xTEhcZk98ZefyQ9lFrvb43HtO_T5ESk1d4_frwKZMbYvlPj-eNlyhVhrwUWR176XklJYHR6Y6sHyFtaOnCWn9UqlRJGNYLfSgBAEWGu9eRENdwpguTaAJuj8BC0owSZscFOne4aDPUgBSNTLhviLMcfJoUFersDlyURIxh-wdeIXIkOVcHR2JNJnZGGFNnhqqD67y_X16TWx2wMbS5XMd_JuOSqs3LPDfy21fdgouEJ2kkUS-4M24KANs_Fy0VmXdC3fBjZPM2SCkSqZg0WY-i4Gt8425LamXx_W5MeTCD64YjE1dNN7IBgpUmy0ONR3VXhKccB4PHEO90vS2CaJZO-u2O-VSbbWetYPpetPGCTQ6z86LEhz1oiF8myQEbX4CJK2pFYTpwxOxqqmbFCFYaGskXnbxaIU-Fv6PcGCsSjdk7oigrL1Z98QXG9Dz1MaoV4Cfi_s6AmSWY",
    "User-Agent": "Minha Loja (suporte@minhaloja.com)"
}

response = requests.post(url, json=payload, headers=headers)

print(f"Status Code: {response.status_code}")
print("Resposta:")
print(response.text)