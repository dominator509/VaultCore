import strings from "../i18n/en.json" with { type: "json" };
import { StatusMessage } from "./StatusMessage";

type VaultHealthProps = {
  chainValid: boolean | null;
  sessionId: string;
};

const auditHeadHash = "genesis";
const lastAuditAppend = "fixture audit append";

export function VaultHealth({ chainValid, sessionId }: VaultHealthProps) {
  const auditStatus = chainValid === false ? "invalid" : "valid";
  const alertKind = chainValid === false ? "error" : "success";
  const auditAlert =
    chainValid === false ? strings.auditAnomalyAlert : strings.auditAlert;

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
          <p>{auditStatus}</p>
          <p className="meta">{auditHeadHash}</p>
        </div>
        <div className="status-card">
          <h3>{strings.lastActivity}</h3>
          <p>{lastAuditAppend}</p>
        </div>
        <div className="status-card">
          <h3>{strings.activeSession}</h3>
          <p>{sessionId}</p>
        </div>
      </div>
      <div className="grid" aria-label={strings.localAlerts}>
        <StatusMessage kind="success">{strings.specAnchorAlert}</StatusMessage>
        <StatusMessage kind={alertKind}>{auditAlert}</StatusMessage>
        <StatusMessage kind="success">{strings.ipcAlert}</StatusMessage>
        <StatusMessage kind="success">{strings.authzAlert}</StatusMessage>
      </div>
    </section>
  );
}
