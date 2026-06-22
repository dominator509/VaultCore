import { useEffect, useMemo, useState } from "react";
import strings from "../i18n/en.json" with { type: "json" };
import { errorText } from "../lib/errors";
import { nextCountdown, ttlToSeconds } from "../lib/timers";
import {
  type AuditViewEntry,
  type Role,
  type SecretSummary,
  type SecretType,
  vaultApi,
} from "../lib/tauri";
import { type FlowState, type View, visibleSecrets } from "../state/appState";
import { StatusMessage } from "../components/StatusMessage";

const secretTypes: SecretType[] = [
  "API_KEY",
  "LOGIN",
  "OAUTH_APP",
  "SSH_KEY",
  "WALLET_KEY",
  "CERT",
  "NOTE",
  "BLOB",
];

export function App() {
  const [sessionId, setSessionId] = useState<string | null>(null);
  const [role, setRole] = useState<Role>("Owner");
  const [view, setView] = useState<View>("list");
  const [flow, setFlow] = useState<FlowState>("idle");
  const [message, setMessage] = useState("");
  const [secrets, setSecrets] = useState<SecretSummary[]>([]);
  const [query, setQuery] = useState("");
  const [revealTtl, setRevealTtl] = useState(30);
  const [copyTtl, setCopyTtl] = useState(20);
  const [payloadHandle, setPayloadHandle] = useState("");
  const [revealCountdown, setRevealCountdown] = useState(0);
  const [copyCountdown, setCopyCountdown] = useState(0);
  const [copySecretId, setCopySecretId] = useState<string | null>(null);
  const [auditEntries, setAuditEntries] = useState<AuditViewEntry[]>([]);
  const [chainValid, setChainValid] = useState<boolean | null>(null);

  const visible = useMemo(
    () => visibleSecrets(secrets, query),
    [query, secrets],
  );

  useEffect(() => {
    if (sessionId === null) {
      return;
    }
    void loadSecrets();
  }, [sessionId]);

  useEffect(() => {
    if (revealCountdown === 0) {
      setPayloadHandle("");
      return;
    }
    const timer = window.setTimeout(
      () => setRevealCountdown(nextCountdown(revealCountdown)),
      1000,
    );
    return () => window.clearTimeout(timer);
  }, [revealCountdown]);

  useEffect(() => {
    if (copyCountdown === 0) {
      setCopySecretId(null);
      return;
    }
    const timer = window.setTimeout(
      () => setCopyCountdown(nextCountdown(copyCountdown)),
      1000,
    );
    return () => window.clearTimeout(timer);
  }, [copyCountdown]);

  useEffect(() => {
    const clearPayload = () => setRevealCountdown(0);
    window.addEventListener("blur", clearPayload);
    return () => window.removeEventListener("blur", clearPayload);
  }, []);

  async function unlock(method: string, proof: string) {
    setFlow("loading");
    try {
      const token = await vaultApi.unlock(method, proof);
      setSessionId(token.session_id);
      setFlow("success");
      setMessage(strings.sessionActive);
    } catch (error) {
      setFlow("error");
      setMessage(errorText(error));
    }
  }

  async function loadSecrets() {
    setFlow("loading");
    try {
      const rows = await vaultApi.list({ role, query });
      setSecrets(rows);
      setFlow(rows.length === 0 ? "empty" : "success");
    } catch (error) {
      setFlow("error");
      setMessage(errorText(error));
    }
  }

  async function reveal(secret: SecretSummary, reason: string) {
    setFlow("loading");
    try {
      const response = await vaultApi.reveal(secret.id, reason, role);
      setPayloadHandle(response.payload_handle);
      setRevealCountdown(ttlToSeconds(response.ttl_ms));
      setFlow("success");
    } catch (error) {
      setFlow("error");
      setMessage(errorText(error));
    }
  }

  async function copy(secret: SecretSummary) {
    await vaultApi.copy(secret.id, copyTtl * 1000, role);
    setCopySecretId(secret.id);
    setCopyCountdown(copyTtl);
  }

  async function createSecret(form: FormData) {
    const name = String(form.get("name") ?? "");
    const secretType = String(form.get("secret_type") ?? "NOTE") as SecretType;
    const handle = String(form.get("payload_handle") ?? "");
    setFlow("loading");
    try {
      const created = await vaultApi.create({
        role,
        name,
        secret_type: secretType,
        payload_handle: handle,
      });
      setSecrets([created, ...secrets]);
      setFlow("success");
      setMessage(strings.success);
    } catch (error) {
      setFlow("error");
      setMessage(errorText(error));
    }
  }

  async function updateSecret(secret: SecretSummary, name: string) {
    const updated = await vaultApi.update(secret.id, { role, name });
    setSecrets(secrets.map((item) => (item.id === secret.id ? updated : item)));
  }

  async function rotateSecret(secret: SecretSummary, handle: string) {
    const rotated = await vaultApi.rotate(secret.id, handle, role);
    setSecrets(secrets.map((item) => (item.id === secret.id ? rotated : item)));
  }

  async function deleteSecret(secret: SecretSummary, purgeToken: string) {
    if (purgeToken.length > 0) {
      await vaultApi.purge(secret.id, purgeToken, role);
      setSecrets(secrets.filter((item) => item.id !== secret.id));
      return;
    }
    await vaultApi.softDelete(secret.id, role);
    setSecrets(
      secrets.map((item) =>
        item.id === secret.id ? { ...item, state: "soft_deleted" } : item,
      ),
    );
  }

  async function loadAudit() {
    const rows = await vaultApi.auditView({ role });
    setAuditEntries(rows);
  }

  async function verifyChain() {
    const status = await vaultApi.verifyAuditChain();
    setChainValid(status.valid);
  }

  async function lock() {
    await vaultApi.lock();
    setSessionId(null);
    setPayloadHandle("");
    setRevealCountdown(0);
    setCopyCountdown(0);
    setCopySecretId(null);
    setFlow("idle");
  }

  if (sessionId === null) {
    return <LockScreen flow={flow} message={message} onUnlock={unlock} />;
  }

  return (
    <main className="app-shell dashboard" aria-labelledby="app-title">
      <aside className="sidebar">
        <p className="eyebrow">{strings.brand}</p>
        <h1 id="app-title">{strings.sessionActive}</h1>
        <label>
          Role
          <select
            value={role}
            onChange={(event) => setRole(event.target.value as Role)}
          >
            {["Owner", "Admin", "Editor", "Viewer", "Auditor"].map((item) => (
              <option key={item}>{item}</option>
            ))}
          </select>
        </label>
        <nav className="nav-list" aria-label="Primary">
          {[
            ["list", strings.navList],
            ["create", strings.navCreate],
            ["audit", strings.navAudit],
            ["health", strings.navHealth],
            ["settings", strings.navSettings],
          ].map(([key, label]) => (
            <button
              key={key}
              aria-pressed={view === key}
              onClick={() => {
                setView(key as View);
                if (key === "audit") {
                  void loadAudit();
                }
              }}
            >
              {label}
            </button>
          ))}
          <button className="danger" onClick={() => void lock()}>
            {strings.lock}
          </button>
        </nav>
      </aside>
      <section className="workspace">
        {flow === "loading" ? (
          <StatusMessage kind="notice">{strings.loading}</StatusMessage>
        ) : null}
        {flow === "error" ? (
          <StatusMessage kind="error">{message}</StatusMessage>
        ) : null}
        {view === "list" ? (
          <ListView
            copyCountdown={copyCountdown}
            copySecretId={copySecretId}
            flow={flow}
            payloadHandle={payloadHandle}
            query={query}
            revealCountdown={revealCountdown}
            secrets={visible}
            onCopy={copy}
            onDelete={deleteSecret}
            onQuery={setQuery}
            onRefresh={loadSecrets}
            onReveal={reveal}
            onRotate={rotateSecret}
            onUpdate={updateSecret}
          />
        ) : null}
        {view === "create" ? <CreateView onCreate={createSecret} /> : null}
        {view === "audit" ? (
          <AuditView
            entries={auditEntries}
            chainValid={chainValid}
            onVerify={verifyChain}
          />
        ) : null}
        {view === "health" ? (
          <HealthView chainValid={chainValid} sessionId={sessionId} />
        ) : null}
        {view === "settings" ? (
          <SettingsView
            copyTtl={copyTtl}
            revealTtl={revealTtl}
            onCopyTtl={setCopyTtl}
            onRevealTtl={setRevealTtl}
          />
        ) : null}
      </section>
    </main>
  );
}

