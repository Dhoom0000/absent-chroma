Absolutely! Here's a **detailed Markdown checklist** for implementing secure KEM-based key exchange in a **Bevy + Renet** server-client setup — covering **every little step**, including function creation, system registration, and message flow.

---

````md
## 🔐 Bevy Renet Secure KEM Handshake (Server Side) - Full Setup Checklist

### 📦 Setup and Imports
- [x] Add dependencies:
  - `bevy_renet`
  - `fips203`
  - `bincode`
  - `hkdf`
  - `sha2`
- [x] Add necessary imports:
  ```rust
  use std::{collections::HashMap, net::{SocketAddr, UdpSocket}, time::{Duration, SystemTime}};
  use bevy::prelude::*;
  use bevy_renet::{renet::*, netcode::*};
  use fips203::ml_kem_512::*;
  use fips203::traits::*;
  use local_ip_address::local_ip;
````

---

### 🛠 Step 1: Define Resources and Constants

* [x] Define protocol ID constant:

  ```rust
  pub const PROTOCOL_ID: u64 = 7;
  ```

* [x] Define `KEMServerState`:

  ```rust
  #[derive(Resource)]
  pub struct KEMServerState {
      pub decaps_key: DecapsKey,
      pub shared_secrets: HashMap<u64, SharedSecretKey>, // client_id -> ssk
  }
  ```

---

### 🛠 Step 2: Setup Custom Channel for Handshake

* [x] Add handshake channel to `ConnectionConfig`:

  ```rust
  let handshake_channel = ChannelConfig {
      channel_id: 7,
      send_type: SendType::ReliableOrdered { resend_time: Duration::from_millis(300) },
      max_memory_usage_bytes: 5 * 1024 * 1024,
  };
  ```

---

### 🛠 Step 3: Create Key Pair in Startup System

* [x] Create `setup_kem_keys` system:

  ```rust
  fn setup_kem_keys(mut commands: Commands) {
      let mut d_buf = [0u8; 32];
      let mut z_buf = [0u8; 32];
      getrandom::fill(&mut d_buf).unwrap();
      getrandom::fill(&mut z_buf).unwrap();

      let (_, decaps_key) = KG::keygen_from_seed(d_buf, z_buf);
      commands.insert_resource(KEMServerState {
          decaps_key,
          shared_secrets: HashMap::new(),
      });
  }
  ```

---

### 🛠 Step 4: Send `encaps_key` When Client Connects

* [x] Create system `send_kem_key_on_connect`:

  ```rust
  fn send_kem_key_on_connect(
      mut events: EventReader<ServerEvent>,
      mut server: ResMut<RenetServer>,
      kem_state: Res<KEMServerState>,
  ) {
      for event in events.read() {
          if let ServerEvent::ClientConnected { client_id } = event {
              let public_key_bytes = kem_state.decaps_key.public().into_bytes();
              let msg = bincode::encode_to_vec::<[u8; 800], _>(public_key_bytes, bincode::config::standard()).unwrap();
              server.send_message(*client_id, 7, msg);
          }
      }
  }
  ```

---

### 🛠 Step 5: Receive Client Ciphertext and Derive Shared Secret

* [x] Create system `receive_secure_cipher`:

  ```rust
  fn receive_secure_cipher(
      mut server: ResMut<RenetServer>,
      mut kem_state: ResMut<KEMServerState>,
  ) {
      for client_id in server.clients_id().iter() {
          while let Some(cipher_msg) = server.receive_message(*client_id, 7) {
              let (raw_cipher, _) = bincode::decode_from_slice::<[u8; 768], _>(&cipher_msg, bincode::config::standard()).unwrap();
              let ciphertext = CipherText::try_from_bytes(raw_cipher).unwrap();
              let ssk = kem_state.decaps_key.try_decaps(&ciphertext).unwrap();
              kem_state.shared_secrets.insert(*client_id, ssk);
              info!("Secure key established for client {client_id}");
          }
      }
  }
  ```

---

### 🛠 Step 6: Optional – Derive `[u8; 32]` Key with HKDF

* [x] Add helper function:

  ```rust
  fn derive_key(ssk: &SharedSecretKey) -> [u8; 32] {
      use hkdf::Hkdf;
      use sha2::Sha256;

      let hk = Hkdf::<Sha256>::new(None, ssk.as_bytes());
      let mut key = [0u8; 32];
      hk.expand(b"renet-kem-handshake", &mut key).unwrap();
      key
  }
  ```

---

### ⚙️ Step 7: Register All Systems in App

* [x] In `App::build()` or `App::new()`:

  ```rust
  .add_startup_system(setup_kem_keys)
  .add_system(send_kem_key_on_connect)
  .add_system(receive_secure_cipher)
  ```

---

### 🛠 Step 8: Server Setup System

* [x] Create `create_renet_server` system:

  ```rust
  fn create_renet_server(mut commands: Commands) {
      let server = RenetServer::new(ConnectionConfig {
          server_channels_config: vec![handshake_channel.clone()],
          client_channels_config: vec![handshake_channel],
          ..Default::default()
      });
      commands.insert_resource(server);

      let server_addr = SocketAddr::new(local_ip().unwrap(), 42069);
      let socket = UdpSocket::bind(server_addr).unwrap();
      let server_config = ServerConfig {
          max_clients: 64,
          protocol_id: PROTOCOL_ID,
          public_addresses: vec![server_addr],
          authentication: ServerAuthentication::Unsecure,
          current_time: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap(),
      };

      let transport = NetcodeServerTransport::new(server_config, socket).unwrap();
      commands.insert_resource(transport);
  }
  ```

---

### ✅ Bonus: Add Ping Listener (Optional)

* [x] Add system `receive_ping` for testing:

  ```rust
  fn receive_ping(mut server: ResMut<RenetServer>) {
      for client_id in server.clients_id().iter() {
          while let Some(msg) = server.receive_message(*client_id, DefaultChannel::ReliableOrdered) {
              // Handle messages
          }
      }
  }
  ```

---

## 🎯 Result

Once all systems are in place:

* Clients receive the server's public KEM key.
* They respond with ciphertext using `try_encaps()`.
* Server uses `try_decaps()` to derive a secure `ssk`.
* Future messages can be encrypted with a symmetric key derived from `ssk`.

---

### ✅ Want a Client Checklist?

Let me know and I’ll give you a matching client-side list!

```
```
