// Copyright 2020, The Tari Project
//
// Redistribution and use in source and binary forms, with or without modification, are permitted provided that the
// following conditions are met:
//
// 1. Redistributions of source code must retain the above copyright notice, this list of conditions and the following
// disclaimer.
//
// 2. Redistributions in binary form must reproduce the above copyright notice, this list of conditions and the
// following disclaimer in the documentation and/or other materials provided with the distribution.
//
// 3. Neither the name of the copyright holder nor the names of its contributors may be used to endorse or promote
// products derived from this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES,
// INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
// DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
// SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY,
// WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE
// USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

use tari_comms::{multiaddr::Multiaddr, socks, tor};

#[derive(Debug, Clone)]
pub enum TransportType {
    /// Use a memory transport. This transport recognises /memory addresses primarily used for local testing.
    Memory { listener_address: Multiaddr },
    /// Use a TcpTransport. This transport recognises /ip{4|6}/.../tcp/xxxx addresses.
    Tcp { listener_address: Multiaddr },
    /// This does not directly map to a transport, but will configure comms to run over a tor hidden service using the
    /// Tor proxy. This transport recognises ip/tcp, onion v2, onion v3 and dns addresses.
    Tor(TorConfig),
    /// Use a SOCKS5 proxy transport. This transport recognises ip/tcp and dns addresses.
    Socks {
        proxy_address: Multiaddr,
        listener_address: Multiaddr,
        authentication: socks::Authentication,
    },
}

#[derive(Debug, Clone)]
pub struct TorConfig {
    /// The Tor control server address
    pub control_server_addr: Multiaddr,
    /// Authentication for the Tor control server
    pub control_server_auth: tor::Authentication,
    /// The private key for the Tor hidden service. If not supplied, a new address and private key will be generated
    pub private_key: Option<Box<tor::PrivateKey>>,
    /// The onion -> local address mapping to use.
    pub port_mapping: tor::PortMapping,
    /// Authentication for the Tor SOCKS5 proxy
    pub socks_auth: socks::Authentication,
}