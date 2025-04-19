# UCAN Playground

The **UCAN Playground** is a WebAssembly-based web application built with **Yew** and **Rust**, designed to create, delegate, verify, and publish UCAN (User-Controlled Authorization Network) tokens for demonstration purposes.

It provides a user friendly interface to interact with the UCAN protocol, allowing users to manage/visualize decentralized authorization tokens.

---

## ✨ Features

- **Create Root UCANs**: Generate UCAN tokens with a specified audience DID and scope.
- **Delegate UCANs**: Create delegated UCANs by referencing a parent UCAN's JWT.
- **Verify UCANs**: Validate UCAN tokens and display their capabilities.
- **Publish to Node**: Send UCAN tokens to a mock server endpoint.
- **Clear Tokens**: Removes all tokens from the UI and localStorage.

---

## 🧾 Create UCAN Form

### Inputs

- **Audience DID**: A valid `did:key` identifier  
  _(e.g., `did:key:z6MknSLrJoTcukLrE435hVNQT4JU3u9EZJzE4C2zN7KKbBZZ`)_
- **Scope**: The resource or capability (e.g., `publish/newsfeed`)
- **JWT**: A UCAN JWT for delegation, verification, or publishing

### Buttons

- `Create Root UCAN`: Generates a new UCAN token with a 1-day lifetime
- `Delegate UCAN`: Creates a delegated UCAN with a 12-hour lifetime
- `Verify UCAN`: Validates a JWT and displays its capabilities
- `Publish to Node`: Sends a JWT to the mock server (`http://localhost:3000/publish`)
- `Clear Tokens`: Removes all tokens from the UI and localStorage

---

## 🔗 Token Chain

Displays a list of created UCAN tokens, showing:

- **JWT**: Truncated for brevity
- **Issuer**: The application's `did:key` (base58-btc encoded)
- **Audience**: The recipient’s DID
- **Created**: Timestamp of token creation
- **Expiration**: Timestamp when the token expires

---

## 🌐 Example Workflow

### 1. Create a Root UCAN

**Input:**

- Audience DID: `did:key:z6MknSLrJoTcukLrE435hVNQT4JU3u9EZJzE4C2zN7KKbBZZ`
- Scope: `publish/newsfeed`

**Action:**
- Click **"Create Root UCAN"**

**Result:**
- A new token appears in the Token Chain
- Console log: `Created root UCAN`

---

### 2. Delegate a UCAN

**Input:**

- Copy a JWT from the Token Chain
- Audience DID: same or another valid DID
- Scope: `publish/topic2`
- JWT: paste copied JWT

**Action:**
- Click **"Delegate UCAN"**

**Result:**
- A delegated token appears
- Console log: `Created delegated UCAN`

---

### 3. Verify a UCAN

**Input:**
- Paste a JWT into the JWT field

**Action:**
- Click **"Verify UCAN"**

**Result:**
- Success message (e.g., `Verified UCAN with 1 capabilities`)
- Console log: `Verified UCAN`

---

### 4. Publish to Node

**Input:**
- Paste a JWT into the JWT field

**Action:**
- Click **"Publish to Node"**

**Result:**
- Success message (e.g., `Published: {}`)
- Console log: `Published to node successfully`

---

## 🔧 Prerequisites

- **Rust**: Install via [rustup](https://rustup.rs) (recommended: `1.81.0+`)
- **Trunk**: WebAssembly build tool. Install with:
  `cargo install trunk`
- **Node.js**: Recommended version: `18+`

---

## 🛠️ Setup Instructions

1. **Clone the Repository**

```bash
git clone https://github.com/publiish/ucan-playground.git
cd ucan-playground
```

2. **Install Mock Server**
```bash
npm install -g json-server
```

3. **Create `db.json` for Mock Server**
```bash
  echo '{"publish": {}}' > db.json
```
`
  {
    "publiish": []
  }
`

4. **Create a cors.js file to handle CORS**
```
module.exports = (req, res, next) => {
    res.header('Access-Control-Allow-Origin', '*');
    res.header('Access-Control-Allow-Methods', 'GET, POST, PUT, DELETE');
    res.header('Access-Control-Allow-Headers', 'Authorization, Content-Type');
    next();
};
```

5. **Run the Mock Server**

```bash
   json-server --watch db.json --port 3000
```

```bash
   json-server --watch db.json --middlewares ./cors.js
```

5. **Build and Serve with Trunk**
```bash
   trunk serve --open
```

```bash
   trunk serve --port 8081
```

```bash
   trunk build --release
```

6. **Build and run the Project with verbose output**
```bash
   cargo build --target=wasm32-unknown-unknown --verbose
``` 
---
