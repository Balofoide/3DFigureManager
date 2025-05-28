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
    "Authorization": "Bearer <token>",
    "User-Agent": "Minha Loja (suporte@minhaloja.com)"
}

response = requests.post(url, json=payload, headers=headers)

print(f"Status Code: {response.status_code}")
print("Resposta:")
print(response.text)