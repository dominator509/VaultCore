import strings from "../i18n/en.json" with { type: "json" };

type Props = {
  kind: "notice" | "success" | "error" | "denied";
  children: React.ReactNode;
  code?: string;
};

export function StatusMessage({ kind, children, code }: Props) {
  const label =
    kind === "denied"
      ? strings.denied
      : kind === "error"
        ? strings.error
        : strings.success;
  return (
    <div
      className={kind}
      role={kind === "error" || kind === "denied" ? "alert" : "status"}
    >
      <strong>{label}</strong>
      {code ? (
        <span>
          {" "}
          {strings.code} {code}
        </span>
      ) : null}
      <p>{children}</p>
    </div>
  );
}