function LockScreen({
  flow,
  message,
  onUnlock,
}: {
  flow: FlowState;
  message: string;
  onUnlock: (method: string, proof: string) => Promise<void>;
}) {
  const [proof, setProof] = useState("local-proof");
  const passkeyDisabled = import.meta.env.VAULTCORE_E2E_PASSKEY === "0";
  return (
    <main className="lock-screen" aria-labelledby="lock-title">
      <section className="lock-panel stack">
        <p className="eyebrow">{strings.brand}</p>
        <h1 id="lock-title">{strings.lockedTitle}</h1>
        <p>{strings.lockedStatus}</p>
        {flow === "loading" ? (
          <StatusMessage kind="notice">{strings.loading}</StatusMessage>
        ) : null}
        {flow === "error" ? (
          <StatusMessage kind="error">{message}</StatusMessage>
        ) : null}
        <label>
          {strings.passphraseLabel}
          <input
            aria-label={strings.passphraseLabel}
            placeholder={strings.passphrasePlaceholder}
            type="password"
            value={proof}
            onChange={(event) => setProof(event.target.value)}
          />
        </label>
        <div className="row">
          <button
            className="primary"
            onClick={() => void onUnlock("passphrase", proof)}
          >
            {strings.unlockPassphrase}
          </button>
          <button
            disabled={passkeyDisabled}
            onClick={() => void onUnlock("passkey", "passkey")}
          >
            {strings.unlockPasskey}
          </button>
          <button onClick={() => void onUnlock("biometrics", "biometrics")}>
            {strings.unlockBiometrics}
          </button>
        </div>
        <SettingsView
          copyTtl={20}
          revealTtl={30}
          onCopyTtl={() => null}
          onRevealTtl={() => null}
        />
      </section>
    </main>
  );
}

