// Copyright 2018-2021 the Deno authors. All rights reserved. MIT license.
"use strict";

((window) => {
  const core = window.Deno.core;
  const { Listener, Conn } = window.__bootstrap.net;

  function opConnectTls(
    args,
  ) {
    return core.opAsync("op_connect_tls", args);
  }

  function opAcceptTLS(rid) {
    return core.opAsync("op_accept_tls", rid);
  }

  function opListenTls(args) {
    return core.opSync("op_listen_tls", args);
  }

  function opStartTls(args) {
    return core.opAsync("op_start_tls", args);
  }

  function opTlsHandshake(rid) {
    return core.opAsync("op_tls_handshake", rid);
  }

  class TlsConn extends Conn {
    handshake() {
      return opTlsHandshake(this.rid);
    }
  }

  async function connectTls({
    port,
    hostname = "127.0.0.1",
    transport = "tcp",
    certFile = undefined,
    caCerts = [],
    certChain = undefined,
    privateKey = undefined,
  }) {
    const res = await opConnectTls({
      port,
      hostname,
      transport,
      certFile,
      caCerts,
      certChain,
      privateKey,
    });
    return new TlsConn(res.rid, res.remoteAddr, res.localAddr);
  }

  class TlsListener extends Listener {
    async accept() {
      const res = await opAcceptTLS(this.rid);
      return new TlsConn(res.rid, res.remoteAddr, res.localAddr);
    }
  }

  function listenTls({
    port,
    certFile,
    keyFile,
    hostname = "0.0.0.0",
    transport = "tcp",
    alpnProtocols,
  }) {
    const res = opListenTls({
      port,
      certFile,
      keyFile,
      hostname,
      transport,
      alpnProtocols,
    });
    return new TlsListener(res.rid, res.localAddr);
  }

  async function startTls(
    conn,
    { hostname = "127.0.0.1", certFile = undefined, caCerts = [] } = {},
  ) {
    const res = await opStartTls({
      rid: conn.rid,
      hostname,
      certFile,
      caCerts,
    });
    return new TlsConn(res.rid, res.remoteAddr, res.localAddr);
  }

  window.__bootstrap.tls = {
    startTls,
    listenTls,
    connectTls,
    TlsConn,
    TlsListener,
  };
})(this);
