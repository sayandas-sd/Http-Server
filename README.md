## solana program

Generate keypairs, handle SPL tokens, sign/verify messages, and construct valid on-chain instructions.

## Route - 

`POST /keypair`

```bash
   
    {
        "success": true,
        "data": {
            "pubkey": "9LuLUKR3vKrHVNPcf4QheW9kyrWf2hgK88dmwUkVKPu6",
            "secret": "4kqCYXdBmuUPe1RgxGYmXi8QR1yQsbRZeo3RNStZ3kkfr9mwVjWKSySQ19jAVA8YdQkjsFSXjmPJUDwjHVCancKW"
        }
    }
```

`POST /token/create`

```bash

    REQ:

    {
        "mintAuthority": "base58-encoded-public-key",
        "mint": "base58-encoded-public-key"
        "decimals": 6
    }

    RES:

    {
    "success": true,
    "data": {
            "program_id": "string",
            "accounts": {
                pubkey: "pubkey", 
                is_signer: boolean, 
                is_writable: boolean
            }...,
            "instruction_data": "base64-encoded-data"
        }
    }

```

`POST /token/mint`

```bash
    REQ:

    {
        "mint": "mint-address",
        "destination": "destination-user-address",
        "authority": "authority-address",
        "amount": 1000000,
    }

    RES:

    {
    "success": true,
    "data": {
            "program_id": "string",
            "accounts": [
            {
                "pubkey": "pubkey",
                "is_signer": false,
                "is_writable": true
            }...,
            ],
            "instruction_data": "base64-encoded-data"
        }
    }

```

`POST /message/sign`

```bash
   
    REQ:

    {
        "message": "Hello, Solana!",
        "secret": "base58-encoded-secret-key"
    }

    RES:

    {
        "success": true,
        "data": {
            "signature": "base64-encoded-signature",
            "public_key": "base58-encoded-public-key",
            "message": "Hello, Solana!"
        }
    }

```

`POST /message/verify`

```bash
   
    REQ:

    {
        "message": "Hello, Solana!",
        "signature": "base64-encoded-signature",
        "pubkey": "base58-encoded-public-key"
    }

    RES:

    {
        "success": true,
        "data": {
            "valid": true,
            "message": "Hello, Solana!",
            "pubkey": "base58-encoded-public-key"
        }
    }

```

`POST /send/sol`

```bash
   
    REQ:

    {
        "from": "sender-address",
        "to": "recipient-address",
        "lamports": 100000,
    }


    RES:

    {
        "success": true,
        "data": {
            "program_id": "respective program id",
            "accounts": [
            "address of first account",
            "address of second account"
            ],
            "instruction_data": "instruction_data"
        }
    }

```