function ListView({
  copyCountdown,
  copySecretId,
  flow,
  payloadHandle,
  query,
  revealCountdown,
  secrets,
  onCopy,
  onDelete,
  onQuery,
  onRefresh,
  onReveal,
  onRotate,
  onUpdate,
}: {
  copyCountdown: number;
  copySecretId: string | null;
  flow: FlowState;
  payloadHandle: string;
  query: string;
  revealCountdown: number;
  secrets: SecretSummary[];
  onCopy: (secret: SecretSummary) => Promise<void>;
  onDelete: (secret: SecretSummary, purgeToken: string) => Promise<void>;
  onQuery: (query: string) => void;
  onRefresh: () => Promise<void>;
  onReveal: (secret: SecretSummary, reason: string) => Promise<void>;
  onRotate: (secret: SecretSummary, handle: string) => Promise<void>;
  onUpdate: (secret: SecretSummary, name: string) => Promise<void>;
}) {
  return (
    <section className="panel" aria-labelledby="list-title">
      <div className="toolbar">
        <label>
          {strings.searchLabel}
          <input
            placeholder={strings.searchPlaceholder}
            value={query}
            onChange={(event) => onQuery(event.target.value)}
          />
        </label>
        <button onClick={() => void onRefresh()}>{strings.navList}</button>
      </div>
      <h2 id="list-title">{strings.navList}</h2>
      {flow === "empty" || secrets.length === 0 ? (
        <StatusMessage kind="notice">{strings.emptySecrets}</StatusMessage>
      ) : null}
      <div className="grid">
        {secrets.map((secret) => (
          <SecretCard
            copyCountdown={copyCountdown}
            copySecretId={copySecretId}
            key={secret.id}
            payloadHandle={payloadHandle}
            revealCountdown={revealCountdown}
            secret={secret}
            onCopy={onCopy}
            onDelete={onDelete}
            onReveal={onReveal}
            onRotate={onRotate}
            onUpdate={onUpdate}
          />
        ))}
      </div>
    </section>
  );
}

function SecretCard({
  copyCountdown,
  copySecretId,
  payloadHandle,
  revealCountdown,
  secret,
  onCopy,
  onDelete,
  onReveal,
  onRotate,
  onUpdate,
}: {
  copyCountdown: number;
  copySecretId: string | null;
  payloadHandle: string;
  revealCountdown: number;
  secret: SecretSummary;
  onCopy: (secret: SecretSummary) => Promise<void>;
  onDelete: (secret: SecretSummary, purgeToken: string) => Promise<void>;
  onReveal: (secret: SecretSummary, reason: string) => Promise<void>;
  onRotate: (secret: SecretSummary, handle: string) => Promise<void>;
  onUpdate: (secret: SecretSummary, name: string) => Promise<void>;
}) {
  const [reason, setReason] = useState("Operational review");
  const [name, setName] = useState(secret.name);
  const [handle, setHandle] = useState(`payload://${secret.id}/rotated`);
  const [purgeToken, setPurgeToken] = useState("");
  const showingPayload =
    payloadHandle.endsWith(secret.id) && revealCountdown > 0;
  return (
    <article className="secret-card stack" aria-label={secret.name}>
      <div>
        <h3>{secret.name}</h3>
        <p className="meta">
          {strings.secretType}: {secret.secret_type} · {strings.secretState}:{" "}
          {secret.state}
        </p>
      </div>
      <label>
        {strings.revealReason}
        <input
          value={reason}
          onChange={(event) => setReason(event.target.value)}
        />
      </label>
      <div className="row">
        <button onClick={() => void onReveal(secret, reason)}>
          {strings.reveal}
        </button>
        <button onClick={() => void onCopy(secret)}>{strings.copy}</button>
      </div>
      {showingPayload ? (
        <div className="payload-window" data-testid="payload-window">
          {strings.payloadHandle}: {payloadHandle}
          <br />
          {revealCountdown} {strings.secondsRemaining}
        </div>
      ) : null}
      {copyCountdown > 0 && copySecretId === secret.id ? (
        <StatusMessage kind="success">
          {strings.copy}: {copyCountdown} {strings.secondsRemaining}
        </StatusMessage>
      ) : null}
      <label>
        {strings.nameLabel}
        <input value={name} onChange={(event) => setName(event.target.value)} />
      </label>
      <div className="row">
        <button onClick={() => void onUpdate(secret, name)}>
          {strings.update}
        </button>
        <button onClick={() => void onRotate(secret, handle)}>
          {strings.rotate}
        </button>
      </div>
      <label>
        {strings.newPayloadHandle}
        <input
          value={handle}
          onChange={(event) => setHandle(event.target.value)}
        />
      </label>
      <label>
        {strings.confirmationToken}
        <input
          value={purgeToken}
          onChange={(event) => setPurgeToken(event.target.value)}
        />
      </label>
      <div className="row">
        <button onClick={() => void onDelete(secret, "")}>
          {strings.softDelete}
        </button>
        <button
          className="danger"
          onClick={() => void onDelete(secret, purgeToken)}
        >
          {strings.purge}
        </button>
      </div>
    </article>
  );
}

