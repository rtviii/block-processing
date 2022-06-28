https://solscan.io/tx/A3Qa57MkJGSVRK31LXLgkuwRCtv4MBJjNWjtLPZo1uDz7kTtKPDvBuYtXngQ7AWTicYYpabfcfLZyng6Au1MuBG


- The general question being if my grasp of this is correct?

```json
{
                "message": {
                    "accountKeys": [
                        "6aCna9ZopJJUuTijkuKLmd57tnMco8KQBH7J8ydCjT2r", // signed   r/w
                        "EfKB2E4kYinooGF4BFMWXgS2gZLFeBDQ2hffo9LSSN9V", // signed   r
                        "ChigE9pK6g4UW3skQnKFAwyGETLzEcS2RYDep77XzmJt", // unsigned r/w
                        "SysvarS1otHashes111111111111111111111111111",  // unsigned r
                        "SysvarC1ock11111111111111111111111111111111",  // unsigned r
                        "Vote111111111111111111111111111111111111111"   // unsigned r
                    ],
                    "header": {
                        "numReadonlySignedAccounts"  : 1,   // <-- of those requiring signatures
                        "numReadonlyUnsignedAccounts": 3,   // <-- last
                        "numRequiredSignatures"      : 2    // <-- first
                    },
                    "instructions": [
                        {
                            "accounts": [
                                2,
                                3,
                                4,
                                1
                            ],
                            "data": "rTDbDtm67JPw9Wo3Q1CoZmTfRQYxDy9k4CaEL5AkEHtF1n7MmYpitzHRci5kiVDDNfQ95xjiwnEYoWyVNe5zvfKYesZjvVcAB7SB4nR5",
                            "programIdIndex": 5
                        }
                    ],
                    "recentBlockhash": "FfVd6vYQjLNE7GAeoFJtaJYbkWYjR5qVBz3pgyNgTCLy"
                },
                "signatures": [
                    "A3Qa57MkJGSVRK31LXLgkuwRCtv4MBJjNWjtLPZo1uDz7kTtKPDvBuYtXngQ7AWTicYYpabfcfLZyng6Au1MuBG",
                    "2ULwuU8MQcyhhydtY2QianRXRpxVMU8ZWus9xmLefatp35NyxKN1Ziubg1npczniaoMSViigbxyJwzt7YGwER59P"
                ]
            }
```

            // TODO: Whether balances have changed: tally the difference in meta tag