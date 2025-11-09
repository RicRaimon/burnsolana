/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/burnsolana.json`.
 */
export type Burnsolana = {
  "address": "burnKNMLEUJ7ENqn3ASwSAKaxdwz7bXq9cDVhM72iDa",
  "metadata": {
    "name": "burnsolana",
    "version": "0.1.0",
    "spec": "0.1.0",
    "description": "Created with Anchor"
  },
  "instructions": [
    {
      "name": "burnSol",
      "discriminator": [
        159,
        187,
        77,
        201,
        116,
        73,
        86,
        88
      ],
      "accounts": [
        {
          "name": "user",
          "writable": true,
          "signer": true
        },
        {
          "name": "burnAccount",
          "writable": true,
          "address": "1nc1nerator11111111111111111111111111111111"
        },
        {
          "name": "userWsolAta",
          "writable": true,
          "optional": true
        },
        {
          "name": "userStatsSol",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  117,
                  115,
                  101,
                  114,
                  95,
                  115,
                  116,
                  97,
                  116,
                  115
                ]
              },
              {
                "kind": "account",
                "path": "user"
              },
              {
                "kind": "const",
                "value": [
                  115,
                  111,
                  108
                ]
              }
            ]
          }
        },
        {
          "name": "feeVault",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  102,
                  101,
                  101,
                  95,
                  118,
                  97,
                  117,
                  108,
                  116
                ]
              }
            ]
          }
        },
        {
          "name": "tokenProgram"
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "lamports",
          "type": "u64"
        },
        {
          "name": "message",
          "type": "string"
        }
      ]
    },
    {
      "name": "burnSpl",
      "discriminator": [
        63,
        32,
        117,
        249,
        87,
        212,
        116,
        68
      ],
      "accounts": [
        {
          "name": "user",
          "writable": true,
          "signer": true
        },
        {
          "name": "tokenAccount",
          "writable": true
        },
        {
          "name": "mint",
          "writable": true
        },
        {
          "name": "userStatsSpl",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  117,
                  115,
                  101,
                  114,
                  95,
                  115,
                  116,
                  97,
                  116,
                  115
                ]
              },
              {
                "kind": "account",
                "path": "user"
              },
              {
                "kind": "account",
                "path": "mint"
              }
            ]
          }
        },
        {
          "name": "feeVault",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  102,
                  101,
                  101,
                  95,
                  118,
                  97,
                  117,
                  108,
                  116
                ]
              }
            ]
          }
        },
        {
          "name": "tokenProgram"
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "amount",
          "type": "u64"
        },
        {
          "name": "message",
          "type": "string"
        }
      ]
    },
    {
      "name": "initializeFeeVault",
      "discriminator": [
        185,
        140,
        228,
        234,
        79,
        203,
        252,
        50
      ],
      "accounts": [
        {
          "name": "payer",
          "writable": true,
          "signer": true
        },
        {
          "name": "feeVault",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  102,
                  101,
                  101,
                  95,
                  118,
                  97,
                  117,
                  108,
                  116
                ]
              }
            ]
          }
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": []
    },
    {
      "name": "initializeUserStatsSol",
      "discriminator": [
        133,
        57,
        62,
        108,
        158,
        85,
        143,
        187
      ],
      "accounts": [
        {
          "name": "userStatsSol",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  117,
                  115,
                  101,
                  114,
                  95,
                  115,
                  116,
                  97,
                  116,
                  115
                ]
              },
              {
                "kind": "account",
                "path": "user"
              },
              {
                "kind": "const",
                "value": [
                  115,
                  111,
                  108
                ]
              }
            ]
          }
        },
        {
          "name": "user",
          "writable": true,
          "signer": true
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": []
    },
    {
      "name": "initializeUserStatsSpl",
      "discriminator": [
        247,
        37,
        48,
        140,
        254,
        139,
        209,
        131
      ],
      "accounts": [
        {
          "name": "userStatsSpl",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  117,
                  115,
                  101,
                  114,
                  95,
                  115,
                  116,
                  97,
                  116,
                  115
                ]
              },
              {
                "kind": "account",
                "path": "user"
              },
              {
                "kind": "account",
                "path": "mint"
              }
            ]
          }
        },
        {
          "name": "user",
          "writable": true,
          "signer": true
        },
        {
          "name": "mint"
        },
        {
          "name": "tokenProgram"
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": []
    },
    {
      "name": "withdrawFee",
      "discriminator": [
        14,
        122,
        231,
        218,
        31,
        238,
        223,
        150
      ],
      "accounts": [
        {
          "name": "admin",
          "writable": true,
          "signer": true
        },
        {
          "name": "feeVault",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  102,
                  101,
                  101,
                  95,
                  118,
                  97,
                  117,
                  108,
                  116
                ]
              }
            ]
          }
        },
        {
          "name": "to",
          "writable": true
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        },
        {
          "name": "rent",
          "address": "SysvarRent111111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "lamports",
          "type": "u64"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "feeVault",
      "discriminator": [
        192,
        178,
        69,
        232,
        58,
        149,
        157,
        132
      ]
    },
    {
      "name": "userStats",
      "discriminator": [
        176,
        223,
        136,
        27,
        122,
        79,
        32,
        227
      ]
    }
  ],
  "events": [
    {
      "name": "burnEvent",
      "discriminator": [
        33,
        89,
        47,
        117,
        82,
        124,
        238,
        250
      ]
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "mintNotInitialezed",
      "msg": "Mint not initialezed"
    },
    {
      "code": 6001,
      "name": "tokenAccountFrozen",
      "msg": "Token account is frozen"
    },
    {
      "code": 6002,
      "name": "mintAuthorityPresent",
      "msg": "Mint authority is present; supply is mutable"
    },
    {
      "code": 6003,
      "name": "mintMismatch",
      "msg": "Mint mismatch between token account and provided mint"
    },
    {
      "code": 6004,
      "name": "notAccountOwner",
      "msg": "The provided signer is not the owner of the token account"
    },
    {
      "code": 6005,
      "name": "amountMustBePositive",
      "msg": "Amount must be positive"
    },
    {
      "code": 6006,
      "name": "insufficientFunds",
      "msg": "Insufficient funds in token account"
    },
    {
      "code": 6007,
      "name": "wrongTokenProgram",
      "msg": "Wrong Token Program for provided mint/token account"
    },
    {
      "code": 6008,
      "name": "overflow",
      "msg": "Arithmetic overflow"
    },
    {
      "code": 6009,
      "name": "messageTooLong",
      "msg": "Message too long (max 256 bytes)"
    },
    {
      "code": 6010,
      "name": "badFeeRecipient",
      "msg": "Bad fee recipient passed by client"
    },
    {
      "code": 6011,
      "name": "missingFeeAccounts",
      "msg": "Missing fee accounts by client"
    },
    {
      "code": 6012,
      "name": "unauthorized",
      "msg": "Unauthorized admin"
    }
  ],
  "types": [
    {
      "name": "burnEvent",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "user",
            "type": "pubkey"
          },
          {
            "name": "mint",
            "type": "pubkey"
          },
          {
            "name": "amount",
            "type": "u64"
          },
          {
            "name": "message",
            "type": "string"
          },
          {
            "name": "timestamp",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "feeVault",
      "type": {
        "kind": "struct",
        "fields": []
      }
    },
    {
      "name": "userStats",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "user",
            "type": "pubkey"
          },
          {
            "name": "mint",
            "type": "pubkey"
          },
          {
            "name": "totalAmount",
            "type": "u64"
          },
          {
            "name": "burnCount",
            "type": "u64"
          },
          {
            "name": "bump",
            "type": "u8"
          }
        ]
      }
    }
  ]
};