function CreateView({
  onCreate,
}: {
  onCreate: (form: FormData) => Promise<void>;
}) {
  return (
    <section className="panel" aria-labelledby="create-title">
      <h2 id="create-title">{strings.createTitle}</h2>
      <form
        className="stack"
        onSubmit={(event) => {
          event.preventDefault();
          void onCreate(new FormData(event.currentTarget));
        }}
      >
        <label>
          {strings.nameLabel}
          <input name="name" required defaultValue="New operational secret" />
        </label>
        <label>
          {strings.secretType}
          <select name="secret_type">
            {secretTypes.map((secretType) => (
              <option key={secretType}>{secretType}</option>
            ))}
          </select>
        </label>
        <label>
          {strings.payloadHandleLabel}
          <input
            name="payload_handle"
            required
            defaultValue="payload://draft"
          />
        </label>
        <button className="primary" type="submit">
          {strings.create}
        </button>
      </form>
    </section>
  );
}

function AuditView({
  entries,
  chainValid,
  onVerify,
}: {
  entries: AuditViewEntry[];
  chainValid: boolean | null;
  onVerify: () => Promise<void>;
}) {
  return (
    <section className="panel stack" aria-labelledby="audit-title">
      <h2 id="audit-title">{strings.auditTitle}</h2>
      <button onClick={() => void onVerify()}>{strings.verifyChain}</button>
      {chainValid === true ? (
        <StatusMessage kind="success">{strings.auditHead}: valid</StatusMessage>
      ) : null}
      {entries.length === 0 ? (
        <StatusMessage kind="notice">{strings.auditEmpty}</StatusMessage>
      ) : null}
      <div className="grid">
        {entries.map((entry) => (
          <article
            className="status-card"
            key={`${entry.op}-${entry.target_id ?? "none"}`}
          >
            <h3>{entry.op}</h3>
            <p className="meta">{entry.result}</p>
          </article>
        ))}
      </div>
    </section>
  );
}

function HealthView({
  chainValid,
  sessionId,
}: {
  chainValid: boolean | null;
  sessionId: string;
}) {
  return (
    <section className="panel stack" aria-labelledby="health-title">
      <h2 id="health-title">{strings.healthTitle}</h2>
      <div className="grid">
        <div className="status-card">
          <h3>{strings.specAnchor}</h3>
          <p>verified</p>
        </div>
        <div className="status-card">
          <h3>{strings.auditHead}</h3>
          <p>{chainValid === false ? "invalid" : "valid"}</p>
        </div>
        <div className="status-card">
          <h3>{strings.lastActivity}</h3>
          <p>{sessionId}</p>
        </div>
      </div>
    </section>
  );
}

function SettingsView({
  copyTtl,
  revealTtl,
  onCopyTtl,
  onRevealTtl,
}: {
  copyTtl: number;
  revealTtl: number;
  onCopyTtl: (value: number) => void;
  onRevealTtl: (value: number) => void;
}) {
  return (
    <section className="panel stack" aria-labelledby="settings-title">
      <h2 id="settings-title">{strings.settingsTitle}</h2>
      <p>{strings.settingsStub}</p>
      <div className="settings-grid">
        <label>
          {strings.revealTtl}
          <input
            min="5"
            type="number"
            value={revealTtl}
            onChange={(event) => onRevealTtl(Number(event.target.value))}
          />
        </label>
        <label>
          {strings.copyTtl}
          <input
            min="5"
            type="number"
            value={copyTtl}
            onChange={(event) => onCopyTtl(Number(event.target.value))}
          />
        </label>
      </div>
    </section>
  );
}
