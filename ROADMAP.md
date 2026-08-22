## Phased Plan:
### Phase 1 – MVP / v0.1
#### UI / Messaging
- Implement a basic IRC-style chat log for text exchange.
- Display both sent and received messages with clear separation (timestamps + sender tag).
- MVP goal: functionality > polish. In later versions, this feature may evolve into a modern chat-like UI.

#### Networking / Security
- All communication must be encrypted end-to-end.
- Accept connections only if both sides can establish a secure TLS handshake.

#### General MVP Scope
- Only manual IP entry for connections (no discovery yet).
- Text-only transfer (no files).
- Focus on functional correctness (connect, send, receive securely) over UI polish.

### Phase 2 — v1.x
#### File Transfer
- Extend the text transfer protocol to handle file transfer via chunking.
- MVP requires only basic chunked transfer (no resume/retry).
- Future: add resumable transfers, integrity checks, and streaming optimizations, multithreading.

#### QR Code Sharing
- Generate a QR code containing a “config” blob.
- MVP includes at minimum:
- IPv4 or IPv6 address
- Public key (if using keypairs for security)
- Reserve structure for future fields:
    - Required plugins
    - Minimum version numbers
    - Metadata (device name, model, OS)
    - Port number and additional options

#### Device Discovery
- Implement passive discovery via broadcast/multicast.
- Listen for discovery packets.
- Occasionally announce presence.
- Future: support active scanning (subnet sweep) as optional mode.

#### Transfer Feedback
- Show progress bar with speed and ETA during transfers.
- MVP may use basic text-based indicators (e.g., % complete, KB/s).
- Later: improve visuals for better UX.

### Phase 3 v2.x
#### Plugin System
- Implement a true plugin ecosystem with dynamic loading (.so, .dll, .dylib).
- Each plugin requires a config.yaml file with:
- Plugin name + description
- Version
- Dependencies (if any)
- Requirement type:
    - Both peers required (e.g., compression)
    - One-sided optional (e.g., UI mods)
- Core app loads plugins at runtime, checks configs, and negotiates compatibility.
- Future: add sandboxing or permission controls for security.

#### Configurable Settings
- Provide a settings UI for enabling/disabling available plugins.
- Allow peers to auto-negotiate plugin compatibility before establishing connection.

#### UI Polish
- Refine interface to match the dark theme palette (#070707 background + chosen accents).
- Add polish: rounded corners, padding, shadows, animations where useful.
- Future-proof: allow integration with external design docs (Figma, etc.)

