# SCHOLAR PAY
# ScholarPay 🎓

> Scholarship funds sent **directly to schools** on Stellar — students never touch the money.

![Stellar](https://img.shields.io/badge/Stellar-Soroban-blue?logo=stellar)
![Network](https://img.shields.io/badge/Network-Testnet-yellow)
![License](https://img.shields.io/badge/License-MIT-green)

---

## 🇵🇭 The Problem

A nursing student at UST Manila receives a ₱25,000 CHED scholarship stipend deposited into her GCash. Rent is due, her siblings need food money, and tuition is still 3 weeks away. She spends it. The scholarship body has no way to know — and no recourse. The student drops out at end of semester.

This is not rare. Scholarship leakage is a documented, widespread problem across Philippine state universities and CHED-funded programs.

---

## 💡 The Solution

**ScholarPay** holds scholarship funds inside a Soroban smart contract. When a grant is created, the destination is **permanently locked to the school's verified wallet** — not the student's. The student can trigger the payment, but they cannot redirect it. The money lands directly at the registrar's office.

> **The student never touches the money. The tuition always gets paid.**

---

## 🔄 How It Works

```
CHED / NGO (Admin)
    │
    ├── register_school(UST wallet)         → Whitelist verified school wallets
    ├── register_scholar(Maria, UST)        → Bind student to their school
    ├── create_grant(GRANT-001, Maria, ₱25,000, Sem1)
    │       └── school_wallet LOCKED to UST's wallet at creation
    │
    └── Funds deposited into contract

Maria (Student)
    │
    └── disburse(GRANT-001)
            └── Contract sends ₱25,000 → UST Registrar wallet
                Student wallet receives ₱0
```

---

## ⚙️ Stellar Features Used

| Feature | Purpose |
|---|---|
| **Soroban smart contracts** | Locks destination wallet at grant creation; enforces transfer rules |
| **token::Client (XLM / USDC)** | Direct on-chain transfer from contract to school — no intermediary |
| **Trustlines** | USDC support for stable-value grants (avoids XLM price volatility) |
| **Contract storage whitelisting** | Only registered school wallets can ever receive funds |

---

## 👥 Target Users

| Who | Where | Why they care |
|---|---|---|
| CHED / DepEd scholarship officers | Manila, Visayas, Mindanao | No more cash leakage, full audit trail |
| University registrars | Any LUC / SUC in PH | Guaranteed tuition receipt, no follow-up needed |
| Scholars (C/D bracket families) | Nationwide | Triggers disbursement, but can't misuse funds |
| NGOs (Ayala Foundation, SM Foundation) | Metro PH | Donors verify 100% of funds reached tuition |

---

## 🗂️ Project Structure

```
contracts/
└── scholar_pay/
    ├── Cargo.toml
    └── src/
        ├── lib.rs       ← smart contract
        └── test.rs      ← 5 tests
Cargo.toml               ← workspace
README.md
```

---

## 📋 Contract Functions

| Function | Who calls it | What it does |
|---|---|---|
| `initialize` | Admin (once) | Sets the admin wallet |
| `register_school` | Admin | Whitelists a school wallet |
| `register_scholar` | Admin | Binds a student to their school |
| `create_grant` | Admin | Creates a grant locked to the school wallet |
| `disburse` | Student or Admin | Sends funds directly to school, never student |
| `get_grant` | Anyone | Reads grant details and status |
| `get_scholar` | Anyone | Reads scholar info and total paid |
| `get_school` | Anyone | Reads school info |

---

## 🎬 MVP Demo Flow (90 seconds)

| Step | Who | Action | On-chain result |
|---|---|---|---|
| 1 | Admin | `register_school` | UST wallet whitelisted |
| 2 | Admin | `register_scholar` | Maria bound to UST |
| 3 | Admin | `create_grant` | Grant locked to UST wallet |
| 4 | Maria | `disburse` | ₱25,000 sent to UST, Maria gets ₱0 |
| 5 | Anyone | `get_grant` | Shows `disbursed: true` ✅ |

---

## 🛠️ Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (stable)
- `rustup target add wasm32-unknown-unknown`
- [Stellar CLI](https://developers.stellar.org/docs/tools/developer-tools/cli/install-stellar-cli) v21+
- [Freighter Wallet](https://freighter.app) (set to Testnet)
- Free testnet XLM from [Friendbot](https://friendbot.stellar.org)

---

## 🔧 Build

```bash
stellar contract build --manifest-path contracts/scholar_pay/Cargo.toml
```

Output: `target/wasm32v1-none/release/scholar_pay.wasm`

---

## 🧪 Test

```bash
cargo test
```

5 tests covering:
- ✅ Full happy path — funds go to school, not student
- ✅ Attacker cannot disburse another's grant
- ✅ State verification after disburse
- ✅ Double disburse blocked
- ✅ Scholar cannot be bound to unregistered school

---

## 🚀 Deploy to Testnet

**1. Generate and fund a key:**
```bash
stellar keys generate mykey --network testnet
stellar keys fund mykey --network testnet
```

**2. Upload the wasm:**
```bash
stellar contract upload \
  --wasm target/wasm32v1-none/release/scholar_pay.wasm \
  --source mykey \
  --network testnet
```

**3. Deploy:**
```bash
stellar contract deploy \
  --wasm target/wasm32v1-none/release/scholar_pay.wasm \
  --source mykey \
  --network testnet
```

Save the **Contract ID** returned — you'll need it for all invocations.

---

## 📞 Invoke Functions

**Initialize:**
```bash
stellar contract invoke --id CONTRACT_ID --source mykey --network testnet \
  -- initialize --admin $(stellar keys address mykey)
```

**Register a school:**
```bash
stellar contract invoke --id CONTRACT_ID --source mykey --network testnet \
  -- register_school \
  --caller $(stellar keys address mykey) \
  --school_wallet GUST_REGISTRAR_WALLET \
  --name "University of Santo Tomas"
```

**Register a scholar:**
```bash
stellar contract invoke --id CONTRACT_ID --source mykey --network testnet \
  -- register_scholar \
  --caller $(stellar keys address mykey) \
  --wallet GSTUDENT_WALLET \
  --school_id "UST-2024-0012" \
  --name "Maria Santos" \
  --school_wallet GUST_REGISTRAR_WALLET
```

**Create a grant (500 XLM):**
```bash
stellar contract invoke --id CONTRACT_ID --source mykey --network testnet \
  -- create_grant \
  --caller $(stellar keys address mykey) \
  --grant_id "GRANT-001" \
  --scholar_wallet GSTUDENT_WALLET \
  --amount 5000000000 \
  --semester "AY2024-2025 Sem1"
```

**Disburse — student triggers, money goes to school:**
```bash
stellar contract invoke --id CONTRACT_ID --source mykey --network testnet \
  -- disburse \
  --caller GSTUDENT_WALLET \
  --grant_id "GRANT-001" \
  --token_address CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC
```

**Verify — show judges this:**
```bash
stellar contract invoke --id CONTRACT_ID --network testnet \
  -- get_grant --grant_id "GRANT-001"
```

Or view live on: `https://stellar.expert/explorer/testnet/contract/CONTRACT_ID`

---

## 🏆 Why This Project

- **Real Philippine problem** — CHED/DepEd scholarship leakage is publicly documented
- **Enforced by code, not policy** — the contract makes misuse physically impossible
- **100% auditable** — every peso is traceable on Stellar Explorer
- **Near-zero fees** — Stellar makes even ₱500 micropayments viable
- **One-liner pitch**: *"A scholarship where the student can never spend the money on anything except school"*

---

## 🔮 Future Plans

- AI-powered enrollment verification before grant creation
- USDC stable grants via trustlines (no XLM volatility risk)
- Mobile app for students to track their grant status
- Multi-school support for nationwide CHED rollout

---

## 📄 License

MIT — Free to use, fork, and build on.

---

*Built for Stellar PH Bootcamp Hackathon 🇵🇭*

- Contract Address: CB2OUA6FKQOB5R5XHJUOLZDB4TN7DR7IS25WNI6LI3LDVGPYBY2P7CZV
  <img width="1920" height="953" alt="image" src="https://github.com/user-attachments/assets/72f4c5a0-a0c3-49e1-9458-09c5659605da" />
